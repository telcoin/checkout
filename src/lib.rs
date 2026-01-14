//! Client library for the [Checkout](https://www.checkout.com) API.
//!
//! Documentation: <https://docs.checkout.com>
//!
//! API Reference: <https://api-reference.checkout.com>
//!

#![forbid(unsafe_code)]
#![deny(missing_docs, missing_debug_implementations)]
#![warn(clippy::missing_panics_doc, clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::missing_errors_doc)]

use std::{convert::TryFrom, fmt, str::FromStr};

pub use reqwest::StatusCode;
use reqwest::{Client as ReqwestClient, Error as ReqwestError, Response};
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// API clients
mod apis;
/// API models
pub mod models;

use apis::CardMetadata;
use apis::Flows;
use apis::Payments;

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
    #[error("API error: {status_code} {error:?}")]
    Api {
        /// The HTTP status code
        status_code: StatusCode,
        /// The API error details
        error: ApiError,
    },

    /// Not authorized
    #[error("Unauthorized")]
    Unauthorized,

    /// To many requests or duplicate request detected
    #[error("Too many requests")]
    TooManyRequests,

    /// An unknown error occurred
    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(StatusCode),

    /// An error that occurred during transport
    #[error("Transport error: {0}")]
    Transport(#[from] ReqwestError),

    /// An error that occurred while reading environment variables
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    /// An error that occurred while parsing the environment
    #[error("Invalid environment: {0:?}")]
    InvalidEnvironment(#[from] InvalidEnvironmentError),
}

/// Could not parse an environment, contains the original string.
#[derive(thiserror::Error, Debug)]
#[error("Could not parse environment: {0}")]
pub struct InvalidEnvironmentError(pub String);

/// API environments to differentiate between testing environments and live.
#[derive(PartialEq, Copy, Clone, Debug)]
#[expect(missing_docs)]
pub enum Environment {
    Production,
    Sandbox,
}

impl FromStr for Environment {
    type Err = InvalidEnvironmentError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "prod" | "production" => Ok(Environment::Production),
            "dev" | "development" | "sandbox" => Ok(Environment::Sandbox),
            val => Err(InvalidEnvironmentError(val.to_owned())),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = InvalidEnvironmentError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<'a> TryFrom<&'a str> for Environment {
    type Error = InvalidEnvironmentError;

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
    #[must_use]
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

/// The request body to be used to authenticate
#[derive(Serialize, Debug, Clone)]
pub struct OAuthTokenRequest {
    /// Probably "`client_credentials`"
    pub grant_type: String,

    /// Determines what endpoints the requested token can access
    ///
    /// See [Authentication](https://api-reference.checkout.com/preview/crusoe/#section/Authentication)
    /// for possible scopes
    pub scope: String,
}

/// The response for a successful authentication
#[derive(Deserialize, Debug, Clone)]
pub struct OAuthTokenResponse {
    /// The token to be used to access endpoints
    pub access_token: String,

    /// Seconds until expiration
    pub expires_in: u32,

    /// Probably "Bearer"
    pub token_type: String,

    /// What endpoints this token is valid to access
    ///
    /// See [Authentication](https://api-reference.checkout.com/preview/crusoe/#section/Authentication)
    /// for possible scopes
    pub scope: String,
}

/// A client that can be used to access the Checkout API
#[derive(Clone, Debug)]
pub struct Client {
    http: ReqwestClient,
    environment: Environment,
    username: SecretString,
    password: SecretString,
}

impl Client {
    /// Creates a new client
    #[must_use]
    pub fn new(username: SecretString, password: SecretString, environment: Environment) -> Client {
        Client {
            http: ReqwestClient::new(),
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
    /// - [`Error::InvalidEnvironment`]
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
            .http
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

    pub(crate) async fn send_get_request<R>(&self, scope: &str, url: &str) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        let token = self.authorize(scope).await?;

        let response = self.http.get(url).bearer_auth(token).send().await?;

        handle_response(response).await
    }

    pub(crate) async fn send_post_request<B, R>(
        &self,
        scope: &str,
        url: &str,
        body: &B,
    ) -> Result<R, Error>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let token = self.authorize(scope).await?;

        let response = self
            .http
            .post(url)
            .bearer_auth(token)
            .json(body)
            .send()
            .await?;

        handle_response(response).await
    }

    /// Access the Payments API.
    #[must_use]
    pub fn payments(&self) -> Payments<'_> {
        Payments::new(self)
    }

    /// Access the Flows API.
    #[must_use]
    pub fn flows(&self) -> Flows<'_> {
        Flows::new(self)
    }

    /// Access the Metadata API.
    #[must_use]
    pub fn metadata(&self) -> CardMetadata<'_> {
        CardMetadata::new(self)
    }
}

async fn handle_response<R>(response: Response) -> Result<R, Error>
where
    R: DeserializeOwned,
{
    let status = response.status();

    if status.is_success() {
        Ok(response.json().await?)
    } else {
        match response.json().await {
            Ok(error) => Err(Error::Api {
                status_code: status,
                error,
            }),
            Err(_) => match status {
                StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
                StatusCode::TOO_MANY_REQUESTS => Err(Error::TooManyRequests),
                _ => Err(Error::UnexpectedStatusCode(status)),
            },
        }
    }
}
