use esewa::{EsewaConfig, EsewaClient, PaymentRequest};

fn main() {
    println!("=== eSewa Payment Gateway Demo ===\n");

    // Create configuration
    let config = EsewaConfig {
        client_id: "JB0BBQ4aD0UqIThFJwAKBgAXEUkEGQUBBAwdOgABHD4DChwUBHwOBgAPEQ==".to_string(),
        client_secret: "8gBm/:&EnhH.1/q".to_string(),
        merchant_code: "EPAYTEST".to_string(),
        sandbox: true,
    };

    // Create client
    let client = EsewaClient::new(config);

    // Create a payment request
    let payment_request = PaymentRequest {
        product_id: "test-product-001".to_string(),
        amount: 100.0,
        success_url: "https://yoursite.com/success".to_string(),
        failure_url: "https://yoursite.com/failure".to_string(),
    };

    // Generate payment form
    let form_html = client.build_payment_form(&payment_request);

    println!("Generated Payment Form HTML:");
    println!("{}", form_html);

    println!("\n=== Configuration Details ===");
    println!("Base URL: {}", client.config.base_url());
    println!("Merchant Code: {}", client.config.merchant_code);
    println!("Is Sandbox: {}", client.config.sandbox);
}