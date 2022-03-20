use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Structured data source
    #[clap(short, long)]
    source: String,
    /// The opration to the data
    #[clap(short, long, default_value = "read")]
    opration: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.source);
}
