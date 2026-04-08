use super::execute;

const EXAMPLE_DIR: &'static str = "./example";
const SHELL: &'static str = "bash";

fn _vec_str(args: Vec<&str>) -> Vec<String> {
    args.iter().map(|s| s.to_string()).collect()
}

/// test exec, which should execute a tome
/// command
#[test]
fn test_exec_simple_script() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            SHELL,
            EXAMPLE_DIR,
            "--",
            "exec",
            "file_example"
        ])),
        Ok(format!("'{}/file_example'", EXAMPLE_DIR))
    );
}

// test exec should work, even when called
// recursively.
#[test]
fn test_exec_recursive_simple_script() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            SHELL,
            EXAMPLE_DIR,
            "--",
            "exec",
            "exec",
            "file_example"
        ])),
        Ok(format!("'{}/file_example'", EXAMPLE_DIR))
    );
}

/// basic test for a simple script.
/// the output should be the path to the script itself.
#[test]
fn test_simple_script() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            SHELL,
            EXAMPLE_DIR,
            "--",
            "file_example"
        ])),
        Ok(format!("'{}/file_example'", EXAMPLE_DIR))
    );
}

/// the output should be the path to the script itself, with the passed arguments
#[test]
fn test_simple_script_with_args() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            SHELL,
            EXAMPLE_DIR,
            "--",
            "file_example",
            "x"
        ])),
        Ok(format!("'{}/file_example' 'x'", EXAMPLE_DIR))
    );
}

#[test]
fn test_simple_script_completion() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-complete",
            "-s",
            SHELL,
            EXAMPLE_DIR,
            "--",
            "file_example",
        ])),
        Ok(String::from("file autocomplete example"))
    );
}

/// Unless the file has the completion annotation
/// do not invoke completion on it and return nothing
/// instead.
#[test]
fn test_simple_script_no_completion() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-complete",
            "-s",
            SHELL,
            EXAMPLE_DIR,
            "--",
            "test_files",
            "file_example_no_completion",
        ])),
        Ok(String::from(""))
    );
}

/// basic test for a script that should be sourced.
/// .source files are sourced when executed
#[test]
fn test_source() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            SHELL,
            EXAMPLE_DIR,
            "--",
            "source_example",
        ])),
        Ok(format!(
            "set --; '.' '{}/source_example.source'",
            EXAMPLE_DIR
        ))
    );
}

#[test]
fn test_source_completion() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-complete",
            "-s",
            "bash",
            EXAMPLE_DIR,
            "--",
            "source_example",
        ])),
        Ok(String::from("foo baz\n"))
    );
}

/// if completion is requested on a directory,
/// return the list of file and directories in there.
#[test]
fn test_directory_completion() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-complete",
            "-s",
            "bash",
            EXAMPLE_DIR,
            "--",
            "dir_example",
        ])),
        Ok("bar baz foo".to_string())
    );
}

/// the root directory should also be completed.
/// non-executable files should not appear in completions
#[test]
fn test_root_directory_completion() {
    assert_eq!(
        execute(_vec_str(vec!["tome", "command-complete", "-s", "bash", EXAMPLE_DIR])),
        // note that we also complete with builtins
        Ok("commands dir_example exec file_example help practical_examples read-from-root source_example source_example_fish test_files tome use-arg".to_string())
    );
}

/// the tome command is a no-op, and shouldn't
/// have completion
#[test]
fn test_tome_completion() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-complete",
            "-s",
            SHELL,
            EXAMPLE_DIR,
            "--",
            "tome",
        ])),
        Ok(String::from(""))
    );
}

/// if completion is requested on a directory,
/// return the list of file and directories in there.
#[test]
fn test_script_in_directory() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            "bash",
            EXAMPLE_DIR,
            "--",
            "dir_example",
            "foo"
        ])),
        Ok(format!("'{}/dir_example/foo'", EXAMPLE_DIR))
    );
}

/// if the script is not found in the directory, provide
/// a clear error message.
#[test]
fn test_script_in_directory_not_found() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            "bash",
            EXAMPLE_DIR,
            "--",
            "dir_example",
            "foo-nonexistent",
            "baz"
        ])),
        Err(format!(
            "command foo-nonexistent not found in directory {}/dir_example",
            EXAMPLE_DIR
        ))
    );
}

/// completing to a directory emits the directory name and some help.
#[test]
fn test_script_directory_argument() {
    let result = execute(_vec_str(vec![
        "tome",
        "command-execute",
        "-s",
        "bash",
        EXAMPLE_DIR,
        "--",
        "dir_example",
    ]))
    .unwrap();
    assert!(result.contains("list of dir_example commands available are:"));
    assert!(result.contains("foo:"));
    assert!(result.contains("bar:"));
}

/// if there is no argument passed in for sourcing, an argument will
/// be added, to ensure that shells don't inherit arguments from the initial shell
/// command.
#[test]
fn test_use_arg() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            "bash",
            EXAMPLE_DIR,
            "--",
            "use-arg"
        ])),
        Ok(format!("set --; '.' '{}/use-arg.source'", EXAMPLE_DIR))
    );
}

