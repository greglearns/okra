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
pub struct SamlApplication {
  #[serde(rename = "settings")]
  settings: Option<::models::SamlApplicationSettings>
}

impl SamlApplication {
  pub fn new() -> SamlApplication {
    SamlApplication {
      settings: None
    }
  }

  pub fn set_settings(&mut self, settings: ::models::SamlApplicationSettings) {
    self.settings = Some(settings);
  }

  pub fn with_settings(mut self, settings: ::models::SamlApplicationSettings) -> SamlApplication {
    self.settings = Some(settings);
    self
  }

  pub fn settings(&self) -> Option<&::models::SamlApplicationSettings> {
    self.settings.as_ref()
  }

  pub fn reset_settings(&mut self) {
    self.settings = None;
  }

}


