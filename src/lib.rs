//! # eSewa Payment Gateway SDK
//! 
//! A simple and easy-to-use Rust SDK for integrating eSewa payment gateway into your applications.
//! 
//! ## Features
//! - ePay integration (form-based payment)
//! - Transaction status checking
//! - HMAC SHA-256 signature generation for security
//! - Support for both sandbox and production environments
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use esewa::{Esewa, PaymentRequest};
//! 
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let esewa = Esewa::new("EPAYTEST", "8gBm/:&EnhH.1/q", true); // sandbox mode
//! 
//! let payment = PaymentRequest::new(
//!     100.0,                    // amount
//!     "product-123",           // transaction_uuid
//!     "https://mysite.com/success",
//!     "https://mysite.com/failure"
//! );
//! 
//! let form_html = esewa.create_payment_form(&payment)?;
//! // Save or serve this HTML to redirect users to eSewa
//! # Ok(())
//! # }
//! ```

pub mod types;
pub mod client;

pub use client::Esewa;
pub use types::{PaymentRequest, PaymentResponse, TransactionStatus, TransactionState, EsewaError, EsewaResult};
