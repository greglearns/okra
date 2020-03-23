#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#UserProfile {
    #[serde(rename = "acceptMarketing", skip_serializing_if = "Option::is_none")]
    r#accept_marketing: Option<bool>,
    #[serde(rename = "acceptPrivacy", skip_serializing_if = "Option::is_none")]
    r#accept_privacy: Option<bool>,
    #[serde(rename = "accessBetaProgram", skip_serializing_if = "Option::is_none")]
    r#access_beta_program: Option<bool>,
    #[serde(rename = "accessCommunity", skip_serializing_if = "Option::is_none")]
    r#access_community: Option<bool>,
    #[serde(rename = "accessFNO", skip_serializing_if = "Option::is_none")]
    r#access_fno: Option<bool>,
    #[serde(rename = "accessMyAlteryx", skip_serializing_if = "Option::is_none")]
    r#access_my_alteryx: Option<bool>,
    #[serde(rename = "accessPartnerPortal", skip_serializing_if = "Option::is_none")]
    r#access_partner_portal: Option<bool>,
    #[serde(rename = "accessShowpad", skip_serializing_if = "Option::is_none")]
    r#access_showpad: Option<bool>,
    #[serde(rename = "accessZift", skip_serializing_if = "Option::is_none")]
    r#access_zift: Option<bool>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    r#company: Option<String>,
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    r#country_code: Option<String>,
    #[serde(rename = "department", skip_serializing_if = "Option::is_none")]
    r#department: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    r#email: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    r#first_name: Option<String>,
    #[serde(rename = "industry", skip_serializing_if = "Option::is_none")]
    r#industry: Option<String>,
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
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    r#zip_code: Option<String>,
}

