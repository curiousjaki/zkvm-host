use operations::Operation;
use rules::{CardinalityRule, ConformanceMetadata, InsertEvent, PrecedenceRule, Rule,CompositeProofInput};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use methods::{
    COMPOSITE_PROVING_ELF, COMPOSITE_PROVING_ID, VERIFIABLE_PROCESSING_ELF,
    VERIFIABLE_PROCESSING_ID,
};
use qfilter::Filter;
use anyhow::Error;


pub fn prove_method(
    a: f64,
    b: f64,
    operation: Operation,
    conformance_metadata: &ConformanceMetadata,
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
    let mut cpi: Vec<CompositeProofInput> = Vec::new();
    for r in receipts.iter() {
        env_builder.add_assumption(r.clone());
        cpi.push(CompositeProofInput{image_id:image_id.clone(),public_data: r.journal.decode().unwrap()});
    }
    let cpi_string = serde_json::to_string(&cpi).unwrap();
    println!("{:?}",cpi_string);
    let env = env_builder
        //.write(&image_id)
        //.unwrap()
        .write(&cpi_string)
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

   //RISC0_DEV_MODE=0 RUST_LOG=info cargo test --release -- --nocapture
    #[test]
    fn test_proving_method(){
        println!("Starting the Program");
        //env_logger::init();
        // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`

        //tracing_subscriber::fmt()
        //    .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        //    .init();
        //let filter = qfilter::Filter::new(1000, 0.01).expect("Failed to create filter");
        //let rule1 = Rule::Cardinality(CardinalityRule{prior: [1,2,3,4,5,6,7,8],max: 1, min: 1});
        //let rule_set: RuleSet = RuleSet{rules: vec![rule1], qf: filter};
        let mut qf = Filter::new(100, 0.01)
            .expect("Failed to create filter");
        qf.insert_event(VERIFIABLE_PROCESSING_ID).unwrap();
        
        let rules1: Vec<Rule> = vec![Rule::Precedence(PrecedenceRule {
        current: VERIFIABLE_PROCESSING_ID,
        preceeding: VERIFIABLE_PROCESSING_ID,
        })];

        let mut cm: ConformanceMetadata = ConformanceMetadata {
            previous_image_id: VERIFIABLE_PROCESSING_ID,
            current_image_id: VERIFIABLE_PROCESSING_ID,
            rules: rules1,
            qf: qf,
        };

        let receipt1 = prove_method(1.0, 2.0, Operation::Add, &cm);
        //&receipt1.verify(cm.current_image_id).unwrap();
        let result_json = receipt1.journal.decode::<(String)>().unwrap();
        let metadata_json = receipt1.journal.decode::<(String)>().unwrap();
        println!("Result: {}", result_json);
        println!("Metadata: {}", metadata_json);

        let receipts: Vec<Receipt> = vec![receipt1]; //, receipt2, receipt3, receipt4];//, receipt3, receipt4];
        println!("Receipt vector created");
        let composite_receipt = perform_composite_prove(receipts, VERIFIABLE_PROCESSING_ID)
            .expect("Failed to prove composite receipt");
        // TODO: Implement code for retrieving receipt journal here.

        // The receipt was verified at the end of proving, but the below code is an
        // example of how someone else could verify this receipt.
        println!("Composite receipt created");
        composite_receipt.verify(COMPOSITE_PROVING_ID).unwrap();
    }
}