use crate::components_test::LearnerComponents;
use crate::metric_test::store::EventStoreClient;
use crate::LearnerSummaryConfig;
use burn_core::tensor::backend::Backend;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Learner struct encapsulating all components necessary to train a Neural Network model.
///
/// To create a learner, use the [builder](crate::learner::LearnerBuilder) struct.
pub struct Learner<LC: LearnerComponents> {
    pub(crate) model: LC::Model,
    pub(crate) devices: Vec<<LC::Backend as Backend>::Device>,
    pub(crate) interrupter: TrainingInterrupter,
    pub(crate) event_processor: LC::EventProcessor,
    pub(crate) event_store: Rc<EventStoreClient>,
    pub(crate) summary: Option<LearnerSummaryConfig>,
}

#[derive(Clone, Default)]
/// A handle that allows aborting the training process early.
pub struct TrainingInterrupter {
    state: Arc<AtomicBool>,
}

impl TrainingInterrupter {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Notify the learner that it should stop.
    pub fn stop(&self) {
        self.state.store(true, Ordering::Relaxed);
    }

    /// True if .stop() has been called.
    pub fn should_stop(&self) -> bool {
        self.state.load(Ordering::Relaxed)
    }
}
