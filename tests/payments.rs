use bigdecimal::BigDecimal;

use checkout::Client;
use checkout::models::payments::{CreatePaymentRequest, CreatePaymentResponse};
use checkout::models::shared::{
    Address, Amount, Currency, DestinationAccountHolder, DestinationInstruction,
    PaymentProcessedSource, PaymentRequestDestination, PaymentRequestSource, PaymentSenderDetails,
    PaymentStatus,
};

fn client() -> Option<Client> {
    dotenvy::dotenv().ok();
    Client::from_env().ok()
}

fn create_payment(
    number: String,
    month: u32,
    year: u32,
    cvv: Option<String>,
    amount: BigDecimal,
    processing_channel_id: String,
) -> CreatePaymentRequest {
    // The Checkout sandbox uses certain card numbers, expiration dates,
    // cvvs, and amounts to trigger failure cases.
    //
    // https://docs.checkout.com/testing

    CreatePaymentRequest::builder()
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .source(PaymentRequestSource::Card {
            number,
            expiry_month: month,
            expiry_year: year,
            name: None,
            cvv,
            stored: None,
            billing_address: None,
            phone: None,
        })
        .amount(Amount::from(Currency::USD, amount))
        .build()
}

fn create_payout(
    number: String,
    month: u32,
    year: u32,
    amount: BigDecimal,
    processing_channel_id: String,
    currency_account_id: String,
) -> CreatePaymentRequest {
    // The Checkout sandbox uses certain card numbers, expiration dates,
    // cvvs, and amounts to trigger failure cases.
    //
    // https://docs.checkout.com/testing

    CreatePaymentRequest::builder()
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .source(PaymentRequestSource::CurrencyAccount {
            id: currency_account_id,
        })
        .destination(PaymentRequestDestination::Card {
            number,
            expiry_month: month,
            expiry_year: year,
            account_holder: DestinationAccountHolder::Individual {
                first_name: Some("Test".to_owned()),
                last_name: Some("User".to_owned()),
                middle_name: None,
            },
        })
        .amount(Amount::from(Currency::USD, amount))
        .instruction(DestinationInstruction {
            funds_transfer_type: Some("FT".to_owned()),
            purpose: None,
        })
        .sender(PaymentSenderDetails::Individual {
            first_name: "Test".to_owned(),
            middle_name: None,
            last_name: "User".to_owned(),
            date_of_birth: None,
            country_of_birth: None,
            nationality: None,
            address: Address {
                address_line1: Some("123 Main St".to_owned()),
                address_line2: None,
                city: Some("Los Angeles".to_owned()),
                state: Some("CA".to_owned()),
                zip: Some("90051".to_owned()),
                country: "US".to_owned(),
            },
            reference: "12345678".to_owned(),
            reference_type: "other".to_owned(),
            source_of_funds: "mobile_money_account".to_owned(),
        })
        .build()
}

#[tokio::test]
async fn payment_request_processed() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };
    let payment = create_payment(
        "4242424242424242".to_string(),
        6,
        2027,
        None,
        BigDecimal::try_from(20.00).unwrap(),
        processing_channel_id,
    );
    let payment: &'static _ = Box::leak(Box::new(payment));

    let response = client.payments().create_payment(payment).await.unwrap();

    let processed_payment = match response {
        CreatePaymentResponse::Processed(processed) => processed,
        CreatePaymentResponse::Pending(pending) => panic!("response is pending: {:?}", pending),
    };

    assert_eq!(processed_payment.approved, Some(true));
    assert_eq!(processed_payment.status, PaymentStatus::Authorized);

    match processed_payment.source {
        Some(PaymentProcessedSource::Card {
            expiry_month,
            expiry_year,
            last4,
            ..
        }) => {
            assert_eq!(expiry_month, 6);
            assert_eq!(expiry_year, 2027);
            assert_eq!(last4, "4242".to_string());
        }
        other => panic!("payment source is not card: {:?}", other),
    };
}

#[tokio::test]
async fn payment_request_declined() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };
    let payment = create_payment(
        "4242424242424242".to_string(),
        6,
        2027,
        None,
        BigDecimal::try_from(123.05).unwrap(),
        processing_channel_id,
    );
    let payment: &'static _ = Box::leak(Box::new(payment));

    let response = client.payments().create_payment(payment).await;

    assert!(response.is_ok());
}

#[tokio::test]
async fn payment_request_invalid() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };
    let payment = create_payment(
        "4242424242424242".to_string(),
        6,
        2027,
        Some("100".to_string()),
        BigDecimal::try_from(123.12).unwrap(),
        processing_channel_id,
    );
    let payment: &'static _ = Box::leak(Box::new(payment));

    let response = client.payments().create_payment(payment).await;

    assert!(response.is_ok());
}

#[tokio::test]
async fn payout_request_processed() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };
    let Ok(currency_account_id) = std::env::var("CKO_CURRENCY_ACCOUNT_ID") else {
        return;
    };
    let payment = create_payout(
        "4242424242424242".to_string(),
        6,
        2027,
        BigDecimal::try_from(20.00).unwrap(),
        processing_channel_id,
        currency_account_id,
    );
    let payment: &'static _ = Box::leak(Box::new(payment));

    let response = client.payments().create_payment(payment).await.unwrap();

    let processed_payment = match response {
        CreatePaymentResponse::Processed(processed) => processed,
        CreatePaymentResponse::Pending(pending) => panic!("response is pending: {:?}", pending),
    };

    assert_eq!(processed_payment.approved, Some(true));
    assert_eq!(processed_payment.status, PaymentStatus::Authorized);

    match processed_payment.source {
        Some(PaymentProcessedSource::Card {
            expiry_month,
            expiry_year,
            last4,
            ..
        }) => {
            assert_eq!(expiry_month, 6);
            assert_eq!(expiry_year, 2027);
            assert_eq!(last4, "4242".to_string());
        }
        other => panic!("payment source is not card: {:?}", other),
    };
}
