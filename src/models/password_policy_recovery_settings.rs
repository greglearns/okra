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
pub struct PasswordPolicyRecoverySettings {
  #[serde(rename = "factors")]
  factors: Option<crate::models::PasswordPolicyRecoveryFactors>
}

impl PasswordPolicyRecoverySettings {
  pub fn new() -> PasswordPolicyRecoverySettings {
    PasswordPolicyRecoverySettings {
      factors: None
    }
  }

  pub fn set_factors(&mut self, factors: crate::models::PasswordPolicyRecoveryFactors) {
    self.factors = Some(factors);
  }

  pub fn with_factors(mut self, factors: crate::models::PasswordPolicyRecoveryFactors) -> PasswordPolicyRecoverySettings {
    self.factors = Some(factors);
    self
  }

  pub fn factors(&self) -> Option<&crate::models::PasswordPolicyRecoveryFactors> {
    self.factors.as_ref()
  }

  pub fn reset_factors(&mut self) {
    self.factors = None;
  }

}


