use std::{
    env::args,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

mod commands;
#[cfg(test)]
mod tests;

pub fn main() {
    let args: Vec<String> = args().peekable().collect();
    let result = match execute(args) {
        Ok(result) => result,
        Err(error_message) => error_message,
    };
    print!("echo {}", result);
}

enum CommandType {
    Execute,
    Completion,
}

enum TargetType {
    File,
    Directory,
}

pub fn execute(raw_args: Vec<String>) -> Result<String, String> {
    let mut args = raw_args.iter().peekable();
    let mut target = PathBuf::from(&match args.next() {
        Some(arg) => arg,
        None => return Err(String::from("at least one argument expected")),
    });
    // next, we determine if we have a file or a directory,
    // recursing down arguments until we've exhausted arguments
    // that match a directory or file.
    let mut target_type = TargetType::File;
    loop {
        if let Some(arg) = args.peek() {
            target.push(arg);
            if target.is_file() {
                target_type = TargetType::File;
                args.next();
                break;
            } else if target.is_dir() {
                target_type = TargetType::Directory;
                args.next();
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
        for arg in args {
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
                    "echo {} is a directory tab-complete to choose subcommands",
                    target.to_str().unwrap_or("")
                ))
            }
        },
        TargetType::File => match command_type {
            CommandType::Completion => {
                // There's a possible optimization here
                // if we just inherit parent file descriptors.
                let command_output = Command::new(target.to_str().unwrap_or_default())
                    .args(&remaining_args)
                    .stdout(Stdio::piped())
                    .output();
                match command_output {
                    Ok(output) => match String::from_utf8(output.stdout) {
                        Err(error) => format!(
                            "unable to parse completion results as a utf8 string: {}",
                            error
                        ),
                        Ok(result) => {
                            println!("{}", result);
                            result
                        }
                    },
                    // TODO: it's hard to get output from a completion call.
                    // possible to print to stderr?
                    Err(result) => format!("completion called failed: {}", result),
                }
            }
            CommandType::Execute => {
                let mut command = vec![target.to_str().unwrap_or("")];
                for arg in remaining_args.iter() {
                    command.push(&arg);
                }
                command.join(" ").to_owned()
            }
        },
    };
    return Ok(output);
}
