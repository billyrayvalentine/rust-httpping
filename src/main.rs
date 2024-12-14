#![allow(unused_imports, unused_variables)]

// Cli app entry point
use clap::Parser;
use std::process;
use ureq::{Error, OrAnyStatus, Response};
use url::Url;
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

    //println!("Got scheme {}", parsed_url.scheme());
    match parsed_url.scheme() {
        "http" | "https" => Ok(()),
        _ => Err("Scheme must be http or https".to_string()),
    }
}

fn do_ping(url: &str) -> Result<(u16, String), ureq::Error> {
    let resp = ureq::request("HEAD", url).call().or_any_status()?;
    //dbg!(&resp);
    let resp_output = (resp.status(), resp.get_url().to_string());
    //dbg!(&resp_output);
    Ok(resp_output)
}

fn main() {
    let args = Args::parse();
    //dbg!(&args);
    let ping_forever = args.count.is_none();

    let url = validate_url_string_error(&args.destination).unwrap_or_else(|error| {
        eprintln!("Error parsing destination: {error:?}");
        process::exit(1);
    });

    let mut keep_pinging = true;
    let mut ping_counter = 1;

    while keep_pinging {
        match do_ping(&args.destination) {
            Err(e) => {
                eprintln!("Got error {}", e);
                process::exit(1);
            }
            Ok(resp) => {
                println!("{} from {} seq={}", resp.0, resp.1, ping_counter);
            }
        }

        if !ping_forever && ping_counter == args.count.unwrap() {
            keep_pinging = false;
        } else {
            ping_counter += 1;
        }
    }
    process::exit(0);
}
