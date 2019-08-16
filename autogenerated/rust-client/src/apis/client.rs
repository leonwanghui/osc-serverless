use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    compute_api: Box<crate::apis::ComputeApi>,
    network_api: Box<crate::apis::NetworkApi>,
    storage_api: Box<crate::apis::StorageApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            compute_api: Box::new(crate::apis::ComputeApiClient::new(rc.clone())),
            network_api: Box::new(crate::apis::NetworkApiClient::new(rc.clone())),
            storage_api: Box::new(crate::apis::StorageApiClient::new(rc.clone())),
        }
    }

    pub fn compute_api(&self) -> &crate::apis::ComputeApi{
        self.compute_api.as_ref()
    }

    pub fn network_api(&self) -> &crate::apis::NetworkApi{
        self.network_api.as_ref()
    }

    pub fn storage_api(&self) -> &crate::apis::StorageApi{
        self.storage_api.as_ref()
    }

}
