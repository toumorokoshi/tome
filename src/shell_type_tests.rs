use rstest::*;

use super::shell_type::{get_shell_type, ShellType};

#[rstest]
#[case::bash("bash", Ok(ShellType::BASH))]
#[case::bin_bash("/bin/bash", Ok(ShellType::BASH))]
// on OSX, "-zsh" is now the default.
#[case::zsh("-zsh", Ok(ShellType::ZSH))]
#[case::bin_zsh("/bin/zsh", Ok(ShellType::ZSH))]
#[case::fish("fish", Ok(ShellType::FISH))]
fn test_shell_type(#[case] input: &str, #[case] expected: Result<ShellType, String>) {
    let result = get_shell_type(input);
    match expected {
        Ok(expected_output) => {
            assert_eq!(expected_output, result.unwrap());
        }
        Err(_) => {
            assert!(result.is_err())
        }
    }
}
