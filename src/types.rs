use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub product_id: String,
    pub amount: f64,
    pub success_url: String,
    pub failure_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub status: String,
    pub transaction_id: String,
    pub amount: f64,
}
