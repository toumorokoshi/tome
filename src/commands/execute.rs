use std::{
    iter::Iterator,
    //iter::Peekable,
    //slice::Iter,
    path::PathBuf,
    io,
    fs,
};
//use clap::ArgMatches;
use super::{TargetType, CommandType, help, Script};
use super::super::script;
use super::super::directory;

pub fn execute(raw_args: Vec<String>) -> Result<String, String> {
    let mut arguments = raw_args.iter().peekable();
    // the first argument should be location of the tome binary.
    let first_arg = match arguments.next() {
        Some(arg) => arg,
        None => return Err(String::from("at least one argument expected")),
    };
    let mut target = PathBuf::from(first_arg);
    // next, we determine if we have a file or a directory,
    // recursing down arguments until we've exhausted arguments
    // that match a directory or file.
    let mut target_type = TargetType::Directory;
    let mut first_arg = true;
    let mut command_type = CommandType::Execute;
    // if no argument is passed, return help.
    if arguments.peek().is_none() {
        match help(target.to_str().unwrap_or_default(), arguments) {
            Ok(message) => return Ok(message),
            Err(io_error) => return Err(format!("{}", io_error)),
        }
    }
    while let Some(arg) = arguments.peek() {
        // match against builtin commands
        if first_arg {
            match arg.as_ref() {
                "--help" => {
                    arguments.next();
                    match help(target.to_str().unwrap_or_default(), arguments) {
                        Ok(message) => return Ok(message),
                        Err(io_error) => return Err(format!("{}", io_error)),
                    }
                }
                "--complete" => {
                    arguments.next();
                    command_type = CommandType::Completion;
                    continue;
                }
                _ => {}
            }
        }
        first_arg = false;
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
    let output: String = match target_type {
        TargetType::Directory => match command_type {
            CommandType::Completion => {
                let mut result = vec![];
                let paths_raw: io::Result<_> = fs::read_dir(target.to_str().unwrap_or(""));
                // TODO(zph) deftly fix panics when this code path is triggered with empty string: ie sc dir_example bar<TAB>
                // current implementation avoids the panic but is crude.
                let mut paths: Vec<_> = match paths_raw {
                    Err(_a) => return Err("Invalid argument to completion".to_string()),
                    Ok(a) => a
                }
                .map(|r| r.unwrap())
                .collect();
                paths.sort_by_key(|f| f.path());
                for path_buf in paths {
                    let path = path_buf.path();
                    if path.is_dir() && !directory::is_tome_script_directory(&path) {
                        continue;
                    }
                    if path.is_file()
                        && !script::is_tome_script(
                            path_buf.file_name().to_str().unwrap_or_default(),
                        )
                    {
                        continue;
                    }
                    result.push(path.file_name().unwrap().to_str().unwrap_or("").to_owned());
                }
                result.join(" ")
            }
            CommandType::Execute => {
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
        },
        TargetType::File => match Script::load(&target.to_str().unwrap_or_default()) {
            Ok(script) => script.get_execution_body(command_type, &remaining_args)?,
            Err(error) => return Err(format!("IOError loading file: {:?}", error)),
        },
    };
    Ok(output)
}