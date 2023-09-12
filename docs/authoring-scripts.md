# Basic Scripts

Any standard script executable by your shell is supported as is: just put it into the directory hierarchy and you're ready to go.

Scripts must not be named with a leading dash! This namespace is reserved for tome commands (e.g. --help).

# Writing Advanced Scripts

Any standard script executable by your shell is supported as is: just put it into the directory hierarchy and you're ready to go.

This page covers some more advanced scenarios.

## Tab Completion by Script

Tab completion for a script can improve the usability significantly. Tab completion can be turned on for a script by including a comment with the string "COMPLETE" in the top of the script:

```
#!/usr/bin/env python
# COMPLETE
```

When tab-completion is requested for a specific script, the script is invoked with the "--complete" argument passed at the end.

For example, in the example, tab-completing the following:

    cb dir_example foo s

Will result in the following being executed:

    ./examples/dir_example/foo s --complete

Completion should return options for the last argument. More complex completion semantics, such as those offered by zsh, are not current available.

You can see an example [in the examples folder](https://github.com/toumorokoshi/tome/blob/master/example/file_example) for more details.

## SOURCE vs executed.

If you want to write a script that modifies your current shell (e.g. navigate to a directory or set environment variables), you can author a script with "# SOURCE" as the first line. In this situation, if the script is invoked, the contents will be sourced instead of executed in a subprocess.

You can see an example [in the examples folder](https://github.com/toumorokoshi/tome/blob/master/example/source_example) for more details.

Source a script helps when you want to modify the state of your current shell, including:

- setting environment variables.
- navigate to a different directory.

## Ignoring scripts

Files that lead with a `.` are ignored.

## Adding help text

Tome has the ability to read in help text for your script, giving you the ability
to describe what the script is intended to do, and outputs that into commands such as `help`.

To add help text, add as many lines as you like between lines with the strings `START HELP` and `END HELP`:

```
# START HELP
# this script will navigate to your work script.
# END HELP
```

## Adding a summary

A summary can also be added, which will be printed out when you run `commands`:

```
# SUMMARY: this is a summary of my script
```

## Locating files relative to the scripts directory

A `_TOME_SCRIPTS_ROOT` environment variable, which points to the root directory
of the scripts,  is provided to help find files that are stored inside (e.g.
depenendencies of other scripts).

This variable can be used safely in your scripts.