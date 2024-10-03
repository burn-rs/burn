use super::{Event, EventProcessor, Metrics};
use crate::metric_test::store::EventStoreClient;
use crate::renderer_test::{MetricState, MetricsRenderer};
use std::rc::Rc;

/// An [event processor](EventProcessor) that handles:
///   - Computing and storing metrics in an [event store](crate::metric::store::EventStore).
///   - Render metrics using a [metrics renderer](MetricsRenderer).
pub struct FullEventProcessor<T> {
    metrics: Metrics<T>,
    renderer: Box<dyn MetricsRenderer>,
    store: Rc<EventStoreClient>,
}

impl<T> FullEventProcessor<T> {
    pub(crate) fn new(
        metrics: Metrics<T>,
        renderer: Box<dyn MetricsRenderer>,
        store: Rc<EventStoreClient>,
    ) -> Self {
        Self {
            metrics,
            renderer,
            store,
        }
    }
}

impl<T> EventProcessor for FullEventProcessor<T> {
    type ItemTrain = T;

    fn process(&mut self, event: Event<Self::ItemTrain>) {
        match event {
            Event::ProcessedItem(item) => {
                let progress = (&item).into();
                let metadata = (&item).into();

                let update = self.metrics.update_train(&item, &metadata);

                self.store
                    .add_event_train(crate::metric_test::store::Event::MetricsUpdate(
                        update.clone(),
                    ));

                update
                    .entries
                    .into_iter()
                    .for_each(|entry| self.renderer.update_train(MetricState::Generic(entry)));

                update
                    .entries_numeric
                    .into_iter()
                    .for_each(|(entry, value)| {
                        self.renderer
                            .update_train(MetricState::Numeric(entry, value))
                    });

                self.renderer.render_train(progress);
            }
            Event::EndEpoch(epoch) => {
                self.metrics.end_epoch_train();
                self.store
                    .add_event_train(crate::metric_test::store::Event::EndEpoch(epoch));
            }
        }
    }
}
