# Completion Handling

authors: @toumorokoshi

## Background

Completion refers to the ability for shells to call some functionality which helps complete an argument to a command. For example, one may want to complete a path to the directory that matches it:

```shell
# In bash, pressing tab here would either
# fill in the argument if there is only one match,
# or list the options with another tab.
cd ~/Do
```

The current behavior of tome is to provide basic completion functionality for
a command in a tome instance by allowing the script to return a list of options for the right most position argument. The behavior is:

- tome indicates that it is looking for completion by passing --complete as the first argument to the script.
- tome expects a list of matching values, and returns that back as completion for the list.

This functionality allows for a simple idiom that makes authoring completion simple, but at the same time restricts the more flexible completion options that are available (in particular zsh)

Therefore, it would be desirable to have a mechanism for completion handling that enables the full completion of the shell using it.

### programmable completion for bash

Bash seems to provide a fairly basic completion: one that only looks at completion for the right-most argument.

Bash also provides the ability to add default completions (directories, commands) via flags when registering a completion:

```
complete -D ls
```

It may be difficult to expose these facilities via any facility that could tome could provider generically, since these completions have to be registered via a command. Alternatively, tome could be modified to allow some way to provide completion via a direct path to the command name:

```
complete -D $PATH_TO_THE_EXECUTABLE_FOR_TOME
```

### programmable completions for zsh

Currently tome forces zsh completion to install bash-compatible completions, for simplicity. Therefore, completion support is done for zsh the same way it is done for bash today.

## Proposal

After investigation of the various shells, there doesn't seem to be a strong reason to introduce a more flexible completion system. The existing system seems to be flexible enough to support completion for bash, zsh, and fish (completing the last argument).

## References

1. [bash completion documentation](https://www.gnu.org/software/bash/manual/html_node/Programmable-Completion.html)
2. [zsh completion documentation](https://zsh.sourceforge.io/Doc/Release/Completion-System.html)
3. [zsh "Completion, old and new"](https://zsh.sourceforge.io/Guide/zshguide06.html)