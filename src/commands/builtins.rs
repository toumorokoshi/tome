use super::{execute::execute, help::help};
use std::collections::HashMap;

pub struct Command {
    pub func: fn(&str, &[String]) -> Result<String, String>,
    pub help_text: &'static str,
}

lazy_static! {
    pub static ref BUILTIN_COMMANDS: HashMap<String, Command> = {
        let mut m = HashMap::new();
        m.insert(
            "commands".to_owned(),
            Command {
                func: help_command as fn(&str, &[String]) -> Result<String, String>,
                help_text: "print all commands",
            },
        );
        m.insert(
            "exec".to_owned(),
            Command {
                func: exec_command as fn(&str, &[String]) -> Result<String, String>,
                help_text: "execute a command",
            },
        );
        m.insert(
            "help".to_owned(),
            Command {
                func: help_command as fn(&str, &[String]) -> Result<String, String>,
                help_text: "print help for the command",
            },
        );
        m.insert(
            "tome".to_owned(),
            Command {
                func: noop_command as fn(&str, &[String]) -> Result<String, String>,
                help_text: "currently a no-op. reserved namespace for future tome commands",
            },
        );
        m
    };
}

fn exec_command(root: &str, args: &[String]) -> Result<String, String> {
    // strip the first argument since it should be "exec"
    execute(root, &args[1..])
}

fn help_command(root: &str, _: &[String]) -> Result<String, String> {
    help(root)
}

fn noop_command(_: &str, _: &[String]) -> Result<String, String> {
    Ok("".to_owned())
}
