# Contributing

### Testing

`cargo test`

### Linting

`cargo clippy`

### Compiling for build platform (ie linux on linux)

```
# debug
cargo build
# release
cargo build --release
```

### Cross compile (linux on darwin) release builds

This is a portable script that sets up ENV variables to allow for cross compiling
from darwin to linux or reverse.

```
./bin/release
```
