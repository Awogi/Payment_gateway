use esewa::{Esewa, PaymentRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== eSewa Payment Gateway - Simple Integration ===\n");

    // Initialize eSewa client with test credentials
    let esewa = Esewa::new(
        "EPAYTEST",                    // Test merchant code
        "8gBm/:&EnhH.1/q",            // Test secret key
        true                           // Sandbox mode
    );

    // Create a simple payment request
    let payment = PaymentRequest::new(
        100.0,                         // Amount: Rs. 100
        "order-12345",                 // Unique transaction ID
        "https://yoursite.com/success", // Success URL
        "https://yoursite.com/failure"  // Failure URL
    );

    println!("Payment Details:");
    println!("Amount: Rs. {}", payment.amount);
    println!("Transaction ID: {}", payment.transaction_uuid);
    println!("Success URL: {}", payment.success_url);
    println!("Failure URL: {}", payment.failure_url);
    println!();

    // Generate payment form HTML
    let form_html = esewa.create_payment_form(&payment)?;

    // Save to file for testing
    std::fs::write("payment_form.html", &form_html)?;
    println!("✅ Payment form generated and saved to 'payment_form.html'");
    println!("Open this file in a browser to test the payment flow.");
    println!();

    // Example with detailed charges
    println!("=== Payment with Detailed Charges ===");
    let detailed_payment = PaymentRequest::with_charges(
        100.0,  // Base amount
        13.0,   // Tax (13% VAT)
        5.0,    // Service charge
        10.0,   // Delivery charge
        "order-detailed-12346",
        "https://yoursite.com/success",
        "https://yoursite.com/failure"
    );

    println!("Detailed Payment:");
    println!("Base Amount: Rs. {}", detailed_payment.amount);
    println!("Tax: Rs. {}", detailed_payment.tax_amount);
    println!("Service Charge: Rs. {}", detailed_payment.product_service_charge);
    println!("Delivery Charge: Rs. {}", detailed_payment.product_delivery_charge);
    println!("Total: Rs. {}", detailed_payment.total_amount);
    println!();

    println!("=== Test User Credentials ===");
    println!("eSewa ID: 9806800001 (or 9806800002/3/4/5)");
    println!("Password: Nepal@123");
    println!("MPIN: 1122");
    println!("Token: 123456");

    Ok(())
}