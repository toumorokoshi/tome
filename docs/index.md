# Introduction

## What is Tome?

Tome provides easy organization
and execution of collections of command line scripts.

Basically take a directory of scripts like this:

```
root/
  go-to-workspace
  android/
    debug
  python/
    start-virtualenv
  work/
    start-my-server
    my-team/
      provision-my-service
```

And be able run them like:

```
$ my-command go-to-workspace
$ my-command android debug
$ my-command work start-my-server
$ my-command work my-team provision-my-service
# print a list of all commands and usage
$ my-command
```

And add in some features for discoverability:

* tab-completion
* search all available commands (just enter your command with no arguments)
* TODO: help commands

## Why?

Everyone has their own way of sharing scripts with each other. In my experience it usually comes down to a git repo and prepending things to the path.

This works well for a small group, but once you're trying to share scripts across a larger organization, issues arise:

* discoverability: how do I see what already exists?
* namespacing: how do I find commands that relate to what I'm working on?
* completion: It's nice

## Getting Started

### 1. Download Tome

Tome is provided as self-encapsulated, broadly compatible binaries. Choose the latest binary that's appropriate for your OS at the [Github releases page](https://github.com/toumorokoshi/tome/releases).

Download it into a well known, persistent location. A recommendedation is in your home directory, such as $HOME/bin/tome

### 2. Create Your Shell Scripts

Create a nested hierarchy of shell (or whatever) scripts:

```
root/
  go-to-workspace
  android/
    debug
  work/
    start-my-server
    my-team/
      provision-my-service
```

(keeping these in version control is recommended)

Put them in a well known, persistent location as well (e.g. ~/my-scripts). 

### 3. Put The Initialization Code in your .rc file

If you want your top-level command
to be named "my-commands" you'd put the following in your .bashrc or .zshrc:

```
# posix example, e.g. .bashrc
eval "`~/bin/tome init my-commands ~/my-scripts $0`"
```

For fish shell:
```
# in ~/.config/fish/conf.d/tome.fish
tome init sc ~/my_script_dir $0 | source
```

*NOTE*: make sure to include the double quotes with the nested backticks. This ensures that newlines are captured
and evaluated appropriately.

### 4. Start a New Shell

Once installed, start a new shell and you should have your new command!