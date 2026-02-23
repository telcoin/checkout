use crate::{Client, Error};

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

use crate::models::payments::{
    CapturePaymentBody, CapturePaymentResponse, CreatePaymentRequest, CreatePaymentResponse,
    GetPaymentActionsResponse, GetPaymentDetailsResponse, RefundPaymentBody, RefundPaymentResponse,
    VoidPaymentBody, VoidPaymentResponse,
};
