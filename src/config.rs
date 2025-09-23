use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};

pub struct EsewaConfig {
    pub client_id: String,
    pub client_secret: String,
    pub merchant_code: String,
    pub sandbox: bool,
}

impl EsewaConfig {
    pub fn base_url(&self) -> &str {
        if self.sandbox {
            "https://uat.esewa.com.np"
        } else {
            "https://esewa.com.np"
        }
    }

    pub fn generate_signature(&self, data: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.client_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(data.as_bytes());
        let result = mac.finalize().into_bytes();
        general_purpose::STANDARD.encode(result)
    }
}
