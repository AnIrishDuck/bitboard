# AGENTS.md

`bitboard` is a small, dependency-light Rust crate (serde + thiserror) for
variable-size chess bitboards. It is consumed by
[thane](https://github.com/AnIrishDuck/thane) as a sibling path dependency
(`../bitboard`), so changes here affect that workspace too.

## Build & test

```bash
cargo build
cargo test --lib --tests
```

Builds in seconds; no system libraries needed. If you're also working in
`../thane`, run `source ../thane/scripts/agent-env.sh` first so both share the
sccache setup (see `../thane/AGENTS.md`).

## Known issues (as of 2026-10-31)

- 10 doctests fail (`src/lib.rs` `BitBoard`, several in `src/square.rs`);
  they're stale examples, not library bugs. Plain `cargo test` reports
  failure because of them; `--lib --tests` (13 tests) passes.
- When changing public APIs, also build `../thane` (`cargo build --workspace`).
