use btleplug::api::{Central, BDAddr, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral};
use uuid::{uuid, Uuid};
use std::thread;
use tokio;

const HID_UUID: Uuid = uuid!("00001124-0000-1000-8000-00805f9b34fb");

struct HidDevice {
    device_id: BDAddr
    //TODO In original code its use Bluez DeviceID https://docs.rs/bluez-async/latest/bluez_async/struct.DeviceId.html i don't know how clone it
}
#[tokio::main]
impl HidDevice {
    async fn new(device_id: Option<&str>) -> HidDevice {
        let manager = Manager::new().await.unwrap();

        // get the first bluetooth adapter
        let adapters = manager.adapters().await?;
        let central = adapters.into_iter().nth(0).unwrap();
    }
}

