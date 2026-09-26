# Basic Scripts

> **Note:** This document pertains to tome 0.11. Historically, tome has not
> required the executable bit — any file in the scripts directory was
> recognized as a command. Starting with tome 0.12, only files with the
> executable bit set are recognized. If you are migrating from an earlier
> version, ensure your scripts are marked executable (`chmod +x <script>`).
> Additionally, the `# SOURCE` comment header for marking sourceable scripts
> has been replaced by the `.source` file suffix (see below).

Any executable script can be added to a tome directory. Only files with the
executable bit set (e.g. `chmod +x my-script`) will be recognized as commands.

Scripts must not be named with a leading dash! This namespace is reserved for tome commands (e.g. --help).

## Non-Executable Files

Files without the executable bit are ignored by tome. This means you can
safely include READMEs, data files, or library scripts in your tome directory
without them appearing in help output or tab completion.

## Sourceable Scripts (.source suffix)

If you want to write a script that modifies your current shell (e.g. navigate
to a directory or set environment variables), name the file with a `.source`
suffix. For example: `navigate_directory.source`.

Sourceable scripts:

- Do **not** need the executable bit set.
- Are invoked by their name **without** the `.source` suffix (e.g. `my-command navigate_directory`).
- Are sourced (`. script`) rather than executed in a subprocess.

Example:

```bash
# my-env.source
# SUMMARY: set up my development environment
export EDITOR=vim
export DEV_ENVIRONMENT="production"
cd ~/workspace
```

You can see an example [in the examples folder](https://github.com/toumorokoshi/tome/blob/master/example/source_example.source) for more details.

Sourcing a script is useful when you want to modify the state of your current shell, including:

- setting environment variables.
- navigate to a different directory.

# Writing Advanced Scripts

This page covers some more advanced scenarios.

## Tab Completion by Script

Tab completion for a script can improve the usability significantly. Tab completion can be enabled for a script by including a comment with the string "COMPLETE" in the header of the script:

```bash
#!/usr/bin/env bash
# COMPLETE
```

When tab-completion is requested for a specific script, the script is invoked with the `--complete` argument passed as the **first** argument, followed by any arguments already typed.

For example, tab-completing the following:

    cb dir_example baz s

Will result in the following being executed:

    ./example/dir_example/baz --complete s

Passing `--complete` as the first argument allows scripts to easily detect completion mode (e.g. checking if `$1` is `--complete`), shift it off, and then inspect the remaining arguments using standard tools like `getopts` or positional arguments.

Here is an example in bash:

```bash
#!/usr/bin/env bash
# COMPLETE

if [[ "$1" == "--complete" ]]; then
    shift
    # "$@" contains any arguments typed after the command
    # Return completion options (separated by whitespace or newlines)
    echo "--option1 --option2"
    exit 0
fi

# Normal script execution continues below
```

Here is an example in python:

```python
#!/usr/bin/env python3
# COMPLETE
import sys

if len(sys.argv) > 1 and sys.argv[1] == "--complete":
    # sys.argv[2:] contains any arguments typed after the command
    print("option1\noption2")
    sys.exit(0)

# Normal script execution continues below
```

Completion should return options for the argument being completed (separated by whitespace or newlines). More complex completion semantics, such as those offered by zsh, are not currently available.

You can see examples in the [example directory](https://github.com/toumorokoshi/tome/blob/main/example/):
- [example/dir_example/baz](https://github.com/toumorokoshi/tome/blob/main/example/dir_example/baz)
- [example/file_example](https://github.com/toumorokoshi/tome/blob/main/example/file_example)

## Ignoring Scripts

The following files are automatically ignored by tome:

- Files that start with a `.` (dot-prefix)
- Files without the executable bit set (unless they have a `.source` suffix)
- Files or directories matched by a `.tomeignore` file

### Using `.tomeignore`

Tome natively supports a `.tomeignore` file in your root scripts directory. This file uses standard `.gitignore` syntax to specify files and directories that should be excluded from command discovery and completions.

**Backwards Compatibility / Legacy Behavior:**
If an empty `.tomeignore` file is placed inside a directory, tome will completely ignore that directory, serving as a simple exclusion marker.

**Performance Note:** 
While `.tomeignore` adds powerful filtering rules, it can add slight parsing overhead during command execution and discovery. However, benchmarks show that this overhead is generally negligible (adding only ~0.2 milliseconds per execution on a complex repository of 2,500 scripts). It is strictly recommended to use the non-executable file approach or dot-prefixed (`.name`) directories only when you require absolute maximum performance without global ignores.

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
of the scripts, is provided to help find files that are stored inside (e.g.
depenendencies of other scripts).

This variable can be used safely in your scripts.
