use race_vision::sdk::IRSDK;
use tokio;

#[tokio::main]
async fn main() {
    let mut irsdk = IRSDK::default();

    irsdk.start_up(None, None).await.unwrap();
    println!("Welcome");
}
