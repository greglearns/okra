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
pub struct PasswordPolicySettings {
  #[serde(rename = "delegation")]
  delegation: Option<::models::PasswordPolicyDelegationSettings>,
  #[serde(rename = "password")]
  password: Option<::models::PasswordPolicyPasswordSettings>,
  #[serde(rename = "recovery")]
  recovery: Option<::models::PasswordPolicyRecoverySettings>
}

impl PasswordPolicySettings {
  pub fn new() -> PasswordPolicySettings {
    PasswordPolicySettings {
      delegation: None,
      password: None,
      recovery: None
    }
  }

  pub fn set_delegation(&mut self, delegation: ::models::PasswordPolicyDelegationSettings) {
    self.delegation = Some(delegation);
  }

  pub fn with_delegation(mut self, delegation: ::models::PasswordPolicyDelegationSettings) -> PasswordPolicySettings {
    self.delegation = Some(delegation);
    self
  }

  pub fn delegation(&self) -> Option<&::models::PasswordPolicyDelegationSettings> {
    self.delegation.as_ref()
  }

  pub fn reset_delegation(&mut self) {
    self.delegation = None;
  }

  pub fn set_password(&mut self, password: ::models::PasswordPolicyPasswordSettings) {
    self.password = Some(password);
  }

  pub fn with_password(mut self, password: ::models::PasswordPolicyPasswordSettings) -> PasswordPolicySettings {
    self.password = Some(password);
    self
  }

  pub fn password(&self) -> Option<&::models::PasswordPolicyPasswordSettings> {
    self.password.as_ref()
  }

  pub fn reset_password(&mut self) {
    self.password = None;
  }

  pub fn set_recovery(&mut self, recovery: ::models::PasswordPolicyRecoverySettings) {
    self.recovery = Some(recovery);
  }

  pub fn with_recovery(mut self, recovery: ::models::PasswordPolicyRecoverySettings) -> PasswordPolicySettings {
    self.recovery = Some(recovery);
    self
  }

  pub fn recovery(&self) -> Option<&::models::PasswordPolicyRecoverySettings> {
    self.recovery.as_ref()
  }

  pub fn reset_recovery(&mut self) {
    self.recovery = None;
  }

}


