use std::{
    iter::{Iterator, Peekable},
    slice::Iter,
};

// unfortunately static strings cannot
// be used in format, so we use a macro instaed.
macro_rules! init_help_body {
    () => {
        r#""
error: {}

An example cookbook init invocation looks like:

source <(./cookbook init sc ~/my_script_dir $0)

The positional arguments are:

1. the string "init"
2. the name of the command you'd like to create
3. the directory that the command should read scripts from
4. the shell you want to use. Using the $0 argument here is recommended,
   as in shells that is the name of the shell in use.

The "source" is important as cookbook init will print a shell snippet
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
    autoload -U +X bashcompinit && bashcompinit
fi

# completion is accomplished by three parts:
# 1. passing all possible completions to execute.py
# 2. filtering for valid options using compgen
# 3. appending to the valid option environment variable.
function {function_name} {{
    local cmd
    cmd=`{cookbook_executable} {script_root} $@`
    # sometimes the output of cookbook includes
    # empty strings, wrapped in quotes. in order
    # to handle those, we need to eval rather than
    # evaluate the variable directly.
    eval $cmd
}}

function _{function_name}_completions {{
    local token_to_complete cookbook_args
    token_to_complete="${{COMP_WORDS[COMP_CWORD]}}";
    cookbook_args=${{COMP_LINE:2}};  # strip the first argument prefix, which is the function name
    # strip the partial token_to_complete, if there is one
    cookbook_args=${{cookbook_args%$token_to_complete}};
    all_options=`{cookbook_executable} {script_root} $cookbook_args --complete`
    valid_options=$(compgen -W "$all_options" "$token_to_complete")
    COMPREPLY=($valid_options)
}}

complete -F _{function_name}_completions {function_name}
"#
    };
}

// given the location of the cookbook executable, return
// back the init script for cookbook.

pub fn init(cookbook_executable: &str, mut args: Peekable<Iter<String>>) -> Result<String, String> {
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
    let _shell_type = match args.next() {
        Some(arg) => arg,
        None => {
            return Err(format!(
                init_help_body!(),
                "function name required for init invocation"
            ))
        }
    };
    // Bootstrapping the sc section requires two parts:
    // 1. creating the function in question
    // 2. wiring up tab completion for the function.
    //
    // functions must be used instead of a script, as
    // cookbook also supports commands that modify the
    // current environment (such as cd you into a specific)
    // directory.
    // TODO: add conditionals if other shells need different support
    Ok(format!(
        bash_zsh_init_body!(),
        cookbook_executable = cookbook_executable,
        script_root = script_root,
        function_name = function_name
    ))
}
