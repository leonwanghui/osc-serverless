#[macro_use]
extern crate serde_derive;

mod models;
mod types;

use serde_json;
use serde_json::json;
use serde_json::value::Value;
use types::{DeleteOpts, JsSpec, ResourceSpec};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn load_resource_to_js(value: &JsValue) -> JsValue {
    let default_cp = String::from("huaweicloud");
    let res: ResourceSpec = value.into_serde().expect("failed to load resource spec!");
    let k = res.get_kind();
    let op = res.get_metadata().get_operation();
    let provider = res.get_metadata().get_cloud_provider();
    let mut js = JsSpec::new(&res.get_kind(), &op, json!(null));
    let json_spec = serde_json::to_string(&res.get_spec()).unwrap();

    match k.as_str() {
        "ComputeResource" => {
            let mut spec: models::ComputeResourceCreateRequest =
                serde_json::from_str(&json_spec).unwrap();
            // Update cloud provider info of compute resource create request
            // from metadata
            spec.cloud_provider = provider;
            let param: Value = match op.as_str() {
                "create" => {
                    let v: Value =
                        serde_json::from_str(&serde_json::to_string(&spec).unwrap()).unwrap();
                    v
                }
                "delete" => {
                    let opt = DeleteOpts::new(&spec.name, &default_cp, false, false);
                    let v: Value =
                        serde_json::from_str(&serde_json::to_string(&opt).unwrap()).unwrap();
                    v
                }
                _ => {
                    log(&format!("operation {} is not supported!", op));
                    json!(null)
                }
            };
            js.set_params(param);
        }
        "StorageResource" => {
            let mut spec: models::StorageResourceCreateRequest =
                serde_json::from_str(&json_spec).unwrap();
            // Update cloud provider info of storage resource create request
            // from metadata
            spec.cloud_provider = provider;
            let param: Value = match op.as_str() {
                "create" => {
                    let v: Value =
                        serde_json::from_str(&serde_json::to_string(&spec).unwrap()).unwrap();
                    v
                }
                "delete" => {
                    let opt = DeleteOpts::new(&spec.name, &default_cp, false, false);
                    let v: Value =
                        serde_json::from_str(&serde_json::to_string(&opt).unwrap()).unwrap();
                    v
                }
                _ => {
                    log(&format!("operation {} is not supported!", op));
                    json!(null)
                }
            };
            js.set_params(param);
        }
        "NetworkResource" => {
            let mut spec: models::NetworkResourceCreateRequest =
                serde_json::from_str(&json_spec).unwrap();
            // Update cloud provider info of network resource create request
            // from metadata
            spec.cloud_provider = provider;
            let param: Value = match op.as_str() {
                "create" => {
                    let v: Value =
                        serde_json::from_str(&serde_json::to_string(&spec).unwrap()).unwrap();
                    v
                }
                "delete" => {
                    let opt = DeleteOpts::new(&spec.name, &default_cp, false, false);
                    let v: Value =
                        serde_json::from_str(&serde_json::to_string(&opt).unwrap()).unwrap();
                    v
                }
                _ => {
                    log(&format!("operation {} is not supported!", op));
                    json!(null)
                }
            };
            js.set_params(param);
        }
        _ => log(&format!("resource kind {} is not supported!", k)),
    };
    JsValue::from_serde(&js).unwrap()
}
