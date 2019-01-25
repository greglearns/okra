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
pub struct TotpFactor {
  #[serde(rename = "profile")]
  profile: Option<crate::models::TotpFactorProfile>
}

impl TotpFactor {
  pub fn new() -> TotpFactor {
    TotpFactor {
      profile: None
    }
  }

  pub fn set_profile(&mut self, profile: crate::models::TotpFactorProfile) {
    self.profile = Some(profile);
  }

  pub fn with_profile(mut self, profile: crate::models::TotpFactorProfile) -> TotpFactor {
    self.profile = Some(profile);
    self
  }

  pub fn profile(&self) -> Option<&crate::models::TotpFactorProfile> {
    self.profile.as_ref()
  }

  pub fn reset_profile(&mut self) {
    self.profile = None;
  }

}



