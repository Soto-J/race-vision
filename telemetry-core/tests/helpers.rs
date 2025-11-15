#![allow(unused)]

use telemetry_core::client::IracingClient;
use std::{
    env,
    error::{self, Error},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct TestApp {
    pub client: IracingClient,
}

impl TestApp {
    pub fn new() -> Self {
        let client = IracingClient::default();

        Self { client }
    }

    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.start_up(None, None).await?;
        Ok(())
    }

    pub fn create_test_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn error::Error>> {
        if !self.client.is_initialized {
            return Err("irsdk not initialized".into());
        }

        let shared_mem = self
            .client
            .mmap
            .snapshot
            .as_ref()
            .ok_or("No shared memory found")?;

        let mut file = File::create(path)?;

        file.write_all(shared_mem)?;

        Ok(())
    }
}

#[tokio::test]
async fn test_create_test_file() {
    let mut app = TestApp::new();

    app.client.start_up(None, None).await.unwrap();

    let mut path = env::current_dir().unwrap();
    path.push("tests/test_irsdk.bin");

    let response = app.create_test_file(&path);

    assert!(response.is_ok())
}

mod test {
    use super::*;

    #[tokio::test]
    async fn test_that_the_test_file_works() {
        let mut app = TestApp::new();

        let mut test_file_path = std::env::current_dir().unwrap();
        test_file_path.push("tests");
        test_file_path.push("test_irsdk.bin");

        println!("Exists? {}", test_file_path.exists());

        let response = app.client.start_up(Some(test_file_path), None).await;

        match &response {
            Ok(_) => println!("Startup succeeded"),
            Err(e) => println!("Startup failed: {}", e),
        }

        assert!(response.is_ok(), "Failed to start up with test file")
    }
}
