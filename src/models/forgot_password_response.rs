/* 
 * Okta API
 *
 * Allows customers to easily access the Okta API
 *
 * OpenAPI spec version: 1.9.0
 * Contact: devex-public@okta.com
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordResponse {
  #[serde(rename = "resetPasswordUrl")]
  reset_password_url: Option<String>
}

impl ForgotPasswordResponse {
  pub fn new() -> ForgotPasswordResponse {
    ForgotPasswordResponse {
      reset_password_url: None
    }
  }

  pub fn set_reset_password_url(&mut self, reset_password_url: String) {
    self.reset_password_url = Some(reset_password_url);
  }

  pub fn with_reset_password_url(mut self, reset_password_url: String) -> ForgotPasswordResponse {
    self.reset_password_url = Some(reset_password_url);
    self
  }

  pub fn reset_password_url(&self) -> Option<&String> {
    self.reset_password_url.as_ref()
  }

  pub fn reset_reset_password_url(&mut self) {
    self.reset_password_url = None;
  }

}



