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
pub struct LogGeographicalContext {
  #[serde(rename = "city")]
  city: Option<String>,
  #[serde(rename = "country")]
  country: Option<String>,
  #[serde(rename = "geolocation")]
  geolocation: Option<::models::LogGeolocation>,
  #[serde(rename = "postalCode")]
  postal_code: Option<String>,
  #[serde(rename = "state")]
  state: Option<String>
}

impl LogGeographicalContext {
  pub fn new() -> LogGeographicalContext {
    LogGeographicalContext {
      city: None,
      country: None,
      geolocation: None,
      postal_code: None,
      state: None
    }
  }

  pub fn set_city(&mut self, city: String) {
    self.city = Some(city);
  }

  pub fn with_city(mut self, city: String) -> LogGeographicalContext {
    self.city = Some(city);
    self
  }

  pub fn city(&self) -> Option<&String> {
    self.city.as_ref()
  }

  pub fn reset_city(&mut self) {
    self.city = None;
  }

  pub fn set_country(&mut self, country: String) {
    self.country = Some(country);
  }

  pub fn with_country(mut self, country: String) -> LogGeographicalContext {
    self.country = Some(country);
    self
  }

  pub fn country(&self) -> Option<&String> {
    self.country.as_ref()
  }

  pub fn reset_country(&mut self) {
    self.country = None;
  }

  pub fn set_geolocation(&mut self, geolocation: ::models::LogGeolocation) {
    self.geolocation = Some(geolocation);
  }

  pub fn with_geolocation(mut self, geolocation: ::models::LogGeolocation) -> LogGeographicalContext {
    self.geolocation = Some(geolocation);
    self
  }

  pub fn geolocation(&self) -> Option<&::models::LogGeolocation> {
    self.geolocation.as_ref()
  }

  pub fn reset_geolocation(&mut self) {
    self.geolocation = None;
  }

  pub fn set_postal_code(&mut self, postal_code: String) {
    self.postal_code = Some(postal_code);
  }

  pub fn with_postal_code(mut self, postal_code: String) -> LogGeographicalContext {
    self.postal_code = Some(postal_code);
    self
  }

  pub fn postal_code(&self) -> Option<&String> {
    self.postal_code.as_ref()
  }

  pub fn reset_postal_code(&mut self) {
    self.postal_code = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> LogGeographicalContext {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

}



