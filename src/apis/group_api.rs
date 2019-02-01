use std::borrow::Borrow;
use futures::Future;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::{configuration, Error};

pub struct GroupApiClient {
    configuration: configuration::Configuration,
}

impl GroupApiClient {
    pub fn new(configuration: configuration::Configuration) -> GroupApiClient {
        GroupApiClient {
            configuration: configuration,
        }
    }
}

pub trait GroupApi {
    fn activate_rule(
        &self,
        rule_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn add_user_to_group(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_group(
        &self,
        group: crate::models::Group,
    ) -> Box<Future<Item = crate::models::Group, Error = Error<serde_json::Value>>>;
    fn create_rule(
        &self,
        group_rule: crate::models::GroupRule,
    ) -> Box<Future<Item = crate::models::GroupRule, Error = Error<serde_json::Value>>>;
    fn deactivate_rule(
        &self,
        rule_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_group(
        &self,
        group_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_rule(
        &self,
        rule_id: &str,
        remove_users: bool,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_group(
        &self,
        group_id: &str,
        expand: &str,
    ) -> Box<Future<Item = crate::models::Group, Error = Error<serde_json::Value>>>;
    fn get_rule(
        &self,
        rule_id: &str,
        expand: &str,
    ) -> Box<Future<Item = crate::models::GroupRule, Error = Error<serde_json::Value>>>;
    fn list_group_users(
        &self,
        group_id: &str,
        after: &str,
        limit: i32,
        managed_by: &str,
    ) -> Box<Future<Item = Vec<crate::models::User>, Error = Error<serde_json::Value>>>;
    fn list_groups(
        &self,
        q: &str,
        filter: &str,
        after: &str,
        limit: i32,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::Group>, Error = Error<serde_json::Value>>>;
    fn list_rules(
        &self,
        limit: i32,
        after: &str,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::GroupRule>, Error = Error<serde_json::Value>>>;
    fn remove_group_user(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_group(
        &self,
        group_id: &str,
        group: crate::models::Group,
    ) -> Box<Future<Item = crate::models::Group, Error = Error<serde_json::Value>>>;
    fn update_rule(
        &self,
        rule_id: &str,
        group_rule: crate::models::GroupRule,
    ) -> Box<Future<Item = crate::models::GroupRule, Error = Error<serde_json::Value>>>;
}

impl GroupApi for GroupApiClient {
    fn activate_rule(
        &self,
        rule_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/groups/rules/{ruleId}/lifecycle/activate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("ruleId".to_string(), rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn add_user_to_group(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/groups/{groupId}/users/{userId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("groupId".to_string(), group_id.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn create_group(
        &self,
        group: crate::models::Group,
    ) -> Box<Future<Item = crate::models::Group, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::POST, "/api/v1/groups".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_body_param(group)
            .execute(self.configuration.borrow())
    }

    fn create_rule(
        &self,
        group_rule: crate::models::GroupRule,
    ) -> Box<Future<Item = crate::models::GroupRule, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::POST, "/api/v1/groups/rules".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_body_param(group_rule)
            .execute(self.configuration.borrow())
    }

    fn deactivate_rule(
        &self,
        rule_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/groups/rules/{ruleId}/lifecycle/deactivate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("ruleId".to_string(), rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn delete_group(
        &self,
        group_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/groups/{groupId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("groupId".to_string(), group_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn delete_rule(
        &self,
        rule_id: &str,
        remove_users: bool,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/groups/rules/{ruleId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("removeUsers".to_string(), remove_users.to_string())
        .with_path_param("ruleId".to_string(), rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn get_group(
        &self,
        group_id: &str,
        expand: &str,
    ) -> Box<Future<Item = crate::models::Group, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::GET, "/api/v1/groups/{groupId}".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("expand".to_string(), expand.to_string())
            .with_path_param("groupId".to_string(), group_id.to_string())
            .execute(self.configuration.borrow())
    }

    fn get_rule(
        &self,
        rule_id: &str,
        expand: &str,
    ) -> Box<Future<Item = crate::models::GroupRule, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/groups/rules/{ruleId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("expand".to_string(), expand.to_string())
        .with_path_param("ruleId".to_string(), rule_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_group_users(
        &self,
        group_id: &str,
        after: &str,
        limit: i32,
        managed_by: &str,
    ) -> Box<Future<Item = Vec<crate::models::User>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/groups/{groupId}/users".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("after".to_string(), after.to_string())
        .with_query_param("limit".to_string(), limit.to_string())
        .with_query_param("managedBy".to_string(), managed_by.to_string())
        .with_path_param("groupId".to_string(), group_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_groups(
        &self,
        q: &str,
        filter: &str,
        after: &str,
        limit: i32,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::Group>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::GET, "/api/v1/groups".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("q".to_string(), q.to_string())
            .with_query_param("filter".to_string(), filter.to_string())
            .with_query_param("after".to_string(), after.to_string())
            .with_query_param("limit".to_string(), limit.to_string())
            .with_query_param("expand".to_string(), expand.to_string())
            .execute(self.configuration.borrow())
    }

    fn list_rules(
        &self,
        limit: i32,
        after: &str,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::GroupRule>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::GET, "/api/v1/groups/rules".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("limit".to_string(), limit.to_string())
            .with_query_param("after".to_string(), after.to_string())
            .with_query_param("expand".to_string(), expand.to_string())
            .execute(self.configuration.borrow())
    }

    fn remove_group_user(
        &self,
        group_id: &str,
        user_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/groups/{groupId}/users/{userId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("groupId".to_string(), group_id.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn update_group(
        &self,
        group_id: &str,
        group: crate::models::Group,
    ) -> Box<Future<Item = crate::models::Group, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::PUT, "/api/v1/groups/{groupId}".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_path_param("groupId".to_string(), group_id.to_string())
            .with_body_param(group)
            .execute(self.configuration.borrow())
    }

    fn update_rule(
        &self,
        rule_id: &str,
        group_rule: crate::models::GroupRule,
    ) -> Box<Future<Item = crate::models::GroupRule, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/groups/rules/{ruleId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("ruleId".to_string(), rule_id.to_string())
        .with_body_param(group_rule)
        .execute(self.configuration.borrow())
    }
}
