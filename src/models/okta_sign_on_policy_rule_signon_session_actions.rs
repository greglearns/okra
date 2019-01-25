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
pub struct OktaSignOnPolicyRuleSignonSessionActions {
  #[serde(rename = "maxSessionIdleMinutes")]
  max_session_idle_minutes: Option<i32>,
  #[serde(rename = "maxSessionLifetimeMinutes")]
  max_session_lifetime_minutes: Option<i32>,
  #[serde(rename = "usePersistentCookie")]
  use_persistent_cookie: Option<bool>
}

impl OktaSignOnPolicyRuleSignonSessionActions {
  pub fn new() -> OktaSignOnPolicyRuleSignonSessionActions {
    OktaSignOnPolicyRuleSignonSessionActions {
      max_session_idle_minutes: None,
      max_session_lifetime_minutes: None,
      use_persistent_cookie: None
    }
  }

  pub fn set_max_session_idle_minutes(&mut self, max_session_idle_minutes: i32) {
    self.max_session_idle_minutes = Some(max_session_idle_minutes);
  }

  pub fn with_max_session_idle_minutes(mut self, max_session_idle_minutes: i32) -> OktaSignOnPolicyRuleSignonSessionActions {
    self.max_session_idle_minutes = Some(max_session_idle_minutes);
    self
  }

  pub fn max_session_idle_minutes(&self) -> Option<&i32> {
    self.max_session_idle_minutes.as_ref()
  }

  pub fn reset_max_session_idle_minutes(&mut self) {
    self.max_session_idle_minutes = None;
  }

  pub fn set_max_session_lifetime_minutes(&mut self, max_session_lifetime_minutes: i32) {
    self.max_session_lifetime_minutes = Some(max_session_lifetime_minutes);
  }

  pub fn with_max_session_lifetime_minutes(mut self, max_session_lifetime_minutes: i32) -> OktaSignOnPolicyRuleSignonSessionActions {
    self.max_session_lifetime_minutes = Some(max_session_lifetime_minutes);
    self
  }

  pub fn max_session_lifetime_minutes(&self) -> Option<&i32> {
    self.max_session_lifetime_minutes.as_ref()
  }

  pub fn reset_max_session_lifetime_minutes(&mut self) {
    self.max_session_lifetime_minutes = None;
  }

  pub fn set_use_persistent_cookie(&mut self, use_persistent_cookie: bool) {
    self.use_persistent_cookie = Some(use_persistent_cookie);
  }

  pub fn with_use_persistent_cookie(mut self, use_persistent_cookie: bool) -> OktaSignOnPolicyRuleSignonSessionActions {
    self.use_persistent_cookie = Some(use_persistent_cookie);
    self
  }

  pub fn use_persistent_cookie(&self) -> Option<&bool> {
    self.use_persistent_cookie.as_ref()
  }

  pub fn reset_use_persistent_cookie(&mut self) {
    self.use_persistent_cookie = None;
  }

}


