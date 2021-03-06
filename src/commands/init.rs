use std::{
    env,
    iter::{Iterator, Peekable},
    slice::Iter,
};

// unfortunately static strings cannot
// be used in format, so we use a macro instaed.
macro_rules! init_help_body {
    () => {
        r#""
error: {}

An example tome init invocation looks like:

# Bash/Zsh
source <(tome init sc ~/my_script_dir $0)

# Fish
tome init sc ~/my_script_dir $0 | source

The positional arguments are:

1. the string "init"
2. the name of the command you'd like to create
3. the directory that the command should read scripts from
4. the shell you want to use. Using the $0 argument here is recommended,
   as in shells that is the name of the shell in use.

The "source" is important as tome init will print a shell snippet
that should be executed to bootstrap your command. the <() syntax creates
a file descriptor that the output has been written to, that should be sourced.
"#
    };
}

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
        autoload +X compinit && compinit
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
    eval `{tome_executable} {script_root} $@`
}}

function _{function_name}_completions {{
    local token_to_complete tome_args
    token_to_complete="${{COMP_WORDS[COMP_CWORD]}}";
    tome_args=${{COMP_LINE:2}};  # strip the first argument prefix, which is the function name
    # strip the partial token_to_complete, if there is one
    tome_args=${{tome_args%$token_to_complete}};
    all_options=`{tome_executable} {script_root} --complete $tome_args`
    valid_options=$(compgen -W "$all_options" "$token_to_complete")
    COMPREPLY=($valid_options)
}}

complete -F _{function_name}_completions {function_name}
"#
    };
}

macro_rules! fish_init_body {
    () => {
        r#"
function __fish_tome_help_message
  echo -e "--help\tPrint help\n"
end

function __fish_tome_complete_subcommands
  # tome directory --complete COMPLETIONPREFIX
  $argv[1] $argv[2] $argv[3] $argv[4..-1] | tr " " "\n"
  return 0
end

function __fish_tome_completion_inner
  set -l help_msg
  set -l tokens $argv
  switch (count $tokens)
    case 0
      # Being used as function in fish
      __fish_tome_complete_subcommands $tokens
      return 0
    case 1
      # only tome
      # return all current directories
      ls -d -1 */
      __fish_tome_help_message
      return 0
    case 2
      # tome ./directory
      __fish_tome_help_message
      echo -e "--complete\tCompletions\n"
      return 0
    case '*'
      switch $tokens[3]
        case "--complete"
          __fish_tome_complete_subcommands $tokens
          return 0
        case '*'
          echo "Unknown command state: $tokens" >&2
          return 1
      end
      return 1
  end
end

function __fish_tome_completion
  set -l args (commandline -co)
  switch $args[1]
    case 'tome'
      __fish_tome_completion_inner $args
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
  set -l args "{tome_executable}" $dir "--complete" $cmd
  __fish_tome_completion_inner $args
end

complete -c tome -f -a "(__fish_tome_completion)"

# Alias for tome command
function {function_name}
  eval ({tome_executable} {script_root} $argv)
end
complete -c {function_name} -f -a "(__fish_tome_completion_fn {script_root} $argv)"
# End tome alias
"#
    };
}

// given the location of the tome executable, return
// back the init script for tome.

pub fn init(tome_executable: &str, mut args: Peekable<Iter<String>>) -> Result<String, String> {
    let function_name = match args.next() {
        Some(arg) => arg,
        None => {
            return Err(format!(
                init_help_body!(),
                "function name required for init invocation"
            ))
        }
    };
    let script_root = match args.next() {
        Some(arg) => arg,
        None => {
            return Err(format!(
                init_help_body!(),
                "function name required for init invocation"
            ))
        }
    };
    let shell_env = match env::var("SHELL") {
        Ok(val) => val,
        Err(e) => return Err(format!("Unable to fetch ENV var $SHELL with error: {}", e)),
    };
    let shell_type = match args.next() {
        Some(arg) => arg,
        None => {
            // fish shell does not pass $0 as fish, fallback to reading $SHELL
            if shell_env.contains("fish") {
                "fish"
            } else {
                return Err(format!(
                    init_help_body!(),
                    "function name required for init invocation"
                ));
            }
        }
    };
    // Bootstrapping the sc section requires two parts:
    // 1. creating the function in question
    // 2. wiring up tab completion for the function.
    //
    // functions must be used instead of a script, as
    // tome also supports commands that modify the
    // current environment (such as cd you into a specific)
    // directory.
    match shell_type {
        "fish" => Ok(format!(
            fish_init_body!(),
            tome_executable = tome_executable,
            script_root = script_root,
            function_name = function_name
        )),
        "bash" | "zsh" => Ok(format!(
            bash_zsh_init_body!(),
            tome_executable = tome_executable,
            script_root = script_root,
            function_name = function_name
        )),
        _ => Err(format!("Unknown shell {}. Unable to init.", shell_type)),
    }
}
