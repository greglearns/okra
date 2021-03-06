#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#EmailFactor {
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    r#profile: Option<EmailFactorProfile>,
}

impl r#EmailFactor {
    pub fn new(
    ) -> Self {
        Self {
          r#profile: None,
        }
    }

    pub fn set_profile(&mut self, r#profile: EmailFactorProfile) {
        self.r#profile = Some(r#profile);
    }

    pub fn with_profile(mut self, r#profile: EmailFactorProfile) -> Self {
        self.r#profile = Some(r#profile);
        self
    }

    pub fn with_option_profile(mut self, r#profile: Option<EmailFactorProfile>) -> Self {
        self.r#profile = r#profile;
        self
    }

    pub fn r#profile(&self) -> Option<&EmailFactorProfile> {
        self.r#profile.as_ref().map(|x| x.borrow())
    }

    pub fn reset_profile(&mut self) {
        self.r#profile = None;
    }
}
