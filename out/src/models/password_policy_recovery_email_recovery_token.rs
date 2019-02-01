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
pub struct PasswordPolicyRecoveryEmailRecoveryToken {
  #[serde(rename = "tokenLifetimeMinutes")]
  token_lifetime_minutes: Option<i32>
}

impl PasswordPolicyRecoveryEmailRecoveryToken {
  pub fn new() -> PasswordPolicyRecoveryEmailRecoveryToken {
    PasswordPolicyRecoveryEmailRecoveryToken {
      token_lifetime_minutes: None
    }
  }

  pub fn set_token_lifetime_minutes(&mut self, token_lifetime_minutes: i32) {
    self.token_lifetime_minutes = Some(token_lifetime_minutes);
  }

  pub fn with_token_lifetime_minutes(mut self, token_lifetime_minutes: i32) -> PasswordPolicyRecoveryEmailRecoveryToken {
    self.token_lifetime_minutes = Some(token_lifetime_minutes);
    self
  }

  pub fn token_lifetime_minutes(&self) -> Option<&i32> {
    self.token_lifetime_minutes.as_ref()
  }

  pub fn reset_token_lifetime_minutes(&mut self) {
    self.token_lifetime_minutes = None;
  }

}



