# eSewa Payment Gateway SDK for Rust

A simple, secure, and easy-to-use Rust SDK for integrating eSewa payment gateway into your applications.

## Features

- **Simple API**: Clean and intuitive interface for payment integration
- **Secure**: HMAC SHA-256 signature generation for secure transactions
- **Complete**: Supports payment form generation, status checking, and response verification
- **Environment Support**: Both sandbox and production environments
- **Type Safe**: Full type safety with comprehensive error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
esewa = "0.1.0"
```

## Quick Start

```rust
use esewa::{Esewa, PaymentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eSewa client
    let esewa = Esewa::new(
        "YOUR_MERCHANT_CODE",    // Your merchant code
        "YOUR_SECRET_KEY",       // Your secret key
        false                    // false for production, true for sandbox
    );

    // Create payment request
    let payment = PaymentRequest::new(
        100.0,                           // Amount
        "order-12345",                   // Unique transaction ID
        "https://yoursite.com/success",  // Success URL
        "https://yoursite.com/failure"   // Failure URL
    );

    // Generate payment form HTML
    let form_html = esewa.create_payment_form(&payment)?;
    
    // Serve this HTML to redirect user to eSewa
    // or save to file for testing
    std::fs::write("payment.html", form_html)?;

    Ok(())
}
```

## Payment with Service Charges

```rust
use esewa::{Esewa, PaymentRequest};

let payment = PaymentRequest::with_charges(
    100.0,  // Base amount
    13.0,   // Tax amount (VAT)
    5.0,    // Service charge
    10.0,   // Delivery charge
    "order-12345",
    "https://yoursite.com/success",
    "https://yoursite.com/failure"
);

let form_html = esewa.create_payment_form(&payment)?;
```

## Transaction Status Check

```rust
// Check transaction status
let status = esewa.check_transaction_status(
    "YOUR_MERCHANT_CODE",
    "transaction-uuid", 
    100.0
).await?;

println!("Status: {:?}", status.status);
println!("Reference ID: {:?}", status.ref_id);
```

## Verify Payment Response

When users complete payment, eSewa redirects them to your success URL with a base64-encoded response. Verify it like this:

```rust
// Decode and verify the response from eSewa
let encoded_response = "eyJ0cmFuc2FjdGlvbl9jb2RlIjoiMDAwQVdFTyI..."; // From URL parameter
let payment_response = esewa.verify_payment_response(encoded_response)?;

if payment_response.status == "COMPLETE" {
    println!("Payment successful!");
    println!("Transaction Code: {}", payment_response.transaction_code);
    println!("Amount: Rs. {}", payment_response.total_amount);
}
```

## Testing

For testing, use the official eSewa test credentials:

```rust
let esewa = Esewa::new("EPAYTEST", "8gBm/:&EnhH.1/q", true); // sandbox mode

// Test user credentials for browser testing:
// eSewa ID: 9806800001, 9806800002, 9806800003, 9806800004, or 9806800005
// Password: Nepal@123
// MPIN: 1122
// Token: 123456
```

Run the example:

```bash
cargo run --example test_credentials
```

This will generate test payment forms that you can open in a browser to test the complete payment flow.

## API Reference

### `Esewa::new(merchant_code, secret_key, sandbox)`
Creates a new eSewa client.

### `esewa.create_payment_form(&payment)`
Generates HTML form that auto-redirects to eSewa payment page.

### `esewa.check_transaction_status(merchant_code, transaction_uuid, amount)`
Checks the current status of a transaction.

### `esewa.verify_payment_response(encoded_response)`
Decodes and verifies a payment response from eSewa.

### `PaymentRequest::new(amount, transaction_uuid, success_url, failure_url)`
Creates a simple payment request.

### `PaymentRequest::with_charges(amount, tax, service_charge, delivery_charge, ...)`
Creates a payment request with detailed breakdown of charges.

## Transaction States

- `COMPLETE` - Payment successful
- `PENDING` - Payment initiated but not completed
- `FULL_REFUND` - Full amount refunded
- `PARTIAL_REFUND` - Partial amount refunded  
- `AMBIGUOUS` - Payment in uncertain state
- `NOT_FOUND` - Transaction not found
- `CANCELED` - Transaction canceled

## Environment URLs

- **Sandbox**: `https://rc-epay.esewa.com.np`
- **Production**: `https://epay.esewa.com.np`

## License

MIT

## Support

For eSewa-specific issues, contact [eSewa Support](https://esewa.com.np/).
For SDK issues, please create an issue in this repository.