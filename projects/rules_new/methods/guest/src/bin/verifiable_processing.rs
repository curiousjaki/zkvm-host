//#![no_main]
//#![no_std]
use risc0_zkvm::guest::env;
use operations::{OperationRequest, Operation};
//use qfilter::Filter;
use std::string::String;
//use serde_json;
use rules::{RuleSet, RuleChecker};

fn main() {
    // read the input
    let or: OperationRequest = env::read();
    let serialized_rule_set: String = env::read();

    let rule_set: RuleSet = serde_json::from_str(&serialized_rule_set).unwrap();

    let result: f64 = match &or.operation {
        Operation::Add => &or.a + &or.b,
        Operation::Sub => &or.a - &or.b,
        Operation::Mul => &or.a * &or.b,
        Operation::Div => &or.a / &or.b,
        _ => 0.0
    };
    for rule in rule_set.rules.iter(){
        assert_eq!(rule.check(&rule_set.qf),true);
    }
    // write public output to the journal
    //env::commit(&process_id);
    //env::commit(&SystemTime::now());

    //let json: String = serde_json::to_string(&rule_set.qf).unwrap();
    env::write(&rule_set);

    env::commit(&result);
    //env::commit(&(&result,&rule_set));
    //env::commit(&json.as_bytes()); 

    //env::commit(&emission_factor);
}
