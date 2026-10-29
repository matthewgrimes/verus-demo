# Rust / Verus examples

Each top-level `.rs` file in this directory is an independent Cargo example target. The root `Cargo.toml` and `Cargo.lock` let rust-analyzer discover all three files and resolve `vstd` without changing their standalone Verus entry points.

## Equal pairs

`equal_pairs.rs` maintains a hash table of frequencies. An element seen `c` times previously contributes `c` new pairs; its frequency then becomes `c + 1`. The standard-library `entry` API updates the count in place with a single entry lookup.

Expected running time is O(n), with O(k) auxiliary space for k distinct values. Hash-table operations are expected amortized constant time, not a worst-case constant-time guarantee. No full-size capacity reservation is made, keeping memory proportional to the number of distinct values for duplicate-heavy inputs. All proof functions, ghost variables, and specification expressions are erased from the executable.

The existing `spec_equal_pairs` definition and `equal_pairs` signature / requires / ensures clauses are preserved. The proof relates the table to the frequency of each value in the processed prefix, and relates the accumulator to the pair specification on that prefix. Prefix monotonicity and the input precondition establish that the addition cannot overflow.

Verify with a Verus executable on PATH:

```sh
verus examples/equal_pairs.rs
```

On this machine Verus is also installed at:

```sh
"$HOME/Library/Application Support/Code/User/globalStorage/verus-lang.verus-analyzer/verus/verus" examples/equal_pairs.rs
```

The interactive shell's `verus` alias points there; aliases need not exist in noninteractive shells. Verification succeeded with Verus 0.2026.10.18.7d82994: **6 verified, 0 errors**. No new assumptions or trusted function bodies were introduced.

Ordinary Rust compilation and regression tests (these do not replace verification):

```sh
cargo check --examples
cargo test --release --example equal_pairs
cargo test --release --example equal_pairs -- --ignored --nocapture
```

The first test run covers all 21,845 sequences of length 0 through 7 over four values, boundary keys including `u64::MAX`, and a duplicate-heavy case whose pair count exceeds `u32::MAX`. The ignored test is an optional timing comparison with a quadratic reference implementation.

The same tests were also compiled and run through Verus:

```sh
verus examples/equal_pairs.rs --compile --test -C opt-level=3 -o /tmp/equal-pairs-tests
/tmp/equal-pairs-tests
```

## Vim: file not included in a crate / module tree

Previously this directory had neither a Cargo manifest nor `rust-project.json`. The installed `coc-rust-analyzer` extension handles such directories by collecting the currently open Rust documents into `detachedFiles` when its language-server client starts. Files opened later can be absent from that initial crate list. This explains how `max.rs` and `max_iter.rs` can work while a newly opened `equal_pairs.rs` reports `unlinked-file`; the live Vim session's startup file list was not captured, so this is the likely sequence rather than a confirmed session trace.

The new Cargo manifest makes all `examples/*.rs` files discoverable as separate crates. No `mod equal_pairs;` declaration is needed. After saving any editor changes, restart Coc with:

```vim
:CocRestart
```

This recreates the client with project discovery instead of relying on the initial detached-file list. For later project refreshes, Coc also provides `:CocCommand rust-analyzer.reloadWorkspace`.

Rust-analyzer semantic highlighting depends on crate membership; Vim's basic `rust.vim` syntax highlighting is separate. If basic colors remain absent, inspect `:setlocal filetype? syntax?` (both should be `rust`). Standard rust-analyzer is not Verus-aware: normal Cargo macro expansion erases proof/specification code. Full Verus-specific editor analysis is a separate concern from the crate-membership error.

References: [rust-analyzer's unlinked-file diagnostic](https://rust-lang.github.io/rust-analyzer/src/ide_diagnostics/handlers/unlinked_file.rs.html), [non-Cargo project discovery](https://rust-analyzer.github.io/book/non_cargo_based_projects.html).
