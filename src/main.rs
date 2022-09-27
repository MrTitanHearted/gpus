extern crate wgpu;

fn main() {
    let mut backends = wgpu::Backends::empty();
    let mut once_backend = false;
    
    let mut device_types = Vec::new();
    let mut once_type = false;

    let mut features = false;
    let mut limits = false;
    
    for (index, arg) in std::env::args().into_iter().enumerate() {
        if arg == "--backend" {
            once_backend = true;
            if let Some(backend) = std::env::args().nth(index + 1) {
                match backend.as_str() {
                    "vulkan" => backends |= wgpu::Backends::VULKAN,
                    "gl" => backends |= wgpu::Backends::GL,
                    "metal" => backends |= wgpu::Backends::METAL,
                    "dx12" => backends |= wgpu::Backends::DX12,
                    "dx11" => backends |= wgpu::Backends::DX11,
                    "browser-webgpu" => backends |= wgpu::Backends::BROWSER_WEBGPU,
                    "primary" => backends |= wgpu::Backends::PRIMARY,
                    "secondary" => backends |= wgpu::Backends::SECONDARY,

                    _=> panic!("There is no such backend as {}", backend),
                }
            } else {
                panic!("No backend specified!");
            }
        } else if arg == "--features" {
            features = true;
        } else if arg == "--limits" {
            limits = true;
        } else if arg == "--device-type" {
            once_type = true;
            if let Some(device_type) = std::env::args().nth(index + 1) {
                match device_type.as_str() {
                    "dedicated-gpu"                 => device_types.push(wgpu::DeviceType::DiscreteGpu),
                    "integrated-gpu"                => device_types.push(wgpu::DeviceType::IntegratedGpu),
                    "cpu"                           => device_types.push(wgpu::DeviceType::Cpu),
                    "software" | "virtual-gpu"      => device_types.push(wgpu::DeviceType::VirtualGpu),

                    _=> panic!("There is no such device type as {}", device_type),
                }
            } else {
                panic!("No device type specified!");
            }
        }
    }

    if !once_backend {
        backends = wgpu::Backends::all();
    }

    if !once_type {
        device_types.push(wgpu::DeviceType::DiscreteGpu);
        device_types.push(wgpu::DeviceType::IntegratedGpu);
        device_types.push(wgpu::DeviceType::Cpu);
        device_types.push(wgpu::DeviceType::VirtualGpu);
    }

    let instances = wgpu::Instance::new(backends);
    for adapter in instances.enumerate_adapters(backends) {
        if device_types.contains(&adapter.get_info().device_type) {
            println!("[INFO]: {:?}", adapter.get_info());
            if features {
                println!("[FEATURES OF {} with backend {:?}]: {:?}", adapter.get_info().name, adapter.get_info().backend, adapter.features());
            }
            if limits {
                println!("[LIMITS of {} with backend {:?}]: {:?}", adapter.get_info().name, adapter.get_info().backend, adapter.limits());
            }
        }
    }
}
