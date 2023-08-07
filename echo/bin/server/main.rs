use clap::Parser;
use echo_proto::echo::proto::{
    echo_server::{Echo, EchoServer},
    EchoRequest, EchoResponse,
};
use tonic::{transport::Server, Request, Response, Status};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { address } = Args::parse();

    let echo_srv = EchoServer::new(MyEcho);
    let (_, health_srv) = tonic_health::server::health_reporter();
    let reflection_srv = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(ECHO_PROTO_DESCRIPTOR)
        .register_encoded_file_descriptor_set(tonic_health::pb::FILE_DESCRIPTOR_SET)
        .build()?;

    println!("server listening on {}", address);
    Server::builder()
        .add_service(echo_srv)
        .add_service(health_srv)
        .add_service(reflection_srv)
        .serve(address.parse()?)
        .await?;

    Ok(())
}

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "[::1]:50051")]
    address: String,
}

pub struct MyEcho;

#[tonic::async_trait]
impl Echo for MyEcho {
    async fn unary_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        println!("handling request: {request:?}");
        Ok(Response::new(EchoResponse {
            payload: request.into_inner().payload,
        }))
    }
}

const ECHO_PROTO_DESCRIPTOR: &[u8] = include_bytes!(env!("ECHO_PROTO_DESCRIPTOR_LOCATION"));
