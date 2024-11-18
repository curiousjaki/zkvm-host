//#![no_std] 
//#![no_main]
use risc0_zkvm::{guest::env, serde};

fn main() {
    // n and e are the public modulus and exponent respectively.
    // x value that will be kept private.
    //let receipt: Receipt = env::read();
    let image_id: [u32; 8] = env::read();
    let results: Vec<f64> = env::read();
    //let mut f = qfilter::Filter::new(10, 0.01);
    //for i in 0..5 {
    //    f.as_mut().expect("REASON").insert(i).unwrap();
    //}
  

    //for r in receipts.iter() {
    //    println!("{:?}", image_ID);
    for r in results.iter() {
        env::verify(image_id, &serde::to_vec(&r).unwrap()).unwrap();
    }
    //}
    // Commit n, e, and x^e mod n.
    env::commit(&(results));
    //,f.expect("REASON").contains(1)));
}