use clap::Parser;

fn main() {
    let Args { name } = Args::parse();
    println!("Hello, {name}!");
}

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "World")]
    name: String,
}
