use super::config as c;
use super::config::config_define::Config;
use rust_client::apis::client::APIClient;
use rust_client::apis::configuration::Configuration;
use rust_client::models;
use serde_json;
use std::borrow::Borrow;
use std::rc::Rc;
use std::string::String;

pub fn initialize_engine(path: &str, op: &str) -> Engine {
    Engine {
        client: APIClient::new(Configuration::new()),
        config: Rc::new(c::load(path, op)),
    }
}

pub struct Engine {
    pub client: APIClient,
    pub config: Rc<Config>,
}

impl Engine {
    pub fn run(&self) -> Result<(), String> {
        println!("The engine has been running!");
        let config: &Config = self.config.borrow();
        let provider = config.get_metadata().get_cloud_provider();
        let op = config.get_metadata().get_operation();
        let k = config.get_kind();
        let json_spec = serde_json::to_string(&config.get_spec()).unwrap();
        match k.as_str() {
            "ComputeResource" => {
                let mut spec: models::ComputeResourceCreateRequest =
                    serde_json::from_str(&json_spec).unwrap();
                // Update cloud provider info of compute resource create request
                // from metadata
                spec.cloud_provider = provider;
                match op.as_str() {
                    "create" => {
                        let cr = self.create_compute_resource(spec);
                        println!("{:#?}", cr);
                    }
                    "delete" => {
                        self.delete_compute_resource(&spec);
                        println!("DELETE operation has been performed successfully!");
                    }
                    _ => eprintln!("operation {} is not supported!", op),
                }
                return Ok(());
            }
            "StorageResource" => {
                let mut spec: models::StorageResourceCreateRequest =
                    serde_json::from_str(&json_spec).unwrap();
                // Update cloud provider info of storage resource create request
                // from metadata
                spec.cloud_provider = provider;
                match op.as_str() {
                    "create" => {
                        let sr = self.create_storage_resource(spec);
                        println!("{:#?}", sr);
                    }
                    "delete" => {
                        self.delete_storage_resource(&spec);
                        println!("DELETE operation has been performed successfully!");
                    }
                    _ => eprintln!("operation {} is not supported!", op),
                }
                return Ok(());
            }
            "NetworkResource" => {
                let mut spec: models::NetworkResourceCreateRequest =
                    serde_json::from_str(&json_spec).unwrap();
                // Update cloud provider info of network resource create request
                // from metadata
                spec.cloud_provider = provider;
                match op.as_str() {
                    "create" => {
                        let nr = self.create_network_resource(spec);
                        println!("{:#?}", nr);
                    }
                    "delete" => {
                        self.delete_network_resource(&spec);
                        println!("DELETE operation has been performed successfully!");
                    }
                    _ => eprintln!("operation {} is not supported!", op),
                }
                return Ok(());
            }
            _ => Err(format!("resource kind {} is not supported!", k)),
        }
    }

    fn create_compute_resource(
        &self,
        copts: models::ComputeResourceCreateRequest,
    ) -> models::ComputeResourceSpec {
        let cr = match self.client.compute_api().compute_resource_create(copts) {
            Ok(v) => v,
            Err(e) => panic!(e),
        };
        cr
    }

    fn delete_compute_resource(&self, copts: &models::ComputeResourceCreateRequest) {
        let name = &copts.name;
        let cp = String::from("huaweicloud");
        match self
            .client
            .compute_api()
            .compute_resource_delete(name, &cp, false, false)
        {
            Ok(v) => println!("Get result={}", v),
            Err(e) => panic!(e),
        }
    }

    fn create_storage_resource(
        &self,
        sopts: models::StorageResourceCreateRequest,
    ) -> models::StorageResourceSpec {
        let sr = match self.client.storage_api().storage_resource_create(sopts) {
            Ok(v) => v,
            Err(e) => panic!(e),
        };
        sr
    }

    fn delete_storage_resource(&self, sopts: &models::StorageResourceCreateRequest) {
        let name = &sopts.name;
        let cp = String::from("huaweicloud");
        match self.client.storage_api().storage_resource_delete(name, &cp) {
            Ok(v) => println!("Get result={}", v),
            Err(e) => panic!(e),
        }
    }

    fn create_network_resource(
        &self,
        nopts: models::NetworkResourceCreateRequest,
    ) -> models::NetworkResourceSpec {
        let nr = match self.client.network_api().network_resource_create(nopts) {
            Ok(v) => v,
            Err(e) => panic!(e),
        };
        nr
    }

    fn delete_network_resource(&self, nopts: &models::NetworkResourceCreateRequest) {
        let name = &nopts.name;
        let cp = String::from("huaweicloud");
        match self.client.network_api().network_resource_delete(name, &cp) {
            Ok(v) => println!("Get result={}", v),
            Err(e) => panic!(e),
        }
    }
}
