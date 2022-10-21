/*
 * Windmill server API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.39.0
 * Contact: contact@windmill.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RetryExponential {
    #[serde(rename = "attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<i32>,
    #[serde(rename = "seconds", skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i32>,
}

impl RetryExponential {
    pub fn new() -> RetryExponential {
        RetryExponential {
            attempts: None,
            multiplier: None,
            seconds: None,
        }
    }
}


