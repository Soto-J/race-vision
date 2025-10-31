#![allow(unused)]

use race_vision::sdk::irsdk::IRSDK;
use std::{
    env,
    error::{self, Error},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct TestApp {
    pub irsdk: IRSDK,
}

impl TestApp {
    pub fn new() -> Self {
        let irsdk = IRSDK::default();

        Self { irsdk }
    }

    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        self.irsdk.start_up(None, None).await?;
        Ok(())
    }

    pub fn create_test_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn error::Error>> {
        if !self.irsdk.is_initialized {
            return Err("irsdk not initialized".into());
        }

        let shared_mem = self
            .irsdk
            .shared_mem
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

    app.irsdk.start_up(None, None).await.unwrap();

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

        let response = app.irsdk.start_up(Some(test_file_path), None).await;

        match &response {
            Ok(_) => println!("Startup succeeded"),
            Err(e) => println!("Startup failed: {}", e),
        }

        assert!(response.is_ok(), "Failed to start up with test file")
    }
}
