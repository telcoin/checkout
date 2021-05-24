//! Client library for the [Checkout](https://www.checkout.com) API.
//!
//! Documentation: <https://docs.checkout.com>
//! API Reference: <https://api-reference.checkout.com>

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all, clippy::pedantic)]

use std::{convert::TryFrom, fmt, str::FromStr};

use futures03::compat::Future01CompatExt;
use reqwest::{r#async::Client as ReqwestClient, Error as ReqwestError, r#async::Response, StatusCode};
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub(crate) mod types;

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
#[error("{0:?}")]
pub enum Error {
    /// An error that was reported by the Checkout API
    Api(ApiError),

    /// Not authorized
    #[error("Unauthorized")]
    Unauthorized,

    /// Invalid data was sent
    InvalidData(ApiError),

    /// To many requests or duplicate request detected
    #[error("TooManyRequests")]
    TooManyRequests,

    /// To many requests or duplicate request detected
    #[error("Unknown({0:?}, {1:?})")]
    Unknown(StatusCode, String),

    /// An error that ocurred during transport
    Transport(#[from] ReqwestError),
}

/// Could not parse an environment, contains the original string.
#[derive(Debug)]
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
        write!(f, "{}", env)
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
    /// - [`std::env::VarError`]
    /// - [`Error::ParseEnvironment`]
    pub fn from_env() -> Result<Client, ParseEnvironmentError> {
        Ok(Client::new(
            SecretString::new(std::env::var("CKO_USERNAME").unwrap()),
            SecretString::new(std::env::var("CKO_PASSWORD").unwrap()),
            std::env::var("CKO_ENVIRONMENT").unwrap().parse()?,
        ))
    }

    async fn authorize(&self) -> Result<String, Error> {
        let url = format!("{}/connect/token", self.environment.access_url());
        let body = OAuthTokenRequest {
            grant_type: "client_credentials".to_string(),
            scope: "gateway".to_string(),
        };

        let mut response = self
            .http_client
            .post(&url)
            .basic_auth(
                self.username.expose_secret(),
                Some(self.password.expose_secret()),
            )
            .form(&body)
            .send()
            .compat()
            .await?;

        let status = response.status();
        match status {
            StatusCode::OK => {
                let body: OAuthTokenResponse = response.json().compat().await?;
                Ok(body.access_token)
            }
            _ => Err(Error::Unauthorized),
        }
    }

    async fn send_get_request<R>(&self, url: &str) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        let token = self.authorize().await?;

        let mut response = self
            .http_client
            .get(url)
            .bearer_auth(token)
            .send()
            .compat()
            .await?;

        if response.status().is_success() {
            Ok(response.json().compat().await?)
        } else {
            Err(Error::Api(response.json().compat().await?))
        }
    }

    async fn send_post_request<B, R>(&self, url: &str, body: &B) -> Result<R, Error>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let token = self.authorize().await?;

        let mut response = self
            .http_client
            .post(url)
            .bearer_auth(token)
            .json(body)
            .send()
            .compat()
            .await?;

        if response.status().is_success() {
            Ok(response.json().compat().await?)
        } else {
            Err(Error::Api(response.json().compat().await?))
        }
    }

    async fn send_post_request_2<B>(&self, url: &str, body: &B) -> Result<Response, Error>
    where
        B: Serialize,
    {
        let token = self.authorize().await?;

        self.http_client
            .post(url)
            .bearer_auth(token)
            .json(body)
            .send()
            .compat()
            .await
            .map_err(Error::from)
    }

    /// Request a payment or payout
    ///
    /// To accept payments from cards, digital wallets and many alternative
    /// payment methods, specify the source.type field, along with the
    /// source-specific data.
    ///
    /// To pay out to a card, specify the destination of your payout using the
    /// destination.type field, along with the destination-specific data.
    ///
    /// To verify the success of the payment, check the approved field in the
    /// response.
    ///
    /// [`POST /payments`](https://api-reference.checkout.com/#operation/requestAPaymentOrPayout)
    pub async fn create_payment(
        &self,
        request: &CreatePaymentRequest,
    ) -> Result<CreatePaymentResponse, Error> {
        let url = format!("{}/payments", self.environment.api_url());
        let mut response = self.send_post_request_2(&url, request).await?;

        let status = response.status();
        match status {
            StatusCode::CREATED => { 
                let body = response.json().compat().await?;
                Ok(CreatePaymentResponse::Processed(body))
            },
            StatusCode::ACCEPTED => {
                let body = response.json().compat().await?;
                Ok(CreatePaymentResponse::Pending(body))
            },
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let body = response.json().compat().await?;
                Err(Error::InvalidData(body))
            },
            StatusCode::TOO_MANY_REQUESTS => {
                Err(Error::TooManyRequests)
            },
            code => {
                let body = response.text().compat().await?;
                Err(Error::Unknown(code, body))
            }
        }
    }

    /// Get payment details
    ///
    /// Returns the details of the payment with the specified identifier
    /// string.
    ///
    /// If the payment method requires a redirection to a third party (e.g., 3D
    /// Secure), the redirect URL back to your site will include a
    /// `cko-session-id` query parameter containing a payment session ID that
    /// can be used to obtain the details of the payment
    ///
    /// [`GET /payments/{id}`](https://api-reference.checkout.com/#operation/getPaymentDetails)
    pub async fn get_payment_details(
        &self,
        request: &GetPaymentDetailsRequest,
    ) -> Result<GetPaymentDetailsResponse, Error> {
        let url = format!(
            "{}/payments/{}",
            self.environment.api_url(),
            request.payment_id
        );
        self.send_get_request(&url).await
    }

    /// Get payment actions
    ///
    /// Returns all the actions associated with a payment ordered by processing
    /// date in descending order (latest first).
    ///
    /// [`GET /payments/{id}/actions`](https://api-reference.checkout.com/#operation/getPaymentActions)
    pub async fn get_payment_actions(
        &self,
        request: &GetPaymentActionsRequest,
    ) -> Result<GetPaymentActionsResponse, Error> {
        let url = format!(
            "{}/payments/{}/actions",
            self.environment.api_url(),
            request.payment_id
        );
        self.send_get_request(&url).await
    }

    /// Capture a payment
    ///
    /// Captures a payment if supported by the payment method.
    ///
    /// For card payments, capture requests are processed asynchronously. You
    /// can use webhooks to be notified if the capture is successful.
    ///
    /// [`POST /payments/{id}/captures`](https://api-reference.checkout.com/#operation/captureAPayment)
    pub async fn capture_payment(
        &self,
        request: &CapturePaymentRequest,
    ) -> Result<CapturePaymentResponse, Error> {
        let url = format!(
            "{}/payments/{}/captures",
            self.environment.api_url(),
            request.payment_id
        );
        self.send_post_request(&url, &request.body).await
    }

    /// Refund a payment
    ///
    /// Refunds a payment if supported by the payment method.
    ///
    /// For card payments, refund requests are processed asynchronously. You
    /// can use webhooks to be notified if the refund is successful.
    ///
    /// [`POST /payments/{id}/refunds`](https://api-reference.checkout.com/#operation/refundAPayment)
    pub async fn refund_payment(
        &self,
        request: &RefundPaymentRequest,
    ) -> Result<RefundPaymentResponse, Error> {
        let url = format!(
            "{}/payments/{}/refunds",
            self.environment.api_url(),
            request.payment_id
        );
        self.send_post_request(&url, &request.body).await
    }

    /// Void a payment
    ///
    /// Voids a payment if supported by the payment method.
    ///
    /// For card payments, void requests are processed asynchronously. You can
    /// use webhooks to be notified if the void is successful.
    ///
    /// [`POST /payments/{id}/voids`](https://api-reference.checkout.com/#operation/voidAPayment)
    pub async fn void_payment(
        &self,
        request: &VoidPaymentRequest,
    ) -> Result<VoidPaymentResponse, Error> {
        let url = format!(
            "{}/payments/{}/voids",
            self.environment.api_url(),
            request.payment_id
        );
        self.send_post_request(&url, &request.body).await
    }
}

