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
pub struct AutoLoginApplicationSettings {
  #[serde(rename = "signOn")]
  sign_on: Option<::models::AutoLoginApplicationSettingsSignOn>
}

impl AutoLoginApplicationSettings {
  pub fn new() -> AutoLoginApplicationSettings {
    AutoLoginApplicationSettings {
      sign_on: None
    }
  }

  pub fn set_sign_on(&mut self, sign_on: ::models::AutoLoginApplicationSettingsSignOn) {
    self.sign_on = Some(sign_on);
  }

  pub fn with_sign_on(mut self, sign_on: ::models::AutoLoginApplicationSettingsSignOn) -> AutoLoginApplicationSettings {
    self.sign_on = Some(sign_on);
    self
  }

  pub fn sign_on(&self) -> Option<&::models::AutoLoginApplicationSettingsSignOn> {
    self.sign_on.as_ref()
  }

  pub fn reset_sign_on(&mut self) {
    self.sign_on = None;
  }

}



