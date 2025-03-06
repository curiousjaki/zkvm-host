//#![no_std] 
//#![no_main]
use risc0_zkvm::{guest::env, serde};
use rules::conformance::{PoamInput,RuleInput,PoamMetadata};

use std::string::String;
use serde_json::from_str;

fn main() {
    let ser_image_id: String = env::read();
    let image_id: [u32; 8] = from_str(&ser_image_id).unwrap();
    
    let ser_po1: String = env::read();
    let ser_po2: String = env::read();
    let decoded_p1: (String, String) = from_str(&ser_po1).unwrap();
    let decoded_p2: (String, String) = from_str(&ser_po2).unwrap();

    
    let result1: f64 = from_str(&decoded_p1.0).unwrap();
    let result2: f64 = from_str(&decoded_p2.0).unwrap();
    let mut pm1 : PoamMetadata = from_str(&decoded_p1.1).unwrap();
    let mut pm2 : PoamMetadata = from_str(&decoded_p2.1).unwrap();

    env::verify(pm1.image_id, &serde::to_vec(&decoded_p1).unwrap()).unwrap();
    env::verify(pm2.image_id, &serde::to_vec(&decoded_p2).unwrap()).unwrap();

    let result = result1 + result2;
    let metadata = PoamMetadata {
        was_first_event: false,
        image_id: image_id,
        qf: qfilter::Filter::new(1, 0.01).unwrap()
    };
    let result_json: String = serde_json::to_string(&result).unwrap();
    let pm_json: String = serde_json::to_string(&metadata).unwrap();
    env::commit(&(result_json,pm_json));
}