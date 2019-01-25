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
pub struct Factor {
  #[serde(rename = "_embedded")]
  _embedded: Option<::std::collections::HashMap<String, Value>>,
  #[serde(rename = "_links")]
  _links: Option<::std::collections::HashMap<String, Value>>,
  #[serde(rename = "device")]
  device: Option<String>,
  #[serde(rename = "deviceType")]
  device_type: Option<String>,
  #[serde(rename = "factorType")]
  factor_type: Option<crate::models::FactorType>,
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "mfaStateTokenId")]
  mfa_state_token_id: Option<String>,
  #[serde(rename = "profile")]
  profile: Option<Value>,
  #[serde(rename = "provider")]
  provider: Option<crate::models::FactorProvider>,
  #[serde(rename = "rechallengeExistingFactor")]
  rechallenge_existing_factor: Option<bool>,
  #[serde(rename = "sessionId")]
  session_id: Option<String>,
  #[serde(rename = "status")]
  status: Option<crate::models::FactorStatus>,
  #[serde(rename = "tokenLifetimeSeconds")]
  token_lifetime_seconds: Option<i32>,
  #[serde(rename = "userId")]
  user_id: Option<String>,
  #[serde(rename = "verify")]
  verify: Option<crate::models::VerifyFactorRequest>
}

impl Factor {
  pub fn new() -> Factor {
    Factor {
      _embedded: None,
      _links: None,
      device: None,
      device_type: None,
      factor_type: None,
      id: None,
      mfa_state_token_id: None,
      profile: None,
      provider: None,
      rechallenge_existing_factor: None,
      session_id: None,
      status: None,
      token_lifetime_seconds: None,
      user_id: None,
      verify: None
    }
  }

  pub fn set__embedded(&mut self, _embedded: ::std::collections::HashMap<String, Value>) {
    self._embedded = Some(_embedded);
  }

  pub fn with__embedded(mut self, _embedded: ::std::collections::HashMap<String, Value>) -> Factor {
    self._embedded = Some(_embedded);
    self
  }

  pub fn _embedded(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self._embedded.as_ref()
  }

  pub fn reset__embedded(&mut self) {
    self._embedded = None;
  }

  pub fn set__links(&mut self, _links: ::std::collections::HashMap<String, Value>) {
    self._links = Some(_links);
  }

  pub fn with__links(mut self, _links: ::std::collections::HashMap<String, Value>) -> Factor {
    self._links = Some(_links);
    self
  }

  pub fn _links(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self._links.as_ref()
  }

  pub fn reset__links(&mut self) {
    self._links = None;
  }

  pub fn set_device(&mut self, device: String) {
    self.device = Some(device);
  }

  pub fn with_device(mut self, device: String) -> Factor {
    self.device = Some(device);
    self
  }

  pub fn device(&self) -> Option<&String> {
    self.device.as_ref()
  }

  pub fn reset_device(&mut self) {
    self.device = None;
  }

  pub fn set_device_type(&mut self, device_type: String) {
    self.device_type = Some(device_type);
  }

  pub fn with_device_type(mut self, device_type: String) -> Factor {
    self.device_type = Some(device_type);
    self
  }

  pub fn device_type(&self) -> Option<&String> {
    self.device_type.as_ref()
  }

  pub fn reset_device_type(&mut self) {
    self.device_type = None;
  }

  pub fn set_factor_type(&mut self, factor_type: crate::models::FactorType) {
    self.factor_type = Some(factor_type);
  }

  pub fn with_factor_type(mut self, factor_type: crate::models::FactorType) -> Factor {
    self.factor_type = Some(factor_type);
    self
  }

  pub fn factor_type(&self) -> Option<&crate::models::FactorType> {
    self.factor_type.as_ref()
  }

