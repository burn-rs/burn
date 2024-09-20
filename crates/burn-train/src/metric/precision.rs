use super::{
    confusion_matrix::ConfusionMatrix,
    state::{FormatOptions, NumericMetricState},
    AggregationType, ClassificationInput, Metric, MetricEntry, MetricMetadata, Numeric,
};
use burn_core::tensor::backend::Backend;
use core::marker::PhantomData;

/// The precision metric.
pub struct PrecisionMetric<B: Backend> {
    state: NumericMetricState,
    _b: PhantomData<B>,
    threshold: f64,
    average: AggregationType,
}

impl<B: Backend> PrecisionMetric<B> {
    /// Sets threshold. Default 0.5
    fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold;
        self
    }

    /// Sets average type. Default Micro
    fn with_average(mut self, average: AggregationType) -> Self {
        self.average = average;
        self
    }
}

impl<B: Backend> Default for PrecisionMetric<B> {
    /// Creates a new metric instance with default values.
    fn default() -> Self {
        Self {
            state: NumericMetricState::default(),
            _b: PhantomData,
            threshold: 0.5,
            average: AggregationType::Micro,
        }
    }
}

impl<B: Backend> Metric for PrecisionMetric<B> {
    const NAME: &'static str = "Precision";
    type Input = ClassificationInput<B>;
    fn update(
        &mut self,
        input: &ClassificationInput<B>,
        _metadata: &MetricMetadata,
    ) -> MetricEntry {
        let [sample_size, _] = input.predictions.dims();

        let conf_mat = ConfusionMatrix::from(input, self.threshold, self.average);
        let agg_precision = conf_mat.clone().true_positive() / conf_mat.predicted_positive();
        let precision = self.average.to_averaged_metric(agg_precision);

        self.state.update(
            100.0 * precision,
            sample_size,
            FormatOptions::new(Self::NAME).unit("%").precision(2),
        )
    }

    fn clear(&mut self) {
        self.state.reset()
    }
}

impl<B: Backend> Numeric for PrecisionMetric<B> {
    fn value(&self) -> f64 {
        self.state.value()
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::{AggregationType, Metric, MetricMetadata, Numeric, PrecisionMetric};
    use crate::metric::test::{dummy_classification_input, THRESHOLD, ClassificationType};
    use crate::TestBackend;
    use strum::IntoEnumIterator;

    #[test]
    fn test_precision() {
        for agg_type in AggregationType::iter() {
            for classification_type in ClassificationType::iter() {
                let (input, target_diff) = dummy_classification_input(&classification_type);
                let mut metric = PrecisionMetric::<TestBackend>::default()
                    .with_threshold(THRESHOLD)
                    .with_average(agg_type);
                let _entry = metric.update(&input, &MetricMetadata::fake());

                //tp/(tp+fp) = 1 - fp/(tp+fp)
                let metric_precision = metric.value();

                //fp/(tp+fp+tn+fn) = fp/(tp+fp)(1 + negative/positive)
                let agg_false_positive_rate =
                    agg_type.aggregate_mean(target_diff.clone().equal_elem(-1).int());
                let pred_positive = input.targets.clone().float() - target_diff.clone();
                let agg_pred_negative =
                    agg_type.aggregate(pred_positive.clone().bool().bool_not().int());
                let agg_pred_positive = agg_type.aggregate(pred_positive.int());
                //1 - fp(1 + negative/positive)/(tp+fp+tn+fn) = 1 - fp/(tp+fp) = tp/(tp+fp)
                let test_precision = agg_type.to_averaged_metric(
                    -agg_false_positive_rate * (agg_pred_negative / agg_pred_positive + 1.0) + 1.0,
                );
                assert_relative_eq!(metric_precision, test_precision * 100.0,  max_relative = 1e-3);
            }
        }
    }

    /*#[test]
    fn test_precision_with_padding() {
        let device = Default::default();
        let mut metric = PrecisionMetric::<TestBackend>::new().with_pad_token(3);
        let input = PrecisionInput::n(
            Tensor::from_data(
                [
                    [0.0, 0.2, 0.8, 0.0], // 2
                    [1.0, 2.0, 0.5, 0.0], // 1
                    [0.4, 0.1, 0.2, 0.0], // 0
                    [0.6, 0.7, 0.2, 0.0], // 1
                    [0.0, 0.1, 0.2, 5.0], // Predicted padding should not count
                    [0.0, 0.1, 0.2, 0.0], // Error on padding should not count
                    [0.6, 0.0, 0.2, 0.0], // Error on padding should not count
                ],
                &device,
            ),
            Tensor::from_data([2, 2, 1, 1, 3, 3, 3], &device),
        );

        let _entry = metric.update(&input, &MetricMetadata::fake());
        assert_eq!(todo!(), metric.value());
    }*/
}
