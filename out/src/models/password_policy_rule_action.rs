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
pub struct PasswordPolicyRuleAction {
  #[serde(rename = "access")]
  access: Option<String>
}

impl PasswordPolicyRuleAction {
  pub fn new() -> PasswordPolicyRuleAction {
    PasswordPolicyRuleAction {
      access: None
    }
  }

  pub fn set_access(&mut self, access: String) {
    self.access = Some(access);
  }

  pub fn with_access(mut self, access: String) -> PasswordPolicyRuleAction {
    self.access = Some(access);
    self
  }

  pub fn access(&self) -> Option<&String> {
    self.access.as_ref()
  }

  pub fn reset_access(&mut self) {
    self.access = None;
  }

}



