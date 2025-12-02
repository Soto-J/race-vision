#[cfg(test)]
use memmap2::MmapOptions;
use std::{env, fs::File, io::Write, path::Path, sync::Arc};
use telemetry_core::{domain::iracing_errors::ClientError, iracing_client::Client};

#[cfg(test)]
pub struct TestApp {
    pub client: Client,
}

#[cfg(test)]
impl TestApp {
    pub fn new() -> Result<Self, ClientError> {
        Ok(Self {
            client: Client::default(),
        })
    }

    pub fn create_test_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ClientError> {
        use telemetry_core::domain::iracing_errors::SharedMemoryError;

        if !self.client.is_initialized {
            return Err(ClientError::NotConnected);
        }

        let snapshot = self
            .client
            .mmap
            .snapshot
            .as_ref()
            .ok_or_else(|| SharedMemoryError::InvalidSharedMemory("Memory not found"))?;

        let mut file = File::create(path)?;

        file.write_all(snapshot)?;

        Ok(())
    }

    pub fn use_test_file<P: AsRef<Path>>(&self, test_file: P) -> Result<Self, ClientError> {
        use telemetry_core::domain::iracing_errors::MMapError;

        let mut client = Client::default();

        let file = File::open(test_file)?;
        let mmap = unsafe { MmapOptions::new().map(&file) }
            .map_err(|e| MMapError::OpenFileMappingFailed(e.to_string()))?;
        let snapshot = Arc::from(mmap.as_ref());

        client.cache.parse_headers(&snapshot)?;
        client.mmap.snapshot = Some(snapshot);

        client.is_initialized = true;

        Ok(Self { client })
    }

    pub fn write_debug_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ClientError> {
        use telemetry_core::domain::iracing_errors::{MMapError, ResolverError};

        if !self.client.is_initialized {
            return Err(ClientError::NotConnected);
        }

        let mut file = File::create(path)?;

        let memory = self
            .client
            .mmap
            .snapshot
            .as_ref()
            .ok_or_else(|| MMapError::SnapshotNotFound)?;

        let header = self
            .client
            .cache
            .header
            .as_ref()
            .ok_or_else(|| ResolverError::HeaderNotFound)?;

        let offset = header.session_info_offset() as usize;
        let len = header.session_info_len() as usize;

        /* Write YAML */
        file.write_all(&memory[offset..offset + len])?;
        file.write_all(b"\n\n")?;

        Ok(())
    }

    pub async fn init(&mut self) -> Result<(), ClientError> {
        self.client.init().await?;
        Ok(())
    }
}

mod test {
    use super::*;

    #[tokio::test]
    async fn test_create_test_file() {
        let mut app = TestApp::new().expect("Failed to create test app");

        app.client.init().await.expect("Client start up failed");

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

        let response = app.client.init().await;

        assert!(response.is_ok(), "Failed to start up with test file")
    }

    #[tokio::test]
    async fn test_write_debug_file() {
        todo!()
    }
}
