use super::types::CommandType;
use std::{
    env::var,
    fs::File,
    io,
    io::{prelude::*, BufReader, Read},
    process::{Command, Stdio},
};
/// Any executable script
/// can be added to be executed, but
/// t's possible to add metadata
/// to the script via comments as well.
pub struct Script {
    /// the path the script is located at.
    pub path: String,
    /// determines if the script should
    /// be sourced or not.
    pub should_source: bool,
    /// the string that should be printed
    /// when help is requested.
    pub help_string: String,
    /// the string that should be used for
    /// usage information
    pub summary_string: String,
}

impl Script {
    pub fn load(path: &str) -> io::Result<Script> {
        let file = Box::new(File::open(path)?) as Box<dyn Read>;
        Ok(Script::load_from_buffer(path.to_owned(), file))
    }
    pub fn load_from_buffer(path: String, body: Box<dyn Read>) -> Script {
        let mut buffer = BufReader::new(body);
        let mut should_source = false;
        let mut help_string = String::new();
        let mut summary_string = String::new();
        let mut line = String::new();
        let mut consuming_help = false;
        loop {
            line.clear();
            match buffer.read_line(&mut line) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }
                }
                Err(_) => break,
            }
            if consuming_help {
                if line.starts_with("# END HELP") {
                    consuming_help = false;
                } else if let Some(rest) = line.strip_prefix("# ") {
                    // omit first two characters since they are
                    // signifying continued help.
                    help_string.push_str(rest);
                }
            } else if line.starts_with("# SOURCE") {
                should_source = true;
            } else if line.starts_with("# START HELP") {
                consuming_help = true;
            } else if line.starts_with("# SUMMARY: ") {
                // 9 = prefix, -1 strips newline
                summary_string.push_str(&line[11..(line.len() - 1)]);
            } else if !line.starts_with("#!") {
                // if a shebang is encountered, we skip.
                // as it can indicate the command to run the script with.
                // metadata lines must be consecutive.
                break;
            }
        }
        Script {
            path,
            should_source,
            help_string,
            summary_string,
        }
    }

    // return the appropriate string that should be exeucted within the
    // function.
    pub fn get_execution_body(
        &self,
        command_type: CommandType,
        args: &[&String],
    ) -> Result<String, String> {
        match command_type {
            CommandType::Completion => {
                // in the completion case, we need to execute the script itself.
                // There's a possible optimization here
                // if we just inherit parent file descriptors.
                let mut command = match self.should_source {
                    true => Command::new(var("SHELL").unwrap_or_default()),
                    false => Command::new(self.path.clone()),
                };
                if self.should_source {
                    command.arg(self.path.clone());
                }
                command.arg("--complete");
                let command_output = command.args(args).stdout(Stdio::piped()).output();
                match command_output {
                    Ok(output) => match String::from_utf8(output.stdout) {
                        Err(error) => Err(format!(
                            "unable to parse completion results as a utf8 string: {}",
                            error
                        )),
                        Ok(result) => Ok(result),
                    },
                    // TODO: it's hard to get output from a completion call.
                    // possible to print to stderr?
                    Err(result) => Err(format!("completion called failed: {}", result)),
                }
            }
            CommandType::Execute => {
                let command_string = if self.should_source {
                    // when sourcing, just return the full body.
                    let mut command = vec![String::from("source"), self.path.clone()];
                    for arg in args.iter() {
                        command.push((**arg).clone());
                    }
                    // in the case of sourcing, at least one variable needs
                    // to be specified, or else arguments will be inherited
                    // from the parent process.
                    if command.len() == 2 {
                        command.push(String::from(""));
                    }
                    command
                } else {
                    vec![self.path.clone()]
                };
                // after figuring out the command, all resolved values
                // should be quoted, to ensure that the shell does not
                // interpret character sequences.
                // TODO: use shell escape library
                let mut escaped_command_string = vec![];
                for mut arg in command_string {
                    arg = arg.replace("'", "\\'");
                    arg.insert(0, '\'');
                    arg.push('\'');
                    escaped_command_string.push(arg);
                }
                // Include commandline arguments
                for a in args {
                    escaped_command_string.push(a.to_string())
                }
                Ok(escaped_command_string.join(" "))
            }
        }
    }
}
