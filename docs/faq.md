# FAQ

## What shells are supported?

Currently, bash, fish and zsh are supported. PRs are welcome to support others!

## Why not Sub?

Tome is inspired primarily by a basecamp open source project called [sub](https://github.com/basecamp/sub). Tome was designed to improve on a few pieces that sub (as of 2019) left to be desired.

### Namespaced Commands

sub did not allow for namespaced commands (e.g. main-command sub-directory sub-command).

### No Need to Fork

The proper way to create a sub repository was to clone the main one, and start modifying the code to your preference, at the very least adding new commands.

Tome was designed as a standalone executable that can be used to bootstrap the command and provides utilities that enable proper completion and navigation. As such it is possible to update tome and gain new functionality and fixes without merge conflicts.

### Enable Sourcable Scripts

A common usecase I had for sub was shortcutting common directories like ~/Downloads, ~/workspace (or whatever for code). This was not possible in the original sub because it always executed the script, which would affect the process that was spawned, but not the main shell.

The design of tome allows for commands to effectively just be sourced scripts as well. This means that you can build shorcuts that affect and modify the parent environment, like set environment variables for development or navigate to some directory.