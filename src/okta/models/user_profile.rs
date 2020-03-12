#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#UserProfile {
    #[serde(rename = "accept_marketing", skip_serializing_if = "Option::is_none")]
    r#accept_marketing: Option<bool>,
    #[serde(rename = "accept_privacy", skip_serializing_if = "Option::is_none")]
    r#accept_privacy: Option<bool>,
    #[serde(rename = "accessBeta", skip_serializing_if = "Option::is_none")]
    r#access_beta: Option<bool>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    r#company: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    r#email: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    r#first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    r#last_name: Option<String>,
    #[serde(rename = "linkid", skip_serializing_if = "Option::is_none")]
    r#linkid: Option<String>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    r#login: Option<String>,
    #[serde(rename = "mobilePhone", skip_serializing_if = "Option::is_none")]
    r#mobile_phone: Option<String>,
    #[serde(rename = "preferredLanguage", skip_serializing_if = "Option::is_none")]
    r#preferred_language: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    r#role: Option<String>,
    #[serde(rename = "secondEmail", skip_serializing_if = "Option::is_none")]
    r#second_email: Option<String>,
}

impl r#UserProfile {
    pub fn new(
    ) -> Self {
        Self {
          r#accept_marketing: None,
          r#accept_privacy: None,
          r#access_beta: None,
          r#company: None,
          r#email: None,
          r#first_name: None,
          r#last_name: None,
          r#linkid: None,
          r#login: None,
          r#mobile_phone: None,
          r#preferred_language: None,
          r#role: None,
          r#second_email: None,
        }
    }

    pub fn set_accept_marketing(&mut self, r#accept_marketing: bool) {
        self.r#accept_marketing = Some(r#accept_marketing);
    }

    pub fn with_accept_marketing(mut self, r#accept_marketing: bool) -> Self {
        self.r#accept_marketing = Some(r#accept_marketing);
        self
    }

    pub fn with_option_accept_marketing(mut self, r#accept_marketing: Option<bool>) -> Self {
        self.r#accept_marketing = r#accept_marketing;
        self
    }

    pub fn r#accept_marketing(&self) -> Option<&bool> {
        self.r#accept_marketing.as_ref().map(|x| x.borrow())
    }

    pub fn reset_accept_marketing(&mut self) {
        self.r#accept_marketing = None;
    }

    pub fn set_accept_privacy(&mut self, r#accept_privacy: bool) {
        self.r#accept_privacy = Some(r#accept_privacy);
    }

    pub fn with_accept_privacy(mut self, r#accept_privacy: bool) -> Self {
        self.r#accept_privacy = Some(r#accept_privacy);
        self
    }

    pub fn with_option_accept_privacy(mut self, r#accept_privacy: Option<bool>) -> Self {
        self.r#accept_privacy = r#accept_privacy;
        self
    }

    pub fn r#accept_privacy(&self) -> Option<&bool> {
        self.r#accept_privacy.as_ref().map(|x| x.borrow())
    }

    pub fn reset_accept_privacy(&mut self) {
        self.r#accept_privacy = None;
    }

    pub fn set_access_beta(&mut self, r#access_beta: bool) {
        self.r#access_beta = Some(r#access_beta);
    }

    pub fn with_access_beta(mut self, r#access_beta: bool) -> Self {
        self.r#access_beta = Some(r#access_beta);
        self
    }

    pub fn with_option_access_beta(mut self, r#access_beta: Option<bool>) -> Self {
        self.r#access_beta = r#access_beta;
        self
    }

    pub fn r#access_beta(&self) -> Option<&bool> {
        self.r#access_beta.as_ref().map(|x| x.borrow())
    }

    pub fn reset_access_beta(&mut self) {
        self.r#access_beta = None;
    }

    pub fn set_company(&mut self, r#company: String) {
        self.r#company = Some(r#company);
    }

    pub fn with_company(mut self, r#company: String) -> Self {
        self.r#company = Some(r#company);
        self
    }

    pub fn with_option_company(mut self, r#company: Option<String>) -> Self {
        self.r#company = r#company;
        self
    }

    pub fn r#company(&self) -> Option<&str> {
        self.r#company.as_ref().map(|x| x.borrow())
    }

    pub fn reset_company(&mut self) {
        self.r#company = None;
    }

    pub fn set_email(&mut self, r#email: String) {
        self.r#email = Some(r#email);
    }

    pub fn with_email(mut self, r#email: String) -> Self {
        self.r#email = Some(r#email);
        self
    }

    pub fn with_option_email(mut self, r#email: Option<String>) -> Self {
        self.r#email = r#email;
        self
    }

    pub fn r#email(&self) -> Option<&str> {
        self.r#email.as_ref().map(|x| x.borrow())
    }

    pub fn reset_email(&mut self) {
        self.r#email = None;
    }

    pub fn set_first_name(&mut self, r#first_name: String) {
        self.r#first_name = Some(r#first_name);
    }

    pub fn with_first_name(mut self, r#first_name: String) -> Self {
        self.r#first_name = Some(r#first_name);
        self
    }

    pub fn with_option_first_name(mut self, r#first_name: Option<String>) -> Self {
        self.r#first_name = r#first_name;
        self
    }

    pub fn r#first_name(&self) -> Option<&str> {
        self.r#first_name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_first_name(&mut self) {
        self.r#first_name = None;
    }

    pub fn set_last_name(&mut self, r#last_name: String) {
        self.r#last_name = Some(r#last_name);
    }

    pub fn with_last_name(mut self, r#last_name: String) -> Self {
        self.r#last_name = Some(r#last_name);
        self
    }

    pub fn with_option_last_name(mut self, r#last_name: Option<String>) -> Self {
        self.r#last_name = r#last_name;
        self
    }

    pub fn r#last_name(&self) -> Option<&str> {
        self.r#last_name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_last_name(&mut self) {
        self.r#last_name = None;
    }

    pub fn set_linkid(&mut self, r#linkid: String) {
        self.r#linkid = Some(r#linkid);
    }

    pub fn with_linkid(mut self, r#linkid: String) -> Self {
        self.r#linkid = Some(r#linkid);
        self
    }

    pub fn with_option_linkid(mut self, r#linkid: Option<String>) -> Self {
        self.r#linkid = r#linkid;
        self
    }

    pub fn r#linkid(&self) -> Option<&str> {
        self.r#linkid.as_ref().map(|x| x.borrow())
    }

    pub fn reset_linkid(&mut self) {
        self.r#linkid = None;
    }

    pub fn set_login(&mut self, r#login: String) {
        self.r#login = Some(r#login);
    }

    pub fn with_login(mut self, r#login: String) -> Self {
        self.r#login = Some(r#login);
        self
    }

    pub fn with_option_login(mut self, r#login: Option<String>) -> Self {
        self.r#login = r#login;
        self
    }

    pub fn r#login(&self) -> Option<&str> {
        self.r#login.as_ref().map(|x| x.borrow())
    }

    pub fn reset_login(&mut self) {
        self.r#login = None;
    }

    pub fn set_mobile_phone(&mut self, r#mobile_phone: String) {
        self.r#mobile_phone = Some(r#mobile_phone);
    }

    pub fn with_mobile_phone(mut self, r#mobile_phone: String) -> Self {
        self.r#mobile_phone = Some(r#mobile_phone);
        self
    }

    pub fn with_option_mobile_phone(mut self, r#mobile_phone: Option<String>) -> Self {
        self.r#mobile_phone = r#mobile_phone;
        self
    }

    pub fn r#mobile_phone(&self) -> Option<&str> {
        self.r#mobile_phone.as_ref().map(|x| x.borrow())
    }

    pub fn reset_mobile_phone(&mut self) {
        self.r#mobile_phone = None;
    }

    pub fn set_preferred_language(&mut self, r#preferred_language: String) {
        self.r#preferred_language = Some(r#preferred_language);
    }

    pub fn with_preferred_language(mut self, r#preferred_language: String) -> Self {
        self.r#preferred_language = Some(r#preferred_language);
        self
    }

    pub fn with_option_preferred_language(mut self, r#preferred_language: Option<String>) -> Self {
        self.r#preferred_language = r#preferred_language;
        self
    }

    pub fn r#preferred_language(&self) -> Option<&str> {
        self.r#preferred_language.as_ref().map(|x| x.borrow())
    }

    pub fn reset_preferred_language(&mut self) {
        self.r#preferred_language = None;
    }

    pub fn set_role(&mut self, r#role: String) {
        self.r#role = Some(r#role);
    }

    pub fn with_role(mut self, r#role: String) -> Self {
        self.r#role = Some(r#role);
        self
    }

    pub fn with_option_role(mut self, r#role: Option<String>) -> Self {
        self.r#role = r#role;
        self
    }

    pub fn r#role(&self) -> Option<&str> {
        self.r#role.as_ref().map(|x| x.borrow())
    }

    pub fn reset_role(&mut self) {
        self.r#role = None;
    }

    pub fn set_second_email(&mut self, r#second_email: String) {
        self.r#second_email = Some(r#second_email);
    }

    pub fn with_second_email(mut self, r#second_email: String) -> Self {
        self.r#second_email = Some(r#second_email);
        self
    }

    pub fn with_option_second_email(mut self, r#second_email: Option<String>) -> Self {
        self.r#second_email = r#second_email;
        self
    }

    pub fn r#second_email(&self) -> Option<&str> {
        self.r#second_email.as_ref().map(|x| x.borrow())
    }

    pub fn reset_second_email(&mut self) {
        self.r#second_email = None;
    }
}
