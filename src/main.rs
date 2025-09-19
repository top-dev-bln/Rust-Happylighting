use tokio;
use btleplug::api::{Central, Manager as _};
use btleplug::platform::{Manager};




#[tokio::main]
async fn main() {
    let manager = Manager::new().await.unwrap();

    let central = manager
    .adapters()
    .await
    .expect("No adapters found.")
    .into_iter()
    .nth(0)
    .unwrap();

   println!("Using adapter: {}", central.adapter_info().await.unwrap());

}




    
