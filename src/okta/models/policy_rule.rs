#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PolicyRule {
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    r#created: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    r#last_updated: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    r#priority: Option<i32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<String>,
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    r#system: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}

impl r#PolicyRule {
    pub fn new(
    ) -> Self {
        Self {
          r#created: None,
          r#id: None,
          r#last_updated: None,
          r#priority: None,
          r#status: None,
          r#system: None,
          r#type: None,
        }
    }

    pub fn set_created(&mut self, r#created: String) {
        self.r#created = Some(r#created);
    }

    pub fn with_created(mut self, r#created: String) -> Self {
        self.r#created = Some(r#created);
        self
    }

    pub fn with_option_created(mut self, r#created: Option<String>) -> Self {
        self.r#created = r#created;
        self
    }

    pub fn r#created(&self) -> Option<&str> {
        self.r#created.as_ref().map(|x| x.borrow())
    }

    pub fn reset_created(&mut self) {
        self.r#created = None;
    }

    pub fn set_id(&mut self, r#id: String) {
        self.r#id = Some(r#id);
    }

    pub fn with_id(mut self, r#id: String) -> Self {
        self.r#id = Some(r#id);
        self
    }

    pub fn with_option_id(mut self, r#id: Option<String>) -> Self {
        self.r#id = r#id;
        self
    }

    pub fn r#id(&self) -> Option<&str> {
        self.r#id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_id(&mut self) {
        self.r#id = None;
    }

    pub fn set_last_updated(&mut self, r#last_updated: String) {
        self.r#last_updated = Some(r#last_updated);
    }

    pub fn with_last_updated(mut self, r#last_updated: String) -> Self {
        self.r#last_updated = Some(r#last_updated);
        self
    }

    pub fn with_option_last_updated(mut self, r#last_updated: Option<String>) -> Self {
        self.r#last_updated = r#last_updated;
        self
    }

    pub fn r#last_updated(&self) -> Option<&str> {
        self.r#last_updated.as_ref().map(|x| x.borrow())
    }

    pub fn reset_last_updated(&mut self) {
        self.r#last_updated = None;
    }

    pub fn set_priority(&mut self, r#priority: i32) {
        self.r#priority = Some(r#priority);
    }

    pub fn with_priority(mut self, r#priority: i32) -> Self {
        self.r#priority = Some(r#priority);
        self
    }

    pub fn with_option_priority(mut self, r#priority: Option<i32>) -> Self {
        self.r#priority = r#priority;
        self
    }

    pub fn r#priority(&self) -> Option<&i32> {
        self.r#priority.as_ref().map(|x| x.borrow())
    }

    pub fn reset_priority(&mut self) {
        self.r#priority = None;
    }

    pub fn set_status(&mut self, r#status: String) {
        self.r#status = Some(r#status);
    }

    pub fn with_status(mut self, r#status: String) -> Self {
        self.r#status = Some(r#status);
        self
    }

    pub fn with_option_status(mut self, r#status: Option<String>) -> Self {
        self.r#status = r#status;
        self
    }

    pub fn r#status(&self) -> Option<&str> {
        self.r#status.as_ref().map(|x| x.borrow())
    }

    pub fn reset_status(&mut self) {
        self.r#status = None;
    }

    pub fn set_system(&mut self, r#system: bool) {
        self.r#system = Some(r#system);
    }

    pub fn with_system(mut self, r#system: bool) -> Self {
        self.r#system = Some(r#system);
        self
    }

    pub fn with_option_system(mut self, r#system: Option<bool>) -> Self {
        self.r#system = r#system;
        self
    }

    pub fn r#system(&self) -> Option<&bool> {
        self.r#system.as_ref().map(|x| x.borrow())
    }

    pub fn reset_system(&mut self) {
        self.r#system = None;
    }

    pub fn set_type(&mut self, r#type: String) {
        self.r#type = Some(r#type);
    }

    pub fn with_type(mut self, r#type: String) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn with_option_type(mut self, r#type: Option<String>) -> Self {
        self.r#type = r#type;
        self
    }

    pub fn r#type(&self) -> Option<&str> {
        self.r#type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_type(&mut self) {
        self.r#type = None;
    }
}
