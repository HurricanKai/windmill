/*
 * Windmill server API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.39.0
 * Contact: contact@windmill.dev
 * Generated by: https://openapi-generator.tech
 */



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "r#type")]
pub enum Job {
}



/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "CompletedJob")]
    CompletedJob,
    #[serde(rename = "QueuedJob")]
    QueuedJob,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::CompletedJob
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobKind {
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "dependencies")]
    Dependencies,
    #[serde(rename = "flow")]
    Flow,
    #[serde(rename = "flowpreview")]
    Flowpreview,
    #[serde(rename = "script_hub")]
    ScriptHub,
}

impl Default for JobKind {
    fn default() -> JobKind {
        Self::Script
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "python3")]
    Python3,
    #[serde(rename = "deno")]
    Deno,
    #[serde(rename = "go")]
    Go,
}

impl Default for Language {
    fn default() -> Language {
        Self::Python3
    }
}

