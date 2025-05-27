//use grpc_streaming::{
pub mod streaming_helpers;
use tonic::{Request, Response, Status};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use std::pin::Pin;

pub mod filestream {
    tonic::include_proto!("filestream");
}

pub use filestream::file_streaming_service_server::{
    FileStreamingService,
};
use filestream::{
    FileRequest,
    FileChunk,
};
//use filestream::file_stream_service_server::{FileStreamService, FileStreamServiceServer};
//use filestream::{FileRequest, FileChunk};

#[derive(Default)]
pub struct MyFileStreamService {}

#[tonic::async_trait]
impl FileStreamingService for MyFileStreamService {
    type StreamFileStream = Pin<Box<dyn futures_core::Stream<Item = Result<FileChunk, Status>> + Send>>;

    async fn stream_file(
        &self,
        request: Request<FileRequest>,
    ) -> Result<Response<Self::StreamFileStream>, Status> {
        let mut filename = request.into_inner().file_name;
        filename = "proofs/".to_string() + &filename; // Ensure the file is in the 'proofs' directory
        let (tx, rx) = mpsc::channel(10);

        tokio::spawn(async move {
            match File::open(&filename).await {
                Ok(file) => {
                    let mut reader = BufReader::new(file);
                    let mut buffer = [0u8; 4096]; // 4KB chunks

                    while let Ok(bytes_read) = reader.read(&mut buffer).await {
                        if bytes_read == 0 {
                            break;
                        }

                        let chunk = FileChunk {
                            chunk: buffer[..bytes_read].to_vec(),
                        };

                        if tx.send(Ok(chunk)).await.is_err() {
                            break;
                        }
                    }
                }
                Err(_) => {
                    let _ = tx.send(Err(Status::not_found("File not found"))).await;
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn upload_file(
        &self,
        request: Request<tonic::Streaming<FileChunk>>,
    ) -> Result<Response<FileRequest>, Status> {
        use tokio::io::AsyncWriteExt;
        use sha2::{Sha256, Digest};

        let mut stream = request.into_inner();
        let mut buffer: Vec<u8> = Vec::new();

        while let Some(chunk_result) = stream.message().await.transpose() {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(e) => return Err(Status::internal(format!("Stream error: {}", e))),
            };
            buffer.extend_from_slice(&chunk.chunk);
        }

        // Compute SHA256 hash of the buffer
        let mut hasher = Sha256::new();
        hasher.update(&buffer);
        let hash = hasher.finalize();
        let directory = "proofs/";
        let filename = format!("{:x}.receipt", hash);
        let full_path = format!("{}{}", &directory, &filename);

        // Write buffer to file
        let mut file = match File::create(&full_path).await {
            Ok(f) => f,
            Err(e) => return Err(Status::internal(format!("Failed to create file: {}", e))),
        };
        if let Err(e) = file.write_all(&buffer).await {
            return Err(Status::internal(format!("Failed to write to file: {}", e)));
        }

        Ok(Response::new(FileRequest {
            file_name: filename,
        }))
    }
}