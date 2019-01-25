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
pub struct PasswordPolicy {
  #[serde(rename = "conditions")]
  conditions: Option<crate::models::PasswordPolicyConditions>,
  #[serde(rename = "settings")]
  settings: Option<crate::models::PasswordPolicySettings>
}

impl PasswordPolicy {
  pub fn new() -> PasswordPolicy {
    PasswordPolicy {
      conditions: None,
      settings: None
    }
  }

  pub fn set_conditions(&mut self, conditions: crate::models::PasswordPolicyConditions) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: crate::models::PasswordPolicyConditions) -> PasswordPolicy {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&crate::models::PasswordPolicyConditions> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

  pub fn set_settings(&mut self, settings: crate::models::PasswordPolicySettings) {
    self.settings = Some(settings);
  }

  pub fn with_settings(mut self, settings: crate::models::PasswordPolicySettings) -> PasswordPolicy {
    self.settings = Some(settings);
    self
  }

  pub fn settings(&self) -> Option<&crate::models::PasswordPolicySettings> {
    self.settings.as_ref()
  }

  pub fn reset_settings(&mut self) {
    self.settings = None;
  }

}



