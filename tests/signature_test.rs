use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};

type HmacSha256 = Hmac<Sha256>;

#[test]
fn reproduces_sample_signature() {
    // These values are taken from payment_form.html
    let total_amount = 100.0_f64;
    let transaction_uuid = "order-12345";
    let product_code = "EPAYTEST";
    let secret = "8gBm/:&EnhH.1/q";

    let signature_data = format!("{},{},{}", total_amount, transaction_uuid, product_code);

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC key ok");
    mac.update(signature_data.as_bytes());
    let result = mac.finalize().into_bytes();
    let sig = general_purpose::STANDARD.encode(result);

    // signature present in payment_form.html
    let expected = "SczXT9SBIbYuHL5fcAJhC6MxMAcj/Fj4BnvZ9+T0/Ko=";

    assert_eq!(sig, expected, "Generated signature did not match the example in payment_form.html");
}
