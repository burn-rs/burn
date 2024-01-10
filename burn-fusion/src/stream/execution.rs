use super::{StreamDescription, TensorOpsDescription};
use crate::stream::optim::{
    Condition, OptimizationAnalysis, OptimizationAnalyzer, StreamOptimizations,
};
use crate::{FusionBackend, HandleContainer, OptimizationBuilder, OptimizationStatus};

/// Execute an optimization following a greedy algorithm.
pub(crate) struct StreamExecutor<B: FusionBackend> {
    analyzer: OptimizationAnalyzer<B::Optimization>,
    builders: Vec<Box<dyn OptimizationBuilder<B>>>,
    num_skipped: usize,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum ExecutionMode {
    // Signal that we execute the graph after a new ops is added to the graph.
    NewOps,
    // Signal that we execute the graph because of a sync without any new ops added to the graph.
    Sync,
}

impl<B: FusionBackend> StreamExecutor<B> {
    /// Create a new graph execution with the given optimization builders.
    pub fn new(optimizations: Vec<Box<dyn OptimizationBuilder<B>>>) -> Self {
        Self {
            analyzer: OptimizationAnalyzer::new(),
            builders: optimizations,
            num_skipped: 0,
        }
    }

    /// Execute the graph with the provided mode.
    pub fn execute(
        &mut self,
        stream: &mut StreamDescription<B>,
        cache: &mut StreamOptimizations<B::Optimization>,
        handles: &mut HandleContainer<B>,
        mode: ExecutionMode,
    ) {
        loop {
            if stream.is_empty() {
                break;
            }

            match self.cache(cache, stream, mode) {
                OptimizationAnalysis::NoneAvailable => {
                    match self.build(cache, stream, mode) {
                        BuildAction::ExecuteOptimization(ops) => {
                            stream.execute_optimization(handles, ops);
                            self.reset(cache, stream);
                        }
                        BuildAction::ExecuteOperations => {
                            stream.execute_operations(handles);
                            self.reset(cache, stream);
                        }
                        BuildAction::ContinueBuilding => {
                            if let ExecutionMode::Sync = mode {
                                panic!("Can't continue building when sync is called.")
                            }
                        }
                    };

                    if self.num_skipped == 0 {
                        break;
                    }
                }
                OptimizationAnalysis::FutureAvailable => {
                    self.num_skipped += 1;

                    match mode {
                        ExecutionMode::NewOps => break,
                        ExecutionMode::Sync => panic!("Can't wait while sync"),
                    };
                }
                OptimizationAnalysis::Found(ops) => {
                    let ops = cache.get_optimization_mut_unckecked(ops);
                    stream.execute_optimization(handles, ops);
                    self.reset(cache, stream);
                }
            };

            if let ExecutionMode::NewOps = mode {
                break;
            }
        }
    }

    fn build<'a>(
        &'a mut self,
        cache: &'a mut StreamOptimizations<B::Optimization>,
        graph: &StreamDescription<B>,
        mode: ExecutionMode,
    ) -> BuildAction<'_, B> {
        // When we are executing with the new ops mode, we need to register the last ops of the
        // graph even when there is no skipped operation.
        let offset = match mode {
            ExecutionMode::NewOps => 1,
            ExecutionMode::Sync => 0,
        };

        for i in (0..self.num_skipped + offset).rev() {
            let index = graph.relative.len() - 1 - i;
            let relative = &graph.relative[index];

            for ops in self.builders.iter_mut() {
                ops.register(relative);
            }
        }
        self.num_skipped = 0;

        // Can only be lazy when not sync.
        if let ExecutionMode::NewOps = mode {
            if still_optimizing(&self.builders) {
                return BuildAction::ContinueBuilding;
            }
        }

        match find_best_optimization_index(&mut self.builders) {
            Some(index) => {
                let (relative, next_ops) = Self::split_relative_graph_owned(graph, mode);
                let optimization = &self.builders[index];
                let id =
                    self.analyzer
                        .new_optimization_built(cache, optimization, relative, next_ops);
                let op = cache.get_optimization_mut_unckecked(id);
                BuildAction::ExecuteOptimization(op)
            }
            None => {
                // TODO: Cache this result too.
                BuildAction::ExecuteOperations
            }
        }
    }

    fn reset(
        &mut self,
        cache: &mut StreamOptimizations<B::Optimization>,
        graph: &StreamDescription<B>,
    ) {
        for ops in self.builders.iter_mut() {
            ops.reset();
        }
        self.num_skipped = graph.relative.len();

        self.analyzer.reset();

        // Reset the policy state.
        for i in 0..self.num_skipped {
            let _ = self.analyzer.new_operation_added(
                cache,
                &graph.relative[0..i],
                Condition::NextOps(&graph.relative[i]),
            );
        }
    }

    fn cache<'a>(
        &'a mut self,
        cache: &'a mut StreamOptimizations<B::Optimization>,
        graph: &StreamDescription<B>,
        mode: ExecutionMode,
    ) -> OptimizationAnalysis {
        let (graph, next_ops) = Self::split_relative_graph_ref(graph, mode);
        let end_condition = next_ops.map(Condition::NextOps).unwrap_or(Condition::Sync);
        let action = self
            .analyzer
            .new_operation_added(cache, graph, end_condition);

        match mode {
            ExecutionMode::NewOps => action,
            ExecutionMode::Sync => match action {
                OptimizationAnalysis::NoneAvailable => OptimizationAnalysis::NoneAvailable,
                OptimizationAnalysis::FutureAvailable => OptimizationAnalysis::NoneAvailable,
                OptimizationAnalysis::Found(ops) => OptimizationAnalysis::Found(ops),
            },
        }
    }

    fn split_relative_graph_owned(
        graph: &StreamDescription<B>,
        mode: ExecutionMode,
    ) -> (Vec<TensorOpsDescription>, Option<TensorOpsDescription>) {
        match mode {
            ExecutionMode::NewOps => {
                let graph = graph.split_relative_graph();
                (graph.0.to_vec(), graph.1.cloned())
            }
            ExecutionMode::Sync => (graph.relative.clone(), None),
        }
    }

    fn split_relative_graph_ref(
        graph: &StreamDescription<B>,
        mode: ExecutionMode,
    ) -> (&[TensorOpsDescription], Option<&TensorOpsDescription>) {
        match mode {
            ExecutionMode::NewOps => graph.split_relative_graph(),
            ExecutionMode::Sync => (graph.relative.as_slice(), None),
        }
    }
}

enum BuildAction<'a, B: FusionBackend> {
    ExecuteOptimization(&'a mut B::Optimization),
    ExecuteOperations,
    ContinueBuilding,
}

fn still_optimizing<B: FusionBackend>(optimizations: &[Box<dyn OptimizationBuilder<B>>]) -> bool {
    let mut num_stopped = 0;

    for optimization in optimizations.iter() {
        if let OptimizationStatus::Closed = optimization.status() {
            num_stopped += 1
        }
    }

    num_stopped < optimizations.len()
}

fn find_best_optimization_index<B: FusionBackend>(
    optimizations: &mut [Box<dyn OptimizationBuilder<B>>],
) -> Option<usize> {
    let mut best_index = None;
    let mut best_score = 0;

    for (i, optimization) in optimizations.iter().enumerate() {
        let properties = optimization.properties();

        if properties.ready && properties.score >= best_score {
            best_index = Some(i);
            best_score = properties.score;
        }
    }

    best_index
}
