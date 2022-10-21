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
pub struct FlowStatusRetry {
    #[serde(rename = "fail_count", skip_serializing_if = "Option::is_none")]
    pub fail_count: Option<i32>,
    #[serde(rename = "failed_jobs", skip_serializing_if = "Option::is_none")]
    pub failed_jobs: Option<Vec<uuid::Uuid>>,
}

impl FlowStatusRetry {
    pub fn new() -> FlowStatusRetry {
        FlowStatusRetry {
            fail_count: None,
            failed_jobs: None,
        }
    }
}


