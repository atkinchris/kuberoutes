#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate ansi_term;
extern crate subprocess;
extern crate text_io;

mod errors;
mod kubectl;
mod policies;

use ansi_term::Colour::Red;
use clap::Arg;
use kubectl::get_network_policies;

fn app() -> Result<(), Box<std::error::Error>> {
    let matches = app_from_crate!()
        .arg(Arg::from_usage(
            "-l, --labels [LABELS] 'labels to match against network policy selector'",
        )).arg(Arg::from_usage(
            "-d, --default-deny 'pods are isolated by default'",
        )).arg(Arg::from_usage(
            "-n, --namespace 'namespace to query the network policies of'",
        )).get_matches();

    let labels = matches.value_of("labels");
    let policies = get_network_policies(labels)?;

    println!("{:#?}", policies);

    return Ok(());
}

fn main() {
    if let Err(err) = app() {
        eprintln!("\n{}", Red.paint(err.description()));
        std::process::exit(1);
    }
}
