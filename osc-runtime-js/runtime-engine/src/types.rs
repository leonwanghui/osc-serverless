use super::models;
use serde_json::value::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct ResourceSpec {
    pub kind: String,
    pub metadata: Metadata,
    pub spec: Value,
}

impl ResourceSpec {
    pub fn get_kind(&self) -> String {
        self.kind.clone()
    }

    pub fn get_metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    pub fn get_spec(&self) -> Value {
        self.spec.clone()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub operation: String,
    pub cloud_provider: models::CloudProviderInfo,
    pub labels: Value,
}

impl Metadata {
    pub fn get_operation(&self) -> String {
        self.operation.clone()
    }

    pub fn get_cloud_provider(&self) -> models::CloudProviderInfo {
        self.cloud_provider.clone()
    }
}

#[derive(Debug, Serialize)]
pub struct JsSpec {
    pub kind: String,
    pub operation: String,
    pub params: Value,
}

impl JsSpec {
    pub fn new(kind: &str, op: &str, param: Value) -> JsSpec {
        JsSpec {
            kind: kind.to_string(),
            operation: op.to_string(),
            params: param,
        }
    }

    pub fn set_params(&mut self, param: Value) {
        self.params = param;
    }
}

#[derive(Debug, Serialize)]
pub struct DeleteOpts {
    pub name: String,
    pub cloud_provider: String,
    pub optional: OptionalParam,
}

impl DeleteOpts {
    pub fn new(
        name: &str,
        cloud_provider: &str,
        delete_publicip: bool,
        delete_volume: bool,
    ) -> DeleteOpts {
        DeleteOpts {
            name: name.to_string(),
            cloud_provider: cloud_provider.to_string(),
            optional: OptionalParam::new(delete_publicip, delete_volume),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OptionalParam {
    pub delete_publicip: bool,
    pub delete_volume: bool,
}

impl OptionalParam {
    fn new(delete_publicip: bool, delete_volume: bool) -> OptionalParam {
        OptionalParam {
            delete_publicip: delete_publicip,
            delete_volume: delete_volume,
        }
    }
}
