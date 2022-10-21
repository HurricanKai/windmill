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
pub struct ConnectSlackCallbackRequest {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "state")]
    pub state: String,
}

impl ConnectSlackCallbackRequest {
    pub fn new(code: String, state: String) -> ConnectSlackCallbackRequest {
        ConnectSlackCallbackRequest {
            code,
            state,
        }
    }
}


