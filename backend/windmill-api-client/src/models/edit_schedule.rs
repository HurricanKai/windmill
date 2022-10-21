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
pub struct EditSchedule {
    #[serde(rename = "schedule")]
    pub schedule: String,
    #[serde(rename = "script_path")]
    pub script_path: String,
    #[serde(rename = "is_flow")]
    pub is_flow: bool,
    #[serde(rename = "args")]
    pub args: ::std::collections::HashMap<String, serde_json::Value>,
}

impl EditSchedule {
    pub fn new(schedule: String, script_path: String, is_flow: bool, args: ::std::collections::HashMap<String, serde_json::Value>) -> EditSchedule {
        EditSchedule {
            schedule,
            script_path,
            is_flow,
            args,
        }
    }
}


