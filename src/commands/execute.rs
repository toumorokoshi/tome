use super::{help, CommandType, Config, Script, TargetType};
use std::{
    iter::Iterator,
    //iter::Peekable,
    //slice::Iter,
    path::PathBuf,
};

pub fn execute(config: Config) -> Result<String, String> {
    let mut arguments = config.paths.iter().peekable();
    let mut target = PathBuf::from(config.directory);
    // next, we determine if we have a file or a directory,
    // recursing down arguments until we've exhausted arguments
    // that match a directory or file.
    let mut target_type = TargetType::Directory;
    let command_type = CommandType::Execute;
    // if no argument is passed, return help.
    if arguments.peek().is_none() {
        match help(target.to_str().unwrap_or_default(), arguments) {
            Ok(message) => return Ok(message),
            Err(io_error) => return Err(format!("{}", io_error)),
        }
    }
    while let Some(arg) = arguments.peek() {
        target.push(arg);
        if target.is_file() {
            target_type = TargetType::File;
            arguments.next();
            break;
        } else if target.is_dir() {
            target_type = TargetType::Directory;
            arguments.next();
        } else {
            // the current argument does not match
            // a directory or a file, so we've landed
            // on the strictest match.
            target.pop();
            break;
        }
    }
    let remaining_args: Vec<_> = arguments.collect();
    log::debug!("Remaining args: {:#?}", remaining_args);
    let output: String = match target_type {
        TargetType::Directory => {
            return match remaining_args.len() {
                0 => Err(format!(
                    "{} is a directory. tab-complete to choose subcommands",
                    target.to_str().unwrap_or("")
                )),
                _ => Err(format!(
                    "command {} not found in directory {}",
                    remaining_args[0],
                    target.to_str().unwrap_or("")
                )),
            };
        }
        TargetType::File => match Script::load(&target.to_str().unwrap_or_default()) {
            Ok(script) => script.get_execution_body(command_type, &remaining_args)?,
            Err(error) => return Err(format!("IOError loading file: {:?}", error)),
        },
    };
    Ok(output)
}
