use checkout::*;

fn client() -> Option<Client> {
    dotenvy::dotenv().ok();
    Client::from_env().ok()
}

fn valid_address() -> Address {
    Address::builder()
        .address_line1("123 Test St")
        .city("London")
        .zip("W1T 4TJ")
        .country("GB")
        .build()
}

fn valid_billing() -> BillingInformation {
    BillingInformation::builder()
        .address(valid_address())
        .build()
}

fn valid_customer() -> CustomerDescriptor {
    CustomerDescriptor::builder()
        .email("test@example.com")
        .name("Test User")
        .build()
}

fn assert_api_error(
    response: Result<CreatePaymentSessionResponse, Error>,
    expected_status: StatusCode,
    expected_error_type: &str,
    expected_error_code: &str,
) {
    let Err(Error::Api { status_code, error }) = response else {
        panic!("Expected Api error");
    };

    assert_eq!(status_code, expected_status);
    assert_eq!(error.error_type, expected_error_type);
    assert!(
        error
            .error_codes
            .iter()
            .any(|code| code == expected_error_code),
        "Expected error code '{}' not found in {:?}",
        expected_error_code,
        error.error_codes
    );
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
        .billing(valid_billing())
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
        .reference("rust-sdk-test-invalid-processing-channel-id")
        .billing(valid_billing())
        .customer(valid_customer())
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client.flows().create_payment_session(&request).await;

    println!("Response: {:#?}", response);

    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "processing_channel_id_invalid",
    );
}

#[tokio::test]
async fn payment_session_request_processed_with_invalid_customer_id() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };

    let request = CreatePaymentSessionRequest::builder()
        .amount(2000)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test-invalid-customer-id")
        .billing(valid_billing())
        .customer(CustomerDescriptor::builder().id("cus_1234567890").build())
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client.flows().create_payment_session(&request).await;

    println!("Response: {:#?}", response);

    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "customer_id_invalid",
    );
}

#[tokio::test]
async fn payment_session_request_invalid_customer_email() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };

    let request = CreatePaymentSessionRequest::builder()
        .amount(2000)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test-invalid-customer-email")
        .billing(valid_billing())
        .customer(
            CustomerDescriptor::builder()
                .email("invalid-email")
                .name("Test User")
                .build(),
        )
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client.flows().create_payment_session(&request).await;

    println!("Response: {:#?}", response);

    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "customer_email_invalid",
    );
}
