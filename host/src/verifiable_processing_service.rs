pub mod call_methods;

use super::proof_streaming_service::streaming_helpers::{
    download_proof,
    upload_proof,
};

pub use call_methods::proto::verifiable_processing_service_server::{
    VerifiableProcessingService, VerifiableProcessingServiceServer,
};
use call_methods::proto::{
    CombinedRequest, CompositionRequest, CompositionResponse, Proof, ProveRequest, ProveResponse,
    VerifyRequest, VerifyResponse,
};
use call_methods::{combined_method, compose_method, prove_method};



use risc0_zkvm::Receipt;
use tonic::{Request, Response, Status};
use methods::{COMBINED_ID, COMPOSE_ID, PROVE_ID}; //,perform_composite_prove};

#[derive(Default)]
pub struct VerifiableProcessingServiceServerImplementation;


#[tonic::async_trait]
impl VerifiableProcessingService for VerifiableProcessingServiceServerImplementation {
    async fn prove(
        &self,
        request: Request<ProveRequest>,
    ) -> Result<Response<ProveResponse>, Status> {
        println!("Got a proving request");

        let request = request.into_inner();
        let prev_proof: Option<Proof> = download_proof(request.previous_proof).await;
        println!("Into Inner");
        let receipt: Receipt =
            prove_method(request.method_payload, prev_proof, PROVE_ID);
        println!("Prove Method Done");
        let (result_json, _metadata_json): (String, String) = receipt.journal.decode().unwrap();
        let mut proof_result = Proof {
                image_id: PROVE_ID.to_vec(),
                receipt: bincode::serialize(&receipt).unwrap(),
            };
        println!("Build Proof Result");
        let proof_file = upload_proof(proof_result.clone()).await.ok().unwrap();
        println!("{:?}", proof_file);
        proof_result.receipt = bincode::serialize(&proof_file).unwrap();
        let reply = ProveResponse {
            public_output: result_json,
            proof_response: Some(proof_result),
        };
        return Ok(Response::new(reply));
    }

    async fn combined(
        &self,
        request: Request<CombinedRequest>,
    ) -> Result<Response<ProveResponse>, Status> {
        println!("Got a combined request");

        let request = request.into_inner();
        let receipt: Receipt = combined_method(request.method_payload);
        let (result_json, _metadata_json): (String, String) = receipt.journal.decode().unwrap();
        let reply = ProveResponse {
            public_output: result_json,
            proof_response: Some(Proof {
                image_id: COMBINED_ID.to_vec(),
                receipt: bincode::serialize(&receipt).unwrap(),
            }),
        };
        return Ok(Response::new(reply));
    }

    async fn compose(
        &self,
        request: Request<CompositionRequest>,
    ) -> Result<Response<CompositionResponse>, Status> {
        println!("Got a composition request");
        let request = request.into_inner();
        let proofs: Vec<Proof> = request.proof_chain;

        let mut prev = proofs[0].clone();
        let mut result: Receipt = bincode::deserialize(&proofs[0].receipt).unwrap();
        for current in &proofs[1..] {
            result = compose_method(&prev, current);
            prev = Proof {
                image_id: COMPOSE_ID.to_vec(),
                receipt: bincode::serialize(&result).unwrap(),
            };
        }

        let reply = CompositionResponse {
            proof_response: Some(Proof {
                image_id: COMPOSE_ID.to_vec(),
                receipt: bincode::serialize(&result).unwrap(),
            }),
            proof_chain: vec![],
        };
        Ok(Response::new(reply))
    }

    async fn verify(
        &self,
        request: Request<VerifyRequest>,
    ) -> Result<Response<VerifyResponse>, Status> {
        println!("Got a verification request");
        let request = request.into_inner();
        let proof = request
            .proof
            .ok_or(Status::invalid_argument("Missing proof"))?;
        let image_id: [u32; 8] = proof
            .image_id
            .try_into()
            .expect("Failed to convert Vec<u32> to [u32; 8]");
        let receipt: Receipt = bincode::deserialize(&proof.receipt).unwrap();

        let verification_result = receipt.verify(image_id);
        let public_data = receipt.journal.decode().unwrap();

        let reply: VerifyResponse;
        match verification_result {
            Ok(_) => {
                reply = VerifyResponse {
                    is_valid_executed: true,
                    public_output: public_data,
                };
            }
            Err(err) => {
                reply = VerifyResponse {
                    is_valid_executed: false,
                    public_output: public_data,
                };
                println!("{:?}", err)
            }
        }
        Ok(Response::new(reply))
    }
}