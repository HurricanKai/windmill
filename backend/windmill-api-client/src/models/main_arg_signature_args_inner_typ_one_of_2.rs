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
pub struct MainArgSignatureArgsInnerTypOneOf2 {
    #[serde(rename = "object")]
    pub object: Vec<crate::models::MainArgSignatureArgsInnerTypOneOf2ObjectInner>,
}

impl MainArgSignatureArgsInnerTypOneOf2 {
    pub fn new(object: Vec<crate::models::MainArgSignatureArgsInnerTypOneOf2ObjectInner>) -> MainArgSignatureArgsInnerTypOneOf2 {
        MainArgSignatureArgsInnerTypOneOf2 {
            object,
        }
    }
}


