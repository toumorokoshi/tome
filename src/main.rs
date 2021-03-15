use clap;
use clap::{App, Arg, ArgMatches};
use std::{env, env::args, path::PathBuf};

mod commands;
mod directory;
mod script;
#[cfg(test)]
mod tests;

fn config() -> App<'static> {
    return App::new("Tome")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Modern implementation of sub")
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .about("Print help information")
                .takes_value(false),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .long("verbose")
                .multiple(true)
                .about("Sets the level of verbosity"),
        )
        .subcommand(
            App::new("init")
                .arg(
                    Arg::new("function_name")
                        .index(1)
                        .about("Function name")
                        .required(true),
                )
                .arg(
                    Arg::new("shell")
                        .index(2)
                        .about("Shell for init")
                        .required(true),
                )
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .about("Directory of scripts")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("exec")
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .about("Directory of scripts")
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::new("files_or_directory").multiple(true)),
        )
        .subcommand(
            App::new("complete")
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .about("Directory of scripts")
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::new("files_or_directory").multiple(true)),
        );
}

pub fn main() {
    env_logger::init();
    let args: Vec<String> = args().peekable().collect();
    match execute(args) {
        Ok(result) => print!("{}", result),
        Err(error_message) => print!("echo {}", error_message),
    }
}

pub fn execute(args: Vec<String>) -> Result<String, String> {
    let application = config();
    let app = application.get_matches_from(args.clone());

    let tome_executable = match args.first() {
        Some(arg) => arg,
        None => "",
    };
    let tome = PathBuf::from(tome_executable).canonicalize().unwrap();
    let tome_s = tome.clone().to_str().unwrap().to_string();

    match app.subcommand() {
        Some(("init", sub_m)) => {
            return commands::init(tome.to_str().unwrap(), args.iter().peekable(), sub_m);
        } // clone was used
        Some(("exec", sub_m)) => {
            log::debug!("Subcommand: {:#?}", sub_m);
            let config = commands::Config {
                executable: tome_s,
                args: app.clone(),
                directory: sub_m.value_of("directory").unwrap().to_string(),
                paths: extract_positionals(sub_m, "files_or_directory"),
            };
            return commands::execute(config);
        }
        Some(("complete", sub_m)) => {
            log::debug!("Subcommand: {:#?}", sub_m);
            let config = commands::Config {
                executable: tome_s,
                args: app.clone(),
                directory: sub_m.value_of("directory").unwrap().to_string(),
                paths: extract_positionals(sub_m, "files_or_directory"),
            };
            return commands::complete(config);
        }
        _ => {
            config().print_help().unwrap_or_default();
            return Ok("".to_string());
            // TODO: return commands::help?
        }
    };
}

fn extract_positionals(app: &ArgMatches, name: &str) -> Vec<String> {
    app.values_of_t(&name).unwrap_or(Vec::new())
}
