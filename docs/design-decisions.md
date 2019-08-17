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
$ time ./target/release/tome ~/workspace/tome/example dir_example --complete bar foo
real    0m0.004s
user    0m0.001s
sys     0m0.003s
```

In addition, this has a significant performance advantage over bash as well. Compare the help operation in sub vs tome:

Example of calling help using sub (bash):

```
$ time sub
real    0m0.122s
user    0m0.081s
sys     0m0.053s
```

Example of calling help using tome (rust):

```
$ time s
real    0m0.003s
user    0m0.003s
sys     0m0.000s
```

## Initialization Architecture

The init command returns back shell code that results in the creation of two functions: the function itself, and the completion function. Both are responsible for passing arguments along to the tome executable, along with the arguments and context like the directory containing the shell scripts.

## Completion: why pass --complete as the first argument?

Completion mode for sub commands is activated by passing "--complete" in as the first argument. This is done as there is not a succinct way in POSIX shell scripts to get something more dynamic, such as the last argument. Consider this example in bourne shell:

    # last argument
    for LAST_ARGUMENT; do true; done
    # first argument
    FIRST_ARG=$1