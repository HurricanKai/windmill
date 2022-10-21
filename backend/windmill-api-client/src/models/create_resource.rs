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
pub struct CreateResource {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "value")]
    pub value: serde_json::Value,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    #[serde(rename = "is_oauth", skip_serializing_if = "Option::is_none")]
    pub is_oauth: Option<bool>,
}

impl CreateResource {
    pub fn new(path: String, value: serde_json::Value, resource_type: String) -> CreateResource {
        CreateResource {
            path,
            value,
            description: None,
            resource_type,
            is_oauth: None,
        }
    }
}


