mod common;
mod system_queries;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Gormless CLI")
        .version("0.1.0")
        .author("Rikk Hill <me@rikkhill.com>")
        .about("A troubleshooting tool for machine learning environments")
        .arg(
            Arg::new("greet")
                .long("greet")
                .help("Prints a greeting message")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("message")
                .long("message")
                .help("Prints a custom message")
                .action(clap::ArgAction::Set)
                .value_name("TEXT"),
        )
        .get_matches();

    if matches.get_flag("greet") {
        println!("{}", common::something_common());
    }

    if let Some(custom_message) = matches.get_one::<String>("message") {
        println!("Custom Message: {}", custom_message);
    }

    system_queries::unified::list_it()
}