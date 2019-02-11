# Writing Advanced Scripts

Any standard script executable by your shell is supported as is: just put it into the directory hierarchy and you're ready to go.

This page covers some more advanced scenarios.

## Tab Completion by Script

Tab completion for a script can improve the usability significantly. When tab-completion is requested for a specific script, the script is invoked with the "--complete" argument passed at the end.

For example, in the example, tab-completing the following:

    cb dir_example foo s

Will result in the following being executed:

    ./examples/dir_example/foo s --complete

Completion should return options for the last argument. More complex completion semantics, such as those offered by zsh, are not current available.

You can see an example [in the examples folder](https://github.com/toumorokoshi/tome/blob/master/example/file_example) for more details.

## SOURCE vs executed.

If you want to write a script that modifies your current shell (e.g. navigate to a directory or set environment variables), you can author a script with "# SOURCE" as the first line. In this situation, if the script is invoked, the contents will be sourced instead of executed in a subprocess.

You can see an example [in the examples folder](https://github.com/toumorokoshi/tome/blob/master/example/source_example) for more details.
