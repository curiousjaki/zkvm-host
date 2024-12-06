//#![no_main]
use risc0_zkvm::guest::env;
use operations::Operation;
use qfilter::Filter;
use std::string::String;
use rules::{RuleSet};

fn main() {
    // read the input
    let (a,b,operation, rule_set): (f64,f64,Operation, RuleSet) = env::read();


    let result: f64 = match operation {
        Operation::Add => a + b,
        Operation::Sub => a - b,
        Operation::Mul => a * b,
        Operation::Div => a / b,
        _ => 0.0
    };
    // write public output to the journal
    //env::commit(&process_id);
    //env::commit(&SystemTime::now());

    let json: String = serde_json::to_string(&rule_set.qf).unwrap();
    //env::write(&json.as_str());

    env::commit(&(&result,&json));
    //env::commit(&json.as_bytes()); 

    //env::commit(&emission_factor);
}
