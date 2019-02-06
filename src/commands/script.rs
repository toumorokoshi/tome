use std::{
    fs::File,
    io,
    io::{prelude::*, BufReader, Read, Seek},
    path::Path,
};
/// Any executable script
/// can be added to be executed, but
/// it's possible to add metadata
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
}
