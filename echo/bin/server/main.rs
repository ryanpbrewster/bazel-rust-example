use clap::Parser;
use echo_proto::echo::proto::{
    EchoRequest, EchoResponse,
    echo_server::{Echo, EchoServer},
};
use tonic::{transport::Server, Request, Response, Status};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { address } = Args::parse();

    println!("server listening on {}", address);
    Server::builder()
        .add_service(EchoServer::new(MyEcho))
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
