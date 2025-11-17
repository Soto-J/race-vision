#![allow(unused)]

#[cfg(test)]
use color_eyre::eyre::{self, ContextCompat, Ok, eyre};
use memmap2::MmapOptions;
use std::{
    env,
    error::{self, Error},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};
use telemetry_core::client::IracingClient;

#[cfg(test)]
pub struct TestApp {
    pub client: IracingClient,
}

#[cfg(test)]
impl TestApp {
    pub fn new() -> eyre::Result<Self> {
        Ok(Self {
            client: IracingClient::default(),
        })
    }

    pub fn create_test_file<P: AsRef<Path>>(&self, path: P) -> eyre::Result<()> {
        if !self.client.is_initialized {
            return Err(eyre!("irsdk not initialized"));
        }

        let snapshot = self
            .client
            .mmap
            .snapshot
            .as_ref()
            .ok_or(eyre!("No shared memory found"))?;

        let mut file = File::create(path)?;

        file.write_all(snapshot)?;

        Ok(())
    }

    pub fn use_test_file<P: AsRef<Path>>(&self, test_file: P) -> eyre::Result<Self> {
        let mut client = IracingClient::default();

        let file = File::open(test_file)?;
        let mmap = unsafe { MmapOptions::new().map(&file) }?;
        let snapshot = Arc::from(mmap.as_ref());

        client.cache.parse_headers(&snapshot)?;
        client.mmap.snapshot = Some(snapshot);

        client.is_initialized = true;

        Ok(Self { client })
    }

    pub fn write_debug_file<P: AsRef<Path>>(&self, path: P) -> eyre::Result<()> {
        if !self.client.is_initialized {
            use color_eyre::eyre::eyre;

            return Err(eyre!("IRSDK client not initialized"));
        }

        let mut file = File::create(path)?;

        let memory = self
            .client
            .mmap
            .snapshot
            .as_ref()
            .ok_or_else(|| eyre!("Snapshot not found"))?;

        let header = self
            .client
            .cache
            .header
            .as_ref()
            .ok_or_else(|| eyre!("Header not found"))?;

        let offset = header.session_info_offset() as usize;
        let len = header.session_info_len() as usize;

        /* Write YAML */
        file.write_all(&memory[offset..offset + len])?;
        file.write_all(b"\n\n")?;

        /* Write Var Headers */
        for (name, var_header) in &self.client.cache.var_headers_hash {
            writeln!(file, "{:32}{}", name, var_header)?;
        }

        Ok(())
    }

    pub async fn init(&mut self) -> eyre::Result<()> {
        self.client.start_up().await?;
        Ok(())
    }
}

mod test {
    use super::*;

    #[tokio::test]
    async fn test_create_test_file() {
        let mut app = TestApp::new().expect("Failed to create test app");

        app.client.start_up().await.expect("Client start up failed");

        let mut path = env::current_dir().unwrap();
        path.push("tests/test_irsdk.bin");

        let response = app.create_test_file(&path);

        assert!(response.is_ok())
    }

    #[tokio::test]
    async fn test_that_the_test_file_works() {
        let mut app = TestApp::new().expect("Failed to start test app");

        let mut test_file_path = std::env::current_dir().unwrap();
        test_file_path.push("tests");
        test_file_path.push("test_irsdk.bin");

        println!("Exists? {}", test_file_path.exists());

        app.use_test_file(test_file_path)
            .expect("Failed to use test file");

        let response = app.client.start_up().await;

        assert!(response.is_ok(), "Failed to start up with test file")
    }

    #[tokio::test]
    async fn test_write_debug_file() {
        todo!()
    }
}
