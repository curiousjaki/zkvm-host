fn char_to_u32_array(ch: char) -> [u32; 8] {
    let mut result = [0u32; 8];
    let code_point = ch as u32; // Convert the char to its Unicode code point

    for i in 0..8 {
        // Extract each bit from the code point and store it in the array
        result[7 - i] = (code_point >> i) & 1;
    }

    result
}

pub trait InsertEvent {
    fn insert_ordered_event(
        &mut self,
        event: [u32; 8],
        rules: Vec<OrderingRule>,
    ) -> Result<(), qfilter::Error>;
    fn insert_event(&mut self, event: [u32; 8]) -> Result<(), qfilter::Error>;
}

impl InsertEvent for qfilter::Filter {
    /// Inserts an event into the filter and adds a FollowedBy relation if an ordering rule is fulfilled
    fn insert_ordered_event(
        &mut self,
        event: [u32; 8],
        rules: Vec<OrderingRule>,
    ) -> Result<(), qfilter::Error> {
        for rule in rules.iter() {
            if rule.next == event {
                if self.contains(rule.prior) {
                    self.insert_duplicated(FollowedBy {
                        prior: rule.prior,
                        next: rule.next,
                    })
                    .unwrap();
                }
            }
        }
        return self.insert_duplicated(event);
    }
    /// Inserts an event into the filter and adds a FollowedBy relation if an ordering rule is fulfilled
    fn insert_event(&mut self, event: [u32; 8]) -> Result<(), qfilter::Error> {
        return self.insert_duplicated(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut qf: qfilter::Filter = qfilter::Filter::new(100, 0.01).unwrap();
        let events: [char; 5] = ['a', 'a', 'b', 'c', 'd'];
        let current_event = char_to_u32_array('d');
        for event in events.iter() {
            qf.insert_ordered_event(
                char_to_u32_array(*event),
                vec![OrderingRule {
                    prior: char_to_u32_array('a'),
                    next: char_to_u32_array('c'),
                }],
            )
            .unwrap();
        }

        let rules: Vec<Rule> = vec![
            Rule::Precedence(PrecedenceRule {
                current: current_event,
                preceeding: char_to_u32_array('d'),
            }),
            //Rule::Precedence(PrecedenceRule{current: current_event, prior: char_to_u32_array('d')}),
            Rule::Cardinality(CardinalityRule {
                prior: char_to_u32_array('b'),
                max: 1,
                min: 1,
            }),
            Rule::Cardinality(CardinalityRule {
                prior: char_to_u32_array('a'),
                max: 2,
                min: 2,
            }),
            Rule::Exclusiveness(ExclusivenessRule {
                prior_a: char_to_u32_array('a'),
                prior_b: char_to_u32_array('e'),
            }),
            Rule::Ordering(OrderingRule {
                prior: char_to_u32_array('a'),
                next: char_to_u32_array('c'),
            }),
        ];
        //let filter_string = serde_json::to_string(&qf).unwrap();
        let task_rules = ConformanceMetadata {
            previous_image_id: current_event,
            current_image_id: current_event,
            rules: rules,
            qf,
        };
        let res = serde_json::to_string(&task_rules).unwrap();
        //write a string to a file
        std::fs::write("rules.json", &res).unwrap();
        println!("{:?}", &res);

        for rule in task_rules.rules.iter() {
            assert_eq!(rule.check(&task_rules.qf), true);
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ConformanceMetadata {
    pub previous_image_id: [u32; 8],
    pub current_image_id: [u32; 8],
    pub rules: Vec<Rule>,
    pub qf: qfilter::Filter,
}

pub trait RuleChecker {
    fn check(&self, qf: &qfilter::Filter) -> bool;
}

impl RuleChecker for Rule {
    fn check(&self, qf: &qfilter::Filter) -> bool {
        match self {
            Rule::Precedence(rule) => rule.check(qf),
            Rule::Cardinality(rule) => rule.check(qf),
            Rule::Exclusiveness(rule) => rule.check(qf),
            Rule::Ordering(rule) => rule.check(qf),
        }
    }
}

#[derive(Debug, Hash, serde::Serialize, serde::Deserialize)]
struct FollowedBy {
    pub prior: [u32; 8],
    pub next: [u32; 8],
}

#[derive(Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct PrecedenceRule {
    pub preceeding: [u32; 8],
    pub current: [u32; 8],
    //prior_image_id: [u32; 8],
}
impl RuleChecker for PrecedenceRule {
    // meaning that the prior event happend anytime before this event. not necessarily directly before
    // to do precedence one would have to evaluate the previous events signature.
    fn check(&self, qf: &qfilter::Filter) -> bool {
        if qf.contains(&self.current) && self.preceeding == self.current {
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
    fn check(&self, qf: &qfilter::Filter) -> bool {
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
    fn check(&self, qf: &qfilter::Filter) -> bool {
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
    fn check(&self, qf: &qfilter::Filter) -> bool {
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
