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
pub struct GroupRuleUserCondition {
  #[serde(rename = "exclude")]
  exclude: Option<Vec<String>>,
  #[serde(rename = "include")]
  include: Option<Vec<String>>
}

impl GroupRuleUserCondition {
  pub fn new() -> GroupRuleUserCondition {
    GroupRuleUserCondition {
      exclude: None,
      include: None
    }
  }

  pub fn set_exclude(&mut self, exclude: Vec<String>) {
    self.exclude = Some(exclude);
  }

  pub fn with_exclude(mut self, exclude: Vec<String>) -> GroupRuleUserCondition {
    self.exclude = Some(exclude);
    self
  }

  pub fn exclude(&self) -> Option<&Vec<String>> {
    self.exclude.as_ref()
  }

  pub fn reset_exclude(&mut self) {
    self.exclude = None;
  }

  pub fn set_include(&mut self, include: Vec<String>) {
    self.include = Some(include);
  }

  pub fn with_include(mut self, include: Vec<String>) -> GroupRuleUserCondition {
    self.include = Some(include);
    self
  }

  pub fn include(&self) -> Option<&Vec<String>> {
    self.include.as_ref()
  }

  pub fn reset_include(&mut self) {
    self.include = None;
  }

}



