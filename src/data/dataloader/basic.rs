use super::{batcher::Batcher, DataLoader, MultiThreadsDataLoader};
use burn_dataset::{transform::PartialDataset, Dataset};
use std::sync::Arc;

pub struct BasicDataLoader<I, O> {
    batch_size: usize,
    dataset: Arc<dyn Dataset<I>>,
    batcher: Arc<dyn Batcher<I, O>>,
}

struct BasicDataloaderIterator<I, O> {
    current_index: usize,
    batch_size: usize,
    dataset: Arc<dyn Dataset<I>>,
    batcher: Arc<dyn Batcher<I, O>>,
}

impl<I, O> BasicDataLoader<I, O> {
    pub fn new(
        batch_size: usize,
        dataset: Arc<dyn Dataset<I>>,
        batcher: Arc<dyn Batcher<I, O>>,
    ) -> Self {
        Self {
            batch_size,
            dataset,
            batcher,
        }
    }
}
impl<I, O> BasicDataLoader<I, O>
where
    I: Send + Sync + Clone + 'static,
    O: Send + Sync + Clone + 'static,
{
    pub fn multi_threads(
        batch_size: usize,
        dataset: Arc<dyn Dataset<I>>,
        batcher: Arc<dyn Batcher<I, O>>,
        num_threads: usize,
    ) -> MultiThreadsDataLoader<O> {
        let datasets = PartialDataset::split(dataset, num_threads);
        let mut dataloaders: Vec<Arc<dyn DataLoader<_> + Send + Sync>> = Vec::new();
        for dataset in datasets {
            let dataloader = BasicDataLoader::new(batch_size, Arc::new(dataset), batcher.clone());
            let dataloader = Arc::new(dataloader);
            dataloaders.push(dataloader);
        }
        MultiThreadsDataLoader::new(dataloaders)
    }
}

impl<I, O> DataLoader<O> for BasicDataLoader<I, O> {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = O> + 'a> {
        Box::new(BasicDataloaderIterator::new(
            self.batch_size,
            self.dataset.clone(),
            self.batcher.clone(),
        ))
    }

    fn len(&self) -> usize {
        self.dataset.len() / self.batch_size
    }
}

impl<I, O> BasicDataloaderIterator<I, O> {
    pub fn new(
        batch_size: usize,
        dataset: Arc<dyn Dataset<I>>,
        batcher: Arc<dyn Batcher<I, O>>,
    ) -> Self {
        BasicDataloaderIterator {
            current_index: 0,
            batch_size,
            dataset,
            batcher,
        }
    }
}

impl<I, O> Iterator for BasicDataloaderIterator<I, O> {
    type Item = O;

    fn next(&mut self) -> Option<O> {
        let mut items = Vec::with_capacity(self.batch_size);
        loop {
            if items.len() >= self.batch_size {
                break;
            }

            let item = self.dataset.get(self.current_index);
            self.current_index += 1;

            let item = match item {
                Some(item) => item,
                None => break,
            };
            items.push(item);
        }
        if items.len() == 0 {
            return None;
        }

        let batch = self.batcher.batch(items);
        Some(batch)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::data::dataloader::batcher::TestBatcher;
    use crate::data::dataset::FakeDataset;

    #[test]
    fn should_iterate_over_all_data() {
        let batcher = Arc::new(TestBatcher::new());
        let dataset = Arc::new(FakeDataset::<String>::new(27));
        let dataloader = BasicDataLoader::new(5, dataset.clone(), batcher);

        let mut items_dataset = HashSet::new();
        let mut items_dataloader = HashSet::new();

        for item in dataset.iter() {
            items_dataset.insert(item);
        }

        for items in dataloader.iter() {
            for item in items {
                items_dataloader.insert(item);
            }
        }

        assert_eq!(items_dataset, items_dataloader);
        assert_eq!(items_dataloader.len(), dataset.len());
    }
}
