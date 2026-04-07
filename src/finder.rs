use super::script;
use std::path::PathBuf;

pub struct ScriptInvocation<'a> {
    pub target: PathBuf,
    pub args: Vec<&'a String>,
}

pub enum FindResult<'a> {
    File(ScriptInvocation<'a>),
    Directory(PathBuf),
}

pub fn find_script<'a>(
    command_directory_path: &str,
    args: &'a [String],
) -> Result<FindResult<'a>, String> {
    let mut target = PathBuf::from(&command_directory_path);
    let mut args_peekable = args.iter().peekable();
    while let Some(arg) = args_peekable.peek() {
        if arg.is_empty() {
            break;
        }
        target.push(arg);
        if let Some(resolved) = script::resolve_source_path(&target) {
            target = resolved;
            args_peekable.next();
            let remaining_args: Vec<_> = args_peekable.collect();
            return Ok(FindResult::File(ScriptInvocation {
                target,
                args: remaining_args,
            }));
        } else if target.is_dir() {
            args_peekable.next();
        } else {
            target.pop();
            break;
        }
    }
    let remaining_args: Vec<_> = args_peekable.collect();
    match remaining_args.len() {
        0 => Ok(FindResult::Directory(target)),
        _ => Err(format!(
            "command {} not found in directory {}",
            remaining_args[0],
            target.to_str().unwrap_or("")
        )),
    }
}
