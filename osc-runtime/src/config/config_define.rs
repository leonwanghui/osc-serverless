use rust_client::models;
#[allow(unused_imports)]
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "cloud_provider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<models::CloudProviderInfo>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            operation: None,
            cloud_provider: None,
            labels: None,
        }
    }

    pub fn get_operation(&self) -> String {
        self.operation.clone().unwrap()
    }

    pub fn set_operation(&mut self, op: String) {
        self.operation = Some(op);
    }

    pub fn get_cloud_provider(&self) -> models::CloudProviderInfo {
        self.cloud_provider.clone().unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Value>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            kind: None,
            metadata: Some(Metadata::new()),
            spec: None,
        }
    }

    pub fn get_kind(&self) -> String {
        self.kind.clone().unwrap()
    }

    pub fn get_metadata(&self) -> Metadata {
        self.metadata.clone().unwrap()
    }

    pub fn muted_with_metadata(&mut self, meta: Metadata) -> &mut Self {
        self.metadata = Some(meta);
        self
    }

    pub fn get_spec(&self) -> Value {
        self.spec.clone().unwrap()
    }

    pub fn validate(&mut self) {
        let k = self.get_kind();
        let op = self.get_metadata().get_operation();
        if k.is_empty() || op.is_empty() {
            panic!("input error, config not supported!")
        }
        if op != "create" && op != "delete" {
            panic!("input error, operation {:?} not supported!", op)
        }
        if k != "ComputeResource" && k != "StorageResource" && k != "NetworkResource" {
            panic!("unspported resource kind {}", k);
        }
    }
}
