# Script Filtering and Sourceable Scripts

authors: @toumorokoshi

## Background

Tome treats every file in the scripts directory as a command. This means
non-executable files such as READMEs, data files, and library scripts all
appear in help output and tab completion. Users have requested a mechanism to
exclude these files ([#53](https://github.com/toumorokoshi/tome/issues/53)).

Additionally, sourceable scripts (scripts that modify the current shell, e.g.
setting environment variables or changing directories) are currently indicated
by a `# SOURCE` comment header at the top of the file. This requires reading
and parsing the file contents to determine whether it should be sourced, which
adds overhead on every execution.

### Alternatives Considered

#### .tomeignore with gitignore syntax

A `.tomeignore` file at the root using `.gitignore`-style glob patterns was
proposed. While this uses a familiar syntax, it has performance drawbacks:

1. Reading and parsing the `.tomeignore` on every execution (or storing a
   compiled equivalent).
2. Performing that match against every single file to determine if it should
   be ignored. This would be required to validate everything from whether the
   script should appear in a list to whether it should execute at all.

These are O(n) operations per file, per invocation — undesirable for a tool
that strives to be fast and responsive.

#### --include=executables flag on init

An `--include=executables` flag passed during `tome init` was proposed. While
this avoids per-file parsing, it pushes the decision to init time and relies
on implicit behavior (filesystem permissions) that may not be obvious to users.

## Proposal

The following rules determine which files tome recognizes:

1. **Ignore any file that is non-executable.** Only files with the executable
   bit set (`chmod +x`) are recognized as commands. This check is a local
   stat call — O(1) per file with no additional parsing.

2. **Ignore any directory with a `.` prefix.** This preserves the existing
   behavior for hidden directories.

3. **If the file is intended to be sourced, it must end with a `.source`
   suffix.** It does not need to be executable. The `.source` suffix replaces
   the `# SOURCE` comment header, removing the need to read file contents to
   determine source eligibility.

### Advantages

- **Performance:** All checks are local filesystem operations (stat for
  executable bit, string suffix check for `.source`). No file reads, no
  pattern matching against an ignore list.
- **Explicitness:** The executable bit and `.source` suffix make the intent
  of each file clear from the directory listing alone.
- **Simplicity:** Users can include READMEs, data files, and helper libraries
  simply by not marking them executable.

### Breaking Changes

- Files that were previously recognized as commands but lack the executable
  bit will no longer appear. Users migrating from tome 1.x must ensure their
  scripts are marked executable (`chmod +x`).
- The `# SOURCE` comment header is no longer recognized. Sourceable scripts
  must be renamed with a `.source` suffix.

### Command Name Resolution

- A file named `foo.source` appears as the command `foo` (the `.source`
  suffix is stripped from the command name).
- When a user invokes `foo`, tome first checks for a file named `foo`, then
  falls back to `foo.source`.

## References

1. [GitHub Issue #53](https://github.com/toumorokoshi/tome/issues/53)
