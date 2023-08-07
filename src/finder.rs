use super::types::TargetType;
use std::path::PathBuf;

pub struct ScriptInvocation<'a> {
    /// The script that that is being invoked.
    pub target: PathBuf,
    /// The arguments that should be passed directly to the script.
    pub args: Vec<&'a String>,
}

pub fn find_script<'a>(
    command_directory_path: &str,
    args: &'a [String],
) -> Result<ScriptInvocation<'a>, String> {
    // next, we determine if we have a file or a directory,
    // recursing down arguments until we've exhausted arguments
    // that match a directory or file.
    let mut target = PathBuf::from(&command_directory_path);
    let mut target_type = TargetType::Directory;
    let mut args_peekable = args.iter().peekable();
    // generic handling
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
        TargetType::Directory => match remaining_args.len() {
            0 => Err(format!(
                "{} is a directory. tab-complete to choose subcommands",
                target.to_str().unwrap_or("")
            )),
            _ => Err(format!(
                "command {} not found in directory {}",
                remaining_args[0],
                target.to_str().unwrap_or("")
            )),
        },
        TargetType::File => Ok(ScriptInvocation {
            target,
            args: remaining_args,
        }),
    };
}
