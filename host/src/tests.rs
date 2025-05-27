#[cfg(test)]
mod tests {
    use operations::OperationRequest;

    use super::*;

    //RISC0_DEV_MODE=0 RUST_LOG=info cargo test --release -- --nocapture
    #[test]
    fn test_proving_method() {
        println!("Starting the Program");
        println!("Prove ID: {:?}", PROVE_ID);
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

        let method_payload1 = serde_json::to_string(&OperationRequest {
            a: 1.0,
            b: 2.0,
            operation: Operation::Add,
        })
        .unwrap();
        println!("Method Payload: {}", method_payload1);

        let pi1: PoamInput = PoamInput {
            image_id: PROVE_ID,
            rule_input: RuleInput {
                //current_image_id: VERIFIABLE_PROCESSING_ID,
                rules: None,
                ordering_rules: None,
            },
            public_data: None,
        };

        let receipt1 = prove_method(&method_payload1, &pi1, None);
        //&receipt1.verify(cm.current_image_id).unwrap();
        let (result_json, metadata_json): (String, String) = receipt1.journal.decode().unwrap();
        println!("Result: {}, Metadata: {}", result_json, metadata_json);

        let pi2: PoamInput = PoamInput {
            image_id: PROVE_ID,
            rule_input: RuleInput {
                //current_image_id: VERIFIABLE_PROCESSING_ID,
                rules: Some(rules1),
                ordering_rules: None,
            },
            public_data: Some((result_json, metadata_json)),
        };
        let receipt2 = prove_method(&method_payload1, &pi2, Some(receipt1));
        //&receipt1.verify(cm.current_image_id).unwrap();
        let (result_json2, metadata_json2): (String, String) = receipt2.journal.decode().unwrap();
        println!("Result: {}, Metadata: {}", result_json2, metadata_json2);

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
