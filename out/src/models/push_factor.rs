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
pub struct PushFactor {
  #[serde(rename = "profile")]
  profile: Option<::models::PushFactorProfile>
}

impl PushFactor {
  pub fn new() -> PushFactor {
    PushFactor {
      profile: None
    }
  }

  pub fn set_profile(&mut self, profile: ::models::PushFactorProfile) {
    self.profile = Some(profile);
  }

  pub fn with_profile(mut self, profile: ::models::PushFactorProfile) -> PushFactor {
    self.profile = Some(profile);
    self
  }

  pub fn profile(&self) -> Option<&::models::PushFactorProfile> {
    self.profile.as_ref()
  }

  pub fn reset_profile(&mut self) {
    self.profile = None;
  }

}



