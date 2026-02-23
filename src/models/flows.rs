use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::models::shared::{BillingInformation, Currency, CustomerDescriptor, Links};

/// Request body for creating a payment session
#[derive(Serialize, Debug, Clone, Builder)]
pub struct CreatePaymentSessionRequest {
    /// The payment amount
    pub amount: u64,

    /// The three-letter ISO currency code
    pub currency: Currency,

    /// The processing channel to be used for the payment
    #[builder(into)]
    pub processing_channel_id: String,

    /// A reference you can later use to identify this payment session
    #[builder(into)]
    pub reference: String,

    /// The billing details
    pub billing: BillingInformation,

    /// The customer's details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerDescriptor>,

    /// The URL to redirect to if the payment is successful
    #[builder(into)]
    pub success_url: String,

    /// The URL to redirect to if the payment fails
    #[builder(into)]
    pub failure_url: String,
}

/// Response for creating a payment session
#[derive(Deserialize, Debug, Clone)]
pub struct CreatePaymentSessionResponse {
    /// The payment session identifier
    pub id: String,

    /// The payment session secret
    pub payment_session_secret: String,

    /// The links related to the payment session
    #[serde(rename = "_links")]
    pub links: Links,
}
