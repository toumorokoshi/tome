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

## Ignoring Scripts

The following files are automatically ignored by tome:

- Files that start with a `.` (dot-prefix)
- Files without the executable bit set (unless they have a `.source` suffix)

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
