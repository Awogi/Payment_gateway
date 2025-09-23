use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Result type for eSewa operations
pub type EsewaResult<T> = Result<T, EsewaError>;

/// Error types for eSewa operations
#[derive(Error, Debug)]
pub enum EsewaError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Signature generation failed: {0}")]
    Signature(String),
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
    #[error("API error: {0}")]
    Api(String),
}

/// Payment request for eSewa ePay
#[derive(Debug, Serialize, Clone)]
pub struct PaymentRequest {
    /// Amount of the product (excluding taxes and charges)
    pub amount: f64,
    /// Tax amount applied on product
    pub tax_amount: f64,
    /// Service charge by merchant on product
    pub product_service_charge: f64,
    /// Delivery charge by merchant on product
    pub product_delivery_charge: f64,
    /// Total payment amount (amount + tax_amount + product_service_charge + product_delivery_charge)
    pub total_amount: f64,
    /// Unique transaction ID (alphanumeric and hyphen only)
    pub transaction_uuid: String,
    /// Merchant code provided by eSewa
    pub product_code: String,
    /// Success redirect URL
    pub success_url: String,
    /// Failure redirect URL
    pub failure_url: String,
    /// Fields used for signature generation (always "total_amount,transaction_uuid,product_code")
    pub signed_field_names: String,
    /// HMAC SHA-256 signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl PaymentRequest {
    /// Create a new payment request with basic parameters
    /// 
    /// # Arguments
    /// * `amount` - Base amount (taxes and charges will be 0)
    /// * `transaction_uuid` - Unique transaction identifier
    /// * `success_url` - URL to redirect on successful payment
    /// * `failure_url` - URL to redirect on failed payment
    pub fn new(amount: f64, transaction_uuid: &str, success_url: &str, failure_url: &str) -> Self {
        Self {
            amount,
            tax_amount: 0.0,
            product_service_charge: 0.0,
            product_delivery_charge: 0.0,
            total_amount: amount,
            transaction_uuid: transaction_uuid.to_string(),
            product_code: String::new(), // Will be set by client
            success_url: success_url.to_string(),
            failure_url: failure_url.to_string(),
            signed_field_names: "total_amount,transaction_uuid,product_code".to_string(),
            signature: None,
        }
    }

    /// Create a payment request with detailed amounts
    pub fn with_charges(
        amount: f64,
        tax_amount: f64,
        service_charge: f64,
        delivery_charge: f64,
        transaction_uuid: &str,
        success_url: &str,
        failure_url: &str,
    ) -> Self {
        let total = amount + tax_amount + service_charge + delivery_charge;
        Self {
            amount,
            tax_amount,
            product_service_charge: service_charge,
            product_delivery_charge: delivery_charge,
            total_amount: total,
            transaction_uuid: transaction_uuid.to_string(),
            product_code: String::new(),
            success_url: success_url.to_string(),
            failure_url: failure_url.to_string(),
            signed_field_names: "total_amount,transaction_uuid,product_code".to_string(),
            signature: None,
        }
    }
}

/// Response from eSewa after successful payment (decoded from base64)
#[derive(Debug, Deserialize, Clone)]
pub struct PaymentResponse {
    pub transaction_code: String,
    pub status: String,
    pub total_amount: f64,
    pub transaction_uuid: String,
    pub product_code: String,
    pub signed_field_names: String,
    pub signature: String,
}

/// Transaction status response from status check API
#[derive(Debug, Deserialize, Clone)]
pub struct TransactionStatus {
    pub product_code: String,
    pub transaction_uuid: String,
    pub total_amount: f64,
    pub status: TransactionState,
    pub ref_id: Option<String>,
}

/// Possible transaction states
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionState {
    Complete,
    Pending,
    FullRefund,
    PartialRefund,
    Ambiguous,
    NotFound,
    Canceled,
}
