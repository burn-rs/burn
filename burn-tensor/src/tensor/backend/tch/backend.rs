use super::TchTensor;
use crate::tensor::{Backend, Data, Element, TensorType};
use rand::distributions::{uniform::SampleUniform, Standard};

#[derive(Debug, Copy, Clone)]
pub enum Device {
    Cpu,
    Cuda(usize),
}

impl Default for Device {
    fn default() -> Self {
        Self::Cpu
    }
}

#[derive(Debug, new)]
pub struct TchBackend<E> {
    _e: E,
}

impl<E: Default> Default for TchBackend<E> {
    fn default() -> Self {
        Self::new(E::default())
    }
}

impl<E: Element + tch::kind::Element + Into<f64> + SampleUniform> Backend for TchBackend<E>
where
    Standard: rand::distributions::Distribution<E>,
{
    type E = E;
    type Device = Device;

    fn name() -> String {
        "Tch Backend".to_string()
    }
}

impl<E: Element + tch::kind::Element + Into<f64> + SampleUniform, const D: usize>
    TensorType<D, Self> for TchBackend<E>
where
    Standard: rand::distributions::Distribution<E>,
{
    type T = TchTensor<E, D>;

    fn from_data(data: Data<E, D>, device: Device) -> Self::T {
        let device = match device {
            Device::Cpu => tch::Device::Cpu,
            Device::Cuda(num) => tch::Device::Cuda(num),
        };
        let tensor = TchTensor::from_data(data, device);
        tensor
    }
}