impl r#UserProfile {
    pub fn new(
    ) -> Self {
        Self {
          r#accept_marketing: None,
          r#accept_privacy: None,
          r#access_beta_program: None,
          r#access_community: None,
          r#access_fno: None,
          r#access_my_alteryx: None,
          r#access_partner_portal: None,
          r#access_showpad: None,
          r#access_zift: None,
          r#company: None,
          r#country_code: None,
          r#department: None,
          r#email: None,
          r#first_name: None,
          r#industry: None,
          r#last_name: None,
          r#linkid: None,
          r#login: None,
          r#mobile_phone: None,
          r#preferred_language: None,
          r#role: None,
          r#second_email: None,
          r#zip_code: None,
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

    pub fn set_access_beta_program(&mut self, r#access_beta_program: bool) {
        self.r#access_beta_program = Some(r#access_beta_program);
    }

    pub fn with_access_beta_program(mut self, r#access_beta_program: bool) -> Self {
        self.r#access_beta_program = Some(r#access_beta_program);
        self
    }

    pub fn with_option_access_beta_program(mut self, r#access_beta_program: Option<bool>) -> Self {
        self.r#access_beta_program = r#access_beta_program;
        self
    }

    pub fn r#access_beta_program(&self) -> Option<&bool> {
        self.r#access_beta_program.as_ref().map(|x| x.borrow())
    }

    pub fn reset_access_beta_program(&mut self) {
        self.r#access_beta_program = None;
    }

    pub fn set_access_community(&mut self, r#access_community: bool) {
        self.r#access_community = Some(r#access_community);
    }

    pub fn with_access_community(mut self, r#access_community: bool) -> Self {
        self.r#access_community = Some(r#access_community);
        self
    }

    pub fn with_option_access_community(mut self, r#access_community: Option<bool>) -> Self {
        self.r#access_community = r#access_community;
        self
    }

    pub fn r#access_community(&self) -> Option<&bool> {
        self.r#access_community.as_ref().map(|x| x.borrow())
    }

    pub fn reset_access_community(&mut self) {
        self.r#access_community = None;
    }

    pub fn set_access_fno(&mut self, r#access_fno: bool) {
        self.r#access_fno = Some(r#access_fno);
    }

    pub fn with_access_fno(mut self, r#access_fno: bool) -> Self {
        self.r#access_fno = Some(r#access_fno);
        self
    }

    pub fn with_option_access_fno(mut self, r#access_fno: Option<bool>) -> Self {
        self.r#access_fno = r#access_fno;
        self
    }

    pub fn r#access_fno(&self) -> Option<&bool> {
        self.r#access_fno.as_ref().map(|x| x.borrow())
    }

    pub fn reset_access_fno(&mut self) {
        self.r#access_fno = None;
    }

    pub fn set_access_my_alteryx(&mut self, r#access_my_alteryx: bool) {
        self.r#access_my_alteryx = Some(r#access_my_alteryx);
    }

    pub fn with_access_my_alteryx(mut self, r#access_my_alteryx: bool) -> Self {
        self.r#access_my_alteryx = Some(r#access_my_alteryx);
        self
    }

    pub fn with_option_access_my_alteryx(mut self, r#access_my_alteryx: Option<bool>) -> Self {
        self.r#access_my_alteryx = r#access_my_alteryx;
        self
    }

    pub fn r#access_my_alteryx(&self) -> Option<&bool> {
        self.r#access_my_alteryx.as_ref().map(|x| x.borrow())
    }

    pub fn reset_access_my_alteryx(&mut self) {
        self.r#access_my_alteryx = None;
    }

    pub fn set_access_partner_portal(&mut self, r#access_partner_portal: bool) {
        self.r#access_partner_portal = Some(r#access_partner_portal);
    }

    pub fn with_access_partner_portal(mut self, r#access_partner_portal: bool) -> Self {
        self.r#access_partner_portal = Some(r#access_partner_portal);
        self
    }

    pub fn with_option_access_partner_portal(mut self, r#access_partner_portal: Option<bool>) -> Self {
        self.r#access_partner_portal = r#access_partner_portal;
        self
    }

    pub fn r#access_partner_portal(&self) -> Option<&bool> {
        self.r#access_partner_portal.as_ref().map(|x| x.borrow())
    }

    pub fn reset_access_partner_portal(&mut self) {
        self.r#access_partner_portal = None;
    }

    pub fn set_access_showpad(&mut self, r#access_showpad: bool) {
        self.r#access_showpad = Some(r#access_showpad);
    }

    pub fn with_access_showpad(mut self, r#access_showpad: bool) -> Self {
        self.r#access_showpad = Some(r#access_showpad);
        self
    }

    pub fn with_option_access_showpad(mut self, r#access_showpad: Option<bool>) -> Self {
        self.r#access_showpad = r#access_showpad;
        self
    }

    pub fn r#access_showpad(&self) -> Option<&bool> {
        self.r#access_showpad.as_ref().map(|x| x.borrow())
    }

    pub fn reset_access_showpad(&mut self) {
        self.r#access_showpad = None;
    }

    pub fn set_access_zift(&mut self, r#access_zift: bool) {
        self.r#access_zift = Some(r#access_zift);
    }

    pub fn with_access_zift(mut self, r#access_zift: bool) -> Self {
        self.r#access_zift = Some(r#access_zift);
        self
    }

    pub fn with_option_access_zift(mut self, r#access_zift: Option<bool>) -> Self {
        self.r#access_zift = r#access_zift;
        self
    }

    pub fn r#access_zift(&self) -> Option<&bool> {
        self.r#access_zift.as_ref().map(|x| x.borrow())
    }

    pub fn reset_access_zift(&mut self) {
        self.r#access_zift = None;
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

    pub fn set_country_code(&mut self, r#country_code: String) {
        self.r#country_code = Some(r#country_code);
    }

    pub fn with_country_code(mut self, r#country_code: String) -> Self {
        self.r#country_code = Some(r#country_code);
        self
    }

    pub fn with_option_country_code(mut self, r#country_code: Option<String>) -> Self {
        self.r#country_code = r#country_code;
        self
    }

    pub fn r#country_code(&self) -> Option<&str> {
        self.r#country_code.as_ref().map(|x| x.borrow())
    }

    pub fn reset_country_code(&mut self) {
        self.r#country_code = None;
    }

    pub fn set_department(&mut self, r#department: String) {
        self.r#department = Some(r#department);
    }

    pub fn with_department(mut self, r#department: String) -> Self {
        self.r#department = Some(r#department);
        self
    }

    pub fn with_option_department(mut self, r#department: Option<String>) -> Self {
        self.r#department = r#department;
        self
    }

    pub fn r#department(&self) -> Option<&str> {
        self.r#department.as_ref().map(|x| x.borrow())
    }

    pub fn reset_department(&mut self) {
        self.r#department = None;
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

    pub fn set_industry(&mut self, r#industry: String) {
        self.r#industry = Some(r#industry);
    }

    pub fn with_industry(mut self, r#industry: String) -> Self {
        self.r#industry = Some(r#industry);
        self
    }

    pub fn with_option_industry(mut self, r#industry: Option<String>) -> Self {
        self.r#industry = r#industry;
        self
    }

    pub fn r#industry(&self) -> Option<&str> {
        self.r#industry.as_ref().map(|x| x.borrow())
    }

    pub fn reset_industry(&mut self) {
        self.r#industry = None;
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

    pub fn set_zip_code(&mut self, r#zip_code: String) {
        self.r#zip_code = Some(r#zip_code);
    }

    pub fn with_zip_code(mut self, r#zip_code: String) -> Self {
        self.r#zip_code = Some(r#zip_code);
        self
    }

    pub fn with_option_zip_code(mut self, r#zip_code: Option<String>) -> Self {
        self.r#zip_code = r#zip_code;
        self
    }

    pub fn r#zip_code(&self) -> Option<&str> {
        self.r#zip_code.as_ref().map(|x| x.borrow())
    }

    pub fn reset_zip_code(&mut self) {
        self.r#zip_code = None;
    }
}
