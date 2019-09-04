/*
 * Open Service Cloud API
 *
 * Open Service Cloud API to manage different backend cloud services.
 *
 * The version of the OpenAPI document: 0.0.3
 * Contact: wanghui71leon@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// ErrorSpec : Detailed HTTP error response, which consists of a HTTP status code, and a custom error message unique for each failure case.

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorSpec {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "message")]
    pub message: String,
}

impl ErrorSpec {
    /// Detailed HTTP error response, which consists of a HTTP status code, and a custom error message unique for each failure case.
    pub fn new(code: i32, message: String) -> ErrorSpec {
        ErrorSpec {
            code: code,
            message: message,
        }
    }
}
