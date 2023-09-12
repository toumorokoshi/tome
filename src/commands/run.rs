use super::super::finder;
use std::os::unix::process::CommandExt;

pub fn run(command_directory_path: &str, args: &[String]) -> Result<String, String> {
    // generic handling
    match finder::find_script(command_directory_path, args) {
        // although both paths will always error, the Ok() path actually will
        // likely complete as expected, as an exec effectively transforms the
        // existing process into the new command, which may complete
        // successfully.
        Ok(script_invocation) => Err(std::process::Command::new(script_invocation.target)
            .args(script_invocation.args)
            .exec()
            .to_string()),
        Err(err) => Err(err),
    }
}
