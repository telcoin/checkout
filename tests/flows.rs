use checkout::*;

fn client() -> Option<Client> {
    dotenvy::dotenv().ok();
    Client::from_env().ok()
}

#[tokio::test]
async fn payment_session_request_processed() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };
    let request = CreatePaymentSessionRequest::builder()
        .amount(2000)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test")
        .billing(
            BillingInformation::builder()
                .address(
                    Address::builder()
                        .address_line1("123 Test St")
                        .city("London")
                        .zip("W1T 4TJ")
                        .country("GB")
                        .build(),
                )
                .build(),
        )
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client
        .flows()
        .create_payment_session(&request)
        .await
        .unwrap();

    println!("Response: {:#?}", response);

    assert!(response.id.starts_with("ps_"));
    assert!(response.payment_session_secret.starts_with("pss_"));
}

#[tokio::test]
async fn payment_session_request_invalid_processing_channel_id() {
    let Some(client) = client() else { return };
    let processing_channel_id = "invalid_channel_id".to_string();

    let request = CreatePaymentSessionRequest::builder()
        .amount(0)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test")
        .billing(
            BillingInformation::builder()
                .address(
                    Address::builder()
                        .address_line1("123 Test St")
                        .city("London")
                        .zip("W1T 4TJ")
                        .country("GB")
                        .build(),
                )
                .build(),
        )
        .customer(
            CustomerDescriptor::builder()
                .email("test@example.com")
                .name("Test User")
                .build(),
        )
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client.flows().create_payment_session(&request).await;

    println!("Response: {:#?}", response);

    let Err(Error::Api { status_code, error }) = response else {
        panic!("Expected Api error");
    };

    assert_eq!(status_code, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(error.error_type, "validation_error");
    assert!(
        error
            .error_codes
            .iter()
            .any(|code| code == "processing_channel_id_invalid")
    );
}
