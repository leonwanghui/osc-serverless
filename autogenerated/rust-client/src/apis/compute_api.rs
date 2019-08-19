/*
 * Open Service Cloud API
 *
 * Open Service Cloud API to manage different backend cloud services.
 *
 * The version of the OpenAPI document: 0.0.3
 * Contact: wanghui71leon@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{Error, configuration};

pub struct ComputeApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ComputeApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ComputeApiClient {
        ComputeApiClient {
            configuration: configuration,
        }
    }
}

pub trait ComputeApi {
    fn compute_resource_create(&self, compute_resource_create_request: crate::models::ComputeResourceCreateRequest) -> Result<crate::models::ComputeResourceSpec, Error>;
    fn compute_resource_delete(&self, cr_id: &str, cloud_provider: &str, delete_publicip: bool, delete_volume: bool) -> Result<serde_json::Value, Error>;
    fn compute_resource_get(&self, cr_id: &str, cloud_provider: &str) -> Result<crate::models::ComputeResourceSpec, Error>;
}

impl ComputeApi for ComputeApiClient {
    fn compute_resource_create(&self, compute_resource_create_request: crate::models::ComputeResourceCreateRequest) -> Result<crate::models::ComputeResourceSpec, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/v1alpha/compute_resources", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&compute_resource_create_request);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn compute_resource_delete(&self, cr_id: &str, cloud_provider: &str, delete_publicip: bool, delete_volume: bool) -> Result<serde_json::Value, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/v1alpha/compute_resources/{cr_id}", configuration.base_path, cr_id=crate::apis::urlencode(cr_id));
        let mut req_builder = client.delete(uri_str.as_str());

        req_builder = req_builder.query(&[("cloud_provider", &cloud_provider.to_string())]);
        req_builder = req_builder.query(&[("delete_publicip", &delete_publicip.to_string())]);
        req_builder = req_builder.query(&[("delete_volume", &delete_volume.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn compute_resource_get(&self, cr_id: &str, cloud_provider: &str) -> Result<crate::models::ComputeResourceSpec, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/v1alpha/compute_resources/{cr_id}", configuration.base_path, cr_id=crate::apis::urlencode(cr_id));
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("cloud_provider", &cloud_provider.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}