  pub fn reset_factor_type(&mut self) {
    self.factor_type = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> Factor {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_mfa_state_token_id(&mut self, mfa_state_token_id: String) {
    self.mfa_state_token_id = Some(mfa_state_token_id);
  }

  pub fn with_mfa_state_token_id(mut self, mfa_state_token_id: String) -> Factor {
    self.mfa_state_token_id = Some(mfa_state_token_id);
    self
  }

  pub fn mfa_state_token_id(&self) -> Option<&String> {
    self.mfa_state_token_id.as_ref()
  }

  pub fn reset_mfa_state_token_id(&mut self) {
    self.mfa_state_token_id = None;
  }

  pub fn set_profile(&mut self, profile: Value) {
    self.profile = Some(profile);
  }

  pub fn with_profile(mut self, profile: Value) -> Factor {
    self.profile = Some(profile);
    self
  }

  pub fn profile(&self) -> Option<&Value> {
    self.profile.as_ref()
  }

  pub fn reset_profile(&mut self) {
    self.profile = None;
  }

  pub fn set_provider(&mut self, provider: crate::models::FactorProvider) {
    self.provider = Some(provider);
  }

  pub fn with_provider(mut self, provider: crate::models::FactorProvider) -> Factor {
    self.provider = Some(provider);
    self
  }

  pub fn provider(&self) -> Option<&crate::models::FactorProvider> {
    self.provider.as_ref()
  }

  pub fn reset_provider(&mut self) {
    self.provider = None;
  }

  pub fn set_rechallenge_existing_factor(&mut self, rechallenge_existing_factor: bool) {
    self.rechallenge_existing_factor = Some(rechallenge_existing_factor);
  }

  pub fn with_rechallenge_existing_factor(mut self, rechallenge_existing_factor: bool) -> Factor {
    self.rechallenge_existing_factor = Some(rechallenge_existing_factor);
    self
  }

  pub fn rechallenge_existing_factor(&self) -> Option<&bool> {
    self.rechallenge_existing_factor.as_ref()
  }

  pub fn reset_rechallenge_existing_factor(&mut self) {
    self.rechallenge_existing_factor = None;
  }

  pub fn set_session_id(&mut self, session_id: String) {
    self.session_id = Some(session_id);
  }

  pub fn with_session_id(mut self, session_id: String) -> Factor {
    self.session_id = Some(session_id);
    self
  }

  pub fn session_id(&self) -> Option<&String> {
    self.session_id.as_ref()
  }

  pub fn reset_session_id(&mut self) {
    self.session_id = None;
  }

  pub fn set_status(&mut self, status: crate::models::FactorStatus) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: crate::models::FactorStatus) -> Factor {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&crate::models::FactorStatus> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_token_lifetime_seconds(&mut self, token_lifetime_seconds: i32) {
    self.token_lifetime_seconds = Some(token_lifetime_seconds);
  }

  pub fn with_token_lifetime_seconds(mut self, token_lifetime_seconds: i32) -> Factor {
    self.token_lifetime_seconds = Some(token_lifetime_seconds);
    self
  }

  pub fn token_lifetime_seconds(&self) -> Option<&i32> {
    self.token_lifetime_seconds.as_ref()
  }

  pub fn reset_token_lifetime_seconds(&mut self) {
    self.token_lifetime_seconds = None;
  }

  pub fn set_user_id(&mut self, user_id: String) {
    self.user_id = Some(user_id);
  }

  pub fn with_user_id(mut self, user_id: String) -> Factor {
    self.user_id = Some(user_id);
    self
  }

  pub fn user_id(&self) -> Option<&String> {
    self.user_id.as_ref()
  }

  pub fn reset_user_id(&mut self) {
    self.user_id = None;
  }

  pub fn set_verify(&mut self, verify: crate::models::VerifyFactorRequest) {
    self.verify = Some(verify);
  }

  pub fn with_verify(mut self, verify: crate::models::VerifyFactorRequest) -> Factor {
    self.verify = Some(verify);
    self
  }

  pub fn verify(&self) -> Option<&crate::models::VerifyFactorRequest> {
    self.verify.as_ref()
  }

  pub fn reset_verify(&mut self) {
    self.verify = None;
  }

}


