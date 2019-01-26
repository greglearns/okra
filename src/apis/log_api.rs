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
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::{configuration, Error};

pub struct LogApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl LogApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> LogApiClient {
        LogApiClient {
            configuration: configuration,
        }
    }
}

pub trait LogApi {
    fn get_logs(
        &self,
        until: &str,
        since: &str,
        filter: &str,
        q: &str,
        limit: i32,
        sort_order: &str,
        after: &str,
    ) -> Box<Future<Item = Vec<crate::models::LogEvent>, Error = Error<serde_json::Value>>>;
}

impl LogApi for LogApiClient {
    fn get_logs(
        &self,
        until: &str,
        since: &str,
        filter: &str,
        q: &str,
        limit: i32,
        sort_order: &str,
        after: &str,
    ) -> Box<Future<Item = Vec<crate::models::LogEvent>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::GET, "/api/v1/logs".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("until".to_string(), until.to_string())
            .with_query_param("since".to_string(), since.to_string())
            .with_query_param("filter".to_string(), filter.to_string())
            .with_query_param("q".to_string(), q.to_string())
            .with_query_param("limit".to_string(), limit.to_string())
            .with_query_param("sortOrder".to_string(), sort_order.to_string())
            .with_query_param("after".to_string(), after.to_string())
            .execute(self.configuration.borrow())
    }
}
