// Cli app entry point
use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, author)]

struct Args {
    /// Destination hostname or ip address
    #[arg(required = true)]
    destination: String,
    /// Stop after <count> replies
    #[arg(short, long, value_parser = clap::value_parser!(u64).range(1..))]
    count: u64,
    /// Follow redirects
    #[arg(short, long)]
    redirects: bool,
}

fn main() {
    let args = Args::parse();
    //println!("Got the filename {}", args.file);
    dbg!(args);

}
