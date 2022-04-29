use super::super::{
    directory, script,
    types::{CommandType, TargetType},
};
use super::builtins::BUILTIN_COMMANDS;
use std::{fs, io, path::PathBuf};

pub fn complete(command_directory_path: &str, args: &[String]) -> Result<String, String> {
    // TODO: refactor to share common logic with execute
    // determine if a file or a directory was passed,
    // recursing down arguments until we've exhausted arguments
    // that match a directory or file.
    let mut target = PathBuf::from(&command_directory_path);
    let mut target_type = TargetType::Directory;
    let mut args_peekable = args.iter().peekable();
    while let Some(arg) = args_peekable.peek() {
        target.push(arg);
        if target.is_file() {
            target_type = TargetType::File;
            args_peekable.next();
            break;
        } else if target.is_dir() {
            target_type = TargetType::Directory;
            args_peekable.next();
        } else {
            // the current argument does not match
            // a directory or a file, so we've landed
            // on the strictest match.
            target.pop();
            break;
        }
    }
    let remaining_args: Vec<_> = args_peekable.collect();
    return match target_type {
        TargetType::Directory => {
            let paths_raw: io::Result<_> = fs::read_dir(target.to_str().unwrap());
            let mut paths: Vec<_> = match paths_raw {
                Err(_a) => return Err("Invalid argument to completion".to_string()),
                Ok(a) => a,
            }
            .filter_map(|r| match r {
                Ok(path_buf) => {
                    let path = path_buf.path();
                    if path.is_dir() && !directory::is_tome_script_directory(&path) {
                        return None;
                    }
                    if path.is_file()
                        && !script::is_tome_script(
                            path_buf.file_name().to_str().unwrap_or_default(),
                        )
                    {
                        return None;
                    }
                    Some(path.file_name().unwrap().to_str().unwrap_or("").to_owned())
                }
                Err(_) => None,
            })
            .collect();
            // if this is the root directory, add the builtin commands
            if target.to_str().unwrap() == command_directory_path {
                for command in BUILTIN_COMMANDS.keys() {
                    paths.push(command.to_owned());
                }
            }
            paths.sort_by_key(|f| f.to_owned());
            Ok(paths.join(" "))
        }
        TargetType::File => match script::Script::load(target.to_str().unwrap_or_default()) {
            Ok(script) => script.get_execution_body(CommandType::Completion, &remaining_args),
            Err(error) => return Err(format!("IOError loading file: {:?}", error)),
        },
    };
}
