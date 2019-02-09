# Welcome to MkDocs

# Overview

This project attempts to provide easy organization
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
```

And add in some features for discoverability:

* search all available commands
* tab-completion
* TBD: help commands
* TBD: interactive search and autocomplete of commands

## Why?

Everyone has their own way of sharing scripts with each other. In my experience it usually comes down to a git repo and prepending things to the path.

This works well for a small group, but once you're trying to share scripts across a larger organization, issues arise:

* discoverability: how do I see what already exists?
* namespacing: how do I find commands that relate to what I'm working on?
* completion: It's nice

## Usage

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

Put them in some known location (~/my-scripts). Initialize your shell with:

```
# bash example, e.g. .bashrc
source <(~/cookbook/init my-commands ~/my-scripts $0)
```






You can 


For full documentation visit [mkdocs.org](https://mkdocs.org).

## Commands

* `mkdocs new [dir-name]` - Create a new project.
* `mkdocs serve` - Start the live-reloading docs server.
* `mkdocs build` - Build the documentation site.
* `mkdocs help` - Print this help message.

## Project layout

    mkdocs.yml    # The configuration file.
    docs/
        index.md  # The documentation homepage.
        ...       # Other markdown pages, images and other files.