#[cfg(test)]
mod tests {
    use futures03::future::{FutureExt, TryFutureExt};
    use once_cell::sync::OnceCell;
    use tokio::runtime::Runtime;

    use super::*;

    fn client() -> &'static Client {
        static INSTANCE: OnceCell<Client> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let dotenv_var = |key: &str| SecretString::new(dotenv::var(key).expect(key));
            Client::new(
                dotenv_var("CKO_USERNAME"),
                dotenv_var("CKO_PASSWORD"),
                Environment::Sandbox,
            )
        })
    }

    #[test]
    fn can_request_payout() {
        let mut rt = Runtime::new().unwrap();

        let payment: &'static _ = Box::leak(Box::new(CreatePaymentRequest {
            source: Some(PaymentRequestSource::Card {
                number: "4242424242424242".to_string(),
                expiry_month: 6,
                expiry_year: 2025,
                name: None,
                cvv: None,
                stored: None,
                billing_address: None,
                phone: None,
            }),
            destination: None,
            amount: Some(2000),
            currency: "USD".to_string(),
            payment_type: PaymentType::Regular,
            merchant_initiated: false,
            reference: None,
            description: None,
            capture: None,
            capture_on: None,
            customer: None,
            billing_descriptor: None,
            shipping: None,
            three_ds: None,
            previous_payment_id: None,
            risk: None,
            success_url: None,
            failure_url: None,
            payment_ip: None,
            recipient: None,
            processing: None,
            metadata: None,
        }));

        let response = rt
            .block_on(client().create_payment(payment).boxed().compat())
            .unwrap();

        println!("{:?}", response);
    }
}
