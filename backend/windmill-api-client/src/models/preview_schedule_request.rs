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
pub struct PreviewScheduleRequest {
    #[serde(rename = "schedule")]
    pub schedule: String,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl PreviewScheduleRequest {
    pub fn new(schedule: String) -> PreviewScheduleRequest {
        PreviewScheduleRequest {
            schedule,
            offset: None,
        }
    }
}


