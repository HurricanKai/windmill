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
pub struct FlowStatus {
    #[serde(rename = "step")]
    pub step: i32,
    #[serde(rename = "modules")]
    pub modules: Vec<crate::models::FlowStatusModule>,
    #[serde(rename = "failure_module")]
    pub failure_module: Box<crate::models::FlowStatusModule>,
    #[serde(rename = "retry", skip_serializing_if = "Option::is_none")]
    pub retry: Option<Box<crate::models::FlowStatusRetry>>,
}

impl FlowStatus {
    pub fn new(step: i32, modules: Vec<crate::models::FlowStatusModule>, failure_module: crate::models::FlowStatusModule) -> FlowStatus {
        FlowStatus {
            step,
            modules,
            failure_module: Box::new(failure_module),
            retry: None,
        }
    }
}


