use nvml_wrapper::Nvml;

#[derive(Debug, Clone)]
pub struct GpuData {
    pub usage_percent: u8,
    pub memory_percent: u8,
    pub memory_used_gb: f32,
    pub memory_total_gb: f32,
    pub name: String,
}

pub fn get_gpu_data(_max_gpus: usize) -> Vec<GpuData> {
    match Nvml::init() {
        Ok(nvml) => {
            let device_count = nvml.device_count().unwrap_or(0);
            
            (0..device_count)
                .filter_map(|i| {
                    nvml.device_by_index(i as u32).ok().and_then(|device| {
                        let name = device.name().unwrap_or_else(|_| format!("GPU {}", i));
                        
                        let usage_percent = device
                            .utilization_rates()
                            .map(|rates| rates.gpu as u8)
                            .unwrap_or(0);
                        
                        let (memory_percent, memory_used_gb, memory_total_gb) = device
                            .memory_info()
                            .map(|info| {
                                let percent = ((info.used as f64 / info.total as f64) * 100.0) as u8;
                                let used_gb = info.used as f32 / (1024.0 * 1024.0 * 1024.0);
                                let total_gb = info.total as f32 / (1024.0 * 1024.0 * 1024.0);
                                (percent, used_gb, total_gb)
                            })
                            .unwrap_or((0, 0.0, 0.0));
                        
                        Some(GpuData {
                            usage_percent,
                            memory_percent,
                            memory_used_gb,
                            memory_total_gb,
                            name,
                        })
                    })
                })
                .collect()
        }
        Err(_) => {
            // Fallback to mock data if NVML is not available - show 2 GPUs for demo
            (0..2)
                .map(|i| GpuData {
                    usage_percent: (i * 25) as u8, // Different values for demo
                    memory_percent: (i * 30 + 20) as u8,
                    memory_used_gb: (i as f32 * 1.5 + 2.0), // Mock data
                    memory_total_gb: 8.0, // Mock 8GB total
                    name: format!("GPU {} (No NVML)", i),
                })
                .collect()
        }
    }
}

// New function to get all available GPUs
pub fn get_all_gpu_data() -> Vec<GpuData> {
    match Nvml::init() {
        Ok(nvml) => {
            let device_count = nvml.device_count().unwrap_or(0);
            
            (0..device_count)
                .filter_map(|i| {
                    nvml.device_by_index(i as u32).ok().and_then(|device| {
                        let name = device.name().unwrap_or_else(|_| format!("GPU {}", i));
                        
                        let usage_percent = device
                            .utilization_rates()
                            .map(|rates| rates.gpu as u8)
                            .unwrap_or(0);
                        
                        let (memory_percent, memory_used_gb, memory_total_gb) = device
                            .memory_info()
                            .map(|info| {
                                let percent = ((info.used as f64 / info.total as f64) * 100.0) as u8;
                                let used_gb = info.used as f32 / (1024.0 * 1024.0 * 1024.0);
                                let total_gb = info.total as f32 / (1024.0 * 1024.0 * 1024.0);
                                (percent, used_gb, total_gb)
                            })
                            .unwrap_or((0, 0.0, 0.0));
                        
                        Some(GpuData {
                            usage_percent,
                            memory_percent,
                            memory_used_gb,
                            memory_total_gb,
                            name,
                        })
                    })
                })
                .collect()
        }
        Err(_) => {
            // Fallback to mock data if NVML is not available - show 2 GPUs for demo
            (0..2)
                .map(|i| GpuData {
                    usage_percent: (i * 25 + 10) as u8, // Different values for demo
                    memory_percent: (i * 30 + 20) as u8,
                    memory_used_gb: (i as f32 * 2.0 + 3.5), // Mock data
                    memory_total_gb: 12.0, // Mock 12GB total
                    name: format!("GPU {} (No NVML)", i),
                })
                .collect()
        }
    }
}

// Keep the old function for backward compatibility during transition
pub fn generate_random_data(n: usize) -> Vec<u8> {
    get_gpu_data(n).iter().map(|gpu| gpu.usage_percent).collect()
}
