#![allow(unused_imports, unused_variables)]

// Cli app entry point
use clap::Parser;
use std::process;
use ureq::{Error, OrAnyStatus, Response};
use url::Url;
//use std::fmt::Error;
#[derive(Parser, Debug)]
#[command(version, about, author)]

struct Args {
    /// Destination hostname or ip address
    #[arg(required = true)]
    destination: String,
    /// Stop after <count> replies
    #[arg(short, long, value_parser = clap::value_parser!(u64).range(1..))]
    count: Option<u64>,
    /// Follow redirects
    #[arg(short, long)]
    redirects: bool,
}

// test destination is parsable by Url::parse() and is http(s)
fn validate_url_string_error(url: &str) -> Result<(), String> {
    let parsed_url = Url::parse(url).map_err(|e| e.to_string())?;

    println!("Got scheme {}", parsed_url.scheme());
    match parsed_url.scheme() {
        "http" | "https" => Ok(()),
        _ => Err("Scheme must be http or https".to_string()),
    }
}

fn do_ping(url: &str) -> Result<ureq::Response, ureq::Error> {
    let resp = ureq::request("HEAD", &url).call().or_any_status()?;
    //dbg!(&resp);
    Ok(resp)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    //dbg!(&args);

    let url = validate_url_string_error(&args.destination).unwrap_or_else(|error| {
        eprintln!("Error parsing destination: {error:?}");
        process::exit(1);
    });

    if let Err(e) = do_ping(&args.destination) {
        eprintln!("Got error {}", e);
        process::exit(1)
    }
    Ok(())
}
