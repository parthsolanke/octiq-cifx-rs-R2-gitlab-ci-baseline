# R2-gitlab-ci-baseline

A small, dependency-free Rust crate (`fixture`) used as a **GitLab CI baseline fixture**. Its purpose is to verify that GitLab CI detection and test execution stay correct: a `.gitlab-ci.yml` at the repo root declares a `test` stage whose `unit` job builds and tests the crate.

## What the CI represents

The `.gitlab-ci.yml` defines:

- `stages: [test]`
- a `unit` job in the `test` stage that runs:
  - `cargo build --all-targets`
  - `cargo test`

This is the baseline case: CI is expected to be detected and tests are required. A correct verdict for this repo is:

```
Tests required - CI runs `cargo test`
```

The build/test outcome should be a real result (e.g. `Tests passed - 4 passed`), never `no recipe`.

## Project layout

- `Cargo.toml` — package manifest (`fixture`, edition 2021, no dependencies)
- `src/lib.rs` — library with `add(a, b)` and `double(a)` plus inline unit tests
- `tests/integration.rs` — integration tests using the public `add`/`double` API
- `.gitlab-ci.yml` — the GitLab CI pipeline shown above

## Build and test

```sh
cargo build --all-targets
cargo test
```

`cargo test` runs both the unit tests in `src/lib.rs` and the integration tests in `tests/integration.rs` (4 tests total).

## How to test the fixture

1. Octiq QA -> Import -> GitHub -> this repo
2. In chat: `add a comment at the top of the README`
3. Read the two lines in the reply.

**Line 1 (the CI verdict — THIS IS THE TEST):**

```
Tests required - CI runs `cargo test`
```

**Line 2 (the build/test outcome):**

```
Tests passed - 4 passed        (or a real failure, never "no recipe")
```
