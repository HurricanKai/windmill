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
pub struct PathScript {
    #[serde(rename = "input_transforms", skip_serializing_if = "Option::is_none")]
    pub input_transforms: Option<::std::collections::HashMap<String, crate::models::InputTransform>>,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl PathScript {
    pub fn new(path: String, r#type: RHashType) -> PathScript {
        PathScript {
            input_transforms: None,
            path,
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "script")]
    Script,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Script
    }
}

