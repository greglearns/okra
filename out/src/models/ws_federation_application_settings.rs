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
pub struct WsFederationApplicationSettings {
  #[serde(rename = "app")]
  app: Option<::models::WsFederationApplicationSettingsApplication>
}

impl WsFederationApplicationSettings {
  pub fn new() -> WsFederationApplicationSettings {
    WsFederationApplicationSettings {
      app: None
    }
  }

  pub fn set_app(&mut self, app: ::models::WsFederationApplicationSettingsApplication) {
    self.app = Some(app);
  }

  pub fn with_app(mut self, app: ::models::WsFederationApplicationSettingsApplication) -> WsFederationApplicationSettings {
    self.app = Some(app);
    self
  }

  pub fn app(&self) -> Option<&::models::WsFederationApplicationSettingsApplication> {
    self.app.as_ref()
  }

  pub fn reset_app(&mut self) {
    self.app = None;
  }

}


