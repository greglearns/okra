#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ActivateFactorRequest {
    #[serde(rename = "clientData", skip_serializing_if = "Option::is_none")]
    r#client_data: Option<String>,
    #[serde(rename = "passCode", skip_serializing_if = "Option::is_none")]
    r#pass_code: Option<String>,
    #[serde(rename = "registrationData", skip_serializing_if = "Option::is_none")]
    r#registration_data: Option<String>,
    #[serde(rename = "stateToken", skip_serializing_if = "Option::is_none")]
    r#state_token: Option<String>,
}

impl r#ActivateFactorRequest {
    pub fn new(
    ) -> Self {
        Self {
          r#client_data: None,
          r#pass_code: None,
          r#registration_data: None,
          r#state_token: None,
        }
    }

    pub fn set_client_data(&mut self, r#client_data: String) {
        self.r#client_data = Some(r#client_data);
    }

    pub fn with_client_data(mut self, r#client_data: String) -> Self {
        self.r#client_data = Some(r#client_data);
        self
    }

    pub fn with_option_client_data(mut self, r#client_data: Option<String>) -> Self {
        self.r#client_data = r#client_data;
        self
    }

    pub fn r#client_data(&self) -> Option<&str> {
        self.r#client_data.as_ref().map(|x| x.borrow())
    }

    pub fn reset_client_data(&mut self) {
        self.r#client_data = None;
    }

    pub fn set_pass_code(&mut self, r#pass_code: String) {
        self.r#pass_code = Some(r#pass_code);
    }

    pub fn with_pass_code(mut self, r#pass_code: String) -> Self {
        self.r#pass_code = Some(r#pass_code);
        self
    }

    pub fn with_option_pass_code(mut self, r#pass_code: Option<String>) -> Self {
        self.r#pass_code = r#pass_code;
        self
    }

    pub fn r#pass_code(&self) -> Option<&str> {
        self.r#pass_code.as_ref().map(|x| x.borrow())
    }

    pub fn reset_pass_code(&mut self) {
        self.r#pass_code = None;
    }

    pub fn set_registration_data(&mut self, r#registration_data: String) {
        self.r#registration_data = Some(r#registration_data);
    }

    pub fn with_registration_data(mut self, r#registration_data: String) -> Self {
        self.r#registration_data = Some(r#registration_data);
        self
    }

    pub fn with_option_registration_data(mut self, r#registration_data: Option<String>) -> Self {
        self.r#registration_data = r#registration_data;
        self
    }

    pub fn r#registration_data(&self) -> Option<&str> {
        self.r#registration_data.as_ref().map(|x| x.borrow())
    }

    pub fn reset_registration_data(&mut self) {
        self.r#registration_data = None;
    }

    pub fn set_state_token(&mut self, r#state_token: String) {
        self.r#state_token = Some(r#state_token);
    }

    pub fn with_state_token(mut self, r#state_token: String) -> Self {
        self.r#state_token = Some(r#state_token);
        self
    }

    pub fn with_option_state_token(mut self, r#state_token: Option<String>) -> Self {
        self.r#state_token = r#state_token;
        self
    }

    pub fn r#state_token(&self) -> Option<&str> {
        self.r#state_token.as_ref().map(|x| x.borrow())
    }

    pub fn reset_state_token(&mut self) {
        self.r#state_token = None;
    }
}
