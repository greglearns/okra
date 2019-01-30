/*
 * Okta API
 *
 * Allows customers to easily access the Okta API
 *
 * OpenAPI spec version: 1.9.0
 * Contact: devex-public@okta.com
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;

use futures::Future;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::{configuration, Error};

pub struct LoginApiClient {
    configuration: configuration::Configuration,
}

impl LoginApiClient {
    pub fn new(configuration: configuration::Configuration) -> LoginApiClient {
        LoginApiClient {
            configuration: configuration,
        }
    }
}

pub trait LoginApi {
    fn login_default(&self) -> Result<reqwest::Response, Error<serde_json::Value>>;
    fn session_cookie_redirect(
        &self,
        check_account_setup_complete: bool,
        token: &str,
        redirect_url: &str,
    ) -> Result<reqwest::Response, Error<serde_json::Value>>;
}

impl LoginApi for LoginApiClient {
    fn login_default(&self) -> Result<reqwest::Response, Error<serde_json::Value>> {
        _internal_request::Request::new(hyper::Method::GET, "/login/default".to_string())
            .response(self.configuration.borrow())
    }

    fn session_cookie_redirect(
        &self,
        check_account_setup_complete: bool,
        token: &str,
        redirect_url: &str,
    ) -> Result<reqwest::Response, Error<serde_json::Value>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/login/sessionCookieRedirect".to_string(),
        )
        .with_query_param(
            "checkAccountSetupComplete".to_string(),
            check_account_setup_complete.to_string(),
        )
        .with_query_param("token".to_string(), token.to_string())
        .with_query_param("redirectUrl".to_string(), redirect_url.to_string())
        .response(self.configuration.borrow())
    }
}