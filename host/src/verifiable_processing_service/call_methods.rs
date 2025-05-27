use operations::{Operation, OperationRequest};
use serde_json::{from_str, Value};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use methods::{COMBINED_ELF, COMPOSE_ELF, COMPOSE_ID, PROVE_ELF, PROVE_ID};
use poam_helper::VerificationMetadata;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub mod proto {
    tonic::include_proto!("poam");
}
use proto::Proof;

static ELF_MAP: Lazy<HashMap<[u32; 8], &[u8]>> =
    Lazy::new(|| HashMap::from([(PROVE_ID, PROVE_ELF), (COMPOSE_ID, COMPOSE_ELF)]));


fn convert_proof_to_verification_metadata(proof: &Proof) -> VerificationMetadata {
    let receipt: Receipt = bincode::deserialize(&proof.receipt).unwrap();
    VerificationMetadata {
        image_id: proof
            .image_id
            .clone()
            .try_into()
            .expect("Expected image_id to be a [u32; 8]"),
        journal_data: receipt.journal.decode().unwrap(),
    }
}

pub fn prove_method(
    method_payload: String,
    previous_proof: Option<Proof>,
    image_id: [u32; 8],
) -> Receipt {
    let mut env_builder = ExecutorEnv::builder();
    let verification_metadata: Option<VerificationMetadata> = match previous_proof {
        Some(proof) => {
            let previous_receipt: Receipt = bincode::deserialize(&proof.receipt).unwrap();
            env_builder.add_assumption(previous_receipt);
            Some(convert_proof_to_verification_metadata(&proof))
        }
        None => None,
    };
    println!("Build Verification metadata");
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
    println!("Created Prover");
    let prove_info = prover.prove(env, elf).unwrap();
    println!("Proved method");
    return prove_info.receipt;
}

pub fn compose_method(p1: &Proof, p2: &Proof) -> Receipt {
    let p1_receipt: Receipt = bincode::deserialize(&p1.receipt).unwrap();
    let p1_vm: VerificationMetadata = convert_proof_to_verification_metadata(&p1);
    let p2_receipt: Receipt = bincode::deserialize(&p2.receipt).unwrap();
    let p2_vm: VerificationMetadata = convert_proof_to_verification_metadata(&p2);

    let mut env_builder = ExecutorEnv::builder();
    env_builder.add_assumption(p1_receipt);
    env_builder.add_assumption(p2_receipt);

    let env = env_builder
        //.write(&serde_json::to_string(&COMPOSE_ID.to_vec()).unwrap())
        //.unwrap()
        .write(&serde_json::to_string(&p1_vm).unwrap())
        .unwrap()
        .write(&serde_json::to_string(&p2_vm).unwrap())
        .unwrap()
        .build()
        .unwrap();

    let composer = default_prover();
    let composition = composer.prove(env, COMPOSE_ELF).unwrap();
    return composition.receipt;
}

pub fn combined_method(method_payload: Vec<String> ) -> Receipt {
    //println!("{:?}",method_payload);
    //let json_value: Value = serde_json::from_str(&method_payload).expect("Failed to parse JSON");
    //println!("{:?}",json_value);
    let mut operation_requests: Vec<OperationRequest> = Vec::new();
    //if let Value::Object(map) = json_value {
    for (_key, value) in method_payload.iter().enumerate() {
        if let Ok(Value::Object(inner_map)) = serde_json::from_str::<Value>(value) {
            let operation: Operation = from_str(&inner_map["operation"].to_string()).unwrap();
            let operation_request = OperationRequest {
                a: inner_map["a"].as_f64().unwrap(),
                b: inner_map["b"].as_f64().unwrap(),
                operation: operation,
            };
            operation_requests.push(operation_request);
        }
    }
    //println!("{:?}",&operation_requests);

    let mut env_builder = ExecutorEnv::builder();
    let env = env_builder
        //.write(&serde_json::to_string(&COMBINED_ID.to_vec()).unwrap())
        //.unwrap()
        .write(&serde_json::to_string(&operation_requests).unwrap())
        .unwrap()
        .build()
        .unwrap();
    let combiner = default_prover();
    let combined = combiner.prove(env, COMBINED_ELF).unwrap();
    return combined.receipt;
}