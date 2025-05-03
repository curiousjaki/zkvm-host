//#![no_std] 
//#![no_main]
use risc0_zkvm::{guest::env, serde};

use std::string::String;
use serde_json::from_str;
use poam_helper::VerificationMetadata;

fn main() {
    let ser_image_id: String = env::read();
    let image_id: [u32; 8] = from_str(&ser_image_id).unwrap();
    
    let ser_vm1: String = env::read();
    let ser_vm2: String = env::read();
    let p1_vm: VerificationMetadata = from_str(&ser_vm1).unwrap();
    let p2_vm: VerificationMetadata = from_str(&ser_vm2).unwrap();

    let result1: f64 = from_str(&p1_vm.journal_data.0).unwrap();
    let result2: f64 = from_str(&p2_vm.journal_data.0).unwrap();

    env::verify(p1_vm.image_id, &serde::to_vec(&p1_vm.journal_data).unwrap()).unwrap();
    env::verify(p2_vm.image_id, &serde::to_vec(&p2_vm.journal_data).unwrap()).unwrap();

    let result = result1 + result2;

    let serialized_result_json: String = serde_json::to_string(&result).unwrap();
    let serialized_metadata_json: String = serde_json::to_string(false).unwrap();
    env::commit(&(serialized_result_json,serialized_metadata_json));
}