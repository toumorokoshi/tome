use std::env::args;
use tome::execute;

pub fn main() {
    let args: Vec<String> = args().peekable().collect();
    match execute(args) {
        Ok(result) => print!("{}", result),
        Err(error_message) => print!("echo {}", error_message),
    };
}
