use esewa::{Esewa, PaymentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== eSewa Test Credentials Demo ===\n");

    // Create eSewa client with official test credentials
    let esewa = Esewa::new(
        "EPAYTEST",                    // Official test merchant code
        "8gBm/:&EnhH.1/q",            // Official test secret key
        true                           // Sandbox mode
    );

    println!("✅ eSewa client initialized with test credentials");
    println!("Environment: Sandbox");
    println!("Merchant Code: EPAYTEST");
    println!();

    // Create test payment
    let payment = PaymentRequest::new(
        50.0,                          // Test amount: Rs. 50
        &format!("test-{}", chrono::Utc::now().timestamp()),
        "https://httpbin.org/get?status=success",
        "https://httpbin.org/get?status=failure"
    );

    println!("Payment Request Created:");
    println!("Amount: Rs. {}", payment.total_amount);
    println!("Transaction UUID: {}", payment.transaction_uuid);
    println!();

    // Generate payment form and save to file
    let form_html = esewa.create_payment_form(&payment)?;
    let filename = "esewa_test_payment.html";
    std::fs::write(filename, &form_html)?;

    println!("✅ Payment form generated and saved to '{}'", filename);
    println!();

    // Demonstrate transaction status checking
    println!("=== Transaction Status Check Demo ===");
    match esewa.check_transaction_status("EPAYTEST", "test-demo-123", 100.0).await {
        Ok(status) => {
            println!("✅ Status check successful:");
            println!("Status: {:?}", status.status);
            println!("Amount: Rs. {}", status.total_amount);
            if let Some(ref_id) = status.ref_id {
                println!("Reference ID: {}", ref_id);
            }
        },
        Err(e) => {
            println!("ℹ️  Expected error for non-existent transaction: {}", e);
            // This is expected since we're checking a non-existent transaction
        }
    }
    println!();

    // Test with various amounts and scenarios
    println!("=== Multiple Payment Scenarios ===");
    let test_scenarios = vec![
        (10.0, "Small amount"),
        (100.0, "Medium amount"), 
        (1000.0, "Large amount"),
        (999.99, "Decimal amount"),
    ];

    for (amount, description) in test_scenarios {
        let test_payment = PaymentRequest::new(
            amount,
            &format!("test-{}-{}", description.replace(" ", ""), chrono::Utc::now().timestamp()),
            "https://httpbin.org/get?status=success",
            "https://httpbin.org/get?status=failure"
        );

        match esewa.create_payment_form(&test_payment) {
            Ok(_) => println!("✅ {} (Rs. {}): Payment form created", description, amount),
            Err(e) => println!("❌ {} (Rs. {}): Error - {}", description, amount, e),
        }
    }
    println!();

    // Show payment with charges
    println!("=== Payment with Service Charges ===");
    let charged_payment = PaymentRequest::with_charges(
        100.0,  // Base amount
        13.0,   // 13% VAT
        5.0,    // Service charge
        10.0,   // Delivery charge
        &format!("charged-{}", chrono::Utc::now().timestamp()),
        "https://httpbin.org/get?status=success", 
        "https://httpbin.org/get?status=failure"
    );

    println!("Payment Breakdown:");
    println!("Base Amount: Rs. {}", charged_payment.amount);
    println!("Tax (VAT): Rs. {}", charged_payment.tax_amount);
    println!("Service Charge: Rs. {}", charged_payment.product_service_charge);
    println!("Delivery Charge: Rs. {}", charged_payment.product_delivery_charge);
    println!("Total Amount: Rs. {}", charged_payment.total_amount);

    let charged_form = esewa.create_payment_form(&charged_payment)?;
    std::fs::write("esewa_charged_payment.html", &charged_form)?;
    println!("✅ Charged payment form saved to 'esewa_charged_payment.html'");
    println!();

    println!("=== Testing Instructions ===");
    println!("1. Open '{}' or 'esewa_charged_payment.html' in a web browser", filename);
    println!("2. The form will auto-redirect to eSewa sandbox payment page");
    println!("3. Use these test credentials to complete payment:");
    println!("   • eSewa ID: 9806800001, 9806800002, 9806800003, 9806800004, or 9806800005");
    println!("   • Password: Nepal@123");
    println!("   • MPIN: 1122 (for mobile app)");
    println!("   • Token: 123456");
    println!("4. After payment, you'll be redirected to the success/failure URL");
    println!("5. In production, you'd decode and verify the response using verify_payment_response()");
    println!();

    println!("=== Integration Code Example ===");
    println!("```rust");
    println!("use esewa::{{Esewa, PaymentRequest}};");
    println!();
    println!("let esewa = Esewa::new(\"YOUR_MERCHANT_CODE\", \"YOUR_SECRET_KEY\", false);");
    println!("let payment = PaymentRequest::new(100.0, \"order-123\", \"success_url\", \"failure_url\");");
    println!("let form_html = esewa.create_payment_form(&payment)?;");
    println!("// Serve form_html to user's browser");
    println!("```");

    Ok(())
}