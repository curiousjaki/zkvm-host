//#![no_main]
//#![no_std]
use risc0_zkvm::{
    guest::env,
    serde
};
use operations::OperationRequest;
use std::string::String;
use serde_json::from_str;
use poam_helper::VerificationMetadata;

fn verify_previous_receipt(ser_verification_metadata: String) -> (bool, f64) { //previous_verificaiton, public_data_output
    let verification_metadata: Option<VerificationMetadata> = 
    from_str(&ser_verification_metadata).unwrap();
    //eprintln!("{:?}", verification_metadata);

    match verification_metadata {
        Some(metadata) => {
            env::verify(
                metadata.image_id, 
                &serde::to_vec(&metadata.journal_data).unwrap())
                .unwrap();
            eprintln!("{:?}", "Successfully verified previous receipt");
            let value : f64 = from_str(&metadata.journal_data.0).unwrap();
            (true, value)
        }
        None => {
            // No previous receipt, so we are the first event
            eprintln!("{:?}", "Not verified previous receipt");
            (false, 0.0)
        }
    }
}

fn main() {
    //let start = env::cycle_count();

    let ser_method_payload: String = env::read();
    let ser_verification_metadata: String = env::read();

    let operation_request: OperationRequest = from_str(&ser_method_payload).unwrap();
    let (verified_previous, public_data_output): (bool, f64) = 
        verify_previous_receipt(ser_verification_metadata);
    //env::write(&verification_metadata);
    
    //let (serialized_public_data_json, serialized_metadata_json): (String, String) = env::read();

    // potentially expensive or frequently called code
    // ...
    //let input_read = env::cycle_count();
    //eprintln!("reading input: {}", input_read - start);
    


    //let string_transformation = env::cycle_count();
    //eprintln!("converting json to structs: {}", string_transformation - input_read);

    //if cci.public_data.1.len() > 0 {
    //    ccr = from_str(&cci.public_data.1).unwrap();
    //    env::verify(ccr.image_id, &serde::to_vec(&cci.public_data).unwrap()).unwrap();
    //}
    // execute the operation
    let result: f64 = operation_request.compute() + public_data_output;
    
    // Chain the result with the previous receipt
    //result = result + match &pi.public_data {
    //    Some((public_data_json, metadata_json)) => {
    //        let value : f64 = from_str(&public_data_json).unwrap();
    //        value
    //    },
    //    None => {
    //        0.0
    //    }
    //};

    //let start_serialization = env::cycle_count();
    // serialize the output to json to avouid type mismatch, especially relevant for all vectors.
    let serialized_result_json: String = serde_json::to_string(&result).unwrap();
    let serialized_metadata_json: String = serde_json::to_string(&verified_previous).unwrap();
    //let end_serialization = env::cycle_count();

    //eprintln!("serialization: {}", end_serialization - start_serialization);
    // commit public data output to the journal
    env::commit(&(serialized_result_json,serialized_metadata_json));
}
