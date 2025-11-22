mod tests {
    use telemetry_core::iracing_client::Client;

    #[tokio::test]
    async fn irsdk_start_up_should_return_200() {
        let mut irsdk = Client::default();
        let response = irsdk.init().await;

        assert!(response.is_ok(), "start_up should succeed.")
    }

    #[tokio::test]
    async fn list_all_available_variables() {
        let mut irsdk = Client::default();

        irsdk.init().await.expect("Failed to start IRSDK");

        let var_headers_hash = &irsdk.cache.var_headers_hash;

        println!(
            "\n===== Available Variables ({}) =====",
            var_headers_hash.len()
        );

        for (i, name) in var_headers_hash.iter().enumerate() {
            if i < 50 {
                println!(
                    "\t{}:\t{}",
                    i + 1,
                    name.1.name_str().expect("Failed to get name_str")
                );
            }
        }
        println!("  ... and {} more", var_headers_hash.len() - 20);
    }

    #[tokio::test]
    async fn test_data_valid_event() {
        let mut sdk = Client::default();

        sdk.init().await.expect("Failed to start IRSDK");

        let data = sdk
            .mmap
            .data_valid_event
            .expect("Failed to get data valid event");

        assert!(
            data.0 != std::ptr::null_mut(),
            "Event handle should be valid"
        );
    }
}
