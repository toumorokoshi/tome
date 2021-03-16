mod complete;
mod execute;
mod help;
mod init;
mod script;
#[cfg(test)]
mod tests;
mod types;
use clap::ArgMatches;

pub use self::complete::complete;
pub use self::execute::execute;
pub use self::help::help;
pub use self::init::init;
pub use self::init::init_v2;
pub use self::script::Script;
pub use self::types::{CommandType, TargetType};

/// to the script via comments as well.
pub struct Config {
    /// the path the executable is located at.
    pub executable: String,
    /// configuration arg matches
    pub args: ArgMatches,
    /// base directory for scripts
    pub directory: String,
    pub paths: Vec<String>,
}
