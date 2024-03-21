use burn::serde::{Deserialize, Serialize};
use std::collections::HashSet;
use sysinfo;
use wgpu;

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct BenchmarkSystemInfo {
    cpus: Vec<String>,
    gpus: Vec<String>,
}

impl BenchmarkSystemInfo {
    pub(crate) fn new() -> Self {
        Self {
            cpus: BenchmarkSystemInfo::enumerate_cpus(),
            gpus: BenchmarkSystemInfo::enumerate_gpus(),
        }
    }

    fn enumerate_cpus() -> Vec<String> {
        let system = sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::new().with_cpu(sysinfo::CpuRefreshKind::everything()),
        );
        let cpu_names: HashSet<String> = system
            .cpus()
            .iter()
            .map(|c| c.brand().to_string())
            .collect();
        cpu_names.into_iter().collect()
    }

    fn enumerate_gpus() -> Vec<String> {
        let instance = wgpu::Instance::default();
        let adapters: Vec<wgpu::Adapter> = instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter(|adapter| {
                let info = adapter.get_info();
                info.device_type == wgpu::DeviceType::DiscreteGpu
                    || info.device_type == wgpu::DeviceType::IntegratedGpu
            })
            .collect();
        let gpu_names: HashSet<String> = adapters
            .iter()
            .map(|adapter| {
                let info = adapter.get_info();
                info.name
            })
            .collect();
        gpu_names.into_iter().collect()
    }
}
