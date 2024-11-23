use ocl::{Platform, Device};
use std::error::Error;
use std::any::Any;
use std::panic;

pub fn has_gpus() -> Result<bool, Box<dyn Error>> {

    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(|_info| {
        // Do nothing on panic
    }));

    // Get all available OpenCL platforms
    // Try to list OpenCL platforms
    let platforms_result: Result<Vec<Platform>, Box<dyn Any + Send>> = std::panic::catch_unwind(|| {
        Platform::list()
    });

    panic::set_hook(original_hook);

    match platforms_result {
        Ok(platforms) if !platforms.is_empty() => {
            //Ok(true) // No GPU devices found, even though platforms exist
            Ok(true)
        }
        Ok(_) => {
            // No platforms found
            //Ok(false)
            Ok(false)
        }
        Err(_e) => {
            // Error retrieving platform list (e.g., OpenCL not supported or installed)
            //Err(Box::new(e))
            Ok(false)
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
