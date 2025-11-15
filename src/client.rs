use crate::types::{PaymentRequest, PaymentResponse, TransactionStatus, EsewaError, EsewaResult};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;

type HmacSha256 = Hmac<Sha256>;

/// Main eSewa client for payment operations
pub struct Esewa {
    merchant_code: String,
    secret_key: String,
    sandbox: bool,
    client: Client,
}

impl Esewa {
    /// Create a new eSewa client
    /// 
    /// # Arguments
    /// * `merchant_code` - Your merchant code (e.g., "EPAYTEST" for testing)
    /// * `secret_key` - Your secret key for HMAC signature generation
    /// * `sandbox` - true for testing environment, false for production
    pub fn new(merchant_code: &str, secret_key: &str, sandbox: bool) -> Self {
        Self {
            merchant_code: merchant_code.to_string(),
            secret_key: secret_key.to_string(),
            sandbox,
            client: Client::new(),
        }
    }

    /// Get the appropriate base URL based on environment
    fn base_url(&self) -> &str {
        if self.sandbox {
            "https://rc-epay.esewa.com.np"
        } else {
            "https://epay.esewa.com.np"
        }
    }

    /// Generate HMAC SHA-256 signature for the given data
    fn generate_signature(&self, data: &str) -> EsewaResult<String> {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .map_err(|e| EsewaError::Signature(format!("Invalid secret key: {}", e)))?;
        
        mac.update(data.as_bytes());
        let result = mac.finalize().into_bytes();
        Ok(general_purpose::STANDARD.encode(result))
    }

    /// Create a payment form HTML that auto-submits to eSewa
    /// 
    /// This creates an HTML form that automatically redirects the user to eSewa payment page.
    /// Save this HTML to a file or serve it directly to initiate payment.
    pub fn create_payment_form(&self, request: &PaymentRequest) -> EsewaResult<String> {
        let mut payment = request.clone();
        payment.product_code = self.merchant_code.clone();

        // Generate signature for total_amount,transaction_uuid,product_code
        let signature_data = format!("{},{},{}", 
            payment.total_amount, 
            payment.transaction_uuid, 
            payment.product_code
        );
        payment.signature = Some(self.generate_signature(&signature_data)?);

        let form_html = format!(
r#"<!DOCTYPE html>
<html>
<head>
    <title>Redirecting to eSewa...</title>
</head>
<body onload="document.forms[0].submit()">
    <div style="text-align: center; padding: 50px; font-family: Arial, sans-serif;">
        <h3>Redirecting to eSewa Payment Gateway...</h3>
        <p>Please wait while we redirect you to complete your payment.</p>
    </div>
    
    <form action="{}/api/epay/main/v2/form" method="POST">
        <input type="hidden" name="amount" value="{}" />
        <input type="hidden" name="tax_amount" value="{}" />
        <input type="hidden" name="total_amount" value="{}" />
        <input type="hidden" name="transaction_uuid" value="{}" />
        <input type="hidden" name="product_code" value="{}" />
        <input type="hidden" name="product_service_charge" value="{}" />
        <input type="hidden" name="product_delivery_charge" value="{}" />
        <input type="hidden" name="success_url" value="{}" />
        <input type="hidden" name="failure_url" value="{}" />
        <input type="hidden" name="signed_field_names" value="{}" />
        <input type="hidden" name="signature" value="{}" />
        
        <noscript>
            <input type="submit" value="Continue to eSewa" />
        </noscript>
    </form>
</body>
</html>"#,
            self.base_url(),
            payment.amount,
            payment.tax_amount,
            payment.total_amount,
            payment.transaction_uuid,
            payment.product_code,
            payment.product_service_charge,
            payment.product_delivery_charge,
            payment.success_url,
            payment.failure_url,
            payment.signed_field_names,
            payment.signature.as_ref().unwrap()
        );

        Ok(form_html)
    }

    /// Check the status of a transaction
    /// 
    /// Use this API when a transaction is initiated but no response is received from eSewa
    /// or when you need to verify the current status of a transaction.
    pub async fn check_transaction_status(
        &self, 
        product_code: &str, 
        transaction_uuid: &str, 
        total_amount: f64
    ) -> EsewaResult<TransactionStatus> {
        let url = format!(
            "{}/api/epay/transaction/status/?product_code={}&total_amount={}&transaction_uuid={}",
            self.base_url(),
            product_code,
            total_amount,
            transaction_uuid
        );

        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(EsewaError::Api(format!("HTTP {}: Failed to check transaction status", response.status())));
        }

        let status: TransactionStatus = response
            .json()
            .await
            .map_err(|e| EsewaError::InvalidResponse(format!("Failed to parse response: {}", e)))?;

        Ok(status)
    }

    /// Verify a payment response received from eSewa
    /// 
    /// When user completes payment, eSewa redirects to your success_url with a base64-encoded response.
    /// Use this method to decode and verify the response signature.
    pub fn verify_payment_response(&self, encoded_response: &str) -> EsewaResult<PaymentResponse> {
        // Decode base64 response
        let decoded_bytes = general_purpose::STANDARD
            .decode(encoded_response)
            .map_err(|e| EsewaError::InvalidResponse(format!("Invalid base64 response: {}", e)))?;

        let decoded_str = String::from_utf8(decoded_bytes)
            .map_err(|e| EsewaError::InvalidResponse(format!("Invalid UTF-8 in response: {}", e)))?;

        let response: PaymentResponse = serde_json::from_str(&decoded_str)
            .map_err(|e| EsewaError::InvalidResponse(format!("Invalid JSON response: {}", e)))?;

        // Verify signature
        let signature_data = format!("{},{},{},{},{}",
            response.transaction_code,
            response.status,
            response.total_amount,
            response.transaction_uuid,
            response.product_code
        );
        
        let expected_signature = self.generate_signature(&signature_data)?;
        
        if response.signature != expected_signature {
            return Err(EsewaError::Api("Invalid response signature".to_string()));
        }

        Ok(response)
    }

    /// Get payment URL for manual redirection (alternative to form HTML)
    /// 
    /// Instead of using create_payment_form(), you can use this to get the payment URL
    /// and handle the form submission yourself.
    pub fn get_payment_url(&self, request: &PaymentRequest) -> EsewaResult<String> {
        let mut payment = request.clone();
        payment.product_code = self.merchant_code.clone();

        let signature_data = format!("{},{},{}", 
            payment.total_amount, 
            payment.transaction_uuid, 
            payment.product_code
        );
        payment.signature = Some(self.generate_signature(&signature_data)?);

        let url = format!("{}/api/epay/main/v2/form", self.base_url());
        
        // You would typically POST these parameters, but this gives you the base URL
        // The actual form data needs to be POST-ed as shown in create_payment_form()
        
        Ok(url)
    }

    /// Helper to return the raw signature data string and the produced signature for a request.
    /// Useful for debugging what is actually being signed and sent to eSewa.
    pub fn signature_for_request(&self, request: &PaymentRequest) -> EsewaResult<(String, String)> {
        let mut payment = request.clone();
        payment.product_code = self.merchant_code.clone();

        let signature_data = format!("{},{},{}",
            payment.total_amount,
            payment.transaction_uuid,
            payment.product_code
        );

        let signature = self.generate_signature(&signature_data)?;
        Ok((signature_data, signature))
    }
}
