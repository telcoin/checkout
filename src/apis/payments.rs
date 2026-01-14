use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    _3DSRequest, Action, Amount, BillingDescriptor, Client, Currency, CustomerDescriptor,
    DestinationInstruction, Error, Links, Metadata, PaymentDetails, PaymentProcessed,
    PaymentProcessingDescriptor, PaymentRecipient, PaymentRequestDestination, PaymentRequestSource,
    PaymentSenderDetails, PaymentType, PendingPayment, RiskRequest, ShippingDescriptor,
};

/// Access the Payments API.
#[derive(Debug, Clone)]
pub struct Payments<'a> {
    client: &'a Client,
}

impl<'a> Payments<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
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
        let url = format!("{}/payments", self.client.environment.api_url());
        self.client
            .send_post_request("gateway", &url, request)
            .await
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
        payment_id: String,
    ) -> Result<GetPaymentDetailsResponse, Error> {
        let url = format!(
            "{}/payments/{}",
            self.client.environment.api_url(),
            payment_id
        );
        self.client.send_get_request("gateway", &url).await
    }

    /// Get payment actions
    ///
    /// Returns all the actions associated with a payment ordered by processing
    /// date in descending order (latest first).
    ///
    /// [`GET /payments/{id}/actions`](https://api-reference.checkout.com/#operation/getPaymentActions)
    pub async fn get_payment_actions(
        &self,
        payment_id: String,
    ) -> Result<GetPaymentActionsResponse, Error> {
        let url = format!(
            "{}/payments/{}/actions",
            self.client.environment.api_url(),
            payment_id
        );
        self.client.send_get_request("gateway", &url).await
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
        payment_id: String,
        body: &CapturePaymentBody,
    ) -> Result<CapturePaymentResponse, Error> {
        let url = format!(
            "{}/payments/{}/captures",
            self.client.environment.api_url(),
            payment_id
        );
        self.client.send_post_request("gateway", &url, &body).await
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
        payment_id: String,
        body: &RefundPaymentBody,
    ) -> Result<RefundPaymentResponse, Error> {
        let url = format!(
            "{}/payments/{}/refunds",
            self.client.environment.api_url(),
            payment_id
        );
        self.client.send_post_request("gateway", &url, &body).await
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
        payment_id: String,
        body: &VoidPaymentBody,
    ) -> Result<VoidPaymentResponse, Error> {
        let url = format!(
            "{}/payments/{}/voids",
            self.client.environment.api_url(),
            payment_id
        );
        self.client.send_post_request("gateway", &url, &body).await
    }
}

/// Request body for a payment or payout
///
/// To accept payments from cards, digital wallets and many alternative payment
/// methods, specify the `source.type` field, along with the source-specific
/// data.
///
/// To pay out to a card, specify the destination of your payout using the
/// `destination.type` field, along with the destination-specific data.
///
/// See: [Payment Methods](https://docs.checkout.com/payments/payment-methods)
#[derive(Serialize, Debug, Clone, Builder)]
pub struct CreatePaymentRequest {
    /// The source of the payment. Use to request a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentRequestSource>,

    /// The destination of the payout. Use to pay out to a card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<PaymentRequestDestination>,

    /// The payment amount. The exact format depends on the currency. Omit the
    /// amount or provide a value of 0 to perform a card verification.
    ///
    /// See: [Calculating the value](https://docs.checkout.com/resources/calculating-the-value)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,

    /// The three-letter ISO country code
    pub currency: Currency,

    /// This must be specified for card payments where the cardholder is not
    /// present (i.e., recurring or mail order / telephone order) (default:
    /// Regular)
    #[builder(default)]
    pub payment_type: PaymentType,

    /// Flags the payment as a merchant-initiated transaction (MIT). Must be
    /// set to true for all MITs.
    ///
    /// See: [Requirements for stored payment details](https://docs.checkout.com/payments/store-payment-details/requirements-for-stored-payment-details)
    #[builder(default)]
    pub merchant_initiated: bool,

    /// A reference you can later use to identify this payment, such as an
    /// order number. Required when processing via dLocal or Bambora. (<= 50
    /// characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub reference: Option<String>,

