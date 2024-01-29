use crate::{
    data::{MNISTBatch, MNISTBatcher},
    mnist::MNISTDataset,
    model::{Model, ModelConfig},
};
use burn::{
    self,
    config::Config,
    data::dataloader::DataLoaderBuilder,
    module::Module,
    nn::loss::CrossEntropyLossConfig,
    optim::AdamConfig,
    record::{BinBytesRecorder, FullPrecisionSettings},
    tensor::{
        backend::{AutodiffBackend, Backend},
        Int, Tensor,
    },
    train::{
        metric::{AccuracyMetric, LossMetric},
        renderer::{MetricState, MetricsRenderer, TrainingProgress},
        ClassificationOutput, LearnerBuilder, TrainOutput, TrainStep, ValidStep,
    },
};
use log::info;

impl<B: Backend> Model<B> {
    pub fn forward_classification(
        &self,
        images: Tensor<B, 3>,
        targets: Tensor<B, 1, Int>,
    ) -> ClassificationOutput<B> {
        let output = self.forward(images);
        let loss = CrossEntropyLossConfig::new()
            .init(&output.device())
            .forward(output.clone(), targets.clone());

        ClassificationOutput::new(loss, output, targets)
    }
}

impl<B: AutodiffBackend> TrainStep<MNISTBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, batch: MNISTBatch<B>) -> TrainOutput<ClassificationOutput<B>> {
        let item = self.forward_classification(batch.images, batch.targets);

        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<MNISTBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, batch: MNISTBatch<B>) -> ClassificationOutput<B> {
        self.forward_classification(batch.images, batch.targets)
    }
}

#[derive(Config)]
pub struct TrainingConfig {
    pub model: ModelConfig,
    pub optimizer: AdamConfig,
    #[config(default = 10)]
    pub num_epochs: usize,
    #[config(default = 64)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 42)]
    pub seed: u64,
    #[config(default = 1.0e-4)]
    pub learning_rate: f64,
}

struct CustomRenderer {}

impl MetricsRenderer for CustomRenderer {
    fn update_train(&mut self, state: MetricState) {
        info!("Updated Training: {:?}", state);
    }

    fn update_valid(&mut self, state: MetricState) {
        info!("Updated Validation: {:?}", state);
    }

    fn render_train(&mut self, item: TrainingProgress) {
        info!("Training Progress: {:?}", item);
    }

    fn render_valid(&mut self, item: TrainingProgress) {
        info!("Validation Progress: {:?}", item);
    }
}

pub fn train<B: AutodiffBackend>(
    artifact_dir: &str,
    config: TrainingConfig,
    device: B::Device,
    train_labels: &[u8],
    train_images: &[u8],
    train_lengths: &[u16],
    test_labels: &[u8],
    test_images: &[u8],
    test_lengths: &[u16],
) -> Vec<u8> {
    // std::fs::create_dir_all(artifact_dir).ok();
    // config
    //     .save(format!("{artifact_dir}/config.json"))
    //     .expect("Save without error");

    B::seed(config.seed);

    let batcher_train = MNISTBatcher::<B>::new(device.clone());
    let batcher_valid = MNISTBatcher::<B::InnerBackend>::new(device.clone());

    // Create the dataloaders.
    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(MNISTDataset::new(train_labels, train_images, train_lengths));

    let dataloader_test = DataLoaderBuilder::new(batcher_valid)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(MNISTDataset::new(test_labels, test_images, test_lengths));

    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(AccuracyMetric::new())
        .metric_valid_numeric(AccuracyMetric::new())
        .metric_train_numeric(LossMetric::new())
        .metric_valid_numeric(LossMetric::new())
        // .with_file_checkpointer(CompactRecorder::new())
        .log_to_file(false)
        .devices(vec![device.clone()])
        .num_epochs(config.num_epochs)
        .renderer(CustomRenderer {})
        .num_epochs(config.num_epochs)
        .build(
            config.model.init::<B>(&device),
            config.optimizer.init(),
            config.learning_rate,
        );

    let model_trained = learner.fit(dataloader_train, dataloader_test);

    model_trained
        .to_bytes(&BinBytesRecorder::<FullPrecisionSettings>::default())
        .expect("Failed to serialize model")
}
