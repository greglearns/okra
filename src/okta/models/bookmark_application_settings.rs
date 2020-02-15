#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#BookmarkApplicationSettings {
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    r#app: Option<BookmarkApplicationSettingsApplication>,
}

impl r#BookmarkApplicationSettings {
    pub fn new(
    ) -> Self {
        Self {
          r#app: None,
        }
    }

    pub fn set_app(&mut self, r#app: BookmarkApplicationSettingsApplication) {
        self.r#app = Some(r#app);
    }

    pub fn with_app(mut self, r#app: BookmarkApplicationSettingsApplication) -> Self {
        self.r#app = Some(r#app);
        self
    }

    pub fn with_option_app(mut self, r#app: Option<BookmarkApplicationSettingsApplication>) -> Self {
        self.r#app = r#app;
        self
    }

    pub fn r#app(&self) -> Option<&BookmarkApplicationSettingsApplication> {
        self.r#app.as_ref().map(|x| x.borrow())
    }

    pub fn reset_app(&mut self) {
        self.r#app = None;
    }
}
