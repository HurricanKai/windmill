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
pub struct ListHubScripts200Response {
    #[serde(rename = "asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<crate::models::ListHubScripts200ResponseAsksInner>>,
}

impl ListHubScripts200Response {
    pub fn new() -> ListHubScripts200Response {
        ListHubScripts200Response {
            asks: None,
        }
    }
}