/// to ensure that character sequences that have special meaning to
/// the shell are not interpreted as such, all values should be single quoted.
#[test]
fn test_dangerous_characters_quoted() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            "bash",
            EXAMPLE_DIR,
            "--",
            "use-arg"
        ])),
        Ok(format!("set --; '.' '{}/use-arg.source'", EXAMPLE_DIR))
    );
}

/// tome should add a "help" command into every instance, to
/// output help documentation.
#[test]
fn test_execute_help() {
    let result = execute(_vec_str(vec![
        "tome",
        "command-execute",
        "-s",
        "bash",
        EXAMPLE_DIR,
        "--",
        "help",
    ]))
    .unwrap();
    assert_is_help_text(&result)
}

/// tome should add a "commands" command which lists all
/// available commands.
#[test]
fn test_execute_commands() {
    let result = execute(_vec_str(vec![
        "tome",
        "command-execute",
        "-s",
        "bash",
        EXAMPLE_DIR,
        "--",
        "commands",
    ]))
    .unwrap();
    assert_is_help_text(&result)
}

/// help should be returned if help is called explicitly
#[test]
fn test_help_page() {
    let result = execute(_vec_str(vec!["tome", "command-help", EXAMPLE_DIR])).unwrap();
    assert_is_help_text(&result)
}

/// help should be returned if no arguments are passed.
#[test]
fn test_help_page_when_execute_no_args() {
    let result = execute(_vec_str(vec![
        "tome",
        "command-execute",
        "-s",
        "bash",
        EXAMPLE_DIR,
        "--",
    ]))
    .unwrap();
    assert_is_help_text(&result)
}

/// if an empty argument is passed in the middle of a path, it should
/// be treated as an argument to the script, rather than a no-op
/// directory.
#[test]
fn test_empty_arg_in_middle_of_path() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            "bash",
            EXAMPLE_DIR,
            "--",
            "",
            "file_example"
        ])),
        Err(format!("command  not found in directory {}", EXAMPLE_DIR))
    );
}

/// sourced scripts should be able to distinguish between zero args and one empty arg.
#[test]
fn test_source_arg_count() {
    // Zero args
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            "bash",
            EXAMPLE_DIR,
            "--",
            "source_example",
        ])),
        Ok(format!(
            "set --; '.' '{}/source_example.source'",
            EXAMPLE_DIR
        ))
    );
    // One empty arg
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "command-execute",
            "-s",
            "bash",
            EXAMPLE_DIR,
            "--",
            "source_example",
            ""
        ])),
        Ok(format!("'.' '{}/source_example.source' ''", EXAMPLE_DIR))
    );
}

// helper function to assert that the output
// is indeed the help text.
fn assert_is_help_text(result: &str) {
    // uncomment to see output
    // println!("{}", result);
    assert_eq!(result.matches("'\\'").count(), 2);
    assert_eq!(result.matches("'").count(), 8);
    assert!(result.contains("echo -e"));
    // verify that builtin tome commands are present
    assert!(result.contains("commands:"));
    assert!(result.contains("exec:"));
    assert!(result.contains("help:"));
    assert!(result.contains("tome:"));
}

/// help for a specific script should print its summary and help text.
#[test]
fn test_help_for_script() {
    let result = execute(_vec_str(vec![
        "tome",
        "command-execute",
        "-s",
        "bash",
        EXAMPLE_DIR,
        "--",
        "help",
        "file_example",
    ]))
    .unwrap();
    assert!(result.contains("file_example"));
    assert!(result.contains("hey"));
    assert!(result.contains("this is an example of a simple script"));
}

/// help for a script via command-help should also print script-specific text.
#[test]
fn test_help_page_for_script() {
    let result = execute(_vec_str(vec![
        "tome",
        "command-help",
        EXAMPLE_DIR,
        "--",
        "file_example",
    ]))
    .unwrap();
    assert!(result.contains("file_example"));
    assert!(result.contains("hey"));
    assert!(result.contains("this is an example of a simple script"));
}

/// help for a directory prefix should list only commands in that directory.
#[test]
fn test_help_for_directory() {
    let result = execute(_vec_str(vec![
        "tome",
        "command-execute",
        "-s",
        "bash",
        EXAMPLE_DIR,
        "--",
        "help",
        "dir_example",
    ]))
    .unwrap();
    assert!(result.contains("foo"));
    assert!(result.contains("bar"));
}

/// help for an unknown command must return eval-safe output (wrapped in echo)
/// so the shell function does not misinterpret the error string as a command.
#[test]
fn test_help_unknown_command_is_eval_safe() {
    let result = execute(_vec_str(vec![
        "tome",
        "command-execute",
        "-s",
        "bash",
        EXAMPLE_DIR,
        "--",
        "help",
        "nonexistent_command",
    ]))
    .unwrap();
    assert!(
        result.starts_with("echo '"),
        "expected echo-wrapped output, got: {}",
        result
    );
    assert!(result.contains(">&2"));
}
