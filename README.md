# fixture

A small, dependency-free Rust library built and tested with GitLab CI. It exposes two public functions:

- `add(a, b)` — returns the sum of two `i32` values.
- `double(a)` — returns an `i32` doubled.

## Project layout

- `src/lib.rs` — the library source and its unit tests (`mod tests`).
- `tests/integration.rs` — integration tests that consume the public API from outside the crate.
- `.gitlab-ci.yml` — the GitLab CI pipeline definition.
- `Cargo.toml` — the crate manifest (name `fixture`, edition 2021, no dependencies).

## Build

```sh
cargo build --all-targets
```

This compiles the library along with its test targets. Because the crate has no dependencies, the build is fast.

## Test

```sh
cargo test
```

`cargo test` runs both the unit tests in `src/lib.rs` and the integration tests in `tests/integration.rs`. All tests should pass.

## GitLab CI pipeline

The pipeline is defined in `.gitlab-ci.yml` and has a single `test` stage with one job, `unit`.

```yaml
stages:
  - test

unit:
  stage: test
  script:
    - cargo build --all-targets
    - cargo test
```

On every push, GitLab runs the `unit` job, which builds all targets and then runs the full test suite with `cargo test`. The pipeline succeeds only when the build and all tests pass.

To run the same steps locally, execute the two script commands above from the repository root with a Rust toolchain installed. To inspect the pipeline in GitLab, open the repository's **CI/CD → Pipelines** page and select the run for your commit.
