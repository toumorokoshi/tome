use super::execute;
use std::env;

const DIRECTORY: &'static str = "./example";
const EXAMPLE_DIR: &'static str = "--directory=./example";

fn _vec_str(args: Vec<&str>) -> Vec<String> {
    args.iter().map(|s| s.to_string()).collect()
}

/// basic test for a simple script.
/// the output should be the path to the script itself.
#[test]
fn test_simple_script() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "exec",
            EXAMPLE_DIR,
            "--",
            "file_example"
        ])),
        Ok(format!("'{}/file_example'", DIRECTORY))
    );
}

#[test]
fn test_simple_script_completion() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "complete",
            EXAMPLE_DIR,
            "--",
            "file_example",
        ])),
        Ok(String::from("file autocomplete example"))
    );
}

/// basic test for a script that should be sourced
#[test]
fn test_source() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "exec",
            EXAMPLE_DIR,
            "--",
            "source_example",
        ])),
        Ok(format!("'source' '{}/source_example' ''", DIRECTORY))
    );
}

#[test]
fn test_source_completion() {
    // Context: https://github.com/toumorokoshi/tome/issues/6
    // Override SHELL env variable due to test depending on sourced file
    // being a bash/zsh compatible sourced file.
    // The behavior of sourcing scripts works in cli and we patch
    // in tests to ensure consistent results.
    env::set_var("SHELL", "bash");
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "complete",
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
            "complete",
            EXAMPLE_DIR,
            "--",
            "dir_example",
        ])),
        Ok("bar foo".to_string())
    );
}

/// the root directory should also be completed
#[test]
fn test_root_directory_completion() {
    assert_eq!(
        execute(_vec_str(vec!["tome", "complete", EXAMPLE_DIR])),
        Ok("dir_example file_example practical_examples source_example use-arg".to_string())
    );
}

/// if completion is requested on a directory,
/// return the list of file and directories in there.
/// TODO: disabled because there should be no more completion
/// done once we exhaust the current directory/file tree
/// beyond args to directory/file --foo. Currently
/// completion of arguments of scripts is broken.
#[test]
#[ignore]
fn test_script_in_directory() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "complete",
            EXAMPLE_DIR,
            "--",
            "dir_example",
            "foo"
        ])),
        Ok(format!("'{}/dir_example/foo'", DIRECTORY))
    );
}

/// if the script is not found in the directory, provide
/// a clear error message.
#[test]
fn test_script_in_directory_not_found() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "exec",
            EXAMPLE_DIR,
            "--",
            "dir_example",
            "foo-nonexistent",
            "baz"
        ])),
        Err(format!(
            "command foo-nonexistent not found in directory {}/dir_example",
            DIRECTORY
        ))
    );
}

/// completing to a directory emits the directory name and some help.
#[test]
fn test_script_directory_argument() {
    assert_eq!(
        execute(_vec_str(vec![
            "tome",
            "complete",
            EXAMPLE_DIR,
            "--",
            "dir_example",
        ])),
        Ok("bar foo".to_string())
    );
}

/// if there is no argument passed in for sourcing, an argument will
/// be added, to ensure that shells don't inherit arguments from the initial shell
/// command.
#[test]
fn test_use_arg() {
    assert_eq!(
        execute(_vec_str(vec!["tome", "exec", EXAMPLE_DIR, "--", "use-arg"])),
        Ok(format!("'source' '{}/use-arg' ''", DIRECTORY))
    );
}

/// to ensure that character sequences that have special meaning to
/// the shell are not interpreted as such, all values should be single quoted.
#[test]
fn test_dangerous_characters_quoted() {
    assert_eq!(
        execute(_vec_str(vec!["tome", "exec", EXAMPLE_DIR, "--", "use-arg"])),
        Ok(format!("'source' '{}/use-arg' ''", DIRECTORY))
    );
}

/// help should be returned in no arguments are passed
/// disabled for now while help text is printed directly
/// to stdout in main.rs
#[test]
#[ignore]
fn test_help_page() {
    let result = execute(_vec_str(vec!["tome"])).unwrap();
    assert_eq!(result.matches("SUBCOMMANDS").count(), 1);
}
