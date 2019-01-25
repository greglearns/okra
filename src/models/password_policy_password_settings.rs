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
pub struct PasswordPolicyPasswordSettings {
  #[serde(rename = "age")]
  age: Option<crate::models::PasswordPolicyPasswordSettingsAge>,
  #[serde(rename = "complexity")]
  complexity: Option<crate::models::PasswordPolicyPasswordSettingsComplexity>,
  #[serde(rename = "lockout")]
  lockout: Option<crate::models::PasswordPolicyPasswordSettingsLockout>
}

impl PasswordPolicyPasswordSettings {
  pub fn new() -> PasswordPolicyPasswordSettings {
    PasswordPolicyPasswordSettings {
      age: None,
      complexity: None,
      lockout: None
    }
  }

  pub fn set_age(&mut self, age: crate::models::PasswordPolicyPasswordSettingsAge) {
    self.age = Some(age);
  }

  pub fn with_age(mut self, age: crate::models::PasswordPolicyPasswordSettingsAge) -> PasswordPolicyPasswordSettings {
    self.age = Some(age);
    self
  }

  pub fn age(&self) -> Option<&crate::models::PasswordPolicyPasswordSettingsAge> {
    self.age.as_ref()
  }

  pub fn reset_age(&mut self) {
    self.age = None;
  }

  pub fn set_complexity(&mut self, complexity: crate::models::PasswordPolicyPasswordSettingsComplexity) {
    self.complexity = Some(complexity);
  }

  pub fn with_complexity(mut self, complexity: crate::models::PasswordPolicyPasswordSettingsComplexity) -> PasswordPolicyPasswordSettings {
    self.complexity = Some(complexity);
    self
  }

  pub fn complexity(&self) -> Option<&crate::models::PasswordPolicyPasswordSettingsComplexity> {
    self.complexity.as_ref()
  }

  pub fn reset_complexity(&mut self) {
    self.complexity = None;
  }

  pub fn set_lockout(&mut self, lockout: crate::models::PasswordPolicyPasswordSettingsLockout) {
    self.lockout = Some(lockout);
  }

  pub fn with_lockout(mut self, lockout: crate::models::PasswordPolicyPasswordSettingsLockout) -> PasswordPolicyPasswordSettings {
    self.lockout = Some(lockout);
    self
  }

  pub fn lockout(&self) -> Option<&crate::models::PasswordPolicyPasswordSettingsLockout> {
    self.lockout.as_ref()
  }

  pub fn reset_lockout(&mut self) {
    self.lockout = None;
  }

}


