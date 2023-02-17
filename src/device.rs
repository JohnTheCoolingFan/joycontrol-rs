use btleplug::api::{BDAddr, Central, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::thread;
use tokio;
use uuid::{uuid, Uuid};

const HID_UUID: Uuid = uuid!("00001124-0000-1000-8000-00805f9b34fb");

struct HidDevice {
    device_id: BDAddr, //TODO In original code it uses Bluez DeviceID https://docs.rs/bluez-async/latest/bluez_async/struct.DeviceId.html and I don't know how to clone it
}

impl HidDevice {
    async fn new(device_id: Option<&str>) -> Result<HidDevice, btleplug::Error> {
        let manager = Manager::new().await?;

        // get the first bluetooth adapter
        let adapters = manager.adapters().await?;
        let central = adapters.into_iter().nth(0).unwrap();
        todo!()
    }
}
