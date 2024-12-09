//#![no_main]
//#![no_std]
use risc0_zkvm::guest::env;
use operations::{OperationRequest, Operation};
//use qfilter::Filter;
use std::string::String;
use serde_json::from_str;
use rules::{ConformanceMetadata, RuleChecker,InsertEvent};

fn main() {
    // read the input
    let or: OperationRequest = env::read();
    let serialized_cm: String = env::read();
    let mut cm: ConformanceMetadata = from_str(&serialized_cm).unwrap();
    cm.qf.insert_event(cm.current_image_id);

    for rule in cm.rules.iter(){
        assert_eq!(rule.check(&cm.qf),true);
    }

    let result: f64 = match &or.operation {
        Operation::Add => &or.a + &or.b,
        Operation::Sub => &or.a - &or.b,
        Operation::Mul => &or.a * &or.b,
        Operation::Div => &or.a / &or.b,
        _ => 0.0
    };


    // write public output to the journal
    //env::commit(&process_id);
    //env::commit(&SystemTime::now());

    let conformance_metadata_json: String = serde_json::to_string(&cm).unwrap();
    env::write(&conformance_metadata_json);
    env::commit(&result);
    //env::commit(&(&result,&rule_set));
    //env::commit(&json.as_bytes()); 

    //env::commit(&emission_factor);
}
