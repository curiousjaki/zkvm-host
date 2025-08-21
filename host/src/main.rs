use host::proof_streaming_service::filestream::file_streaming_service_server::FileStreamingServiceServer;
pub use host::proof_streaming_service::{MyFileStreamService};
pub use host::verifiable_processing_service::{
    VerifiableProcessingServiceServerImplementation, VerifiableProcessingServiceServer,
};
use methods::{PROVE_ID};
use tonic::{transport::Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", PROVE_ID);

    //let addr: std::net::SocketAddr = "[::1]:50051".parse()?;
    let vpssi: VerifiableProcessingServiceServerImplementation =
        VerifiableProcessingServiceServerImplementation::default();

    println!(
        "VerifiableProcessingService listening on {}",
        "0.0.0.0:50051"
    );

    let filestramssi: MyFileStreamService =
        MyFileStreamService::default();

    Server::builder()
        .add_service(VerifiableProcessingServiceServer::new(vpssi))
        .add_service(FileStreamingServiceServer::new(filestramssi))
        .serve(([0, 0, 0, 0], 50051).into())
        .await?;

    Ok(())
}
