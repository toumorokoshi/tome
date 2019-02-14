use super::super::CommandType;
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
}

impl Script {
    pub fn load(path: &str) -> io::Result<Script> {
        let file = Box::new(File::open(path)?) as Box<Read>;
        Ok(Script::load_from_buffer(path.to_owned(), file))
    }
    pub fn load_from_buffer(path: String, body: Box<Read>) -> Script {
        let mut buffer = BufReader::new(body);
        let mut should_source = false;
        let mut help_string = String::new();
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
                } else if line.starts_with("# ") {
                    // omit first two characters since they are
                    // signifying continued help.
                    help_string.push_str(&line[2..]);
                }
            } else {
                if line.starts_with("# SOURCE") {
                    should_source = true;
                } else if line.starts_with("# START HELP") {
                    consuming_help = true;
                } else if !line.starts_with("#!") {
                    // if a shebang is encountered, we skip.
                    // as it can indicate the command to run the script with.
                    // metadata lines must be consecutive.
                    break;
                }
            }
        }
        Script {
            path,
            should_source,
            help_string,
        }
    }

    // return the appropriate string that should be exeucted within the
    // function.
    pub fn get_execution_body(
        &self,
        command_type: CommandType,
        args: &Vec<&String>,
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
                if self.should_source {
                    // when sourcing, just return the full body.
                    let mut command = vec![String::from("source"), self.path.clone()];
                    for arg in args.iter() {
                        command.push((**arg).clone());
                    }
                    Ok(command.join(" ").to_owned())
                } else {
                    // the command should be run directly by the outer shell, so
                    // output that.
                    let mut command = vec![self.path.clone()];
                    for arg in args.iter() {
                        command.push((**arg).clone());
                    }
                    Ok(command.join(" ").to_owned())
                }
            }
        }
    }
}
