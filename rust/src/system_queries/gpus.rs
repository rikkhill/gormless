use ocl::{Platform, Device, Result as OclResult};
use std::error::Error;

pub fn has_gpus() -> Result<bool, Box<dyn Error>> {
    // Get all available OpenCL platforms
     // Try to list OpenCL platforms
    let platforms_result: OclResult<Vec<Platform>> = Ok(Platform::list());

    match platforms_result {
        Ok(platforms) if !platforms.is_empty() => {
            Ok(true) // No GPU devices found, even though platforms exist
        }
        Ok(_) => {
            // No platforms found
            Ok(false)
        }
        Err(e) => {
            // Error retrieving platform list (e.g., OpenCL not supported or installed)
            Err(Box::new(e))
        }
    }
}

pub fn get_gpu_devices() -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut device_info = Vec::new();

    // Iterate over all OpenCL platforms
    for platform in Platform::list() {
        // Get all GPU devices for the platform
        let devices = Device::list_all(&platform)?;

        for device in devices {
            let name = device.name()?;
            let vendor = device.vendor()?;
            device_info.push((name, vendor));
        }
    }

    Ok(device_info)
}
