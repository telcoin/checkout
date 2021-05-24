#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod links;
pub mod requests;
pub mod responses;

use links::Links;
pub use requests::*;
pub use responses::*;

/// The details of a payment
#[derive(Deserialize, Debug, Clone)]
pub struct PaymentDetails {
    /// The payment's unique identifier (<= 30 characters, format `pay_*`)
    pub id: String,

    /// The date/time the payment was requested
    pub requested_on: String,

    /// The source of the payment
    pub source: Option<PaymentProcessedSource>,

    /// The destination of the payment
    pub destination: Option<PaymentProcessedDestination>,

    /// The payment amount
    pub amount: u64,

    /// The three-letter ISO currency code of the payment (3 characters)
    pub currency: String,

    /// This must be specified for card payments where the cardholder is not
    /// present (i.e., recurring or mail order / telephone order)
    pub payment_type: PaymentType,

    /// Your reference for the payment
    pub reference: Option<String>,

    /// A description of the payment
    pub description: Option<String>,

    /// Whether or not the authorization or capture was successful
    pub approved: bool,

    /// The status of the payment
    pub status: PaymentStatus,

    /// Provides information relating to the processing of 3D Secure payments
    #[serde(rename = "3ds")]
    pub three_ds: Option<_3dsStatus>,

    /// Returns the payment's risk assessment results
    pub risk: Option<RiskResults>,

    /// The customer associated with the payment, if provided in the request
    pub customer: Option<CustomerInfo>,

    /// An optional dynamic billing descriptor displayed on the account owner's
    /// statement
    pub billing_descriptor: Option<BillingDescriptor>,

    /// The shipping details
    pub shipping: Option<ShippingDescriptor>,

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

    /// A set of key-value pairs that you can attach to a payment. It can be
    /// useful for storing additional information in a structured format
    pub metadata: Option<Metadata>,

    /// The final Electronic Commerce Indicator (ECI) security level used to
    /// authorize the payment. Applicable for 3D Secure, digital wallet, and
    /// network token payments
    pub eci: Option<String>,

    /// The scheme transaction identifier
    pub scheme_id: Option<String>,

    /// A summary of the payment's actions, returned when a session ID is used
    /// to get the payment details
    pub actions: Option<Vec<ActionSummary>>,

    /// The links related to the payment
    ///
    /// - Required: `"self"`, `"actions"`
    /// - Optional: `"void"`, `"capture"`, `"refund"`
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// The payment source type
///
/// Note: To make a payment with full card details, you must be SAQ D PCI
/// compliant.
///
/// See [PCI Compliance](https://docs.checkout.com/risk-management/pci-compliance)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum PaymentRequestSource {
    /// A debit/credit/etc card
    #[serde(rename = "card")]
    Card {
        /// The card number (without separators) (<=19 characters)
        number: String,

        /// The expiry month of the card (1-2 characters)
        expiry_month: u32,

        /// The expiry year of the card (4 characters)
        expiry_year: u32,

        /// The name of the cardholder
        name: Option<String>,

        /// The card verification value/code. 3 digits, except for Amex (4
        /// digits)
        cvv: Option<String>,

        /// This must be set to true for payments that use stored card details
        /// (default: false)
        ///
        /// See: [Requirements for stored payment details](https://docs.checkout.com/payments/store-payment-details/requirements-for-stored-payment-details)
        stored: Option<bool>,

        /// The billing address of the cardholder
        billing_address: Option<Address>,

        /// The phone number of the cardholder
        phone: Option<PhoneNumber>,
    },
}

/// The payout destination type
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum PaymentRequestDestination {
    /// A debit/credit/etc card
    #[serde(rename = "card")]
    Card {
        /// The card number
        number: String,

        /// The expiry month of the card (1-2 characters)
        expiry_month: String,

        /// The expiry year of the card (4 characters)
        expiry_year: String,

        /// The payout destination owner's first name
        first_name: String,

        /// The payout destination owner's last name
        last_name: String,

        /// The name of the cardholder
        name: Option<String>,

        /// The billing address of the cardholder
        billing_address: Option<Address>,

        /// The phone number of the cardholder
        phone: Option<PhoneNumber>,
    },
}

