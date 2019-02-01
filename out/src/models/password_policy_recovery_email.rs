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
pub struct PasswordPolicyRecoveryEmail {
  #[serde(rename = "properties")]
  properties: Option<::models::PasswordPolicyRecoveryEmailProperties>,
  #[serde(rename = "status")]
  status: Option<String>
}

impl PasswordPolicyRecoveryEmail {
  pub fn new() -> PasswordPolicyRecoveryEmail {
    PasswordPolicyRecoveryEmail {
      properties: None,
      status: None
    }
  }

  pub fn set_properties(&mut self, properties: ::models::PasswordPolicyRecoveryEmailProperties) {
    self.properties = Some(properties);
  }

  pub fn with_properties(mut self, properties: ::models::PasswordPolicyRecoveryEmailProperties) -> PasswordPolicyRecoveryEmail {
    self.properties = Some(properties);
    self
  }

  pub fn properties(&self) -> Option<&::models::PasswordPolicyRecoveryEmailProperties> {
    self.properties.as_ref()
  }

  pub fn reset_properties(&mut self) {
    self.properties = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> PasswordPolicyRecoveryEmail {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



