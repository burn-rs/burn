use crate::data::MNISTBatch;

use burn::{
    module::{Module, Param},
    nn::{self, loss::CrossEntropyLoss},
    tensor::{
        backend::{ADBackend, Backend},
        Tensor,
    },
    train::{ClassificationOutput, TrainOutput, TrainStep, ValidStep},
};

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv1: Param<ConvBlock<B>>,
    conv2: Param<ConvBlock<B>>,
    conv3: Param<ConvBlock<B>>,
    dropout: nn::Dropout,
    fc1: Param<nn::Linear<B>>,
    fc2: Param<nn::Linear<B>>,
    activation: nn::GELU,
}

const NUM_CLASSES: usize = 10;

impl<B: Backend> Model<B> {
    pub fn new() -> Self {
        let conv1 = ConvBlock::new([1, 8], [3, 3]); // out: [Batch,1,26,26]
        let conv2 = ConvBlock::new([8, 16], [3, 3]); // out: [Batch,1,24x24]
        let conv3 = ConvBlock::new([16, 24], [3, 3]); // out: [Batch,1,22x22]

        let fc1 = nn::Linear::new(&nn::LinearConfig::new(24 * 22 * 22, 32).with_bias(false));
        let fc2 = nn::Linear::new(&nn::LinearConfig::new(32, NUM_CLASSES).with_bias(false));

        let dropout = nn::Dropout::new(&nn::DropoutConfig::new(0.3));

        Self {
            conv1: Param::from(conv1),
            conv2: Param::from(conv2),
            conv3: Param::from(conv3),
            fc1: Param::from(fc1),
            fc2: Param::from(fc2),
            dropout,
            activation: nn::GELU::new(),
        }
    }

    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
        let [batch_size, heigth, width] = input.dims();

        let x = input.reshape([batch_size, 1, heigth, width]).detach();
        let x = self.conv1.forward(x);
        let x = self.conv2.forward(x);
        let x = self.conv3.forward(x);

        let x = x.reshape([batch_size, 24 * 22 * 22]);

        let x = self.fc1.forward(x);
        let x = self.activation.forward(x);
        let x = self.dropout.forward(x);

        self.fc2.forward(x)
    }

    pub fn forward_classification(&self, item: MNISTBatch<B>) -> ClassificationOutput<B> {
        let targets = item.targets;
        let output = self.forward(item.images);
        let loss = CrossEntropyLoss::new(None);
        let loss = loss.forward(output.clone(), targets.clone());

        ClassificationOutput {
            loss,
            output,
            targets,
        }
    }
}

#[derive(Module, Debug)]
pub struct ConvBlock<B: Backend> {
    conv: Param<nn::conv::Conv2d<B>>,
    activation: nn::GELU,
}

impl<B: Backend> ConvBlock<B> {
    pub fn new(channels: [usize; 2], kernel_size: [usize; 2]) -> Self {
        let conv = nn::conv::Conv2d::new(
            &nn::conv::Conv2dConfig::new(channels, kernel_size).with_bias(false),
        );

        Self {
            conv: Param::from(conv),
            activation: nn::GELU::new(),
        }
    }

    pub fn forward(&self, input: Tensor<B, 4>) -> Tensor<B, 4> {
        let x = self.conv.forward(input);
        self.activation.forward(x)
    }
}

impl<B: ADBackend> TrainStep<MNISTBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, item: MNISTBatch<B>) -> TrainOutput<ClassificationOutput<B>> {
        let item = self.forward_classification(item);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<MNISTBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, item: MNISTBatch<B>) -> ClassificationOutput<B> {
        self.forward_classification(item)
    }
}
