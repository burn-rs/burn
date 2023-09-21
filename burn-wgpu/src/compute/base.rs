use super::WgpuServer;
use crate::{compute::WgpuStorage, GraphicsApi, WgpuDevice};
use alloc::sync::Arc;
use burn_compute::{
    channel::MutexComputeChannel,
    client::ComputeClient,
    memory_management::{DeallocStrategy, SimpleMemoryManagement},
    Compute,
};
use wgpu::{DeviceDescriptor, DeviceType};

type WgpuChannel = MutexComputeChannel<WgpuServer>;

pub type WgpuComputeClient = ComputeClient<WgpuServer, WgpuChannel>;
pub type WgpuHandle = burn_compute::server::Handle<WgpuServer>;

/// Compute handle for the wgpu backend.
static COMPUTE: Compute<WgpuDevice, WgpuServer, WgpuChannel> = Compute::new();

/// Get the [compute client](ComputeClient) for the given [device](WgpuDevice).
pub fn compute_client<G: GraphicsApi>(
    device: &WgpuDevice,
) -> ComputeClient<WgpuServer, WgpuChannel> {
    let device = Arc::new(device);

    COMPUTE.client(&device, move || {
        let (device_wgpu, queue, info) = pollster::block_on(select_device::<G>(&device));

        log::info!(
            "Created wgpu compute server on device {:?} => {:?}",
            device,
            info
        );

        // TODO: Support a way to modify max_tasks without std.
        let max_tasks = match std::env::var("BURN_WGPU_MAX_TASKS") {
            Ok(value) => value
                .parse::<usize>()
                .expect("BURN_WGPU_MAX_TASKS should be a positive integer."),
            Err(_) => 16, // 16 tasks by default
        };

        let device = Arc::new(device_wgpu);
        let storage = WgpuStorage::new(device.clone());
        // Maximum reusability.
        let memory_management = SimpleMemoryManagement::new(storage, DeallocStrategy::Never);
        let server = WgpuServer::new(memory_management, device, queue, max_tasks);
        let channel = WgpuChannel::new(server);

        ComputeClient::new(channel)
    })
}

pub async fn select_device<G: GraphicsApi>(
    device: &WgpuDevice,
) -> (wgpu::Device, wgpu::Queue, wgpu::AdapterInfo) {
    let adapter = select_adapter::<G>(device);
    let limits = adapter.limits();

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits,
            },
            None,
        )
        .await
        .map_err(|err| {
            format!(
                "Unable to request the device with the adapter {:?}, err {:?}",
                adapter.get_info(),
                err
            )
        })
        .unwrap();

    (device, queue, adapter.get_info())
}

fn select_adapter<G: GraphicsApi>(device: &WgpuDevice) -> wgpu::Adapter {
    let instance = wgpu::Instance::default();

    let mut adapters_other = Vec::new();
    let mut adapters = Vec::new();

    instance
        .enumerate_adapters(G::backend().into())
        .for_each(|adapter| {
            let device_type = adapter.get_info().device_type;

            if let DeviceType::Other = device_type {
                adapters_other.push(adapter);
                return;
            }

            let is_same_type = match device {
                WgpuDevice::DiscreteGpu(_) => device_type == DeviceType::DiscreteGpu,
                WgpuDevice::IntegratedGpu(_) => device_type == DeviceType::IntegratedGpu,
                WgpuDevice::VirtualGpu(_) => device_type == DeviceType::VirtualGpu,
                WgpuDevice::Cpu => device_type == DeviceType::Cpu,
                WgpuDevice::BestAvailable => true,
            };

            if is_same_type {
                adapters.push(adapter);
            }
        });

    fn select(
        num: usize,
        error: &str,
        mut adapters: Vec<wgpu::Adapter>,
        mut adapters_other: Vec<wgpu::Adapter>,
    ) -> wgpu::Adapter {
        if adapters.len() <= num {
            if adapters_other.len() <= num {
                panic!(
                    "{}, adapters {:?}, other adapters {:?}",
                    error,
                    adapters
                        .into_iter()
                        .map(|adapter| adapter.get_info())
                        .collect::<Vec<_>>(),
                    adapters_other
                        .into_iter()
                        .map(|adapter| adapter.get_info())
                        .collect::<Vec<_>>(),
                );
            } else {
                return adapters_other.remove(num);
            }
        }

        adapters.remove(num)
    }

    let adapter = match device {
        WgpuDevice::DiscreteGpu(num) => select(
            *num,
            "No Discrete GPU device found",
            adapters,
            adapters_other,
        ),
        WgpuDevice::IntegratedGpu(num) => select(
            *num,
            "No Integrated GPU device found",
            adapters,
            adapters_other,
        ),
        WgpuDevice::VirtualGpu(num) => select(
            *num,
            "No Virtual GPU device found",
            adapters,
            adapters_other,
        ),
        WgpuDevice::Cpu => select(0, "No CPU device found", adapters, adapters_other),
        WgpuDevice::BestAvailable => {
            let mut most_performant_adapter = None;
            let mut current_score = -1;

            adapters.into_iter().for_each(|adapter| {
                let info = adapter.get_info();
                let score = match info.device_type {
                    DeviceType::DiscreteGpu => 5,
                    DeviceType::Other => 4, // Let's be optimistic with the Other device, it's
                    // often a Discrete Gpu.
                    DeviceType::IntegratedGpu => 3,
                    DeviceType::VirtualGpu => 2,
                    DeviceType::Cpu => 1,
                };

                if score > current_score {
                    most_performant_adapter = Some(adapter);
                    current_score = score;
                }
            });

            if let Some(adapter) = most_performant_adapter {
                adapter
            } else {
                panic!("No adapter found for graphics API {:?}", G::default());
            }
        }
    };

    log::info!("Using adapter {:?}", adapter.get_info());

    adapter
}
