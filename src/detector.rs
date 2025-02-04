use sysinfo::{System, SystemExt};
use windows::Win32::System::Performance;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GpuInfo {
    vendor: String,
    model: String,
    driver_version: String,
    vram_mb: u64,
}

pub fn detect_gpu() -> Result<GpuInfo, crate::error::Error> {
    let mut system = System::new();
    system.refresh_all();
    
    let mut gpu = None;
    
    for device in system.get_components() {
        if device.get_label().contains("GPU") {
            let vendor = device.get_vendor_id().unwrap_or_default();
            let model = device.get_product_id().unwrap_or_default();
            
            gpu = Some(GpuInfo {
                vendor: vendor.to_string(),
                model: model.to_string(),
                driver_version: "".to_string(),
                vram_mb: 0,
            });
        }
    }
    
    gpu.ok_or(crate::error::Error::DetectionFailed)
}