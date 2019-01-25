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
pub struct AppUserCredentials {
  #[serde(rename = "password")]
  password: Option<crate::models::AppUserPasswordCredential>,
  #[serde(rename = "userName")]
  user_name: Option<String>
}

impl AppUserCredentials {
  pub fn new() -> AppUserCredentials {
    AppUserCredentials {
      password: None,
      user_name: None
    }
  }

  pub fn set_password(&mut self, password: crate::models::AppUserPasswordCredential) {
    self.password = Some(password);
  }

  pub fn with_password(mut self, password: crate::models::AppUserPasswordCredential) -> AppUserCredentials {
    self.password = Some(password);
    self
  }

  pub fn password(&self) -> Option<&crate::models::AppUserPasswordCredential> {
    self.password.as_ref()
  }

  pub fn reset_password(&mut self) {
    self.password = None;
  }

  pub fn set_user_name(&mut self, user_name: String) {
    self.user_name = Some(user_name);
  }

  pub fn with_user_name(mut self, user_name: String) -> AppUserCredentials {
    self.user_name = Some(user_name);
    self
  }

  pub fn user_name(&self) -> Option<&String> {
    self.user_name.as_ref()
  }

  pub fn reset_user_name(&mut self) {
    self.user_name = None;
  }

}


