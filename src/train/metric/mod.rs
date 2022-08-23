use crate::tensor::back::Backend;
use crate::tensor::ElementConversion;
use crate::tensor::Tensor;

pub trait RunningMetric<T>: Send + Sync {
    fn update(&mut self, item: &T) -> RunningMetricResult;
    fn clear(&mut self);
}

#[derive(new)]
pub struct RunningMetricResult {
    pub name: String,
    pub formatted: String,
    pub raw_running: String,
    pub raw_current: String,
}

pub struct LossMetric {
    current: f64,
    count: usize,
    total: f64,
}

impl LossMetric {
    pub fn new() -> Self {
        Self {
            count: 0,
            current: 0.0,
            total: 0.0,
        }
    }
    pub fn update_<B: Backend>(&mut self, loss: &Tensor<B, 1>) -> RunningMetricResult {
        let loss = f64::from_elem(loss.to_data().value[0]);

        self.count += 1;
        self.total += loss;
        self.current = loss;

        let name = String::from("Loss");
        let running = self.total / self.count as f64;
        let raw_running = format!("{}", running);
        let raw_current = format!("{}", self.current);
        let formatted = format!("running {:.3} current {:.3}", running, self.current);

        RunningMetricResult {
            name,
            formatted,
            raw_running,
            raw_current,
        }
    }

    pub fn clear_(&mut self) {
        self.count = 0;
        self.total = 0.0;
        self.current = 0.0;
    }
}
