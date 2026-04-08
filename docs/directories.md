# Directories

Directories in a tome namespace may contain scripts. These scripts can be activated as a sub command of the directory name.

For example, a file "foo" in a directory "dir" can be called as:

    tome-command dir foo


## Ignoring Directories

Directories can be ignored by either:

- Adding a `.tomeignore` file in the directory.
- Naming the directory with a `.` prefix (e.g. `.hidden-dir`).

## Tab completion of Directories

If one attempts to tab complete a directory, all valid tome scripts and 
subdirectories will be valid options. Non-executable files and ignored
directories are excluded from completion results.
