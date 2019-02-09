# Design Decisions

If you're interested in the design rationalle, read on!

## Why Rust?

For prototyping this tool was written in Python (execute.py can still be found in older versions of this code base), but due to how often the executable is invoked (e.g. multiple times for table completion), even relatively minor shutdown/startup times becamse apparent.

Rust was chosen because it allows performance, compiled binaries. Here's a comparison of how long the python script took to run vs the rust version:

Python:

```
$ time ./execute.py ~/workspace/tome/example dir_example --complete
bar foo

real    0m0.119s
user    0m0.086s
sys     0m0.038s
```

Rust:

```
$ time ./target/release/tome ~/workspace/tome/example dir_example --complete
bar foo
real    0m0.004s
user    0m0.001s
sys     0m0.003s
```

## Why Source the Initialization Script?