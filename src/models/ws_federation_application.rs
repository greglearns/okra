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
pub struct WsFederationApplication {
  #[serde(rename = "name")]
  name: Option<Value>,
  #[serde(rename = "settings")]
  settings: Option<crate::models::WsFederationApplicationSettings>
}

impl WsFederationApplication {
  pub fn new() -> WsFederationApplication {
    WsFederationApplication {
      name: None,
      settings: None
    }
  }

  pub fn set_name(&mut self, name: Value) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: Value) -> WsFederationApplication {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&Value> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_settings(&mut self, settings: crate::models::WsFederationApplicationSettings) {
    self.settings = Some(settings);
  }

  pub fn with_settings(mut self, settings: crate::models::WsFederationApplicationSettings) -> WsFederationApplication {
    self.settings = Some(settings);
    self
  }

  pub fn settings(&self) -> Option<&crate::models::WsFederationApplicationSettings> {
    self.settings.as_ref()
  }

  pub fn reset_settings(&mut self) {
    self.settings = None;
  }

}



