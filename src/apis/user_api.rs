use std::borrow::Borrow;

use futures::Future;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::{configuration, Error};

pub struct UserApiClient {
    configuration: configuration::Configuration,
}

impl UserApiClient {
    pub fn new(configuration: configuration::Configuration) -> UserApiClient {
        UserApiClient {
            configuration: configuration,
        }
    }
}

pub trait UserApi {
    fn activate_user(
        &self,
        user_id: &str,
        send_email: bool,
    ) -> Box<Future<Item = crate::models::UserActivationToken, Error = Error<serde_json::Value>>>;
    fn add_group_target_to_role(
        &self,
        user_id: &str,
        role_id: &str,
        group_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn add_role_to_user(
        &self,
        user_id: &str,
        role: crate::models::Role,
    ) -> Box<Future<Item = crate::models::Role, Error = Error<serde_json::Value>>>;
    fn change_password(
        &self,
        user_id: &str,
        change_password_request: crate::models::ChangePasswordRequest,
    ) -> Box<Future<Item = crate::models::UserCredentials, Error = Error<serde_json::Value>>>;
    fn change_recovery_question(
        &self,
        user_id: &str,
        user_credentials: crate::models::UserCredentials,
    ) -> Box<Future<Item = crate::models::UserCredentials, Error = Error<serde_json::Value>>>;
    fn create_user(
        &self,
        body: crate::models::User,
        activate: bool,
        provider: bool,
        next_login: &str,
    ) -> Box<Future<Item = crate::models::User, Error = Error<serde_json::Value>>>;
    fn deactivate_or_delete_user(
        &self,
        user_id: &str,
        send_email: bool,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn deactivate_user(
        &self,
        user_id: &str,
        send_email: bool,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn end_all_user_sessions(
        &self,
        user_id: &str,
        oauth_tokens: bool,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn expire_password(
        &self,
        user_id: &str,
        temp_password: bool,
    ) -> Box<Future<Item = crate::models::TempPassword, Error = Error<serde_json::Value>>>;
    fn forgot_password(
        &self,
        user_id: &str,
        send_email: bool,
        user_credentials: crate::models::UserCredentials,
    ) -> Box<Future<Item = crate::models::ForgotPasswordResponse, Error = Error<serde_json::Value>>>;
    fn get_user(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = crate::models::User, Error = Error<serde_json::Value>>>;
    fn list_app_links(
        &self,
        user_id: &str,
        show_all: bool,
    ) -> Box<Future<Item = Vec<crate::models::AppLink>, Error = Error<serde_json::Value>>>;
    fn list_assigned_roles(
        &self,
        user_id: &str,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::Role>, Error = Error<serde_json::Value>>>;
    fn list_group_targets_for_role(
        &self,
        user_id: &str,
        role_id: &str,
        after: &str,
        limit: i32,
    ) -> Box<Future<Item = Vec<crate::models::Group>, Error = Error<serde_json::Value>>>;
    fn list_user_groups(
        &self,
        user_id: &str,
        after: &str,
        limit: i32,
    ) -> Box<Future<Item = Vec<crate::models::Group>, Error = Error<serde_json::Value>>>;
    fn list_users(
        &self,
        q: &str,
        after: &str,
        limit: i32,
        filter: &str,
        format: &str,
        search: &str,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::User>, Error = Error<serde_json::Value>>>;
    fn remove_group_target_from_role(
        &self,
        user_id: &str,
        role_id: &str,
        group_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn remove_role_from_user(
        &self,
        user_id: &str,
        role_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn reset_all_factors(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn reset_password(
        &self,
        user_id: &str,
        provider: &str,
        send_email: bool,
    ) -> Box<Future<Item = crate::models::ResetPasswordToken, Error = Error<serde_json::Value>>>;
    fn suspend_user(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn unlock_user(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn unsuspend_user(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_user(
        &self,
        user_id: &str,
        user: crate::models::User,
    ) -> Box<Future<Item = crate::models::User, Error = Error<serde_json::Value>>>;
}

impl UserApi for UserApiClient {
    fn activate_user(
        &self,
        user_id: &str,
        send_email: bool,
    ) -> Box<Future<Item = crate::models::UserActivationToken, Error = Error<serde_json::Value>>>
    {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/activate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("sendEmail".to_string(), send_email.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn add_group_target_to_role(
        &self,
        user_id: &str,
        role_id: &str,
        group_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/users/{userId}/roles/{roleId}/targets/groups/{groupId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_path_param("roleId".to_string(), role_id.to_string())
        .with_path_param("groupId".to_string(), group_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn add_role_to_user(
        &self,
        user_id: &str,
        role: crate::models::Role,
    ) -> Box<Future<Item = crate::models::Role, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/roles".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_body_param(role)
        .execute(self.configuration.borrow())
    }

    fn change_password(
        &self,
        user_id: &str,
        change_password_request: crate::models::ChangePasswordRequest,
    ) -> Box<Future<Item = crate::models::UserCredentials, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/credentials/change_password".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_body_param(change_password_request)
        .execute(self.configuration.borrow())
    }

    fn change_recovery_question(
        &self,
        user_id: &str,
        user_credentials: crate::models::UserCredentials,
    ) -> Box<Future<Item = crate::models::UserCredentials, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/credentials/change_recovery_question".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_body_param(user_credentials)
        .execute(self.configuration.borrow())
    }

    fn create_user(
        &self,
        body: crate::models::User,
        activate: bool,
        provider: bool,
        next_login: &str,
    ) -> Box<Future<Item = crate::models::User, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::POST, "/api/v1/users".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("activate".to_string(), activate.to_string())
            .with_query_param("provider".to_string(), provider.to_string())
            .with_query_param("nextLogin".to_string(), next_login.to_string())
            .with_body_param(body)
            .execute(self.configuration.borrow())
    }

    fn deactivate_or_delete_user(
        &self,
        user_id: &str,
        send_email: bool,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::DELETE, "/api/v1/users/{userId}".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("sendEmail".to_string(), send_email.to_string())
            .with_path_param("userId".to_string(), user_id.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn deactivate_user(
        &self,
        user_id: &str,
        send_email: bool,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/deactivate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("sendEmail".to_string(), send_email.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn end_all_user_sessions(
        &self,
        user_id: &str,
        oauth_tokens: bool,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/users/{userId}/sessions".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("oauthTokens".to_string(), oauth_tokens.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn expire_password(
        &self,
        user_id: &str,
        temp_password: bool,
    ) -> Box<Future<Item = crate::models::TempPassword, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/expire_password".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("tempPassword".to_string(), temp_password.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn forgot_password(
        &self,
        user_id: &str,
        send_email: bool,
        user_credentials: crate::models::UserCredentials,
    ) -> Box<Future<Item = crate::models::ForgotPasswordResponse, Error = Error<serde_json::Value>>>
    {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/credentials/forgot_password".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("sendEmail".to_string(), send_email.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_body_param(user_credentials)
        .execute(self.configuration.borrow())
    }

    fn get_user(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = crate::models::User, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::GET, "/api/v1/users/{userId}".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_path_param("userId".to_string(), user_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn list_app_links(
        &self,
        user_id: &str,
        show_all: bool,
    ) -> Box<Future<Item = Vec<crate::models::AppLink>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/appLinks".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("showAll".to_string(), show_all.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_assigned_roles(
        &self,
        user_id: &str,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::Role>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/roles".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("expand".to_string(), expand.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_group_targets_for_role(
        &self,
        user_id: &str,
        role_id: &str,
        after: &str,
        limit: i32,
    ) -> Box<Future<Item = Vec<crate::models::Group>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/roles/{roleId}/targets/groups".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("after".to_string(), after.to_string())
        .with_query_param("limit".to_string(), limit.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_path_param("roleId".to_string(), role_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_user_groups(
        &self,
        user_id: &str,
        after: &str,
        limit: i32,
    ) -> Box<Future<Item = Vec<crate::models::Group>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/groups".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("after".to_string(), after.to_string())
        .with_query_param("limit".to_string(), limit.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_users(
        &self,
        q: &str,
        after: &str,
        limit: i32,
        filter: &str,
        format: &str,
        search: &str,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::User>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::GET, "/api/v1/users".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("q".to_string(), q.to_string())
            .with_query_param("after".to_string(), after.to_string())
            .with_query_param("limit".to_string(), limit.to_string())
            .with_query_param("filter".to_string(), filter.to_string())
            .with_query_param("format".to_string(), format.to_string())
            .with_query_param("search".to_string(), search.to_string())
            .with_query_param("expand".to_string(), expand.to_string())
            .execute(self.configuration.borrow())
    }

    fn remove_group_target_from_role(
        &self,
        user_id: &str,
        role_id: &str,
        group_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/users/{userId}/roles/{roleId}/targets/groups/{groupId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_path_param("roleId".to_string(), role_id.to_string())
        .with_path_param("groupId".to_string(), group_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn remove_role_from_user(
        &self,
        user_id: &str,
        role_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/users/{userId}/roles/{roleId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_path_param("roleId".to_string(), role_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn reset_all_factors(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/reset_factors".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn reset_password(
        &self,
        user_id: &str,
        provider: &str,
        send_email: bool,
    ) -> Box<Future<Item = crate::models::ResetPasswordToken, Error = Error<serde_json::Value>>>
    {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/reset_password".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("provider".to_string(), provider.to_string())
        .with_query_param("sendEmail".to_string(), send_email.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn suspend_user(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/suspend".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn unlock_user(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/unlock".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn unsuspend_user(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/unsuspend".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn update_user(
        &self,
        user_id: &str,
        user: crate::models::User,
    ) -> Box<Future<Item = crate::models::User, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::PUT, "/api/v1/users/{userId}".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_path_param("userId".to_string(), user_id.to_string())
            .with_body_param(user)
            .execute(self.configuration.borrow())
    }
}
