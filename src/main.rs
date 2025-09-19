use tokio;
use btleplug::api::{Central, Manager as _, Peripheral as _,CharPropFlags, WriteType, Characteristic, ScanFilter};
use btleplug::platform::{Adapter, Manager,Peripheral};
use tokio::time::{sleep,Duration};
use std::error::Error;

const TARGET_DEVICE_NAME: &str = "QHM-D03D";

pub struct Scanner {
    adapter: Adapter
}

impl Scanner {
    pub async fn try_create() -> Result<Self, Box<dyn std::error::Error>> {
        
        let manager = Manager::new().await.unwrap();
        //get the adapter
        let central = manager
            .adapters()
            .await
            .expect("No adapters found.")
            .into_iter()
            .nth(0)
            .unwrap();

        //println!("Using adapter: {}", central.adapter_info().await.unwrap());

        Ok(Self { adapter: central })
    }


    pub async fn connect(&self, id: &str)-> Result<Peripheral, Box<dyn std::error::Error>>  {
        //println!("Connecting to peripheral with ID: {}", id);
        let peripherals = self.scan().await?;

        if let Some(peripheral) = peripherals.into_iter().find(|p| p.id().to_string() == id) {
            peripheral.connect().await?;
            Ok(peripheral)
        } else {
            Err(format!("Peripheral with address {} not found", id).into())
        }
    }



    pub async fn scan(&self) -> Result<Vec<Peripheral>, Box<dyn std::error::Error>> {
        // start scanning
        self.adapter.start_scan(ScanFilter::default()).await?;
        sleep(Duration::from_secs(2)).await;

        // get the list of peripherals
        let peripherals = self.adapter.peripherals().await?;

        Ok(peripherals)
    }


}

pub struct Controller {
    peripheral: Option<Peripheral>,
    char: Option<Characteristic>,
}
impl Controller {

    pub fn new() -> Self {
        Self {
            peripheral: None,
            char: None
        }
    }

    
    pub fn set_peripheral(&mut self, p: &Peripheral) {
        self.peripheral = Some(p.clone());
    }

    pub async fn set_char(&mut self, c: &Characteristic) {
        self.char = Some(c.clone());
    }

    pub async fn set_power(&self, state: bool) -> Result<(), Box<dyn Error>> {
    let data: [u8; 3] = if state { [204, 35, 51] } else { [204, 36, 51] };
    self.peripheral
        .as_ref()
        .unwrap()
        .write(
            &self.char.as_ref().unwrap(),
            &data,
            WriteType::WithoutResponse,
        )
        .await?;
    Ok(())
}

pub async fn disconnect(&self) {
        self.peripheral.as_ref().unwrap().disconnect().await.unwrap();
    }

}


async fn find_target_id(scanner: &Scanner) -> Result<String, Box<dyn std::error::Error>> {
    let peripherals = scanner.scan().await?; // get all peripherals
    //find the target device by name
    for p in peripherals {
        if let Some(props) = p.properties().await? {
            if let Some(name) = props.local_name {
                if name == TARGET_DEVICE_NAME {
                    let id = p.id().to_string();
                    //println!("Found target device: {} with ID: {}", name, id);
                    return Ok(id);
                }
            }
        }
    }

    Err("Device not found".into())
}




#[tokio::main]
async fn main() {
    let scanner = Scanner::try_create().await.unwrap();
    let mut controller = Controller::new();

    let id = find_target_id(&scanner).await.unwrap();

    let peripheral = scanner.connect(&id).await.unwrap();   
    peripheral.discover_services().await.unwrap();
    let characteristics = peripheral.characteristics();
    controller.set_peripheral(&peripheral);
    let char = characteristics.iter().find(|c| c.properties.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE | CharPropFlags::WRITE)).unwrap();
    controller.set_char(char).await;

    controller.set_power(true).await.unwrap();
    sleep(Duration::from_millis(250)).await;
    controller.set_power(false).await.unwrap();
    sleep(Duration::from_millis(250)).await;
    controller.set_power(true).await.unwrap();
    sleep(Duration::from_millis(250)).await;
    controller.set_power(false).await.unwrap();
    sleep(Duration::from_millis(250)).await;
    controller.set_power(true).await.unwrap();
    sleep(Duration::from_millis(250)).await;
    controller.set_power(false).await.unwrap();

    controller.disconnect().await;
    println!("Disconnected.");


}





    
