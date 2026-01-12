//! Client library for the [Checkout](https://www.checkout.com) API.
//!
//! Documentation: <https://docs.checkout.com>
//! API Reference: <https://api-reference.checkout.com>

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all, clippy::pedantic)]

use std::{convert::TryFrom, fmt, str::FromStr};

use reqwest::{Client as ReqwestClient, Error as ReqwestError, Response, StatusCode};
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod flows;
pub mod payments;
pub(crate) mod types;

pub use flows::*;
pub use payments::*;
pub use types::*;

/// An error that was reported by the Checkout API
#[derive(Deserialize, Debug)]
pub struct ApiError {
    /// The unique identifier of the request
    pub request_id: String,

    /// The type of error
    pub error_type: String,

    /// A list of errors
    pub error_codes: Vec<String>,
}

/// Encapsulates any error that can occur when sending a request to the
/// Checkout API
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error that was reported by the Checkout API
    #[error("API error: {0:?}")]
    Api(ApiError),

    /// Not authorized
    #[error("Unauthorized")]
    Unauthorized,

    /// Invalid data was sent
    #[error("Invalid data: {0:?}")]
    InvalidData(ApiError),

    /// To many requests or duplicate request detected
    #[error("Too many requests")]
    TooManyRequests,

    /// An unknown error occurred
    #[error("Unknown error: {0:?} {1:?}")]
    Unknown(StatusCode, String),

    /// An error that occurred during transport
    #[error("Transport error: {0}")]
    Transport(#[from] ReqwestError),

    /// An error that occurred while reading environment variables
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    /// An error that occurred while parsing the environment
    #[error("Parse environment error: {0:?}")]
    ParseEnvironment(#[from] ParseEnvironmentError),
}

/// Could not parse an environment, contains the original string.
#[derive(thiserror::Error, Debug)]
#[error("Could not parse environment: {0}")]
pub struct ParseEnvironmentError(pub String);

/// API environments to differentiate between testing environments and live.
#[derive(PartialEq, Copy, Clone, Debug)]
#[allow(missing_docs)]
pub enum Environment {
    Production,
    Sandbox,
}

impl FromStr for Environment {
    type Err = ParseEnvironmentError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "prod" | "production" => Ok(Environment::Production),
            "dev" | "development" | "sandbox" => Ok(Environment::Sandbox),
            val => Err(ParseEnvironmentError(val.to_owned())),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = ParseEnvironmentError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<'a> TryFrom<&'a str> for Environment {
    type Error = ParseEnvironmentError;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl fmt::Display for Environment {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let env = match &self {
            Environment::Production => "production",
            Environment::Sandbox => "sandbox",
        };
        write!(f, "{env}")
    }
}

impl Environment {
    /// Returns the appropriate url for the api depending on the environment
    pub fn api_url(&self) -> &str {
        match self {
            Environment::Sandbox => "https://api.sandbox.checkout.com",
            Environment::Production => "https://api.checkout.com",
        }
    }

    /// Returns the appropriate url for authentication depending on the
    /// environment
    #[must_use]
    pub fn access_url(&self) -> &str {
        match self {
            Environment::Sandbox => "https://access.sandbox.checkout.com",
            Environment::Production => "https://access.checkout.com",
        }
    }
}

/// A client that can be used to access the Checkout API
#[derive(Clone, Debug)]
pub struct Client {
    http_client: ReqwestClient,
    environment: Environment,
    username: SecretString,
    password: SecretString,
}

impl Client {
    /// Creates a new client
    #[must_use]
    pub fn new(username: SecretString, password: SecretString, environment: Environment) -> Client {
        Client {
            http_client: ReqwestClient::new(),
            environment,
            username,
            password,
        }
    }

    /// Creates a new `Client` from the following environment variables:
    ///
    /// - `CKO_ENVIRONMENT`
    /// - `CKO_USERNAME`
    /// - `CKO_PASSWORD`
    ///
    /// # Errors
    ///
    /// - [`Error::EnvVar`]
    /// - [`Error::ParseEnvironment`]
    pub fn from_env() -> Result<Client, Error> {
        Ok(Client::new(
            SecretString::new(std::env::var("CKO_USERNAME")?.into()),
            SecretString::new(std::env::var("CKO_PASSWORD")?.into()),
            std::env::var("CKO_ENVIRONMENT")?.parse()?,
        ))
    }

    async fn authorize(&self, scope: &str) -> Result<String, Error> {
        let url = format!("{}/connect/token", self.environment.access_url());
        let body = OAuthTokenRequest {
            grant_type: "client_credentials".to_string(),
            scope: scope.to_owned(),
        };

        let response = self
            .http_client
            .post(&url)
            .basic_auth(
                self.username.expose_secret(),
                Some(self.password.expose_secret()),
            )
            .form(&body)
            .send()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: OAuthTokenResponse = response.json().await?;
                Ok(body.access_token)
            }
            _ => Err(Error::Unauthorized),
        }
    }

    async fn send_get_request<R>(&self, scope: &str, url: &str) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        let token = self.authorize(scope).await?;

        let response = self.http_client.get(url).bearer_auth(token).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Error::Api(response.json().await?))
        }
    }

    async fn send_post_request<B, R>(&self, scope: &str, url: &str, body: &B) -> Result<R, Error>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let token = self.authorize(scope).await?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(token)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Error::Api(response.json().await?))
        }
    }

    async fn send_post_request_2<B>(
        &self,
        scope: &str,
        url: &str,
        body: &B,
    ) -> Result<Response, Error>
    where
        B: Serialize,
    {
        let token = self.authorize(scope).await?;

        self.http_client
            .post(url)
            .bearer_auth(token)
            .json(body)
            .send()
            .await
            .map_err(Error::from)
    }

    /// Access the Payments API.
    pub fn payments(&self) -> Payments<'_> {
        Payments::new(self)
    }

    /// Access the Flows API.
    pub fn flows(&self) -> Flows<'_> {
        Flows::new(self)
    }

    /// Returns a single metadata record for the card specified by the Primary
    /// Account Number (PAN), Bank Identification Number (BIN), token, or
    /// instrument supplied.
    pub async fn get_card_metadata(
        &self,
        source: CardMetadataSource,
        format: Option<CardMetadataFormat>,
    ) -> Result<CardMetadataResponse, Error> {
        let body = CardMetadataRequest { source, format };
        let url = format!("{}/metadata/card", self.environment.api_url());

        self.send_post_request("vault:card-metadata", &url, &body)
            .await
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use once_cell::sync::OnceCell;

    use super::*;

    fn client() -> Option<&'static Client> {
        dotenvy::dotenv().ok();
        static INSTANCE: OnceCell<Client> = OnceCell::new();
        INSTANCE.get_or_try_init(Client::from_env).ok()
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
                    country: Some("US".to_owned()),
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
            2025,
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
                assert_eq!(expiry_year, 2025);
                assert_eq!(last4, "4242".to_string());
            }
            other => panic!("payment source is not card: {:?}", other),
        };
    }

    #[tokio::test]
    #[ignore] // response code is 10000 (Approved) even with XXX05 as the amount
    async fn payment_request_declined() {
        let Some(client) = client() else { return };
        let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
            return;
        };
        let payment = create_payment(
            "4242424242424242".to_string(),
            6,
            2025,
            None,
            BigDecimal::try_from(123.05).unwrap(),
            processing_channel_id,
        );
        let payment: &'static _ = Box::leak(Box::new(payment));

        let response = client.payments().create_payment(payment).await;

        assert!(matches!(response, Ok(_)));
    }

    #[ignore] // response code is 10000 (Approved) even with XXX12 as the amount
    #[tokio::test]
    async fn payment_request_invalid() {
        let Some(client) = client() else { return };
        let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
            return;
        };
        let payment = create_payment(
            "4242424242424242".to_string(),
            6,
            2025,
            Some("100".to_string()),
            BigDecimal::try_from(123.12).unwrap(),
            processing_channel_id,
        );
        let payment: &'static _ = Box::leak(Box::new(payment));

        let response = client.payments().create_payment(payment).await;

        assert!(matches!(response, Ok(_)));
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
            2025,
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
                assert_eq!(expiry_year, 2025);
                assert_eq!(last4, "4242".to_string());
            }
            other => panic!("payment source is not card: {:?}", other),
        };
    }

    #[tokio::test]
    async fn request_card_metadata() {
        let Some(client) = client() else { return };
        let response = client
            .get_card_metadata(
                CardMetadataSource::Card {
                    number: "4242424242424242".to_owned(),
                },
                None,
            )
            .await
            .unwrap();

        assert_eq!(response.scheme, "visa");
        assert_eq!(response.issuer_country.as_deref(), Some("GB"));
    }

    #[tokio::test]
    async fn payment_session_request_processed() {
        let Some(client) = client() else { return };
        let request = CreatePaymentSessionRequest {
            amount: 2000,
            currency: Currency::USD,
            reference: "rust-sdk-test".to_string(),
            billing: None,
            customer: None,
            success_url: "https://example.com/success".to_string(),
            failure_url: "https://example.com/failure".to_string(),
        };

        let response = client.flows().create_payment_session(&request).await.unwrap();

        assert!(response.id.starts_with("ps_"));
        assert!(response.payment_session_secret.starts_with("pss_"));
    }

    #[tokio::test]
    async fn payment_session_request_declined() {
        let Some(client) = client() else { return };
        let request = CreatePaymentSessionRequest {
            amount: 0,
            currency: Currency::USD,
            reference: "rust-sdk-test".to_string(),
            billing: None,
            customer: None,
            success_url: "https://example.com/success".to_string(),
            failure_url: "https://example.com/failure".to_string(),
        };

        let response = client.flows().create_payment_session(&request).await;

        assert!(matches!(response, Err(Error::InvalidData(_))));
    }
}
