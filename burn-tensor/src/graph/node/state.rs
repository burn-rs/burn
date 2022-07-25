use crate::node::Zeros;
use std::{cell::RefCell, ops::Add};

#[derive(new, Debug)]
pub struct ForwardNodeState<Out> {
    value: Out,
}
impl<Out> ForwardNodeState<Out>
where
    Out: Clone,
{
    pub fn value(&self) -> Out {
        self.value.clone()
    }
}

#[derive(Debug)]
pub struct BackwardNodeState<Out> {
    pub value: Out,
    pub grad: RefCell<Out>,
}

impl<Out: Zeros<Out>> BackwardNodeState<Out> {
    fn new(value: Out) -> Self {
        let grad = value.zeros();
        let grad = RefCell::new(grad);

        Self { value, grad }
    }
    pub fn new_mut(value: Out) -> BackwardNodeState<Out> {
        Self::new(value)
    }
}
impl<Out> BackwardNodeState<Out>
where
    Out: Clone,
{
    pub fn value(&self) -> Out {
        self.value.clone()
    }
}

impl<Out> BackwardNodeState<Out>
where
    Out: Zeros<Out> + Clone + Add<Output = Out>,
    Out: std::fmt::Debug,
{
    pub fn grad(&self) -> Out {
        self.grad.borrow().clone()
    }

    pub fn update_grad(&self, grad: Out) {
        self.grad.swap(&RefCell::new(self.grad() + grad));
    }
}
