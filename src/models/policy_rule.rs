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
pub struct PolicyRule {
  #[serde(rename = "created")]
  created: Option<String>,
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "lastUpdated")]
  last_updated: Option<String>,
  #[serde(rename = "priority")]
  priority: Option<i32>,
  #[serde(rename = "status")]
  status: Option<String>,
  #[serde(rename = "system")]
  system: Option<bool>,
  #[serde(rename = "type")]
  _type: Option<String>
}

impl PolicyRule {
  pub fn new() -> PolicyRule {
    PolicyRule {
      created: None,
      id: None,
      last_updated: None,
      priority: None,
      status: None,
      system: None,
      _type: None
    }
  }

  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> PolicyRule {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&String> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> PolicyRule {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_last_updated(&mut self, last_updated: String) {
    self.last_updated = Some(last_updated);
  }

  pub fn with_last_updated(mut self, last_updated: String) -> PolicyRule {
    self.last_updated = Some(last_updated);
    self
  }

  pub fn last_updated(&self) -> Option<&String> {
    self.last_updated.as_ref()
  }

  pub fn reset_last_updated(&mut self) {
    self.last_updated = None;
  }

  pub fn set_priority(&mut self, priority: i32) {
    self.priority = Some(priority);
  }

  pub fn with_priority(mut self, priority: i32) -> PolicyRule {
    self.priority = Some(priority);
    self
  }

  pub fn priority(&self) -> Option<&i32> {
    self.priority.as_ref()
  }

  pub fn reset_priority(&mut self) {
    self.priority = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> PolicyRule {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_system(&mut self, system: bool) {
    self.system = Some(system);
  }

  pub fn with_system(mut self, system: bool) -> PolicyRule {
    self.system = Some(system);
    self
  }

  pub fn system(&self) -> Option<&bool> {
    self.system.as_ref()
  }

  pub fn reset_system(&mut self) {
    self.system = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> PolicyRule {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



