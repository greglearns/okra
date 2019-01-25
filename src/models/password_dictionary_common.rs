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
pub struct PasswordDictionaryCommon {
  #[serde(rename = "exclude")]
  exclude: Option<bool>
}

impl PasswordDictionaryCommon {
  pub fn new() -> PasswordDictionaryCommon {
    PasswordDictionaryCommon {
      exclude: None
    }
  }

  pub fn set_exclude(&mut self, exclude: bool) {
    self.exclude = Some(exclude);
  }

  pub fn with_exclude(mut self, exclude: bool) -> PasswordDictionaryCommon {
    self.exclude = Some(exclude);
    self
  }

  pub fn exclude(&self) -> Option<&bool> {
    self.exclude.as_ref()
  }

  pub fn reset_exclude(&mut self) {
    self.exclude = None;
  }

}



