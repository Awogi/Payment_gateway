use crate::{config::EsewaConfig, types::{PaymentRequest, PaymentResponse}};
use reqwest::Client;

pub struct EsewaClient {
    pub config: EsewaConfig,
    client: Client,
}

impl EsewaClient {
    pub fn new(config: EsewaConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Build an auto-submit payment form (HTML string)
    pub fn build_payment_form(&self, request: &PaymentRequest) -> String {
        let signature = self.config.generate_signature(&format!(
            "{}|{}|{}",
            request.product_id, request.amount, self.config.merchant_code
        ));

        format!(
r#"
<html>
  <body onload="document.forms[0].submit()">
    <form method="POST" action="{base}/epay/main">
      <input type="hidden" name="amt" value="{amount}" />
      <input type="hidden" name="scd" value="{merchant}" />
      <input type="hidden" name="pid" value="{pid}" />
      <input type="hidden" name="su" value="{success}" />
      <input type="hidden" name="fu" value="{failure}" />
      <input type="hidden" name="signature" value="{signature}" />
    </form>
  </body>
</html>
"#,
            base = self.config.base_url(),
            amount = request.amount,
            merchant = self.config.merchant_code,
            pid = request.product_id,
            success = request.success_url,
            failure = request.failure_url,
            signature = signature
        )
    }

    /// Verify transaction with eSewa API
    pub async fn verify_payment(&self, transaction_id: &str, amount: f64) -> reqwest::Result<PaymentResponse> {
        let url = format!("{}/epay/transrec", self.config.base_url());
        let resp = self.client
            .post(&url)
            .form(&[
                ("amt", amount.to_string()),
                ("pid", transaction_id.to_string()),
                ("scd", self.config.merchant_code.clone())
            ])
            .send()
            .await?
            .json::<PaymentResponse>()
            .await?;

        Ok(resp)
    }
}
