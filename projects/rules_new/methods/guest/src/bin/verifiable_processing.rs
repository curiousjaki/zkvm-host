//#![no_main]
//#![no_std]
use risc0_zkvm::{guest::env,serde};
//use risc0_zkvm::serde;
use operations::{OperationRequest, Operation};
use std::string::String;
use serde_json::from_str;
use rules::{RuleChecker};
use rules::conformance::{CompositeConformanceInput, RuleInput, ConformanceCheckedReceipt};
use rules::event_filter::InsertEvent;

// TODO: implement prior proof verification
fn main() {
    let start = env::cycle_count();

    let method_payload: String = env::read();
    let ser_cci: String = env::read();

    // potentially expensive or frequently called code
    // ...
    let input_read = env::cycle_count();
    eprintln!("reading input: {}", input_read - start);
    
    let or: OperationRequest = from_str(&method_payload).unwrap();
    let cci: CompositeConformanceInput = from_str(&ser_cci).unwrap();

    let mut ccr: ConformanceCheckedReceipt;

    match cci.public_data {
        Some((public_data_json, metadata_json)) => {
            ccr = from_str(&metadata_json).unwrap();
            env::write(&format!("{:?}",&ccr.image_id));
            env::write(&format!("{:?}",(&public_data_json,&metadata_json)));
            env::verify(ccr.image_id, &serde::to_vec(&(&public_data_json, &metadata_json)).unwrap()).unwrap();
        },
        None => {
            ccr  = ConformanceCheckedReceipt{
                was_first_event: true,
                image_id: cci.rule_input.current_image_id, 
                qf: qfilter::Filter::new(100, 0.01).unwrap()
            };
        }
    }

    let string_transformation = env::cycle_count();
    eprintln!("converting json to structs: {}", string_transformation - input_read);

    //if cci.public_data.1.len() > 0 {
    //    ccr = from_str(&cci.public_data.1).unwrap();
    //    env::verify(ccr.image_id, &serde::to_vec(&cci.public_data).unwrap()).unwrap();
    //}

    match cci.rule_input.rules {
        Some(rules) => {
            for rule in rules.iter(){
                env::write(&format!("{:?}",rule));
                assert_eq!(rule.check(&ccr.qf,&cci.rule_input.current_image_id),true);
            }
        },
        None => {}
    }
    // add this current event to the filter
    ccr.qf.insert_event(cci.rule_input.current_image_id);

    // execute the operation
    let result: f64 = match &or.operation {
        Operation::Add => &or.a + &or.b,
        Operation::Sub => &or.a - &or.b,
        Operation::Mul => &or.a * &or.b,
        Operation::Div => &or.a / &or.b,
        _ => 0.0
    };

    let start_serialization = env::cycle_count();
    // serialize the output to json to avouid type mismatch, especially relevant for all vectors.
    let result_json: String = serde_json::to_string(&result).unwrap();
    let ccr_json: String = serde_json::to_string(&ccr).unwrap();
    let end_serialization = env::cycle_count();

    eprintln!("serialization: {}", end_serialization - start_serialization);
    // commit public data output to the journal
    env::commit(&(result_json,ccr_json));
}
