#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#SecurityQuestionFactor {
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    r#profile: Option<SecurityQuestionFactorProfile>,
}

impl r#SecurityQuestionFactor {
    pub fn new(
    ) -> Self {
        Self {
          r#profile: None,
        }
    }

    pub fn set_profile(&mut self, r#profile: SecurityQuestionFactorProfile) {
        self.r#profile = Some(r#profile);
    }

    pub fn with_profile(mut self, r#profile: SecurityQuestionFactorProfile) -> Self {
        self.r#profile = Some(r#profile);
        self
    }

    pub fn with_option_profile(mut self, r#profile: Option<SecurityQuestionFactorProfile>) -> Self {
        self.r#profile = r#profile;
        self
    }

    pub fn r#profile(&self) -> Option<&SecurityQuestionFactorProfile> {
        self.r#profile.as_ref().map(|x| x.borrow())
    }

    pub fn reset_profile(&mut self) {
        self.r#profile = None;
    }
}
