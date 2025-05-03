use super::{OrderingRule, Rule};
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PoamInput {
    pub image_id: [u32; 8],
    pub rule_input: RuleInput,
    pub public_data: Option<(String, String)>, //public_data_json, conformance_checked_receipt_json
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PoamMetadata {
    pub was_first_event: bool,
    pub image_id: [u32; 8],
    pub qf: qfilter::Filter,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RuleInput {
    pub rules: Option<Vec<Rule>>,
    pub ordering_rules: Option<Vec<OrderingRule>>,
}
