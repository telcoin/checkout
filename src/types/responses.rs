use super::{Action, Deserialize, Links, PaymentDetails, PaymentProcessed, PendingPayment};

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

/// Response with card metadata
#[derive(Deserialize, Debug, Clone)]
pub struct CardMetadataResponse {
    /// The issuer's Bank Identification Number (BIN)
    pub bin: String,

    /// The global card scheme. For example, "american_express",
    /// "cartes_bancaires", "diners_club_international", "discover",
    /// "jcb", "mastercard", or "visa".
    pub scheme: String,

    /// The card type: "CREDIT" "DEBIT" "PREPAID" "CHARGE" "DEFERRED DEBIT"
    pub card_type: Option<String>,

    /// The card category: "CONSUMER" "COMMERCIAL"
    pub card_category: Option<String>,

    /// The card issuer
    pub issuer: Option<String>,

    /// The card issuer's country, as an ISO-2 code
    pub issuer_country: Option<String>,
    // and more
}
