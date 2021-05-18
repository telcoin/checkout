//! Client library for the [Checkout](https://www.checkout.com) API.
//!
//! Documentation: <https://docs.checkout.com>
//! API Reference: <https://api-reference.checkout.com>

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all, clippy::pedantic)]

use std::{convert::TryFrom, fmt, str::FromStr};

use futures03::compat::Future01CompatExt;
use reqwest::{r#async::Client as ReqwestClient, Error as ReqwestError};
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

    /// An error that ocurred during transport
    Transport(#[from] ReqwestError),
}

/// Could not parse an environment, contains the original string.
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
    /// Returns the appropriate url depending on the environment
    pub fn api_url(&self) -> String {
        match self {
            Environment::Sandbox => "https://api.sandbox.checkout.com".to_string(),
            Environment::Production => "https://api.checkout.com".to_string(),
        }
    }
}

/// A client that can be used to access the Checkout API
#[derive(Clone, Debug)]
pub struct Client {
    http_client: ReqwestClient,
    url: String,
    api_secret_key: SecretString,
}

impl Client {
    /// Creates a new client
    #[must_use]
    pub fn new(api_secret_key: SecretString, environment: Environment) -> Client {
        Client {
            http_client: ReqwestClient::new(),
            url: environment.api_url(),
            api_secret_key,
        }
    }

    async fn send_get_request<R>(&self, url: &str) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        let mut response = self
            .http_client
            .get(url)
            .header("authorization", self.api_secret_key.expose_secret())
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
        let mut response = self
            .http_client
            .post(url)
            .header("authorization", self.api_secret_key.expose_secret())
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

    async fn send_patch_request_no_response_body<B>(&self, url: &str, body: &B) -> Result<(), Error>
    where
        B: Serialize,
    {
        let mut response = self
            .http_client
            .patch(url)
            .header("authorization", self.api_secret_key.expose_secret())
            .json(body)
            .send()
            .compat()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Api(response.json().compat().await?))
        }
    }

    async fn send_delete_request(&self, url: &str) -> Result<(), Error> {
        let mut response = self
            .http_client
            .delete(url)
            .header("authorization", self.api_secret_key.expose_secret())
            .send()
            .compat()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Api(response.json().compat().await?))
        }
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
        let url = format!("{}/payments", self.url);
        self.send_post_request(&url, request).await
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
        let url = format!("{}/payments/{}", self.url, request.payment_id);
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
        let url = format!("{}/payments/{}/actions", self.url, request.payment_id);
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
        let url = format!("{}/payments/{}/captures", self.url, request.payment_id);
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
        let url = format!("{}/payments/{}/refunds", self.url, request.payment_id);
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
        let url = format!("{}/payments/{}/voids", self.url, request.payment_id);
        self.send_post_request(&url, &request.body).await
    }

    /// Create a customer
    ///
    /// Create a customer which can be linked to one or more payment
    /// instruments, and can be passed as a source when making a payment, using
    /// the customer’s default instrument.
    ///
    /// [`POST /customers`](https://api-reference.checkout.com/#operation/createCustomer)
    pub async fn create_customer(
        &self,
        request: &CreateCustomerRequest,
    ) -> Result<CreateCustomerResponse, Error> {
        let url = format!("{}/customers", self.url);
        self.send_post_request(&url, &request).await
    }

    /// Get customer details
    ///
    /// Returns details of a customer and their instruments
    ///
    /// [`GET /customers/{id}`](https://api-reference.checkout.com/#operation/getCustomerDetails)
    pub async fn get_customer_details(
        &self,
        request: &GetCustomerDetailsRequest,
    ) -> Result<GetCustomerDetailsResponse, Error> {
        let url = format!("{}/customers/{}", self.url, request.customer_id_or_email);
        self.send_get_request(&url).await
    }

    /// Update customer details
    ///
    /// Update details of a customer
    ///
    /// [`PATCH /customers/{id}`](https://api-reference.checkout.com/#operation/updateCustomerDetails)
    pub async fn update_customer_details(
        &self,
        request: &UpdateCustomerDetailsRequest,
    ) -> Result<(), Error> {
        let url = format!("{}/customers/{}", self.url, request.customer_id);
        self.send_patch_request_no_response_body(&url, &request.body)
            .await
    }

    /// Delete a customer
    ///
    /// Delete a customer and all of their linked payment instruments
    ///
    /// [`DELETE /customers/{id}`](https://api-reference.checkout.com/#operation/deleteCustomerDetails)
    pub async fn delete_customer(&self, request: &DeleteCustomerRequest) -> Result<(), Error> {
        let url = format!("{}/customers/{}", self.url, request.customer_id);
        self.send_delete_request(&url).await
    }
}
