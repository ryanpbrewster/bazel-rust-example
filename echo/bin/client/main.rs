use clap::Parser;
use echo_proto::echo::proto::{
    EchoRequest,
    echo_client::EchoClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { target } = Args::parse();

    println!("connecting to server @ {target}...");
    let mut client = EchoClient::connect(target).await?;

    println!("sending request");
    let request = tonic::Request::new(EchoRequest {
        payload: "bazel-rust-example sample message payload".into(),
        ..Default::default()
    });
    let response = client.unary_echo(request).await?;
    println!("received response: {response:?}");

    Ok(())
}

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "http://[::1]:50051")]
    target: String,
}
