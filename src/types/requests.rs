use super::*;

/// The request body to be used to authenticate
#[derive(Serialize, Debug, Clone)]
pub struct OAuthTokenRequest {
    /// Probably "client_credentials"
    pub grant_type: String,

    /// Determines what endpoints the requested token can access
    ///
    /// See [Authentication](https://api-reference.checkout.com/preview/crusoe/#section/Authentication)
    /// for possible scopes
    pub scope: String,
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
#[derive(Serialize, Debug, Clone)]
pub struct CreatePaymentRequest {
    /// The source of the payment. Use to request a payment.
    pub source: Option<PaymentRequestSource>,

    /// The destination of the payout. Use to pay out to a card.
    pub destination: Option<PaymentRequestDestination>,

    /// The payment amount. The exact format depends on the currency. Omit the
    /// amount or provide a value of 0 to perform a card verification.
    ///
    /// See: [Calculating the value](https://docs.checkout.com/resources/calculating-the-value)
    pub amount: Option<u64>,

    /// The three-letter ISO country code
    pub currency: String,

    /// This must be specified for card payments where the cardholder is not
    /// present (i.e., recurring or mail order / telephone order) (default:
    /// Regular)
    pub payment_type: PaymentType,

    /// Flags the payment as a merchant-initiated transaction (MIT). Must be
    /// set to true for all MITs.
    ///
    /// See: [Requirements for stored payment details](https://docs.checkout.com/payments/store-payment-details/requirements-for-stored-payment-details)
    pub merchant_initiated: bool,

    /// A reference you can later use to identify this payment, such as an
    /// order number. Required when processing via dLocal or Bambora. (<= 50
    /// characters)
    pub reference: Option<String>,

    /// A description of the payment (<= 100 characters)
    pub description: Option<String>,

    /// Whether to capture the payment (if applicable) (default: true)
    pub capture: Option<bool>,

    /// A timestamp (ISO 8601 code) that determines when the payment should be
    /// captured. Providing this field will automatically set capture to true
    pub capture_on: Option<String>,

    /// The customer's details
    pub customer: Option<CustomerDescriptor>,

    /// An optional dynamic billing descriptor displayed on the account owner's
    /// statement
    pub billing_descriptor: Option<BillingDescriptor>,

    /// The shipping details
    pub shipping: Option<ShippingDescriptor>,

    /// Information required for 3D Secure payments
    #[serde(rename = "3ds")]
    pub three_ds: Option<_3DSRequest>,

    /// For payments that use stored card details, such as recurring payments –
    /// an existing payment identifier from the recurring series or the Scheme
    /// Transaction Id (<= 100 characters)
    ///
    /// See: [Requirements for stored payment details](https://docs.checkout.com/payments/store-payment-details/requirements-for-stored-payment-details)
    pub previous_payment_id: Option<String>,

    /// Configures the risk assessment performed during the processing of the
    /// payment
    pub risk: Option<RiskRequest>,

    /// For redirect payment methods, this overrides the default success
    /// redirect URL configured on your account (<= 255 characters)
    pub success_url: Option<String>,

    /// For redirect payment methods, this overrides the default failure
    /// redirect URL configured on your account (<= 255 characters)
    pub failure_url: Option<String>,

    /// The IP address used to make the payment. Required for some risk checks
    /// (<= 45 characters)
    pub payment_ip: Option<String>,

    /// Information about the recipient of the payment's funds. Relevant for
    /// both Account Funding Transactions and VISA or MasterCard domestic UK
    /// transactions processed by Financial Institutions.
    ///
    /// See: [Account Funding Transactions](https://docs.checkout.com/payments/manage-payments/account-funding-transactions)
    /// and [Requirements for financial institutions](https://docs.checkout.com/risk-management/requirements-for-financial-institutions)
    pub recipient: Option<PaymentRecipient>,

    /// Use the processing object to influence or override the data sent during
    /// card processing
    pub processing: Option<PaymentProcessingDescriptor>,

