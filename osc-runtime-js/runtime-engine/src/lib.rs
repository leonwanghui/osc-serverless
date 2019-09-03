#[macro_use]
extern crate serde_derive;

use serde_json::value::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

#[derive(Debug, Clone, Deserialize)]
pub struct ResourceSpec {
    pub kind: String,
    pub metadata: Metadata,
    pub spec: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub operation: String,
    pub cloud_provider: String,
    pub labels: Value,
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
}

#[wasm_bindgen]
pub fn load_resource_to_js(value: &JsValue) -> JsValue {
    let res: ResourceSpec = value.into_serde().expect("failed to load resource spec!");
    let js = JsSpec::new(&res.kind, &res.metadata.clone().operation, res.spec.clone());
    JsValue::from_serde(&js).unwrap()
}
