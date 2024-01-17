use burn::{
    module::Module,
    nn::conv::{ConvTranspose1d, ConvTranspose1dConfig},
    tensor::{backend::Backend, Tensor},
};

#[derive(Module, Debug)]
struct Net<B: Backend> {
    conv1: ConvTranspose1d<B>,
    conv2: ConvTranspose1d<B>,
}

impl<B: Backend> Net<B> {
    /// Create a new model from the given record.
    pub fn new_with(record: NetRecord<B>) -> Self {
        let conv1 = ConvTranspose1dConfig::new([2, 2], 2).init_with(record.conv1);
        let conv2 = ConvTranspose1dConfig::new([2, 2], 2).init_with(record.conv2);
        Self { conv1, conv2 }
    }

    /// Forward pass of the model.
    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 3> {
        let x = self.conv1.forward(x);

        self.conv2.forward(x)
    }
}

#[cfg(test)]
mod tests {
    type Backend = burn_ndarray::NdArray<f32>;

    use burn::record::{FullPrecisionSettings, Recorder};
    use burn_import::pytorch::PyTorchFileRecorder;

    use super::*;

    #[test]
    fn conv_transpose1d() {
        let device = Default::default();

        let record = PyTorchFileRecorder::<FullPrecisionSettings>::default()
            .load("tests/conv_transpose1d/conv_transpose1d.pt".into())
            .expect("Failed to decode state");

        let model = Net::<Backend>::new_with(record);

        let input = Tensor::<Backend, 3>::from_data(
            [[[0.93708336, 0.65559506], [0.31379688, 0.19801933]]],
            &device,
        );

        let output = model.forward(input);

        let expected = Tensor::<Backend, 3>::from_data(
            [[
                [0.02935525, 0.01119324, -0.01356167, -0.00682688],
                [0.01644749, -0.01429807, 0.00083987, 0.00279229],
            ]],
            &device,
        );

        output.to_data().assert_approx_eq(&expected.to_data(), 8);
    }
}
