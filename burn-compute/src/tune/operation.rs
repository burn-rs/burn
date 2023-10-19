use crate::server::{ComputeServer, Handle};

/// Type of operation for the kernel
pub trait AutotuneOperation<S>: Send
where
    S: ComputeServer,
{
    fn key(&self) -> String {
        let mut key = String::new();
        key.push_str(&self.operation_key());
        key.push_str(&self.input_key());
        key
    }
    fn operation_key(&self) -> String;
    fn input_key(&self) -> String;
    fn autotunables(&self) -> Vec<Operation<S>>;
    fn inputs(&self) -> Vec<Vec<u8>>;
    fn fastest(&self, fastest_index: usize) -> Operation<S> {
        self.autotunables().remove(fastest_index)
    }
}

#[derive(new, Clone)]
pub struct Operation<S: ComputeServer> {
    kernel: S::Kernel,
    parameters: Option<Vec<Handle<S>>>,
}

impl<S: ComputeServer> Operation<S> {
    pub fn execute(self, inputs: Vec<Handle<S>>, server: &mut S) {
        let mut all_handles = inputs;
        if let Some(vec) = self.parameters {
            all_handles.extend(vec);
        }
        let slice = &all_handles
            .iter()
            .map(|h| h as &Handle<S>)
            .collect::<Vec<&Handle<S>>>();
        server.execute_kernel(self.kernel, slice);
    }

    pub fn get_kernel(self) -> S::Kernel {
        self.kernel
    }

    pub(crate) fn clone(&self) -> Self {
        Operation {
            kernel: self.kernel.clone(),
            parameters: self.parameters.clone(),
        }
    }
}