    /// Allows you to store additional information about a transaction with
    /// custom fields and up to five user-defined fields (`udf1` to `udf5`),
    /// which can be used for reporting purposes. `udf1` is also used for some
    /// of our risk rules.
    pub metadata: Option<Metadata>,
}

/// Request to get payment details
#[derive(Serialize, Debug, Clone)]
pub struct GetPaymentDetailsRequest {
    /// The payment or payment session identifier (format: `pay_*` or `sid_*`)
    pub payment_id: String,
}

/// Request to get payment actions
#[derive(Serialize, Debug, Clone)]
pub struct GetPaymentActionsRequest {
    /// The payment identifier (format: `pay_*`)
    pub payment_id: String,
}

/// Request to capture a payment
#[derive(Debug, Clone)]
pub struct CapturePaymentRequest {
    /// The payment identifier (format: `pay_*`)
    pub payment_id: String,

    /// The request body
    pub body: CapturePaymentBody,
}

/// Body used in the request to capture a payment
#[derive(Serialize, Debug, Clone)]
pub struct CapturePaymentBody {
    /// The amount to capture. If not specified, the full payment amount will
    /// be captured
    pub amount: Option<u64>,

    /// A reference you can later use to identify this capture request
    pub reference: Option<String>,

    /// A set of key-value pairs that you can attach to the capture request.
    /// This can be useful for storing additional information in a structured
    /// format
    pub metadata: Option<Metadata>,
}

/// Request to refund a payment
#[derive(Debug, Clone)]
pub struct RefundPaymentRequest {
    /// The payment identifier (format: `pay_*`)
    pub payment_id: String,

    /// The request body
    pub body: RefundPaymentBody,
}

/// Body used in the request to refund a payment
#[derive(Serialize, Debug, Clone)]
pub struct RefundPaymentBody {
    /// The amount to refund. If not specified, the full payment amount will
    /// be refunded
    pub amount: Option<u64>,

    /// A reference you can later use to identify this refund request
    pub reference: Option<String>,

    /// A set of key-value pairs that you can attach to the refund request.
    /// This can be useful for storing additional information in a structured
    /// format
    pub metadata: Option<Metadata>,
}

/// Request to void a payment
#[derive(Debug, Clone)]
pub struct VoidPaymentRequest {
    /// The payment identifier (format: `pay_*`)
    pub payment_id: String,

    /// The request body
    pub body: VoidPaymentBody,
}

/// Body used in the request to void a payment
#[derive(Serialize, Debug, Clone)]
pub struct VoidPaymentBody {
    /// A reference you can later use to identify this void request
    pub reference: Option<String>,

    /// A set of key-value pairs that you can attach to the void request.
    /// This can be useful for storing additional information in a structured
    /// format
    pub metadata: Option<Metadata>,
}

/// Request body to create a customer
#[derive(Serialize, Debug, Clone)]
pub struct CreateCustomerRequest {
    /// The customer's email address (<= 255 characters)
    pub email: String,

    /// The customer's name (<= 255 characters)
    pub name: Option<String>,

    /// The customer's phone number
    pub phone: Option<PhoneNumber>,

    /// Allows you to store additional information about a customer. You can
    /// include a maximum of 10 key-value pairs. Each key and value can be up
    /// to 100 characters long.
    pub metadata: Option<Metadata>,
}

/// Request body to get customer details
#[derive(Serialize, Debug, Clone)]
pub struct GetCustomerDetailsRequest {
    /// The customer's ID (format: `cus_*`) or the customer's email address (<=
    /// 255 characters) 
    pub customer_id_or_email: String,
}

/// Request to update customer details
#[derive(Debug, Clone)]
pub struct UpdateCustomerDetailsRequest {
    /// The customer id (format: `cus_*`)
    pub customer_id: String,

    /// The request body
    pub body: UpdateCustomerDetailsBody,
}

/// Request body to update customer details
#[derive(Serialize, Debug, Clone)]
pub struct UpdateCustomerDetailsBody {
    /// The email address of the customer
    pub email: Option<String>,

    /// The name of the customer
    pub name: Option<String>,

    /// The instrument ID for this customer’s default instrument
    pub default: Option<String>,

    /// The customer's phone number
    pub phone: Option<PhoneNumber>,

    /// Allows you to store additional information about a customer. You can
    /// include a maximum of 10 key-value pairs. Each key and value can be up
    /// to 100 characters long.
    pub metadata: Option<Metadata>,
}

/// Request to delete a customer
#[derive(Debug, Clone)]
pub struct DeleteCustomerRequest {
    /// The customer id (format: `cus_*`)
    pub customer_id: String,
}

/// Request body to create an instrument
#[derive(Serialize, Debug, Clone)]
pub struct CreateInstrumentBody {
    /// The instrument type
    #[serde(rename = "type")]
    ty: String,
}
