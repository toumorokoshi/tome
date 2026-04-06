use super::super::constants::SCRIPT_ROOT_ENVIRONMENT_VARIABLE;
use super::super::finder::{self, FindResult};
use std::os::unix::process::CommandExt;

pub fn run(command_directory_path: &str, args: &[String]) -> Result<String, String> {
    match finder::find_script(command_directory_path, args) {
        Ok(FindResult::File(invocation)) => {
            Err(std::process::Command::new(invocation.target)
                .args(invocation.args)
                .env(SCRIPT_ROOT_ENVIRONMENT_VARIABLE, command_directory_path)
                .exec()
                .to_string())
        }
        Ok(FindResult::Directory(path)) => Err(format!(
            "{} is a directory. tab-complete to choose subcommands",
            path.to_str().unwrap_or("")
        )),
        Err(err) => Err(err),
    }
}
