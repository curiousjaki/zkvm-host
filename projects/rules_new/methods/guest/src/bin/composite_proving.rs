//#![no_std] 
//#![no_main]
use risc0_zkvm::{guest::env, serde};
use rules::{CompositeProofInput};
use std::string::String;
use serde_json::from_str;

fn main() {
    //let image_id: [u32; 8] = env::read();
    let cpi_vec_string: String = env::read();
    let cpi_vec: Vec<CompositeProofInput> = serde_json::from_str(&cpi_vec_string).unwrap();
    for cpi in cpi_vec.iter() {
    //    // Panics on verifiable failure.
        env::verify(cpi.image_id, &serde::to_vec(&cpi.public_data).unwrap()).unwrap();
    }
    env::commit(&(true));
}