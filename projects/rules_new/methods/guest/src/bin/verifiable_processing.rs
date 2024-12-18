//#![no_main]
//#![no_std]
use risc0_zkvm::{
    guest::env,
    serde
};
use operations::{
    OperationRequest, 
    Operation
};
use std::string::String;
use serde_json::from_str;
use rules::RuleChecker;
use rules::conformance::{
    RuleInput,
    PoamInput,
    PoamMetadata,
};
use rules::event_filter::InsertEvent;


fn verify_previous_receipt(pi: PoamInput) -> PoamMetadata {
    match pi.public_data {
        Some((public_data_json, metadata_json)) => {
            let mut pm : PoamMetadata = from_str(&metadata_json).unwrap();
            //env::write(&format!("{:?}",&ccr.image_id));
            //env::write(&format!("{:?}",(&public_data_json,&metadata_json)));
            env::verify(pm.image_id, &serde::to_vec(&(&public_data_json, &metadata_json)).unwrap()).unwrap();
            pm.was_first_event = false;
            return pm;
        },
        None => {
            return PoamMetadata {
                was_first_event: true,
                image_id: pi.image_id, 
                qf: qfilter::Filter::new(100, 0.01).unwrap()
            };
        }
    }
}


fn main() {
    //let start = env::cycle_count();

    let method_payload: String = env::read();
    let ser_pi: String = env::read();

    // potentially expensive or frequently called code
    // ...
    //let input_read = env::cycle_count();
    //eprintln!("reading input: {}", input_read - start);
    
    let operation_request: OperationRequest = from_str(&method_payload).unwrap();
    let pi: PoamInput = from_str(&ser_pi).unwrap();

    let mut pm: PoamMetadata;

    

    //let string_transformation = env::cycle_count();
    //eprintln!("converting json to structs: {}", string_transformation - input_read);

    //if cci.public_data.1.len() > 0 {
    //    ccr = from_str(&cci.public_data.1).unwrap();
    //    env::verify(ccr.image_id, &serde::to_vec(&cci.public_data).unwrap()).unwrap();
    //}

    match pi.rule_input.rules {
        Some(rules) => {
            for rule in rules.iter(){
                //env::write(&format!("{:?}",rule));
                assert_eq!(rule.check(&pm.qf,&pi.image_id),true);
            }
        },
        None => {}
    }
    // add this current event to the filter
    pm.qf.insert_event(pi.image_id);

    // execute the operation
    let result: f64 = operation_request.compute();

    //let start_serialization = env::cycle_count();
    // serialize the output to json to avouid type mismatch, especially relevant for all vectors.
    let result_json: String = serde_json::to_string(&result).unwrap();
    let pm_json: String = serde_json::to_string(&pm).unwrap();
    //let end_serialization = env::cycle_count();

    //eprintln!("serialization: {}", end_serialization - start_serialization);
    // commit public data output to the journal
    env::commit(&(result_json,pm_json));
}
