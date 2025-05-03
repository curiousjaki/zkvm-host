//#![no_main]
//#![no_std]
use risc0_zkvm::guest::env;
use operations::OperationRequest;
use std::string::String;
use serde_json::from_str;

fn main() {

    let ser_or: String = env::read();
    let operation_requests: Vec<OperationRequest> = from_str(&ser_or).unwrap();
 
    let mut result: f64 = 0.0;
    for or in operation_requests.iter() {
        result = result + or.compute();
    }

    let result_json: String = serde_json::to_string(&result).unwrap();
    let pm_json: String = serde_json::to_string(&true).unwrap();

    env::commit(&(result_json,pm_json));
}
