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
pub struct SwaThreeFieldApplicationSettings {
  #[serde(rename = "app")]
  app: Option<::models::SwaThreeFieldApplicationSettingsApplication>
}

impl SwaThreeFieldApplicationSettings {
  pub fn new() -> SwaThreeFieldApplicationSettings {
    SwaThreeFieldApplicationSettings {
      app: None
    }
  }

  pub fn set_app(&mut self, app: ::models::SwaThreeFieldApplicationSettingsApplication) {
    self.app = Some(app);
  }

  pub fn with_app(mut self, app: ::models::SwaThreeFieldApplicationSettingsApplication) -> SwaThreeFieldApplicationSettings {
    self.app = Some(app);
    self
  }

  pub fn app(&self) -> Option<&::models::SwaThreeFieldApplicationSettingsApplication> {
    self.app.as_ref()
  }

  pub fn reset_app(&mut self) {
    self.app = None;
  }

}



