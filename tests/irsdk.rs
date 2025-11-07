mod tests {
    use race_vision::client::IracingClient;

    #[tokio::test]
    async fn irsdk_start_up_should_return_200() {
        let mut irsdk = IracingClient::default();
        let response = irsdk.start_up(None, None).await;

        assert!(response.is_ok(), "start_up should succeed.")
    }

    // #[tokio::test]
    // async fn fails_when_iracing_closed() {
    //     let mut irsdk = IRSDK::default();
    //     let response = irsdk.start_up(None, None).await;

    //     assert!(
    //         response.is_err(),
    //         "start_up should fail when iRacing is not running"
    //     )
    // }

    #[tokio::test]
    async fn list_all_available_variables() {
        let mut irsdk = IracingClient::default();

        irsdk
            .start_up(None, None)
            .await
            .expect("Failed to start IRSDK");

        let var_headers = &irsdk.var_headers;

        println!("\n===== Available Variables ({}) =====", var_headers.len());

        for (i, name) in var_headers.iter().enumerate() {
            if i < 50 {
                println!(
                    "\t{}:\t{}",
                    i + 1,
                    name.name_str().expect("Failed to get name_str")
                );
            }
        }
        println!("  ... and {} more", var_headers.len() - 20);
    }

    #[tokio::test]
    async fn test_data_valid_event() {
        let mut sdk = IracingClient::default();

        sdk.start_up(None, None)
            .await
            .expect("Failed to start IRSDK");

        let data = sdk
            .data_valid_event
            .expect("Failed to get data valid event");

        assert!(
            data.0 != std::ptr::null_mut(),
            "Event handle should be valid"
        );
    }
}
