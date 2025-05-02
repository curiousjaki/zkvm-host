use std::collections::HashMap;
use rules::event_filter::InsertEvent;
use operations::{Operation, OperationRequest};
use poam_helper::VerificationMetadata;
use rules::conformance::{PoamInput, PoamMetadata, RuleInput};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use methods::{
    COMPOSE_ELF, COMPOSE_ID, PROVE_ELF, PROVE_ID, COMBINED_ELF, COMBINED_ID
};
use serde_json::{from_str, Value};
use once_cell::sync::Lazy;
pub mod proto {
    tonic::include_proto!("poam");
}
use proto::Proof;

static ELF_MAP: Lazy<HashMap<[u32; 8], &[u8]>> = Lazy::new(|| {
    HashMap::from([
        (PROVE_ID, PROVE_ELF),
        (COMPOSE_ID, COMPOSE_ELF)
    ])
});

pub fn prove_method(method_payload: String, previous_proof: Option<Proof>, image_id: [u32; 8],) -> Receipt {
    let mut env_builder = ExecutorEnv::builder();
    let verification_metadata: Option<VerificationMetadata> = match previous_proof {
        Some(proof) => {
            let previous_receipt: Receipt = bincode::deserialize(&proof.receipt).unwrap();
            let metadata = VerificationMetadata {
                image_id: proof.image_id.try_into().expect("Expected image_id to be a [u32; 8]"),
                journal_data: previous_receipt.journal.decode().unwrap(),
            };
            env_builder.add_assumption(previous_receipt);
            Some(metadata)
        }
        None => None,
    };

    let env = env_builder
        .write(&method_payload)
        .unwrap()
        .write(&serde_json::to_string(&verification_metadata).unwrap())
        .unwrap()
        .build()
        .unwrap();
    // read the input
    let elf = ELF_MAP.get(&image_id).unwrap();
    let prover = default_prover();
    let prove_info = prover.prove(env, elf).unwrap();
    return prove_info.receipt;
}


pub fn compose_method(p1: &Proof, p2: &Proof) -> Receipt{
    let p1_receipt: Receipt = bincode::deserialize(&p1.receipt).unwrap();
    let decoded_p1: (String, String) = p1_receipt.journal.decode().unwrap();
    let ser_po1: String = serde_json::to_string(&decoded_p1).unwrap();
    let p2_receipt: Receipt = bincode::deserialize(&p2.receipt).unwrap();
    let decoded_p2: (String, String) = p2_receipt.journal.decode().unwrap();
    let ser_po2: String = serde_json::to_string(&decoded_p2).unwrap();
    let mut env_builder = ExecutorEnv::builder();
    env_builder.add_assumption(p1_receipt);
    env_builder.add_assumption(p2_receipt);
    println!("Compose Method");
    let env = env_builder
        .write(&serde_json::to_string(&COMPOSE_ID.to_vec()).unwrap())
        .unwrap()
        .write(&ser_po1)
        .unwrap()
        .write(&ser_po2)
        .unwrap()
        .build()
        .unwrap();
    let composer = default_prover();
    let composition = composer.prove(env, COMPOSE_ELF).unwrap();
    return composition.receipt;
}

pub fn combined_method(method_payload: &String) -> Receipt{
    //println!("{:?}",method_payload);
    let json_value: Value = serde_json::from_str(&method_payload).expect("Failed to parse JSON");
    //println!("{:?}",json_value);
    let mut operation_requests: Vec<OperationRequest> = Vec::new();
    if let Value::Object(map) = json_value {
        for (key, value) in &map {
            if let Value::Object(inner_map) = value {
                let operation : Operation = from_str(&inner_map["operation"].to_string()).unwrap();
                let operation_request = OperationRequest {
                    a: inner_map["a"].as_f64().unwrap(),
                    b: inner_map["b"].as_f64().unwrap(),
                    operation: operation
                };
                operation_requests.push(operation_request);
            }
        }
    }
    //println!("{:?}",&operation_requests);

    let mut env_builder = ExecutorEnv::builder();
    let env = env_builder
        .write(&serde_json::to_string(&COMBINED_ID.to_vec()).unwrap())
        .unwrap()
        .write(&serde_json::to_string(&operation_requests).unwrap())
        .unwrap()
        .build()
        .unwrap();
    let combiner = default_prover();
    let composition = combiner.prove(env, COMBINED_ELF).unwrap();
    return composition.receipt;
}

#[cfg(test)]
mod tests {
    use operations::OperationRequest;

    use super::*;

   //RISC0_DEV_MODE=0 RUST_LOG=info cargo test --release -- --nocapture
    #[test]
    fn test_proving_method(){
        println!("Starting the Program");
        println!("Prove ID: {:?}",PROVE_ID);
        //env_logger::init();
        // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`

        //tracing_subscriber::fmt()
        //    .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        //    .init();
        //let filter = qfilter::Filter::new(1000, 0.01).expect("Failed to create filter");
        //let rule1 = Rule::Cardinality(CardinalityRule{prior: [1,2,3,4,5,6,7,8],max: 1, min: 1});
        //let rule_set: RuleSet = RuleSet{rules: vec![rule1], qf: filter};
        //let mut qf = Filter::new(100, 0.01)
        //    .expect("Failed to create filter");
        //qf.insert_event(VERIFIABLE_PROCESSING_ID).unwrap();
        
        let rules1: Vec<Rule> = vec![Rule::Precedence(PrecedenceRule {
        //current: VERIFIABLE_PROCESSING_ID,
        preceeding: PROVE_ID,
        })];

        let method_payload1 = serde_json::to_string(
            &OperationRequest{a: 1.0, b: 2.0, operation: Operation::Add })
            .unwrap();
        println!("Method Payload: {}",method_payload1);

        let pi1: PoamInput = PoamInput {
            image_id: PROVE_ID,
            rule_input: RuleInput {
                //current_image_id: VERIFIABLE_PROCESSING_ID,
                rules: None,
                ordering_rules: None,
            },
            public_data: None,
        };

        let receipt1 = prove_method(
            &method_payload1,
            &pi1,None);
        //&receipt1.verify(cm.current_image_id).unwrap();
        let (result_json,metadata_json):(String,String) = receipt1.journal.decode().unwrap();
        println!("Result: {}, Metadata: {}",result_json, metadata_json);

        let pi2: PoamInput = PoamInput {
            image_id: PROVE_ID,
            rule_input: RuleInput {
                //current_image_id: VERIFIABLE_PROCESSING_ID,
                rules: Some(rules1),
                ordering_rules: None,
            },
            public_data: Some((result_json,metadata_json)),
        };
        let receipt2 = prove_method(
            &method_payload1,
            &pi2,Some(receipt1));
        //&receipt1.verify(cm.current_image_id).unwrap();
        let (result_json2,metadata_json2):(String,String) = receipt2.journal.decode().unwrap();
        println!("Result: {}, Metadata: {}",result_json2, metadata_json2);

        //let receipts: Vec<Receipt> = vec![receipt1]; //, receipt2, receipt3, receipt4];//, receipt3, receipt4];
        //println!("Receipt vector created");
        //let composite_receipt = perform_composite_prove(receipts, VERIFIABLE_PROCESSING_ID)
        //    .expect("Failed to prove composite receipt");
        // TODO: Implement code for retrieving receipt journal here.

        // The receipt was verified at the end of proving, but the below code is an
        // example of how someone else could verify this receipt.
        //println!("Composite receipt created");
        //composite_receipt.verify(COMPOSITE_PROVING_ID).unwrap();
    }
}