/// A type of payment
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PaymentType {
    /// A regular payment
    Regular,

    /// A merchant-initiated recurring payment
    Recurring,

    /// A Merchant Offline Telephone Order
    #[serde(rename = "MOTO")]
    Moto,
}

/// A phone number
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhoneNumber {
    /// The international country calling code (1-7 characters)
    pub country_code: String,

    /// The phone number (6-25 characters)
    pub number: String,
}

/// A physical address
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    /// The first line of the address (<= 200 characters)
    pub address_line1: Option<String>,

    /// The second line of the address (<= 200 characters)
    pub address_line2: Option<String>,

    /// The address city (<= 50 characters)
    pub city: Option<String>,

    /// The address state (<= 50 characters)
    pub state: Option<String>,

    /// The address zip/postal code (<= 50 characters)
    pub zip: Option<String>,

    /// The two-letter ISO country code of the address (2 characters)
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerDescriptor {
    /// The identifier of an existing customer. (format: `cus_*`)
    pub id: Option<String>,

    /// The customer's email address. Providing this will create a new
    /// customer, unless you have already stored a customer with the same
    /// email.
    pub email: Option<String>,

    /// The customer's name. This will only set the name for new customers
    pub name: Option<String>,
}

/// A description of the billing as it would appear on the account owner's
/// statement
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BillingDescriptor {
    /// A dynamic description of the charge (<= 25 characters)
    pub name: String,

    /// The city from which the charge originated (1-13 characters)
    pub city: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippingDescriptor {
    /// The shipping address
    pub address: Option<Address>,

    /// The phone number associated with the shipping address
    pub phone: Option<PhoneNumber>,
}

/// Information for 3D Secure payments
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct _3DSRequest {
    /// Whether to process this payment as a 3D Secure payment (default: false)
    pub enabled: Option<bool>,

    /// Determines whether to attempt a 3D Secure payment as non-3D Secure
    /// should the card issuer not be enrolled (default: false)
    pub attempt_n3d: Option<bool>,

    /// The Electronic Commerce Indicator security level associated with the 3D
    /// Secure enrollment result. Required if using a third-party merchant
    /// plug-in (MPI) (<= 2 characters)
    pub sci: Option<String>,

    /// A Base64 encoded cryptographic identifier (CAVV) used by the card
    /// schemes to validate the cardholder authentication result (3D Secure).
    /// Required if using an external MPI (<= 50 characters)
    pub cryptogram: Option<String>,

    /// The 3D Secure transaction identifier. Required if using an external MPI
    /// with 3D Secure 2.X.X and a Mastercard card, or with 3D Secure 1.X.X for
    /// any supported card scheme (<= 100 characters)
    pub xid: Option<String>,

    /// Indicates the version of 3D Secure that was used for authentication.
    /// Defaults to 1.0.0 if not provided (<= 10 characters)
    pub version: Option<String>,

    /// Specifies an exemption reason so that the payment is not processed
    /// using 3D Secure authentication. Learn more about exemptions in our SCA
    /// compliance guide.
    pub exemption: Option<ScaExemption>,
}

/// A type of exemption from 3DS authentication 
/// 
/// See: [Possible SCA exemptions](https://docs.checkout.com/risk-management/sca-compliance-guide)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ScaExemption {
    /// Payments below €30 are considered low-value and may be exempt. However,
    /// the customer’s bank may still trigger strong authentication if, within
    /// a 24-hour period, this exemption has been used five times since the
    /// customer's last successful authentication or the total value spent on
    /// the card without SCA exceeds €100.
    #[serde(rename = "low_value")]
    LowValue,

    /// Corporate payments made with virtual and lodge cards (typically used
    /// for business travel expenses) or from central travel accounts are
    /// exempt.
    #[serde(rename = "secure_corporate_payment")]
    SecureCorporatePayment,

    /// The customer may add a merchant to a whitelist after the initial strong
    /// authentication, meaning all subsequent payments to that business will
    /// be exempt.
    #[serde(rename = "trusted_listing")]
    TrustedListing,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RiskRequest {
    /// Whether a risk assessment should be performed (default: true)
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentRecipient {
    /// The recipient's date of birth (yyyy-mm-dd) (<= 10 characters)
    pub dob: Option<String>,

    /// The first six digits and the last four digits of the primary
    /// recipient's card, without spaces, or up to ten digits of the primary
    /// recipient's account number (10 characters)
    pub account_number: Option<String>,

    /// The first part of the UK postcode (e.g., W1T 4TJ would be W1T) (<= 50
    /// characters)
    pub zip: Option<String>,

    /// The recipient's first name (<= 50 characters)
    pub first_name: Option<String>,

    /// The recipient's last name (<= 50 characters)
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentProcessingDescriptor {
    /// Indicates whether the payment is an Account Funding Transaction
    pub aft: bool,
    // /// Processing information required for dLocal payments.
    // dlocal: Option<DLocalPaymentProcessing>,
}

/// Used to store metadata on customers, payments, disputes, etc.
///
/// Allows you to store additional information about a transaction with custom
/// fields and up to five user-defined fields (`udf1` to `udf5`), which can be
/// used for reporting purposes. `udf1` is also used for some of our risk rules
///
/// # Example
///
/// ```json
/// "metadata": {
///     "coupon_code": "NY2018",
///     "partner_id": 123989
/// }
/// ```
pub type Metadata = HashMap<String, String>;

/// The response when a payment was processed successfully
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentProcessed {
    /// The payment's unique identifier (<= 30 characters, format `pay_*`)
    pub id: String,

    /// The unique identifier for the action performed against this payment (<=
    /// 30 characters, format: `act_*`)
    pub action_id: String,

    /// The payment amount
    pub amount: u64,

    /// The three-letter ISO currency code of the payment (3 characters)
    pub currency: String,

    /// Whether or not the authorization or capture was successful
    pub approved: bool,

    /// The status of the payment
    pub status: PaymentStatus,

    /// The acquirer authorization code if the payment was authorized
    pub auth_code: Option<String>,

    /// The Gateway response code
    pub response_code: String,

    /// The Gateway response summary
    pub response_summary: Option<String>,

    /// Provides 3D Secure enrollment status if the payment was downgraded to
    /// non-3D Secure
    #[serde(rename = "3ds")]
    pub three_ds: Option<_3dsStatus>,

    /// Returns the payment's risk assessment results
    pub risk: Option<RiskResults>,

    /// The source of the payment
    pub source: Option<PaymentProcessedSource>,

    /// The customer associated with the payment, if provided in the request
    pub customer: Option<CustomerInfo>,

    /// The date/time the payment was processed
    pub processed_on: String,

    /// Your reference for the payment
    pub reference: Option<String>,

    /// Returns information related to the processing of the payment
    pub processing: Option<PaymentProcessingInfo>,

    /// The final Electronic Commerce Indicator (ECI) security level used to
    /// authorize the payment. Applicable for 3D Secure, digital wallet, and
    /// network token payments
    pub eci: Option<String>,

    /// The scheme transaction identifier
    pub scheme_id: Option<String>,

    /// The links related to the payment
    ///
    /// - Required: `"self"`, `"actions"`
    /// - Optional: `"void"`, `"capture"`, `"refund"`
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// The response when a payment is being processed asynchronously or further
/// action is required
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PendingPayment {
    /// The payment's unique identifier (<= 30 characters, format `pay_*`)
    pub id: String,

    /// The status of the payment
    pub status: PaymentStatus,

    /// The customer associated with the payment, if provided in the request
    pub customer: Option<CustomerInfo>,

    /// Your reference for the payment
    pub reference: Option<String>,

    /// Provides 3D Secure enrollment status if the payment was downgraded to
    /// non-3D Secure
    #[serde(rename = "3ds")]
    pub three_ds: Option<_3dsStatus>,

    /// The links related to the payment
    ///
    /// - Required: `"self"`
    /// - Optional: `"redirect"`
    #[serde(rename = "_links")]
    pub links: Option<Links>,
}

/// The status of the payment
///
/// See: [Get Payment Details](https://docs.checkout.com/payments/manage-payments/get-payment-details)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PaymentStatus {
    Authorized,
    Pending,
    #[serde(rename = "Card Verified")]
    CardVerified,
    Voided,
    #[serde(rename = "Partially Captured")]
    PartiallyCaptured,
    Captured,
    #[serde(rename = "Partially Refunded")]
    PartiallyRefunded,
    Refunded,
    Declined,
    Cancelled,
    Paid,
    Expired,
}

/// Information relating to the processing of 3D Secure payments
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct _3dsStatus {
    /// Indicates whether this was a 3D Secure payment downgraded to
    /// non-3D-Secure (when `attempt_n3d` is specified)
    pub downgraded: bool,

    /// Indicates the 3D Secure enrollment status of the issuer
    pub enrolled: _3dsEnrollmentStatus,

    /// Verification to ensure the integrity of the response
    ///
    /// Example: `"Y"`
    pub signature_valid: Option<String>,

    /// Indicates whether or not the cardholder was authenticated
    pub authentication_response: Option<_3dsAuthenticationStatus>,

    /// Base64 encoded cryptographic identifier (CAVV) used by the card schemes
    /// to validate the integrity of the 3D secure payment data
    pub cryptogram: Option<String>,

    /// Unique identifier for the transaction assigned by the MPI
    pub xid: Option<String>,

    /// Indicates the version of 3D Secure that was used for authentication
    pub version: Option<String>,

    /// Specifies an exemption reason so that the payment is not processed
    /// using 3D Secure authentication
    pub exemption: Option<ScaExemption>,
}

/// The 3D Secure enrollment status
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum _3dsEnrollmentStatus {
    /// `Y` - Issuer enrolled
    #[serde(rename = "Y")]
    IssuerEnrolled,

    /// `N` - Customer not enrolled
    #[serde(rename = "N")]
    NotEnrolled,

    /// `U` - Unknown
    #[serde(rename = "U")]
    Unknown,
}

/// Response of cardholder authentication
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum _3dsAuthenticationStatus {
    /// `Y` - Customer authenticated
    #[serde(rename = "Y")]
    Authenticated,

    /// `N` - Customer not authenticated
    #[serde(rename = "N")]
    NotAuthenticated,

    /// `A` - An authentication attempt occurred but could not be completed
    #[serde(rename = "A")]
    Attempted,

    /// `U` - Unable to perform authentication
    #[serde(rename = "U")]
    Unable,
}

/// The results of a risk assessment
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RiskResults {
    /// Whether or not the payment was flagged by a risk check
    pub flagged: bool,
}

/// The processed payment's source type
///
/// The payment source type. For any payment request sources that result in a
/// card token (token`, source ID, etc.), this will be `card`; otherwise it
/// will be the name of the alternative payment method
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum PaymentProcessedSource {
    /// A debit/credit/etc card
    #[serde(rename = "card")]
    Card {
        /// The payment source identifier that can be used for subsequent
        /// payments. For new sources, this will only be returned if the
        /// payment was approved
        id: Option<String>,

        /// The payment source owner's billing address
        billing_address: Option<Address>,

        /// The payment source owner's phone number
        phone: Option<PhoneNumber>,

        /// The expiry month (1-2 characters)
        expiry_month: u32,

        /// The expiry year (4 characters)
        expiry_year: u32,

        /// The cardholder's name
        name: Option<String>,

        /// The card scheme
        scheme: Option<String>,

        /// The last four digits of the card number
        last4: String,

        /// Uniquely identifies this particular card number. You can use this
        // to compare cards across customers.
        fingerprint: String,

        /// The card issuer's Bank Identification Number (BIN) (<= 6
        /// characters)
        bin: String,

        /// The card type
        card_type: Option<CardType>,

        /// The card category
        card_category: CardCategory,

        /// The name of the card issuer
        issuer: Option<String>,

        /// The card issuer's country (two-letter ISO code) (2 characters)
        issuer_country: Option<String>,

        /// The issuer/card scheme product identifier
        product_id: Option<String>,

        /// The issuer/card scheme product type
        product_type: Option<String>,

        /// The card verification value (CVV) check result
        cvv_result: Option<String>,

        /// Whether the card supports payouts
        payouts: Option<bool>,

        /// The fast funds eligibility of the card
        ///
        /// See: [Card Payouts](https://docs.checkout.com/card-payouts)
        fast_funds: Option<bool>,

        /// A unique reference to the underlying card for network tokens (e.g.
        /// Apple Pay, Google Pay)
        payment_account_reference: Option<String>,
    },
}

/// The processed payment's destination type
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum PaymentProcessedDestination {
    /// A debit/credit/etc card
    #[serde(rename = "card")]
    Card {
        /// The payment source identifier that can be used for subsequent
        /// payments. For new sources, this will only be returned if the
        /// payment was approved
        id: Option<String>,

        /// The payment source owner's billing address
        billing_address: Option<Address>,

        /// The payment source owner's phone number
        phone: Option<PhoneNumber>,

        /// The expiry month (1-2 characters)
        expiry_month: u32,

        /// The expiry year (4 characters)
        expiry_year: u32,

        /// The cardholder's name
        name: Option<String>,

        /// The card scheme
        scheme: Option<String>,

        /// The last four digits of the card number
        last4: String,

        /// Uniquely identifies this particular card number. You can use this
        // to compare cards across customers.
        fingerprint: String,

        /// The card issuer's Bank Identification Number (BIN) (<= 6
        /// characters)
        bin: String,

        /// The card type
        card_type: Option<CardType>,

        /// The card category
        card_category: CardCategory,

        /// The name of the card issuer
        issuer: Option<String>,

        /// The card issuer's country (two-letter ISO code) (2 characters)
        issuer_country: Option<String>,

        /// The issuer/card scheme product identifier
        product_id: Option<String>,

        /// The issuer/card scheme product type
        product_type: Option<String>,

        /// The card verification value (CVV) check result
        cvv_result: Option<String>,

        /// Whether the card supports payouts
        payouts: Option<bool>,

        /// The fast funds eligibility of the card
        ///
        /// See: [Card Payouts](https://docs.checkout.com/card-payouts)
        fast_funds: Option<bool>,

        /// A unique reference to the underlying card for network tokens (e.g.
        /// Apple Pay, Google Pay)
        payment_account_reference: Option<String>,
    },
}

/// A card's type
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
    Charge,
    #[serde(rename = "DEFERRED DEBIT")]
    DeferredDebit,
}

/// A card's category
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum CardCategory {
    Consumer,
    Commercial,
}

/// Identifying fields for a customer
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerInfo {
    /// The customer's unique identifier. This can be passed as a source when
    /// making a payment (format: `cus_*`)
    pub id: String,

    /// The customer's email address
    pub email: Option<String>,

    /// The customer's name
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentProcessingInfo {
    /// A unique identifier for the authorization that is submitted to the card
    /// scheme during processing
    pub retrieval_reference_number: Option<String>,

    /// A unique identifier for the transaction generated by the acquirer
    pub acquirer_transaction_id: Option<String>,
}

/// A shortened summary of a payment action
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionSummary {
    /// The unique identifier of the payment action (format: `act_*`)
    id: String,

    /// The type of action
    #[serde(rename = "type")]
    ty: String,

    /// The Gateway response code
    response_code: String,

    /// The Gateway response summary
    response_summary: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    /// The unique identifier of the payment action (format: `act_*`)
    id: String,

    /// The type of action
    #[serde(rename = "type")]
    ty: String,

    /// The date/time the action was processed
    processed_on: String,

    // The action amount
    amount: u64,

    /// Whether the action was successful
    approved: Option<bool>,

    /// The acquirer authorization code for cards
    auth_code: Option<String>,

    /// The Gateway response code
    response_code: String,

    /// The Gateway response summary
    response_summary: Option<String>,

    /// Your reference for the action
    reference: Option<String>,

    /// Returns information related to the processing of the payment
    processing: Option<ActionProcessingInfo>,

    /// A set of key-value pairs that you can attach to an action
    metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionProcessingInfo {
    /// A unique identifier for the authorization that is submitted to the card
    /// scheme during processing
    pub retrieval_reference_number: Option<String>,

    /// A unique identifier for the capture that is submitted to the card
    /// scheme during processing
    pub acquirer_reference_number: Option<String>,

    /// A unique identifier for the transaction generated by the acquirer
    pub acquirer_transaction_id: Option<String>,
}

/// The type of an action
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionType {
    Authorization,
    #[serde(rename = "Card Verification")]
    CardVerification,
    Void,
    Capture,
    Refund,
    Payout,
}
