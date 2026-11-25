# R2-gitlab-ci-baseline

A minimal, dependency-free Rust library used as a baseline fixture for Octiq's
GitLab CI detection and import QA flow. It exists so that building and testing
stay fast while still exercising a real `cargo build` / `cargo test` pipeline.

AC 3. GitLab CI detection must be unchanged.

## Overview

The crate is named `fixture` (see `Cargo.toml`, edition 2021) and has no
dependencies. It intentionally does very little: the point is the surrounding
CI configuration and the way the project is detected and imported.

## What it provides

`src/lib.rs` exposes two public functions:

- `add(a: i32, b: i32) -> i32` — returns the sum of two integers.
- `double(a: i32) -> i32` — returns the value multiplied by two.

Both are covered by inline unit tests under `#[cfg(test)]`.

## Project layout

- `Cargo.toml` — crate metadata for the `fixture` package; no dependencies.
- `src/lib.rs` — library implementation and unit tests.
- `tests/integration.rs` — integration tests importing `fixture::{add, double}`
  from outside the crate to confirm the public API works externally.
- `.gitlab-ci.yml` — CI pipeline with a single `test` stage running
  `cargo build --all-targets` and `cargo test`.
- `.gitignore` — ignores the `target/` build directory.

## Building and testing

```sh
cargo build --all-targets
cargo test
```

The test suite comprises eleven unit tests and six integration tests, for
seventeen tests total.

## How to test

1. Octiq QA -> Import -> GitHub -> this repo
2. In chat: `add a comment at the top of the README`
3. Read the two lines in the reply.

**Line 1 (the CI verdict - THIS IS THE TEST):**

    Tests required - CI runs `cargo test`

**Line 2 (the build/test outcome):**

    Tests passed - 17 passed        (or a real failure, never "no recipe")
