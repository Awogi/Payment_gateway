use esewa::{Esewa, PaymentRequest};

const TEST_MERCHANT_CODE: &str = "EPAYTEST";
const TEST_SECRET_KEY: &str = "8gBm/:&EnhH.1/q";

#[test]
fn test_esewa_client_creation() {
    let _esewa = Esewa::new(TEST_MERCHANT_CODE, TEST_SECRET_KEY, true);
    // Just ensure it can be created without panicking
    assert!(true);
}

#[test]
fn test_payment_request_creation() {
    let payment = PaymentRequest::new(
        100.0,
        "test-123",
        "https://example.com/success",
        "https://example.com/failure"
    );

    assert_eq!(payment.amount, 100.0);
    assert_eq!(payment.total_amount, 100.0);
    assert_eq!(payment.transaction_uuid, "test-123");
    assert_eq!(payment.tax_amount, 0.0);
    assert_eq!(payment.product_service_charge, 0.0);
    assert_eq!(payment.product_delivery_charge, 0.0);
}

#[test]
fn test_payment_request_with_charges() {
    let payment = PaymentRequest::with_charges(
        100.0, // base
        13.0,  // tax
        5.0,   // service
        2.0,   // delivery
        "test-456",
        "https://example.com/success",
        "https://example.com/failure"
    );

    assert_eq!(payment.amount, 100.0);
    assert_eq!(payment.tax_amount, 13.0);
    assert_eq!(payment.product_service_charge, 5.0);
    assert_eq!(payment.product_delivery_charge, 2.0);
    assert_eq!(payment.total_amount, 120.0); // 100 + 13 + 5 + 2
}

#[test]
fn test_payment_form_generation() {
    let esewa = Esewa::new(TEST_MERCHANT_CODE, TEST_SECRET_KEY, true);
    let payment = PaymentRequest::new(
        50.0,
        "test-form-123",
        "https://example.com/success",
        "https://example.com/failure"
    );

    let form_html = esewa.create_payment_form(&payment).unwrap();

    // Check HTML structure
    assert!(form_html.contains("<!DOCTYPE html>"));
    assert!(form_html.contains("<form"));
    assert!(form_html.contains("method=\"POST\""));
    assert!(form_html.contains("https://rc-epay.esewa.com.np/api/epay/main/v2/form"));
    
    // Check form fields
    assert!(form_html.contains("name=\"amount\" value=\"50\""));
    assert!(form_html.contains("name=\"total_amount\" value=\"50\""));
    assert!(form_html.contains("name=\"transaction_uuid\" value=\"test-form-123\""));
    assert!(form_html.contains("name=\"product_code\" value=\"EPAYTEST\""));
    assert!(form_html.contains("name=\"success_url\" value=\"https://example.com/success\""));
    assert!(form_html.contains("name=\"failure_url\" value=\"https://example.com/failure\""));
    assert!(form_html.contains("name=\"signed_field_names\""));
    assert!(form_html.contains("name=\"signature\""));
    
    // Check auto-submit
    assert!(form_html.contains("onload=\"document.forms[0].submit()\""));
}

#[test]
fn test_sandbox_vs_production_urls() {
    let sandbox_esewa = Esewa::new(TEST_MERCHANT_CODE, TEST_SECRET_KEY, true);
    let prod_esewa = Esewa::new(TEST_MERCHANT_CODE, TEST_SECRET_KEY, false);
    
    let payment = PaymentRequest::new(100.0, "test", "success", "failure");
    
    let sandbox_form = sandbox_esewa.create_payment_form(&payment).unwrap();
    let prod_form = prod_esewa.create_payment_form(&payment).unwrap();
    
    assert!(sandbox_form.contains("rc-epay.esewa.com.np"));
    assert!(prod_form.contains("epay.esewa.com.np"));
    assert!(!prod_form.contains("rc-epay"));
}

#[test]
fn test_different_amounts() {
    let esewa = Esewa::new(TEST_MERCHANT_CODE, TEST_SECRET_KEY, true);
    
    let test_amounts = vec![1.0, 10.5, 100.0, 999.99, 1000.0];
    
    for amount in test_amounts {
        let payment = PaymentRequest::new(
            amount,
            &format!("test-{}", amount),
            "success",
            "failure"
        );
        
        let form = esewa.create_payment_form(&payment).unwrap();
        assert!(form.contains(&format!("value=\"{}\"", amount)));
    }
}

#[tokio::test]
async fn test_transaction_status_check_structure() {
    let esewa = Esewa::new(TEST_MERCHANT_CODE, TEST_SECRET_KEY, true);
    
    // This will likely fail since we don't have a real transaction,
    // but it tests that the API call structure is correct
    let result = esewa.check_transaction_status(
        TEST_MERCHANT_CODE,
        "non-existent-transaction",
        100.0
    ).await;
    
    // We expect this to fail with a proper error, not panic
    match result {
        Ok(status) => {
            // If it somehow succeeds, verify the structure
            assert!(!status.product_code.is_empty());
        },
        Err(e) => {
            // Expected - verify it's a proper API error, not a panic
            let error_str = e.to_string();
            assert!(error_str.contains("HTTP") || 
                   error_str.contains("API") || 
                   error_str.contains("Invalid") ||
                   error_str.contains("request"));
        }
    }
}

#[test]
fn test_payment_url_method() {
    let esewa = Esewa::new(TEST_MERCHANT_CODE, TEST_SECRET_KEY, true);
    let payment = PaymentRequest::new(100.0, "test", "success", "failure");
    
    let url = esewa.get_payment_url(&payment).unwrap();
    assert!(url.contains("rc-epay.esewa.com.np"));
    assert!(url.contains("/api/epay/main/v2/form"));
}

#[cfg(test)]
mod integration_examples {
    use super::*;
    
    #[test]
    fn test_real_world_usage_example() {
        // This shows how someone would actually use the library
        let esewa = Esewa::new("EPAYTEST", "8gBm/:&EnhH.1/q", true);
        
        // Simple payment
        let payment = PaymentRequest::new(
            250.0,
            "order-2024-001", 
            "https://mystore.com/payment/success",
            "https://mystore.com/payment/failure"
        );
        
        let form_html = esewa.create_payment_form(&payment).unwrap();
        
        // Verify it has all the required elements for a real integration
        assert!(form_html.len() > 500); // Should be a substantial HTML form
        assert!(form_html.contains("Redirecting to eSewa"));
        assert!(form_html.contains("order-2024-001"));
        assert!(form_html.contains("250"));
        
        println!("✅ Real-world integration example works correctly");
    }
    
    #[test] 
    fn test_ecommerce_scenario() {
        let esewa = Esewa::new("EPAYTEST", "8gBm/:&EnhH.1/q", true);
        
        // E-commerce scenario with breakdown
        let payment = PaymentRequest::with_charges(
            1000.0, // Product price
            130.0,  // 13% VAT
            50.0,   // Service charge
            100.0,  // Delivery charge
            "ecom-order-12345",
            "https://shop.com/success",
            "https://shop.com/cancel"
        );
        
        assert_eq!(payment.total_amount, 1280.0);
        
        let form = esewa.create_payment_form(&payment).unwrap();
        assert!(form.contains("1280")); // Total amount
        assert!(form.contains("ecom-order-12345"));
        
        println!("✅ E-commerce scenario works correctly");
    }
}