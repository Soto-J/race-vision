use race_vision::sdk::irsdk::IRSDK;

// Individual Field Tests (10 tests)

#[tokio::test]
async fn test_header_reads_correctly() {
    let mut irsdk = IRSDK::default();

    irsdk
        .start_up(None, None)
        .await
        .expect("Failed to start up");

    let header = irsdk.header.as_ref().expect("No headers found");

    dbg!(header.version(), header.num_vars(), header.buf_len());
    assert_eq!(header.version(), 2);
    assert!(header.num_vars() > 0);
    assert!(header.buf_len() > 0);

    irsdk.shutdown()
}

#[tokio::test]
async fn test_header_version_reads_correctly_with_test_file() {
    let mut irsdk = IRSDK::default();

    let mut test_file_path = std::env::current_dir().unwrap();
    test_file_path.push("tests");
    test_file_path.push("test_irsdk.bin");

    assert!(test_file_path.exists(), "Test file does not exist!");

    irsdk
        .start_up(Some(test_file_path), None)
        .await
        .expect("Failed to start up");

    let shared_mem = irsdk.shared_mem.as_ref().unwrap();
    let version_bytes = &shared_mem[0..4];
    let version = i32::from_le_bytes(version_bytes.try_into().unwrap());

    assert_eq!(version, 2);

    irsdk.shutdown()
}

#[tokio::test]
async fn test_header_status() {}

#[tokio::test]
async fn test_header_tick_rate() {}

#[tokio::test]
async fn test_header_session_info_update() {}

#[tokio::test]
async fn test_header_session_info_len() {}

#[tokio::test]
async fn test_header_session_info_offset() {}

#[tokio::test]
async fn test_header_num_vars() {}

#[tokio::test]
async fn test_header_var_header_offset() {}

#[tokio::test]
async fn test_header_num_buf() {}

#[tokio::test]
async fn test_header_buf_len() {}

// Integration Tests (3 tests)

#[tokio::test]
async fn test_header_all_fields() {}

#[tokio::test]
async fn test_header_realistic_iracing_values() {}

#[tokio::test]
async fn test_header_zero_values() {}

// Edge Cases (3 tests)
#[tokio::test]
async fn test_header_negative_values_handled() {}
#[tokio::test]
async fn test_header_var_buffers_negative_num_buf() {}
#[tokio::test]
async fn test_header_var_buffers_empty() {}
