use clap::{App, Arg, ArgMatches};
use std::{env, env::args};

mod commands;
mod directory;
mod script;
#[cfg(test)]
mod tests;

fn directory_arg() -> clap::Arg<'static> {
    Arg::new("directory")
        .short('d')
        .long("directory")
        .about("Directory of scripts")
        .takes_value(true)
        .required(true)
        .value_hint(clap::ValueHint::DirPath)
}

fn files_or_directory_arg() -> clap::Arg<'static> {
    Arg::new("files_or_directory")
        .multiple(true)
        .value_hint(clap::ValueHint::AnyPath)
}

fn function_name_arg() -> clap::Arg<'static> {
    Arg::new("function_name")
        .index(1)
        .about("Function name")
        .required(true)
}

fn init_directory_arg() -> clap::Arg<'static> {
    Arg::new("directory")
        .index(2)
        .about("Directory of scripts")
        .takes_value(true)
        .required(true)
        .value_hint(clap::ValueHint::DirPath)
}

fn shell_arg() -> clap::Arg<'static> {
    Arg::new("shell")
        .index(3)
        .about("Shell for init")
        .required(true)
}

fn config() -> App<'static> {
    return App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("v")
                .short('v')
                .long("verbose")
                .multiple(true)
                .about("Sets the level of verbosity"),
        )
        .subcommand(
            App::new("help")
                .about("Print help information")
            .arg(directory_arg()),
        )
        .subcommand(
            App::new("tome")
                .about("Print help information")
            .arg(directory_arg()),
        )
        .subcommand(
            App::new("commands")
                .about("List available scripts")
            .arg(directory_arg())
        )
        .subcommand(
            App::new("init")
                .about("Print shell completion")
                .arg(function_name_arg())
                .arg(init_directory_arg())
                .arg(shell_arg()))
        .subcommand(
            App::new("init_v2")
                .about("Print shell completion")
                .arg(function_name_arg())
                .arg(init_directory_arg())
                .arg(shell_arg()))
        .subcommand(
            App::new("exec")
                .about("Excute script")
                .arg(directory_arg())
                .arg(files_or_directory_arg()))
        .subcommand(
            App::new("complete")
                .about("Output commandline autocompletion results")
                .arg(directory_arg())
                .arg(files_or_directory_arg()))
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
    let tome_s = tome.to_str().unwrap().to_string();

    match app.subcommand() {
        Some(("init", sub_m)) => {
            commands::init(tome.to_str().unwrap(), args.iter().peekable(), sub_m)
        }
        Some(("init_v2", sub_m)) => commands::init_v2(tome_s, config(), sub_m),
        Some(("commands", sub_m)) => {
            // Unwrap/rewrap here due to lack of familiarity with Rust types for Result
            // ie converting io::Result<String> -> Result<String, String>
            Ok(commands::help(sub_m.value_of("directory").unwrap()).unwrap())
        }
        Some(("exec", sub_m)) => {
            log::debug!("Subcommand: {:#?}", sub_m);
            let config = commands::Config {
                executable: tome_s,
                args: app.clone(),
                directory: sub_m.value_of("directory").unwrap().to_string(),
                paths: extract_positionals(sub_m, "files_or_directory"),
            };
            commands::execute(config)
        }
        Some(("complete", sub_m)) => {
            log::debug!("Subcommand: {:#?}", sub_m);
            let config = commands::Config {
                executable: tome_s,
                args: app.clone(),
                directory: sub_m.value_of("directory").unwrap().to_string(),
                paths: extract_positionals(sub_m, "files_or_directory"),
            };
            commands::complete(config)
        }
        _ => {
            // TODO: rework this to capture stdout
            config().print_help().unwrap_or_default();
            Ok("".to_string())
        }
    }
}

fn extract_positionals(app: &ArgMatches, name: &str) -> Vec<String> {
    app.values_of_t(&name).unwrap_or_default()
}
