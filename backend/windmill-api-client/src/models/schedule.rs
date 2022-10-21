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
pub struct Schedule {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "edited_by")]
    pub edited_by: String,
    #[serde(rename = "edited_at")]
    pub edited_at: String,
    #[serde(rename = "schedule")]
    pub schedule: String,
    #[serde(rename = "offset_")]
    pub offset_: i32,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "script_path")]
    pub script_path: String,
    #[serde(rename = "is_flow")]
    pub is_flow: bool,
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "extra_perms")]
    pub extra_perms: ::std::collections::HashMap<String, bool>,
}

impl Schedule {
    pub fn new(path: String, edited_by: String, edited_at: String, schedule: String, offset_: i32, enabled: bool, script_path: String, is_flow: bool, extra_perms: ::std::collections::HashMap<String, bool>) -> Schedule {
        Schedule {
            path,
            edited_by,
            edited_at,
            schedule,
            offset_,
            enabled,
            script_path,
            is_flow,
            args: None,
            extra_perms,
        }
    }
}


