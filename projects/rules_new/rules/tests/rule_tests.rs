use rules::{CardinalityRule, PrecedenceRule, Rule, ExclusivenessRule, OrderingRule, RuleChecker};
use rules::event_filter::InsertEvent;
use rules::conformance::{PoamInput, PoamMetadata, RuleInput};
pub fn char_to_u32_array(ch: char) -> [u32; 8] {
    let mut result = [0u32; 8];
    let code_point = ch as u32; // Convert the char to its Unicode code point

    for i in 0..8 {
        // Extract each bit from the code point and store it in the array
        result[7 - i] = (code_point >> i) & 1;
    }
    result
}

pub fn generate_event_filter(events: [char; 5], or: Option<OrderingRule>) -> qfilter::Filter {
    let mut qf: qfilter::Filter = qfilter::Filter::new(100, 0.01).unwrap();
    //let  = ['a', 'a', 'b', 'c', preceeding_event];
    for event in events.iter() {
        match or {
            Some(ref rule) => qf.insert_ordered_event(
                char_to_u32_array(*event),
                vec![rule],
            )
            .unwrap(),
            None => qf.insert_event(char_to_u32_array(*event)).unwrap(),
        }
    }
    return qf;
}

pub fn generate_rule_set(preceeding_event:char) -> Vec<Rule> {
    let rules: Vec<Rule> = vec![
        Rule::Precedence(PrecedenceRule {
            preceeding: char_to_u32_array(preceeding_event),
        }),
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
    return rules;
}

//cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_rules() {
        let preceeding_event = 'c';
        let qf: qfilter::Filter = generate_event_filter(
                ['a', 'a', 'b', 'c', preceeding_event],
                Some(OrderingRule {
                prior: char_to_u32_array('a'),
                next: char_to_u32_array('c'),
            }));
        let rules: Vec<Rule> = generate_rule_set( preceeding_event);
        for rule in rules.iter() {
            println!("({:?})", rule);
            assert_eq!(rule.check(&qf,&char_to_u32_array(preceeding_event)), true);
        }
    }
        //let filter_string = serde_json::to_string(&qf).unwrap();
        //let cci = CompositeConformanceInput {
        //    rule_input: serde_json::to_string(&RuleInput {
        //        current_image_id: current_event,
        //        rules: Some(rules),
        //        ordering_rules: Some(vec![]),
        //    }).unwrap(),
        //    public_data: ("1".to_string(),"asdf".to_string()),
        //    //previous_image_id: Some(current_event),
        //    //current_image_id: current_event,
        //    //rules: Some(rules),
        //    //ordering_rules: Some(vec![]),
        //    //qf:Some(qf),
        //};
        //let res = serde_json::to_string(&cm).unwrap();
        //write a string to a file
        //std::fs::write("rules.json", &res).unwrap();
        //println!("{:?}", &res);
        
    }