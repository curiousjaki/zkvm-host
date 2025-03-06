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
use serde_json::{from_str, Value};

use rules::conformance::{PoamMetadata};

fn main() {
    //let start = env::cycle_count();
    let ser_image_id: String = env::read();
    let image_id: [u32; 8] = from_str(&ser_image_id).unwrap();
    let ser_or: String = env::read();
    let operation_requests: Vec<OperationRequest> = from_str(&ser_or).unwrap();
    //let method_payload: String = env::read();
    //let json_value: Value = from_str(&method_payload).unwrap();
    //let mut result: f64 = 0.1;
    //if let Value::Object(map) = json_value {
    //    for (key, value) in &map {
    //        if let Value::Object(inner_map) = value {
    //            let operation : Operation = from_str(&inner_map["operation"].to_string()).unwrap();
    //            let operation_request = OperationRequest {
    //                a: inner_map["a"].as_f64().unwrap(),
    //                b: inner_map["b"].as_f64().unwrap(),
    //                operation: operation
    //            };
    //            env::write(&serde::to_vec(&operation_request.compute()).unwrap());
    //            result = result + operation_request.compute();
    //        }
    //    }
    //}
    let mut result: f64 = 0.0;
    for or in operation_requests.iter() {
        result = result + or.compute();
    }

    let metadata = PoamMetadata {
        was_first_event: true,
        image_id: image_id,
        qf: qfilter::Filter::new(1, 0.01).unwrap()
    };
    let result_json: String = serde_json::to_string(&result).unwrap();
    let pm_json: String = serde_json::to_string(&metadata).unwrap();

    env::commit(&(result_json,pm_json));
}
