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
pub struct ListHubFlows200ResponseFlowsInner {
    #[serde(rename = "id")]
    pub id: f32,
    #[serde(rename = "flow_id")]
    pub flow_id: f32,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "apps")]
    pub apps: Vec<String>,
    #[serde(rename = "approved")]
    pub approved: bool,
    #[serde(rename = "votes")]
    pub votes: f32,
}

impl ListHubFlows200ResponseFlowsInner {
    pub fn new(id: f32, flow_id: f32, summary: String, apps: Vec<String>, approved: bool, votes: f32) -> ListHubFlows200ResponseFlowsInner {
        ListHubFlows200ResponseFlowsInner {
            id,
            flow_id,
            summary,
            apps,
            approved,
            votes,
        }
    }
}


