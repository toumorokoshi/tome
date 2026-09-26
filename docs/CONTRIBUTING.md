# Contributing

### Testing

`cargo test`

### Linting

`cargo clippy`

### Compiling for build platform (ie linux on linux)

```bash
# debug
cargo build
# release
cargo build --release
```

### Running with example scripts

To test tome locally against the example scripts:

```bash
./scripts/test-examples [ARGS...]
```