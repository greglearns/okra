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
pub struct TokenFactor {
  #[serde(rename = "profile")]
  profile: Option<::models::TokenFactorProfile>
}

impl TokenFactor {
  pub fn new() -> TokenFactor {
    TokenFactor {
      profile: None
    }
  }

  pub fn set_profile(&mut self, profile: ::models::TokenFactorProfile) {
    self.profile = Some(profile);
  }

  pub fn with_profile(mut self, profile: ::models::TokenFactorProfile) -> TokenFactor {
    self.profile = Some(profile);
    self
  }

  pub fn profile(&self) -> Option<&::models::TokenFactorProfile> {
    self.profile.as_ref()
  }

  pub fn reset_profile(&mut self) {
    self.profile = None;
  }

}



