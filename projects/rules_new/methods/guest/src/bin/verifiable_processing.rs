//#![no_main]
use risc0_zkvm::guest::env;
use operations::Operation;
fn main() {
    // read the input
    let (a,b,operation): (f64,f64,Operation) = env::read();
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
    env::commit(&result);

    //env::commit(&emission_factor);
}
