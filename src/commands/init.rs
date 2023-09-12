use super::super::{
    cli::InitArgs,
    shell_type::{get_shell_type, ShellType},
};

// because this string is intended to be formatted,
// this can be very hard to read.
// strings that will be evaluated by rust have single brackets: { }
// strings that will be evaluated as part of the script have double brackets: {{ }}
macro_rules! bash_zsh_init_body {
  () => {
    r#"
if [[ -n ${{ZSH_VERSION-}} ]]; then
    # bash completion emulation requires that zsh's completion has
    # already been initialized. In addition, running the autoload
    # expression with the ampersands seems to always result in
    # compinit being executed, which clear completions that were
    # previously bound by tome, resulting in the inability to
    # instantiate multiple tome command sets.
    if ! type complete > /dev/null; then
        # add -C "" to skip compaudit, which forces
        # an interactive dialog for compinit which doesn't
        # work for interactive shells.
        autoload +X compinit && compinit -C ""
    fi
    autoload +X bashcompinit && bashcompinit

fi

# completion is accomplished by three parts:
# 1. passing all possible completions to {tome_executable}
# 2. filtering for valid options using compgen
# 3. appending to the valid option environment variable.
function {function_name} {{
    # capturing the results as a variable led to the command
    # being to long for zsh to execute. (literally raising
    # "command too long" )
    eval `{tome_executable} command-execute {script_root} -s {shell} -- $@`
}}

function _{function_name}_completions {{
    local token_to_complete tome_args
    token_to_complete="${{COMP_WORDS[COMP_CWORD]}}";
    tome_args=${{COMP_WORDS[@]:1}};  # strip the first argument prefix, which is the function name
    # strip the partial token_to_complete, if there is one
    tome_args=${{tome_args%$token_to_complete}};
    all_options=`{tome_executable} command-complete {script_root} -s {shell} -- $tome_args`
    valid_options=$(compgen -W "$all_options" -- "$token_to_complete")
    COMPREPLY=($valid_options)
}}

complete -F _{function_name}_completions {function_name}
"#
  };
}

macro_rules! fish_init_body {
    () => {
        r#"
# Note: to perform debugging when interface for tome changes
# set set -l fish_trace on before the buggy part of fish script
function __fish_tome_help_message
  echo -e "--help\tPrint help\n"
end

function __fish_tome_complete_subcommands
  # Arguments: tome command-complete ./example -s fish -- ARGS
  $argv[1] $argv[2] $argv[3] $argv[4] $argv[5] $argv[6..-1] | tr " " "\n"
  return 0
end

function __fish_tome_completion_inner
  set -l help_msg
  set -l tokens $argv
  switch (count $tokens)
  case 0
    # Being used as function in fish
    __fish_tome_complete_subcommands $tokens
  case '*'
    __fish_tome_complete_subcommands $tokens
  end
end

function __fish_tome_completion_binary
  set -l help_msg
  set -l tokens $argv
  switch (count $tokens)
  case 0
    echo "Bad codepath"
    exit 1
  case 1
    # > tome
    # needs commands
    __fish_tome_help_message
    echo -e "command-complete\tCompletions\n"
    echo -e "command-execute\tExecute\n"
    echo -e "help\tHelp\n"
    echo -e "init\tInitialize Tome\n"
  case 2
    # > tome CMD
    # return all current directories
    ls -d -1 */
    __fish_tome_help_message
  case 3
    echo -e "--shell\tShell\n"
  case 4
    echo -e "bash\tBash\n"
    echo -e "zsh\tZsh\n"
    echo -e "fish\tFish\n"
  case 5
    echo -e "--\tSeparator\n"
  case '*'
    __fish_tome_complete_subcommands $tokens
  end
end

function __fish_tome_completion
  set -l args (commandline -co)
  switch $args[1]
    case 'tome'
      __fish_tome_completion_binary $args
    case '*'
      echo "Bad codepath"
      exit 1
  end
end

function __fish_tome_completion_fn
  set -l dir $argv[1]
  set -l cmdline (commandline -co)
  # Drop function name
  set -l cmd $cmdline[2..-1]
  set -l args "{tome_executable}" command-complete $dir -s "{shell}" -- $cmd
  __fish_tome_completion_inner $args
end

complete -c tome -f -a "(__fish_tome_completion)"

# Alias for tome command
function {function_name}
  eval ({tome_executable} command-execute {script_root} -s {shell} -- $argv)
end
complete -c {function_name} -f -a "(__fish_tome_completion_fn {script_root} $argv)"
# End tome alias
"#
    };
}

// given the location of the tome executable, return
// back the init script for tome.
pub fn init(tome_executable: &str, init_args: &InitArgs) -> Result<String, String> {
    let shell = get_shell_type(&init_args.shell_type_or_path)?;
    // Bootstrapping the sc section requires two parts:
    // 1. creating the function in question
    // 2. wiring up tab completion for the function.
    //
    // functions must be used instead of a script, as
    // tome also supports commands that modify the
    // current environment (such as cd you into a specific)
    // directory.
    match shell {
        ShellType::FISH => Ok(format!(
            fish_init_body!(),
            tome_executable = tome_executable,
            shell = &init_args.shell_type_or_path,
            script_root = init_args.command_directory_path,
            function_name = init_args.command_name,
        )),
        ShellType::BASH | ShellType::ZSH => Ok(format!(
            bash_zsh_init_body!(),
            tome_executable = tome_executable,
            shell = &init_args.shell_type_or_path,
            script_root = init_args.command_directory_path,
            function_name = init_args.command_name,
        )),
        ShellType::UNKNOWN => Err(format!(
            "could not determine shell from {}. Unable to init.",
            &init_args.shell_type_or_path
        )),
    }
}
