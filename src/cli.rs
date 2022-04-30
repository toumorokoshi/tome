use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct TomeArgs {
    #[clap(subcommand)]
    pub commands: TomeCommands,
}

#[derive(Debug, Subcommand)]
pub enum TomeCommands {
    // if we use "Help" directly, this
    // subcommand is overshaddowed by clap's own help
    // command. Therefore this is named CommandHelp
    CommandHelp(HelpArgs),
    CommandComplete(CompleteArgs),
    CommandExecute(ExecuteArgs),
    Init(InitArgs),
}

#[derive(Debug, Args)]
pub struct CompleteArgs {
    pub command_directory_path: String,
    /// The type of the shell that is invoking completion
    #[clap(short, long)]
    pub shell: String,
    #[clap(last = true)]
    pub args: Vec<String>,
}

#[derive(Debug, Args)]
pub struct ExecuteArgs {
    pub command_directory_path: String,
    #[clap(short, long)]
    pub shell: String,
    #[clap(last = true)]
    pub args: Vec<String>,
}

#[derive(Debug, Args)]
pub struct HelpArgs {
    pub command_directory_path: String,
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// The name for the generated command.
    pub command_name: String,
    /// The path to the directory that contains the commands to load.
    pub command_directory_path: String,
    /// The type of the shell to use, or the path to the shell executable being used.
    pub shell_type_or_path: String,
}
