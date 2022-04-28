use clap::Parser;

#[macro_use]
extern crate lazy_static;

pub mod cli;
pub mod commands;
pub mod directory;
pub mod script;
pub mod shell_type;
pub mod types;

#[cfg(test)]
mod shell_type_tests;

#[cfg(test)]
mod lib_tests;

pub fn execute(raw_args: Vec<String>) -> Result<String, String> {
    let mut arguments = raw_args.iter().peekable();
    // the first argument should be location of the tome binary.
    let tome_executable = match arguments.peek() {
        Some(arg) => (*arg).clone(),
        None => return Err(String::from("0th argument should be the tome binary")),
    };
    match cli::TomeArgs::try_parse_from(arguments) {
        Ok(tome_args) => match &tome_args.commands {
            cli::TomeCommands::CommandComplete(complete_args) => commands::complete::complete(
                &complete_args.command_directory_path,
                &complete_args.args,
            ),
            cli::TomeCommands::CommandExecute(execute_args) => {
                commands::execute::execute(&execute_args.command_directory_path, &execute_args.args)
            }
            cli::TomeCommands::CommandHelp(help_args) => {
                commands::help::help(&help_args.command_directory_path)
            }
            cli::TomeCommands::Init(init_args) => commands::init::init(&tome_executable, init_args),
        },
        Err(msg) => Err(msg.to_string()),
    }
}
