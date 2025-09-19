use tokio;
use btleplug::api::{Central, Manager as _, ScanFilter, Peripheral as _};
use btleplug::platform::{Manager};
use tokio::time::{Duration};
use tokio::time::sleep;

const TARGET_DEVICE_NAME: &str = "QHM-D03D";


#[tokio::main]
async fn main() {

    // Create the manager
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let central = manager
    .adapters()
    .await
    .expect("No adapters found.")
    .into_iter()
    .nth(0)
    .unwrap();
    println!("Using adapter: {}", central.adapter_info().await.unwrap());


    // start scanning for devices and get a list of peripherals
    central.start_scan(ScanFilter::default()).await.unwrap();
    sleep(Duration::from_secs(2)).await;

    let peripherals = central.peripherals().await.expect("No peripherals found");

    for p in peripherals{
    let name = p.properties().await.unwrap().unwrap().local_name.unwrap_or("unknown".to_string());
    println!("Found {}", name);
    }
   

}




    
