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
pub struct SamlAttributeStatement {
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "namespace")]
  namespace: Option<String>,
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "values")]
  values: Option<Vec<String>>
}

impl SamlAttributeStatement {
  pub fn new() -> SamlAttributeStatement {
    SamlAttributeStatement {
      name: None,
      namespace: None,
      _type: None,
      values: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> SamlAttributeStatement {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_namespace(&mut self, namespace: String) {
    self.namespace = Some(namespace);
  }

  pub fn with_namespace(mut self, namespace: String) -> SamlAttributeStatement {
    self.namespace = Some(namespace);
    self
  }

  pub fn namespace(&self) -> Option<&String> {
    self.namespace.as_ref()
  }

  pub fn reset_namespace(&mut self) {
    self.namespace = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> SamlAttributeStatement {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_values(&mut self, values: Vec<String>) {
    self.values = Some(values);
  }

  pub fn with_values(mut self, values: Vec<String>) -> SamlAttributeStatement {
    self.values = Some(values);
    self
  }

  pub fn values(&self) -> Option<&Vec<String>> {
    self.values.as_ref()
  }

  pub fn reset_values(&mut self) {
    self.values = None;
  }

}



