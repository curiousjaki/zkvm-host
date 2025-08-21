//use super::super::proof_streaming_service::filestream::{
//    file_streaming_service_client::FileStreamingServiceClient,
//    FileRequest,
//    FileChunk};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, AsyncReadExt, BufReader};
use tokio::sync::mpsc;
use tonic::{Status, Request};

use super::super::verifiable_processing_service::call_methods::proto::{
    CombinedRequest, CompositionRequest, CompositionResponse, Proof, ProveRequest, ProveResponse,
    VerifyRequest, VerifyResponse,
};
use super::filestream::{
    file_streaming_service_client::FileStreamingServiceClient,
    FileRequest,
    FileChunk,
};

//use super::super::verifiable_processing_service::call_methods::proto::{
//    CombinedRequest, CompositionRequest, CompositionResponse, Proof, ProveRequest, ProveResponse,
//    VerifyRequest, VerifyResponse,
//};

async fn download_file(client: &mut FileStreamingServiceClient<tonic::transport::Channel>,filename: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use sha2::{Sha256, Digest};

    let request = FileRequest {
        file_name: filename.to_string(),
    };

    let mut stream = client.stream_file(request).await?.into_inner();
    let mut buffer: Vec<u8> = Vec::new();

    //println!("Receiving file...");

    while let Some(chunk) = stream.message().await? {
        buffer.extend_from_slice(&chunk.chunk);
    }

    // Compute SHA256 hash of the buffer
    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let hash = hasher.finalize();
    let download_path = format!("proofs/{:x}.receipt", hash);

    let mut file = File::create(&download_path).await?;
    file.write_all(&buffer).await?;

    Ok(buffer)
}

pub async fn download_proof(previous_proof: Option<Proof>) -> Option<Proof> {
    if let Some(proof) = previous_proof {
        let filename: String = bincode::deserialize(&proof.receipt).unwrap();
        //let filename = String::from_utf8(proof.receipt).ok()?;
        let mut client = FileStreamingServiceClient::connect("http://[::1]:50052").await.ok()?;
        match download_file(&mut client, &filename).await {
            Ok(buffer) => {
                // Assuming Receipt is defined and bincode is imported
                // let receipt: Receipt = bincode::deserialize(&buffer).unwrap();
                Some(Proof {
                    image_id: proof.image_id,
                    receipt: buffer,
                })
            },
            Err(_) => {
                println!("Failed to download proof");
                None
            }
        }
    } else {
        println!("The required proof was not provided.");
        None
    }
}

async fn upload_file(client: &mut FileStreamingServiceClient<tonic::transport::Channel>, buffer: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    use tokio_stream::wrappers::ReceiverStream;

    let (tx, rx) = mpsc::channel(1);

    // Spawn a task to send the buffer in chunks
    tokio::spawn(async move {
        let mut offset = 0;
        let chunk_size = 4096;
        while offset < buffer.len() {
            let end = std::cmp::min(offset + chunk_size, buffer.len());
            let chunk = FileChunk {
                chunk: buffer[offset..end].to_vec(),
            };
            if tx.send(chunk).await.is_err() {
                break;
            }
            offset = end;
        }
        // Drop tx to close the stream
    });

    let response = client.upload_file(Request::new(ReceiverStream::new(rx))).await?;
    Ok(response.into_inner().file_name)
}


pub async fn upload_proof(proof: Proof) -> Result<String, Box<dyn std::error::Error>> {
    let mut client = FileStreamingServiceClient::connect("http://[::1]:50052").await?;
    let file_name =  upload_file(&mut client, proof.receipt.clone()).await?;    
    Ok(file_name)
}