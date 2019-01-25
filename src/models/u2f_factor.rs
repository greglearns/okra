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
pub struct U2fFactor {
  #[serde(rename = "profile")]
  profile: Option<Value>
}

impl U2fFactor {
  pub fn new() -> U2fFactor {
    U2fFactor {
      profile: None
    }
  }

  pub fn set_profile(&mut self, profile: Value) {
    self.profile = Some(profile);
  }

  pub fn with_profile(mut self, profile: Value) -> U2fFactor {
    self.profile = Some(profile);
    self
  }

  pub fn profile(&self) -> Option<&Value> {
    self.profile.as_ref()
  }

  pub fn reset_profile(&mut self) {
    self.profile = None;
  }

}



