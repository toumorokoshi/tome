use super::super::script;
use std::io::{Cursor, Read};
/// if a script has "SOURCE"
/// at the top, it should be sourced in.
#[test]
fn test_should_source() {
    let script = script::Script::load_from_buffer(
        String::from("./example/foo"),
        Box::new(Cursor::new(
            "# SOURCE
cd /tmp/
    ",
        )) as Box<dyn Read>,
    );
    assert_eq!(script.should_source, true);
}

/// if a script does not have "SOURCE"
/// at the top, it should be sourced in.
#[test]
fn test_should_not_source() {
    let script = script::Script::load_from_buffer(
        String::from("./example/foo"),
        Box::new(Cursor::new(
            "#!/usr/bin/env bash
echo foo
    ",
        )) as Box<dyn Read>,
    );
    assert_eq!(script.should_source, false);
}

#[test]
fn test_help() {
    let script = script::Script::load_from_buffer(
        String::from("./example/foo"),
        Box::new(Cursor::new(
            "#!/usr/bin/env bash
# START HELP
# foo bar baz
# END HELP
    ",
        )) as Box<dyn Read>,
    );
    assert_eq!(&script.help_string, "foo bar baz\n");
}

#[test]
fn test_usage() {
    let script = script::Script::load_from_buffer(
        String::from("./example/foo"),
        Box::new(Cursor::new(
            "#!/usr/bin/env bash
# SUMMARY: this is the usage
# START HELP
# foo bar baz
# END HELP
    ",
        )) as Box<dyn Read>,
    );
    assert_eq!(&script.summary_string, "this is the usage");
}
