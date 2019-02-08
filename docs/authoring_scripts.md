# Authoring Scripts

Any standard script executable by your shell is supported as is: just put it into the directory hierarchy and you're ready to go.

This page covers some more advanced scenarios.

## Tab Completion by Script

Tab completion for a script can improve the usability significantly. When tab-completion is requested for a specific script, the script is invoked with the "--complete" argument passed at the end.

For example, in the example, tab-completing the following:

    cb dir_example foo s

Will result in the following being executed ()

