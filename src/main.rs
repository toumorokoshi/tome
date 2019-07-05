use std::{env::args, fs, path::PathBuf};

mod commands;
mod directory;
#[cfg(test)]
mod tests;


pub fn main() {
    let args: Vec<String> = args().peekable().collect();
    match execute(args) {
        Ok(result) => print!("{}", result),
        Err(error_message) => print!("echo {}", error_message),
    };
}

pub enum CommandType {
    Execute,
    Completion,
}

enum TargetType {
    File,
    Directory,
}

pub fn execute(raw_args: Vec<String>) -> Result<String, String> {
    let mut arguments = raw_args.iter().peekable();
    // the first argument should be location of the tome binary.
    let tome_executable = match arguments.next() {
        Some(arg) => arg,
        None => return Err(String::from("0th argument should be the tome binary")),
    };
    let first_arg = match arguments.next() {
        Some(arg) => arg,
        None => return Err(String::from("at least one argument expected")),
    };
    // if the first command is init, then we should print the
    // the contents of init, since a user is trying to instantiate.
    if first_arg == "init" {
        return commands::init(tome_executable, arguments);
    }

    let mut target = PathBuf::from(first_arg);
    // next, we determine if we have a file or a directory,
    // recursing down arguments until we've exhausted arguments
    // that match a directory or file.
    let mut target_type = TargetType::Directory;
    let mut first_arg = true;
    loop {
        if let Some(arg) = arguments.peek() {
            // match against builtin commands
            if first_arg {
                if *arg == "--help" {
                    arguments.next();
                    match commands::help(target.to_str().unwrap_or_default(), arguments) {
                        Ok(message) => return Ok(message),
                        Err(io_error) => return Err(format!("{}", io_error)),
                    }
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
        } else {
            break;
        }
    }
    let mut command_type = CommandType::Execute;
    let remaining_args = {
        let mut remaining_args = vec![];
        for arg in arguments {
            if arg == "--complete" {
                command_type = CommandType::Completion;
            }
            remaining_args.push(arg);
        }
        remaining_args
    };
    let output: String = match target_type {
        TargetType::Directory => match command_type {
            CommandType::Completion => {
                let mut result = vec![];
                let paths = fs::read_dir(target.to_str().unwrap_or("")).unwrap();
                for path in paths {
                    result.push(path.unwrap().file_name().to_str().unwrap_or("").to_owned());
                }
                result.join(" ").to_owned()
            }
            CommandType::Execute => {
                return Err(format!(
                    "{} is a directory. tab-complete to choose subcommands",
                    target.to_str().unwrap_or("")
                ));
            }
        },
        TargetType::File => match commands::Script::load(&target.to_str().unwrap_or_default()) {
            Ok(script) => script.get_execution_body(command_type, &remaining_args)?,
            Err(error) => return Err(format!("IOError loading file: {:?}", error)),
        },
    };
    return Ok(output);
}
