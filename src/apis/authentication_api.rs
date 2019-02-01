use std::borrow::Borrow;

use futures::Future;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::{configuration, Error};

pub struct AuthenticationApiClient {
    configuration: configuration::Configuration,
}

impl AuthenticationApiClient {
    pub fn new(configuration: configuration::Configuration) -> AuthenticationApiClient {
        AuthenticationApiClient {
            configuration: configuration,
        }
    }
}

pub trait AuthenticationApi {
    fn auth_activate_factor(
        &self,
        factor_id: &str,
        body: crate::models::ActivateFactorRequest,
    ) -> Box<
        Future<Item = crate::models::AuthenticationTransaction, Error = Error<serde_json::Value>>,
    >;
    fn auth_change_password(
        &self,
        body: crate::models::ChangePasswordRequest,
    ) -> Box<
        Future<Item = crate::models::AuthenticationTransaction, Error = Error<serde_json::Value>>,
    >;
    fn auth_verify_factor(
        &self,
        factor_id: &str,
        body: crate::models::AuthVerifyFactorRequest,
        remember_device: bool,
        auto_push: bool,
    ) -> Box<
        Future<Item = crate::models::AuthenticationTransaction, Error = Error<serde_json::Value>>,
    >;
    fn authenticate(
        &self,
        body: crate::models::AuthenticationRequest,
    ) -> Box<
        Future<Item = crate::models::AuthenticationTransaction, Error = Error<serde_json::Value>>,
    >;
    fn enroll_factor(
        &self,
        body: crate::models::EnrollFactorRequest,
    ) -> Box<
        Future<Item = crate::models::AuthenticationTransaction, Error = Error<serde_json::Value>>,
    >;
}

impl AuthenticationApi for AuthenticationApiClient {
    fn auth_activate_factor(
        &self,
        factor_id: &str,
        body: crate::models::ActivateFactorRequest,
    ) -> Box<
        Future<Item = crate::models::AuthenticationTransaction, Error = Error<serde_json::Value>>,
    > {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/authn/factors/{factorId}/lifecycle/activate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("factorId".to_string(), factor_id.to_string())
        .with_body_param(body)
        .execute(self.configuration.borrow())
    }

    fn auth_change_password(
        &self,
        body: crate::models::ChangePasswordRequest,
    ) -> Box<
        Future<Item = crate::models::AuthenticationTransaction, Error = Error<serde_json::Value>>,
    > {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/authn/credentials/change_password".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_body_param(body)
        .execute(self.configuration.borrow())
    }

    fn auth_verify_factor(
        &self,
        factor_id: &str,
        body: crate::models::AuthVerifyFactorRequest,
        remember_device: bool,
        auto_push: bool,
    ) -> Box<
        Future<Item = crate::models::AuthenticationTransaction, Error = Error<serde_json::Value>>,
    > {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/authn/factors/{factorId}/verify".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("rememberDevice".to_string(), remember_device.to_string())
        .with_query_param("autoPush".to_string(), auto_push.to_string())
        .with_path_param("factorId".to_string(), factor_id.to_string())
        .with_body_param(body)
        .execute(self.configuration.borrow())
    }

    fn authenticate(
        &self,
        body: crate::models::AuthenticationRequest,
    ) -> Box<
        Future<Item = crate::models::AuthenticationTransaction, Error = Error<serde_json::Value>>,
    > {
        _internal_request::Request::new(hyper::Method::POST, "/api/v1/authn".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_body_param(body)
            .execute(self.configuration.borrow())
    }

    fn enroll_factor(
        &self,
        body: crate::models::EnrollFactorRequest,
    ) -> Box<
        Future<Item = crate::models::AuthenticationTransaction, Error = Error<serde_json::Value>>,
    > {
        _internal_request::Request::new(hyper::Method::POST, "/api/v1/authn/factors".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_body_param(body)
            .execute(self.configuration.borrow())
    }
}
