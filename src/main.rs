use std::{env::args, fs, path::PathBuf};

enum CommandType {
    Execute,
    Completion,
}

enum TargetType {
    File,
    Directory,
}

fn main() {
    let mut args = args().peekable();
    let mut target = PathBuf::from(&match args.next() {
        Some(arg) => arg,
        None => panic!("at least one argument expected"),
    });
    // next, we determine if we have a file or a directory,
    // recursing down arguments until we've exhausted arguments
    // that match a directory or file.
    let mut target_type = TargetType::File;
    loop {
        if let Some(arg) = args.peek() {
            target.push(arg);
            if target.is_file() {
                target_type = TargetType::File;
                args.next();
                break;
            } else if target.is_dir() {
                target_type = TargetType::Directory;
                args.next();
            } else {
                // the current argument does not match
                // a directory or a file, so we've landed
                // on the strictest match.
                target.pop();
                break;
            }
        } else {
            break;
        }
    }
    let mut command_type = CommandType::Execute;
    let remaining_args = {
        let mut remaining_args = vec![];
        for arg in args {
            if arg == "--complete" {
                command_type = CommandType::Completion;
            }
            remaining_args.push(arg);
        }
        remaining_args
    };
    let output: String = match target_type {
        TargetType::Directory => match command_type {
            CommandType::Completion => {
                let mut result = vec![];
                let paths = fs::read_dir(target.to_str().unwrap_or("")).unwrap();
                for path in paths {
                    result.push(path.unwrap().file_name().to_str().unwrap_or("").to_owned());
                }
                result.join(" ").to_owned()
            }
            CommandType::Execute => format!(
                "echo {} is a directory tab-complete to choose subcommands",
                target.to_str().unwrap_or("")
            ),
        },
        TargetType::File => {
            let mut command = vec![target.to_str().unwrap_or("")];
            for arg in remaining_args.iter() {
                command.push(&arg);
            }
            command.join(" ").to_owned()
        }
    };
    print!("{}", &output);
}
