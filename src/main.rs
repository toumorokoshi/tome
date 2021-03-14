use std::env::args;
use clap;
use clap::{Arg, App, ArgMatches};

mod commands;
mod directory;
mod script;
#[cfg(test)]
mod tests;

fn config(args: Vec<String>) -> ArgMatches {
    return App::new("Tome")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Modern implementation of sub")
        .arg(Arg::new("help")
            .short('h')
            .long("help")
            .about("Print help information")
            .takes_value(false))
        .arg(Arg::new("v")
            .short('v')
            .long("verbose")
            .multiple(true)
            .about("Sets the level of verbosity"))
        .subcommand(App::new("init"))
        .subcommand(App::new("exec")
            .arg(Arg::new("DIRECTORY")
                .short('d')
                .long("directory")
                .about("Directory of scripts")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("file_or_directory").multiple(true))
        )
        .subcommand(App::new("complete")
            .arg(Arg::new("DIRECTORY")
                .short('d')
                .long("directory")
                .about("Directory of scripts")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("file_or_directory").multiple(true))
            )
        .get_matches_from(args);
}

pub fn main() {
    let args: Vec<String> = args().peekable().collect();
    match execute(args) {
        Ok(result) => print!("{}", result),
        Err(error_message) => print!("echo {}", error_message),
    }
}

pub fn execute(args: Vec<String>) -> Result<String, String> {
    let app = config(args.clone());

    println!("{:#?}", app);
    let tome_executable = match args.first() {
        Some(arg) => arg,
        None => "",
    };
    match app.subcommand() {
        Some(("init",  _sub_m)) => {
          return commands::init(tome_executable, args.iter().peekable(), app);
        }, // clone was used
        Some(("exec",   sub_m)) => {
            return commands::execute(extract_positionals(sub_m, "files_or_directory"));
        },
        Some(("complete", sub_m)) => {
            return commands::complete(extract_positionals(sub_m, "files_or_directory"));
        },
        _                       => {
            return Err("Not implemented".to_string());
        // TODO: return commands::help?
        },
    };
}

fn extract_positionals(app: &ArgMatches, name: &str) -> Vec<String> {
    app.values_of_t(&name).unwrap_or_else(|e| e.exit() )
}