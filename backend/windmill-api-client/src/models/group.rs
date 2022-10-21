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
pub struct Group {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    #[serde(rename = "extra_perms", skip_serializing_if = "Option::is_none")]
    pub extra_perms: Option<::std::collections::HashMap<String, bool>>,
}

impl Group {
    pub fn new(name: String) -> Group {
        Group {
            name,
            summary: None,
            members: None,
            extra_perms: None,
        }
    }
}


