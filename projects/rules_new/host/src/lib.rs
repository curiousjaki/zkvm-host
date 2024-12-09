use operations::Operation;
use rules::{CardinalityRule, ConformanceMetadata, InsertEvent, PrecedenceRule, Rule};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use methods::{
    COMPOSITE_PROVING_ELF, COMPOSITE_PROVING_ID, VERIFIABLE_PROCESSING_ELF,
    VERIFIABLE_PROCESSING_ID,
};
use anyhow::Error;


pub fn prove_method(
    a: f64,
    b: f64,
    operation: Operation,
    conformance_metadata: ConformanceMetadata,
) -> Receipt {
    println!("Build Proof and send to vm");
    let or = operations::OperationRequest { a, b, operation };
    let serialized_conformance_metadata: String =
        serde_json::to_string(&conformance_metadata).unwrap();
    let env = ExecutorEnv::builder()
        .write(&or)
        .unwrap()
        .write(&serialized_conformance_metadata)
        .unwrap()
        .build()
        .unwrap();
    let prover = default_prover();
    // read the input
    let prove_info = prover.prove(env, VERIFIABLE_PROCESSING_ELF).unwrap();
    return prove_info.receipt;
}

pub fn perform_composite_prove(receipts: Vec<Receipt>, image_id: [u32; 8]) -> Result<Receipt, Error> {
    let mut env_builder = ExecutorEnv::builder();
    let mut journals: Vec<([u32; 8], f64)> = Vec::new();
    for r in receipts.iter() {
        env_builder.add_assumption(r.clone());
        journals.push((image_id.clone(), r.journal.decode().unwrap()));
    }
    let env = env_builder
        //.write(&image_id)
        //.unwrap()
        .write(&journals)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    // read the input
    let prove = prover.prove(env, COMPOSITE_PROVING_ELF);
    match prove {
        Ok(prove_info) => {
            return Ok(prove_info.receipt);
        }
        Err(e) => {
            return Err(e);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = 4;
        println!("result is {}", result);
        assert_eq!(result, 4);
    }
}