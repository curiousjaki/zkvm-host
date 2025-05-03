pub mod conformance;
pub mod event_filter;
use event_filter::FollowedBy;
pub trait RuleChecker {
    fn check(&self, qf: &qfilter::Filter, previous_image_id: &[u32; 8]) -> bool;
}

impl RuleChecker for Rule {
    fn check(&self, qf: &qfilter::Filter, previous_image_id: &[u32; 8]) -> bool {
        match self {
            Rule::Precedence(rule) => rule.check(qf, previous_image_id),
            Rule::Cardinality(rule) => rule.check(qf, previous_image_id),
            Rule::Exclusiveness(rule) => rule.check(qf, previous_image_id),
            Rule::Ordering(rule) => rule.check(qf, previous_image_id),
        }
    }
}

#[derive(Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct PrecedenceRule {
    pub preceeding: [u32; 8],
    //prior_image_id: [u32; 8],
}
impl RuleChecker for PrecedenceRule {
    fn check(&self, qf: &qfilter::Filter, previous_image_id: &[u32; 8]) -> bool {
        if qf.contains(&self.preceeding) && self.preceeding == *previous_image_id {
            return true;
        }
        false
    }
}

#[derive(Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct CardinalityRule {
    pub prior: [u32; 8],
    pub min: u64,
    pub max: u64,
}

impl RuleChecker for CardinalityRule {
    fn check(&self, qf: &qfilter::Filter, _previous_image_id: &[u32; 8]) -> bool {
        let mut mut_qf = qf.clone();
        if mut_qf.count(self.prior) >= self.min && mut_qf.count(self.prior) <= self.max {
            return true;
        }
        false
    }
}

#[derive(Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct ExclusivenessRule {
    pub prior_a: [u32; 8],
    pub prior_b: [u32; 8],
}

impl RuleChecker for ExclusivenessRule {
    fn check(&self, qf: &qfilter::Filter, _previous_image_id: &[u32; 8]) -> bool {
        if qf.contains(&self.prior_a) || qf.contains(&self.prior_b) {
            if qf.contains(&self.prior_a) && qf.contains(&self.prior_b) {
                return false;
            }
            return true;
        }
        false
    }
}

#[derive(Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct OrderingRule {
    pub prior: [u32; 8],
    pub next: [u32; 8],
}

impl RuleChecker for OrderingRule {
    fn check(&self, qf: &qfilter::Filter, _previous_image_id: &[u32; 8]) -> bool {
        if qf.contains(&FollowedBy {
            prior: self.prior,
            next: self.next,
        }) {
            return true;
        }
        false
    }
}

#[derive(Debug, Hash, serde::Serialize, serde::Deserialize)]
pub enum Rule {
    Precedence(PrecedenceRule),
    Cardinality(CardinalityRule),
    Exclusiveness(ExclusivenessRule),
    Ordering(OrderingRule),
}
