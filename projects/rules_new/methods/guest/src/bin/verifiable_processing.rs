//#![no_main]
//#![no_std]
use risc0_zkvm::guest::env;
use operations::{OperationRequest, Operation};
use std::string::String;
use serde_json::from_str;
use rules::{ConformanceMetadata, RuleChecker, InsertEvent};

// TODO: implement prior proof verification
fn main() {

    // read the operation input
    let method_payload: String = env::read();
    let or: OperationRequest = from_str(&method_payload).unwrap();
    // read the conformance metadata input
    let serialized_cm: String = env::read();
    let mut cm: ConformanceMetadata = from_str(&serialized_cm).unwrap();
    
    // check each conformance rule for this event
    for rule in cm.rules.iter(){
        assert_eq!(rule.check(&cm.qf),true);
    }
    // add this current event to the filter
    cm.qf.insert_event(cm.current_image_id);

    // execute the operation
    let result: f64 = match &or.operation {
        Operation::Add => &or.a + &or.b,
        Operation::Sub => &or.a - &or.b,
        Operation::Mul => &or.a * &or.b,
        Operation::Div => &or.a / &or.b,
        _ => 0.0
    };

    // serialize the output to json to avouid type mismatch, especially relevant for all vectors.
    let result_json: String = serde_json::to_string(&result).unwrap();
    let conformance_metadata_json: String = serde_json::to_string(&cm).unwrap();

    // commit public data output to the journal
    env::commit(&(result_json,conformance_metadata_json));
}
