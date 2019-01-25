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
pub struct OktaSignOnPolicyRuleSignonActions {
  #[serde(rename = "access")]
  access: Option<String>,
  #[serde(rename = "factorLifetime")]
  factor_lifetime: Option<i32>,
  #[serde(rename = "factorPromptMode")]
  factor_prompt_mode: Option<String>,
  #[serde(rename = "rememberDeviceByDefault")]
  remember_device_by_default: Option<bool>,
  #[serde(rename = "requireFactor")]
  require_factor: Option<bool>,
  #[serde(rename = "session")]
  session: Option<crate::models::OktaSignOnPolicyRuleSignonSessionActions>
}

impl OktaSignOnPolicyRuleSignonActions {
  pub fn new() -> OktaSignOnPolicyRuleSignonActions {
    OktaSignOnPolicyRuleSignonActions {
      access: None,
      factor_lifetime: None,
      factor_prompt_mode: None,
      remember_device_by_default: None,
      require_factor: None,
      session: None
    }
  }

  pub fn set_access(&mut self, access: String) {
    self.access = Some(access);
  }

  pub fn with_access(mut self, access: String) -> OktaSignOnPolicyRuleSignonActions {
    self.access = Some(access);
    self
  }

  pub fn access(&self) -> Option<&String> {
    self.access.as_ref()
  }

  pub fn reset_access(&mut self) {
    self.access = None;
  }

  pub fn set_factor_lifetime(&mut self, factor_lifetime: i32) {
    self.factor_lifetime = Some(factor_lifetime);
  }

  pub fn with_factor_lifetime(mut self, factor_lifetime: i32) -> OktaSignOnPolicyRuleSignonActions {
    self.factor_lifetime = Some(factor_lifetime);
    self
  }

  pub fn factor_lifetime(&self) -> Option<&i32> {
    self.factor_lifetime.as_ref()
  }

  pub fn reset_factor_lifetime(&mut self) {
    self.factor_lifetime = None;
  }

  pub fn set_factor_prompt_mode(&mut self, factor_prompt_mode: String) {
    self.factor_prompt_mode = Some(factor_prompt_mode);
  }

  pub fn with_factor_prompt_mode(mut self, factor_prompt_mode: String) -> OktaSignOnPolicyRuleSignonActions {
    self.factor_prompt_mode = Some(factor_prompt_mode);
    self
  }

  pub fn factor_prompt_mode(&self) -> Option<&String> {
    self.factor_prompt_mode.as_ref()
  }

  pub fn reset_factor_prompt_mode(&mut self) {
    self.factor_prompt_mode = None;
  }

  pub fn set_remember_device_by_default(&mut self, remember_device_by_default: bool) {
    self.remember_device_by_default = Some(remember_device_by_default);
  }

  pub fn with_remember_device_by_default(mut self, remember_device_by_default: bool) -> OktaSignOnPolicyRuleSignonActions {
    self.remember_device_by_default = Some(remember_device_by_default);
    self
  }

  pub fn remember_device_by_default(&self) -> Option<&bool> {
    self.remember_device_by_default.as_ref()
  }

  pub fn reset_remember_device_by_default(&mut self) {
    self.remember_device_by_default = None;
  }

  pub fn set_require_factor(&mut self, require_factor: bool) {
    self.require_factor = Some(require_factor);
  }

  pub fn with_require_factor(mut self, require_factor: bool) -> OktaSignOnPolicyRuleSignonActions {
    self.require_factor = Some(require_factor);
    self
  }

  pub fn require_factor(&self) -> Option<&bool> {
    self.require_factor.as_ref()
  }

  pub fn reset_require_factor(&mut self) {
    self.require_factor = None;
  }

  pub fn set_session(&mut self, session: crate::models::OktaSignOnPolicyRuleSignonSessionActions) {
    self.session = Some(session);
  }

  pub fn with_session(mut self, session: crate::models::OktaSignOnPolicyRuleSignonSessionActions) -> OktaSignOnPolicyRuleSignonActions {
    self.session = Some(session);
    self
  }

  pub fn session(&self) -> Option<&crate::models::OktaSignOnPolicyRuleSignonSessionActions> {
    self.session.as_ref()
  }

  pub fn reset_session(&mut self) {
    self.session = None;
  }

}



