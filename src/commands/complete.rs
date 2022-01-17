use super::super::{
    directory, script,
    types::{CommandType, TargetType},
};
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
            let mut result = vec![];
            let paths_raw: io::Result<_> = fs::read_dir(target.to_str().unwrap_or(""));
            // TODO(zph) deftly fix panics when this code path is triggered with empty string: ie sc dir_example bar<TAB>
            // current implementation avoids the panic but is crude.
            let mut paths: Vec<_> = match paths_raw {
                Err(_a) => return Err("Invalid argument to completion".to_string()),
                Ok(a) => a,
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
                    && !script::is_tome_script(path_buf.file_name().to_str().unwrap_or_default())
                {
                    continue;
                }
                result.push(path.file_name().unwrap().to_str().unwrap_or("").to_owned());
            }
            Ok(result.join(" "))
        }
        TargetType::File => match script::Script::load(target.to_str().unwrap_or_default()) {
            Ok(script) => script.get_execution_body(CommandType::Completion, &remaining_args),
            Err(error) => return Err(format!("IOError loading file: {:?}", error)),
        },
    };
}
