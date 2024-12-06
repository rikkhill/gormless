use rust_gpu_tools::{Framework, Device};

pub fn list_it() {
    // Query all GPU devices

    let devices = Device::all();

    println!("Device length: {}", devices.len());

    for (index, device) in devices.iter().enumerate() {
        println!("Device {}: {:?}", index, device.name());
    }
}