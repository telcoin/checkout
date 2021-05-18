//! Responses may have a `_links` object that holds a mapping to other
//! resources.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A mapping of links
///
/// # Example
///
/// ```json
/// {
///     "self": {
///         "href": "https://api.sandbox.checkout.com/payments/pay_mbabizu24mvu3mela5njyhpit4"
///     },
///     "action": {
///         "href": "https://api.sandbox.checkout.com/payments/pay_mbabizu24mvu3mela5njyhpit4/actions"
///     },
///     "void": {
///         "href": "https://api.sandbox.checkout.com/payments/pay_mbabizu24mvu3mela5njyhpit4/voids"
///     },
///     "capture": {
///         "href": "https://api.sandbox.checkout.com/payments/pay_mbabizu24mvu3mela5njyhpit4/captures"
///     }
/// }
/// ```
pub type Links = HashMap<String, Link>;

/// A link to another resource
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    /// The link URL
    href: String,
}

/// A link to the associated request for the current response
pub const SELF_LINK: &str = "self";

/// A link to the payment's associated actions
pub const ACTION_LINK: &str = "actions";

/// A link to void the payment
pub const VOID_LINK: &str = "void";

/// A link to capture the payment
pub const CAPTURE_LINK: &str = "capture";

/// A link to refund the payment
pub const REFUND_LINK: &str = "refund";

/// A link to the associated payment
pub const PAYMENT_LINK: &str = "payment";

/// A link that the customer should be redirected to in order to complete the
/// payment
pub const REDIRECT_LINK: &str = "redirect";

/// A link to the next object
///
/// This link allows you to move to the next page of results in the response.
/// Responses are paginated at the payout level
pub const NEXT_LINK: &str = "next";
