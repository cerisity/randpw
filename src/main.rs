use clap::Parser;
use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let args = Args::parse();
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(args.length)
        .map(char::from)
        .collect();
    println!("{}", password);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 24)]
    length: usize,
}