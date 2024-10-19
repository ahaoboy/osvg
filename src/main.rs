use clap::Parser;
use osvg::osvg;
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
    output: String,
}

fn main() {
    let Args { input, output } = Args::parse();
    let svg = std::fs::read_to_string(input).unwrap();
    let s = osvg(&svg, None).unwrap();
    std::fs::write(output, s).unwrap();
}