    /// A description of the payment (<= 100 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

    /// Whether to capture the payment (if applicable) (default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,

    /// A timestamp (ISO 8601 code) that determines when the payment should be
    /// captured. Providing this field will automatically set capture to true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub capture_on: Option<String>,

    /// The customer's details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerDescriptor>,

    /// An optional dynamic billing descriptor displayed on the account owner's
    /// statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_descriptor: Option<BillingDescriptor>,

    /// The shipping details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDescriptor>,

    /// Information required for 3D Secure payments
    #[serde(rename = "3ds", skip_serializing_if = "Option::is_none")]
    pub three_ds: Option<_3DSRequest>,

    /// For payments that use stored card details, such as recurring payments –
    /// an existing payment identifier from the recurring series or the Scheme
    /// Transaction Id (<= 100 characters)
    ///
    /// See: [Requirements for stored payment details](https://docs.checkout.com/payments/store-payment-details/requirements-for-stored-payment-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub previous_payment_id: Option<String>,

    /// Configures the risk assessment performed during the processing of the
    /// payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk: Option<RiskRequest>,

    /// For redirect payment methods, this overrides the default success
    /// redirect URL configured on your account (<= 255 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub success_url: Option<String>,

    /// For redirect payment methods, this overrides the default failure
    /// redirect URL configured on your account (<= 255 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub failure_url: Option<String>,

    /// The IP address used to make the payment. Required for some risk checks
    /// (<= 45 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub payment_ip: Option<String>,

    /// Information about the recipient of the payment's funds. Relevant for
    /// both Account Funding Transactions and VISA or `MasterCard` domestic UK
    /// transactions processed by Financial Institutions.
    ///
    /// See: [Account Funding Transactions](https://docs.checkout.com/payments/manage-payments/account-funding-transactions)
    /// and [Requirements for financial institutions](https://docs.checkout.com/risk-management/requirements-for-financial-institutions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<PaymentRecipient>,

    /// Use the processing object to influence or override the data sent during
    /// card processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<PaymentProcessingDescriptor>,

    /// The processing channel to be used for the payment
    ///
    /// This can be found under a Payment Method in the Checkout dashboard.
    #[builder(into)]
    pub processing_channel_id: String,

    /// Additional details about the payout instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<DestinationInstruction>,

    /// The sender of the payout.
    ///
    /// This field is required for money transfer card payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<PaymentSenderDetails>,

    /// Allows you to store additional information about a transaction with
    /// custom fields and up to five user-defined fields (`udf1` to `udf5`),
    /// which can be used for reporting purposes. `udf1` is also used for some
    /// of our risk rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Response to create a payment
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreatePaymentResponse {
    /// The payment was processed immediately
    Processed(PaymentProcessed),

    /// The payment is being processed asynchronously or further action is
    /// required
    Pending(PendingPayment),
}

/// Body used in the request to capture a payment
#[derive(Serialize, Debug, Clone, Builder)]
pub struct CapturePaymentBody {
    /// A set of key-value pairs that you can attach to the capture request.
    /// This can be useful for storing additional information in a structured
    /// format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The amount to capture. If not specified, the full payment amount will
    /// be captured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,

    /// Your reference for the capture request
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub reference: Option<String>,
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

/// Body used in the request to refund a payment
#[derive(Serialize, Debug, Clone, Builder)]
pub struct RefundPaymentBody {
    /// A set of key-value pairs that you can attach to the refund request.
    /// This can be useful for storing additional information in a structured
    /// format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The amount to refund. If not specified, the full payment amount will
    /// be refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,

    /// Your reference for the refund request
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub reference: Option<String>,
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

/// Body used in the request to void a payment
#[derive(Serialize, Debug, Clone, Builder)]
pub struct VoidPaymentBody {
    /// A set of key-value pairs that you can attach to the void request.
    /// This can be useful for storing additional information in a structured
    /// format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// A reference you can later use to identify this void request
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub reference: Option<String>,
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

/// Response to get payment details
pub type GetPaymentDetailsResponse = PaymentDetails;

/// Response to get payment actions
pub type GetPaymentActionsResponse = Vec<Action>;
