use clap;
use clap::{App, Arg, ArgMatches};
use std::{env, env::args};

mod commands;
mod directory;
mod script;
#[cfg(test)]
mod tests;

fn config() -> App<'static> {
    return App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
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
                        .index(3)
                        .about("Shell for init")
                        .required(true),
                )
                .arg(
                    Arg::new("directory")
                        .index(2)
                        .about("Directory of scripts")
                        .takes_value(true)
                        .required(true)
                        .value_hint(clap::ValueHint::DirPath),
                ),
        )
        .subcommand(
            App::new("init_v2")
                .arg(
                    Arg::new("function_name")
                        .index(1)
                        .about("Function name")
                        .required(true),
                )
                .arg(
                    Arg::new("shell")
                        .index(3)
                        .about("Shell for init")
                        .required(true),
                )
                .arg(
                    Arg::new("directory")
                        .index(2)
                        .about("Directory of scripts")
                        .takes_value(true)
                        .required(true)
                        .value_hint(clap::ValueHint::DirPath),
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
                        .required(true)
                        .value_hint(clap::ValueHint::DirPath),
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
                        .required(true)
                        .value_hint(clap::ValueHint::DirPath),
                )
                .arg(Arg::new("files_or_directory")
                    .multiple(true)
                    .value_hint(clap::ValueHint::AnyPath)
                ),
        );
}

pub fn main() {
    env_logger::init();
    let args: Vec<String> = args().peekable().collect();
    match execute(args) {
        Ok(result) => println!("{}", result),
        Err(error_message) => eprintln!("{}", error_message),
    }
}

pub fn execute(args: Vec<String>) -> Result<String, String> {
    let application = config();
    let app = application.get_matches_from(args.clone());

    let tome = std::env::current_exe().unwrap().canonicalize().unwrap();
    log::debug!("Executable: tome: {:#?}", tome);
    let tome_s = tome.clone().to_str().unwrap().to_string();

    match app.subcommand() {
        Some(("init", sub_m)) => {
            return commands::init(tome.to_str().unwrap(), args.iter().peekable(), sub_m);
        }
        Some(("init_v2", sub_m)) => {
            commands::init_v2(tome_s, config(), sub_m);
            return Ok("".to_string());
        }
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
        }
    };
}

fn extract_positionals(app: &ArgMatches, name: &str) -> Vec<String> {
    app.values_of_t(&name).unwrap_or(Vec::new())
}
