use std::borrow::Borrow;

use futures::Future;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::{configuration, Error};

pub struct ApplicationApiClient {
    configuration: configuration::Configuration,
}

impl ApplicationApiClient {
    pub fn new(configuration: configuration::Configuration) -> ApplicationApiClient {
        ApplicationApiClient {
            configuration: configuration,
        }
    }
}

pub trait ApplicationApi {
    fn activate_application(
        &self,
        app_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn assign_user_to_application(
        &self,
        app_id: &str,
        app_user: crate::models::AppUser,
    ) -> Box<Future<Item = crate::models::AppUser, Error = Error<serde_json::Value>>>;
    fn clone_application_key(
        &self,
        app_id: &str,
        key_id: &str,
        target_aid: &str,
    ) -> Box<Future<Item = crate::models::JsonWebKey, Error = Error<serde_json::Value>>>;
    fn create_application(
        &self,
        application: crate::models::Application,
        activate: bool,
    ) -> Box<Future<Item = crate::models::Application, Error = Error<serde_json::Value>>>;
    fn create_application_group_assignment(
        &self,
        app_id: &str,
        group_id: &str,
        application_group_assignment: crate::models::ApplicationGroupAssignment,
    ) -> Box<
        Future<Item = crate::models::ApplicationGroupAssignment, Error = Error<serde_json::Value>>,
    >;
    fn deactivate_application(
        &self,
        app_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_application(
        &self,
        app_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_application_group_assignment(
        &self,
        app_id: &str,
        group_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_application_user(
        &self,
        app_id: &str,
        user_id: &str,
        send_email: bool,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_application(
        &self,
        app_id: &str,
        expand: &str,
    ) -> Box<Future<Item = crate::models::Application, Error = Error<serde_json::Value>>>;
    fn get_application_group_assignment(
        &self,
        app_id: &str,
        group_id: &str,
        expand: &str,
    ) -> Box<
        Future<Item = crate::models::ApplicationGroupAssignment, Error = Error<serde_json::Value>>,
    >;
    fn get_application_key(
        &self,
        app_id: &str,
        key_id: &str,
    ) -> Box<Future<Item = crate::models::JsonWebKey, Error = Error<serde_json::Value>>>;
    fn get_application_user(
        &self,
        app_id: &str,
        user_id: &str,
        expand: &str,
    ) -> Box<Future<Item = crate::models::AppUser, Error = Error<serde_json::Value>>>;
    fn list_application_group_assignments(
        &self,
        app_id: &str,
        q: &str,
        after: &str,
        limit: i32,
        expand: &str,
    ) -> Box<
        Future<
            Item = Vec<crate::models::ApplicationGroupAssignment>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn list_application_keys(
        &self,
        app_id: &str,
    ) -> Box<Future<Item = Vec<crate::models::JsonWebKey>, Error = Error<serde_json::Value>>>;
    fn list_application_users(
        &self,
        app_id: &str,
        q: &str,
        query_scope: &str,
        after: &str,
        limit: i32,
        filter: &str,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::AppUser>, Error = Error<serde_json::Value>>>;
    fn list_applications(
        &self,
        q: &str,
        after: &str,
        limit: i32,
        filter: &str,
        expand: &str,
        include_non_deleted: bool,
    ) -> Box<Future<Item = Vec<crate::models::Application>, Error = Error<serde_json::Value>>>;
    fn update_application(
        &self,
        app_id: &str,
        application: crate::models::Application,
    ) -> Box<Future<Item = crate::models::Application, Error = Error<serde_json::Value>>>;
    fn update_application_user(
        &self,
        app_id: &str,
        user_id: &str,
        app_user: crate::models::AppUser,
    ) -> Box<Future<Item = crate::models::AppUser, Error = Error<serde_json::Value>>>;
}

impl ApplicationApi for ApplicationApiClient {
    fn activate_application(
        &self,
        app_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps/{appId}/lifecycle/activate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("appId".to_string(), app_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn assign_user_to_application(
        &self,
        app_id: &str,
        app_user: crate::models::AppUser,
    ) -> Box<Future<Item = crate::models::AppUser, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps/{appId}/users".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("appId".to_string(), app_id.to_string())
        .with_body_param(app_user)
        .execute(self.configuration.borrow())
    }

    fn clone_application_key(
        &self,
        app_id: &str,
        key_id: &str,
        target_aid: &str,
    ) -> Box<Future<Item = crate::models::JsonWebKey, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps/{appId}/credentials/keys/{keyId}/clone".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("targetAid".to_string(), target_aid.to_string())
        .with_path_param("appId".to_string(), app_id.to_string())
        .with_path_param("keyId".to_string(), key_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn create_application(
        &self,
        application: crate::models::Application,
        activate: bool,
    ) -> Box<Future<Item = crate::models::Application, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::POST, "/api/v1/apps".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("activate".to_string(), activate.to_string())
            .with_body_param(application)
            .execute(self.configuration.borrow())
    }

    fn create_application_group_assignment(
        &self,
        app_id: &str,
        group_id: &str,
        application_group_assignment: crate::models::ApplicationGroupAssignment,
    ) -> Box<
        Future<Item = crate::models::ApplicationGroupAssignment, Error = Error<serde_json::Value>>,
    > {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/apps/{appId}/groups/{groupId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("appId".to_string(), app_id.to_string())
        .with_path_param("groupId".to_string(), group_id.to_string())
        .with_body_param(application_group_assignment)
        .execute(self.configuration.borrow())
    }

    fn deactivate_application(
        &self,
        app_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps/{appId}/lifecycle/deactivate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("appId".to_string(), app_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn delete_application(
        &self,
        app_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::DELETE, "/api/v1/apps/{appId}".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_path_param("appId".to_string(), app_id.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn delete_application_group_assignment(
        &self,
        app_id: &str,
        group_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/apps/{appId}/groups/{groupId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("appId".to_string(), app_id.to_string())
        .with_path_param("groupId".to_string(), group_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn delete_application_user(
        &self,
        app_id: &str,
        user_id: &str,
        send_email: bool,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/apps/{appId}/users/{userId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("sendEmail".to_string(), send_email.to_string())
        .with_path_param("appId".to_string(), app_id.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn get_application(
        &self,
        app_id: &str,
        expand: &str,
    ) -> Box<Future<Item = crate::models::Application, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::GET, "/api/v1/apps/{appId}".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("expand".to_string(), expand.to_string())
            .with_path_param("appId".to_string(), app_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_application_group_assignment(
        &self,
        app_id: &str,
        group_id: &str,
        expand: &str,
    ) -> Box<
        Future<Item = crate::models::ApplicationGroupAssignment, Error = Error<serde_json::Value>>,
    > {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/groups/{groupId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("expand".to_string(), expand.to_string())
        .with_path_param("appId".to_string(), app_id.to_string())
        .with_path_param("groupId".to_string(), group_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn get_application_key(
        &self,
        app_id: &str,
        key_id: &str,
    ) -> Box<Future<Item = crate::models::JsonWebKey, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/credentials/keys/{keyId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("appId".to_string(), app_id.to_string())
        .with_path_param("keyId".to_string(), key_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn get_application_user(
        &self,
        app_id: &str,
        user_id: &str,
        expand: &str,
    ) -> Box<Future<Item = crate::models::AppUser, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/users/{userId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("expand".to_string(), expand.to_string())
        .with_path_param("appId".to_string(), app_id.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_application_group_assignments(
        &self,
        app_id: &str,
        q: &str,
        after: &str,
        limit: i32,
        expand: &str,
    ) -> Box<
        Future<
            Item = Vec<crate::models::ApplicationGroupAssignment>,
            Error = Error<serde_json::Value>,
        >,
    > {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/groups".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("q".to_string(), q.to_string())
        .with_query_param("after".to_string(), after.to_string())
        .with_query_param("limit".to_string(), limit.to_string())
        .with_query_param("expand".to_string(), expand.to_string())
        .with_path_param("appId".to_string(), app_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_application_keys(
        &self,
        app_id: &str,
    ) -> Box<Future<Item = Vec<crate::models::JsonWebKey>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/credentials/keys".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("appId".to_string(), app_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_application_users(
        &self,
        app_id: &str,
        q: &str,
        query_scope: &str,
        after: &str,
        limit: i32,
        filter: &str,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::AppUser>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/users".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("q".to_string(), q.to_string())
        .with_query_param("query_scope".to_string(), query_scope.to_string())
        .with_query_param("after".to_string(), after.to_string())
        .with_query_param("limit".to_string(), limit.to_string())
        .with_query_param("filter".to_string(), filter.to_string())
        .with_query_param("expand".to_string(), expand.to_string())
        .with_path_param("appId".to_string(), app_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_applications(
        &self,
        q: &str,
        after: &str,
        limit: i32,
        filter: &str,
        expand: &str,
        include_non_deleted: bool,
    ) -> Box<Future<Item = Vec<crate::models::Application>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::GET, "/api/v1/apps".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("q".to_string(), q.to_string())
            .with_query_param("after".to_string(), after.to_string())
            .with_query_param("limit".to_string(), limit.to_string())
            .with_query_param("filter".to_string(), filter.to_string())
            .with_query_param("expand".to_string(), expand.to_string())
            .with_query_param(
                "includeNonDeleted".to_string(),
                include_non_deleted.to_string(),
            )
            .execute(self.configuration.borrow())
    }

    fn update_application(
        &self,
        app_id: &str,
        application: crate::models::Application,
    ) -> Box<Future<Item = crate::models::Application, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::PUT, "/api/v1/apps/{appId}".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_path_param("appId".to_string(), app_id.to_string())
            .with_body_param(application)
            .execute(self.configuration.borrow())
    }

    fn update_application_user(
        &self,
        app_id: &str,
        user_id: &str,
        app_user: crate::models::AppUser,
    ) -> Box<Future<Item = crate::models::AppUser, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps/{appId}/users/{userId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("appId".to_string(), app_id.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_body_param(app_user)
        .execute(self.configuration.borrow())
    }
}
