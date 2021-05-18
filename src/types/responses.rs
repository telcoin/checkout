use super::*;

/// Response to create a payment
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CreatePaymentResponse {
    /// The payment was processed immediately
    Processed(PaymentProcessed),

    /// The payment is being processed asynchronously or further action is
    /// required
    Pending(PendingPayment),
}

/// Response to get payment details
pub type GetPaymentDetailsResponse = PaymentDetails;

/// Response to get payment actions
pub type GetPaymentActionsResponse = Vec<Action>;

/// Response to capture a payment
#[derive(Deserialize, Debug, Clone)]
pub struct CapturePaymentResponse {
    /// The unique identifier for the capture action (format: `act_*`)
    pub action_id: String,

    /// Your reference for the capture request
    pub reference: Option<String>,

    /// The links related to the capture
    ///
    /// - Required: `"payment"`
    /// - Optional: `"redirect"`
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Response to refund a payment
#[derive(Deserialize, Debug, Clone)]
pub struct RefundPaymentResponse {
    /// The unique identifier for the refund action (format: `act_*`)
    pub action_id: String,

    /// Your reference for the refund request
    pub reference: Option<String>,

    /// The links related to the refund
    ///
    /// - Required: `"payment"`
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Response to void a payment
#[derive(Deserialize, Debug, Clone)]
pub struct VoidPaymentResponse {
    /// The unique identifier for the void action (format: `act_*`)
    pub action_id: String,

    /// Your reference for the void request
    pub reference: Option<String>,

    /// The links related to the void
    ///
    /// - Required: `"payment"`
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// Response to create a customer
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCustomerResponse {
    /// The customer's unique identifier (format: `cus_*`)
    pub id: String,
}

/// Response to get customer details
#[derive(Deserialize, Debug, Clone)]
pub struct GetCustomerDetailsResponse {
    /// The customer's unique identifier (format: `cus_*`)
    pub id: String,

    /// The customer's email address
    pub email: String,

    /// The ID for this customer's default instrument
    pub default: Option<String>,

    /// The customer's name
    pub name: Option<String>,

    /// The customer's phone number
    pub phone: Option<PhoneNumber>,

    /// A set of key-value pairs that is attached to a customer
    pub metadata: Option<Metadata>,

    /// The details of the instruments linked to this customer
    pub instruments: Option<Vec<Instrument>>,
}
