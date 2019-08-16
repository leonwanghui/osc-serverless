pub mod config_define;
pub use self::config_define::{Config, Metadata};

use serde_json;
use serde_yaml;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn load(path: &str, op: &str) -> Config {
    assert!(!path.is_empty());
    assert!(!op.is_empty());

    let file = File::open(path).expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Unable to read file");

    let v: Vec<&str> = path.split(".").collect();
    let ext = v[v.len() - 1].to_string();
    let mut config: Config = match ext.to_ascii_lowercase().as_str() {
        "json" => {
            let conf: Config = serde_json::from_str(&contents).unwrap();
            conf
        }
        "yml" | "yaml" => {
            let conf: Config = serde_yaml::from_str(&contents).unwrap();
            conf
        }
        _ => {
            panic!("cannot load {} file", path);
        }
    };

    let mut meta = config.get_metadata();
    meta.set_operation(op.to_ascii_lowercase());
    config.muted_with_metadata(meta).validate();

    config
}
