# Birthday app

## Develop

Ensure you have [rustup](https://rustup.rs/) toolchain working and PostgreSQL running

```
cp env.example .env # edit values there
cargo run
```

## Test

You need `cargo` and [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) working

```
cargo test
```

## Coverage

For some reason coverage directly inside Github Action crashes

```
cargo tarpaulin
```

Can show you the coverage:

```
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s

INFO cargo_tarpaulin::report: Coverage Results:
|| Uncovered Lines:
|| Tested/Total Lines:
|| src/app.rs: 8/8 +0.00%
|| src/errors.rs: 13/13 +0.00%
|| src/handlers.rs: 44/44 +0.00%
|| src/main.rs: 2/2 +0.00%
|| src/validation.rs: 8/8 +0.00%
|| src/view.rs: 4/4 +0.00%
||
100.00% coverage, 79/79 lines covered, +0.00% change in coverage

```