use super::{
    BackwardRecordedOps, BackwardRecordedOpsRef, BinaryOpsNodeState,
    Forward2BackwardGraphConverter, ForwardRecordedOps, RecordedOpsParentRef,
};
use crate::node::{BackwardNodeRef, BackwardNodeStateRef, ForwardNodeRef, Zeros};
use std::{ops::Add, rc::Rc, sync::Arc};

pub trait BinaryOps<Lhs, Rhs, Out>: std::fmt::Debug + Send + Sync {
    fn partial_left(&self, state: &BinaryOpsNodeState<Lhs, Rhs, Out>) -> Lhs;
    fn partial_right(&self, state: &BinaryOpsNodeState<Lhs, Rhs, Out>) -> Rhs;
}

#[derive(new, Debug)]
pub struct ForwardBinaryRecordedOps<Lhs, Rhs, Ops> {
    lhs: ForwardNodeRef<Lhs>,
    rhs: ForwardNodeRef<Rhs>,
    ops: Arc<Ops>,
}
impl<Lhs, Rhs, Out, Ops> ForwardRecordedOps<Out> for ForwardBinaryRecordedOps<Lhs, Rhs, Ops>
where
    Lhs: Clone + Zeros<Lhs> + Add<Output = Lhs> + std::fmt::Debug + 'static + Send + Sync,
    Rhs: Clone + Zeros<Rhs> + Add<Output = Rhs> + std::fmt::Debug + 'static + Send + Sync,
    Out: Clone + Zeros<Out> + Add<Output = Out> + std::fmt::Debug + 'static,
    Ops: BinaryOps<Lhs, Rhs, Out> + std::fmt::Debug + 'static + Send + Sync,
{
    fn as_backward(
        &self,
        graph: &mut Forward2BackwardGraphConverter,
    ) -> BackwardRecordedOpsRef<Out> {
        let lhs = graph.from(&self.lhs);
        let rhs = graph.from(&self.rhs);
        let ops = self.ops.clone();

        Rc::new(BackwardBinaryRecordedOps::new(lhs, rhs, ops))
    }
}

#[derive(new, Debug)]
pub struct BackwardBinaryRecordedOps<Lhs, Rhs, Ops> {
    lhs: BackwardNodeRef<Lhs>,
    rhs: BackwardNodeRef<Rhs>,
    ops: Arc<Ops>,
}

impl<Lhs, Rhs, Out, Ops> BackwardRecordedOps<Out> for BackwardBinaryRecordedOps<Lhs, Rhs, Ops>
where
    Lhs: Clone + Zeros<Lhs> + Add<Output = Lhs> + std::fmt::Debug + 'static,
    Rhs: Clone + Zeros<Rhs> + Add<Output = Rhs> + std::fmt::Debug + 'static,
    Out: Clone + Zeros<Out> + Add<Output = Out> + std::fmt::Debug + 'static,
    Ops: BinaryOps<Lhs, Rhs, Out> + std::fmt::Debug + 'static,
{
    fn backward_step(&self, state: &BackwardNodeStateRef<Out>) {
        let state = BinaryOpsNodeState::new(&self.lhs.state, &self.rhs.state, state);

        let partial_left: Lhs = self.ops.partial_left(&state);
        let partial_right: Rhs = self.ops.partial_right(&state);

        self.lhs.state.borrow_mut().update_grad(partial_left);
        self.rhs.state.borrow_mut().update_grad(partial_right);
    }

    fn backward_parents(&self) -> Vec<RecordedOpsParentRef> {
        vec![self.lhs.clone(), self.rhs.clone()]
    }
}
