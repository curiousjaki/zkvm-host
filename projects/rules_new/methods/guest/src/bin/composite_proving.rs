//#![no_std] 
//#![no_main]
use risc0_zkvm::{guest::env, serde};

fn main() {
    //let image_id: [u32; 8] = env::read();
    let journals: Vec<([u32;8],f64)> = env::read();
    for j in journals.iter() {
        // Panics on verifiable failure.
        env::verify(j.0, &serde::to_vec(&j.1).unwrap()).unwrap();
    }
    env::commit(&(journals));
}