# Rewrite log (progress record)

## Revisions

- Foundation: `main` @ `82bccf5a3ea5369fac45f1d799e3ee3c4e5c60a2`.
- Session branch: `arena/699178e0-subsystem-for-arxumbra` (branched from
  exactly that commit — the main baseline is the branch point; no merge was
  needed).
- `arena/fae34dcf-subsystem-for-arxumbra`: not present on `origin`
  (`git ls-remote origin 'refs/heads/arena/*'` → empty). Comparison therefore
  uses the mandate's recorded observations only; documented in
  docs/architecture.md §2.
- Bevy: `v0.20.0` = `b3bdd75becaffe9a16677cf5bae5bd6d198f052b`;
  previous pin `v0.20.0-rc.2` = `343a0233d6788a49f224a3684c8995e246663df3`.
- wgpu (transitive): 30.x; default-semantics verified against `v30.0.1`.
- Toolchain pinned: stable `1.99.1` (latest stable per rust-lang/rust GitHub
  releases API on 2026-10-16; Bevy MSRV `1.97.1`).

## Stage status

- **A — repository & behavior audit**: done. No AGENTS.md/README existed;
  `.cargo/config.toml` (nightly, `target-cpu=native`, `build-std`, clang/lld),
  `Cargo.toml` (rc.2 pin + direct `wgpu = "30"`), single-file app
  (`src/main.rs`, 1745 lines, already single-chain strict style), six PNG
  assets, one release workflow with the old naming contract.
- **B — Bevy source audit & syntax feasibility**: done at source level.
  Both Bevy trees downloaded; per-crate drift diff rc.2→0.20.0 is additive in
  every used API; all constructed struct fields, enum variants, re-export
  paths, feature gates, and MSRV verified (docs/bevy-audit.md). Feasibility
  of the strict contract: the main baseline itself already demonstrates it
  (the app is written in exactly this style); the only Bevy-mandated
  relaxation is `&mut` in signature positions (documented, machine-checked).
- **C — minimal vertical slice**: not executed as a separate compile step —
  **blocked**: no Rust toolchain installable in the sandbox (rustup's
  distribution server `static.rust-lang.org` and crates.io are outside the
  network allowlist). Mitigation: every symbol verified against the pinned
  source trees instead of the compiler; first compile must happen downstream.
- **D — complete functional migration**: done (source-complete).
  `Cargo.toml`, `rust-toolchain.toml`, `src/main.rs` (helpers + single-chain
  main + 13 tests), `.github/workflows/{ci,release}.yml`, docs, README.
  Behavior preserved per docs/architecture.md §2.
- **E — evidence-based optimization**: partially done. Statically-justified
  work removal only (monitor fold merge, exit-check algebra, kept update
  gate); **no measurements possible** (see docs/performance.md — everything
  honestly labeled `not measured`).
- **F — CI and release packaging**: done (source-complete). Exact six-entry
  matrix, SHA-pinned actions (SHAs resolved via GitHub API and cross-checked
  with `git ls-remote`), least-privilege permissions, deterministic package
  layout, archives + checksums, guarded publication. YAML parse-validated
  locally with PyYAML; matrix parse simulated against the Rust test's
  algorithm. Remote execution not performed.
- **G — final verification**: partially done — see commands below.

## Commands executed in this environment and outcomes

| command | outcome |
| --- | --- |
| `git ls-remote origin` | only `main` + tag `v0.0.0.0.0.0-s0`; arena branch absent |
| GitHub refs API for `v0.20.0` / `v0.20.0-rc.2` | commits recorded above |
| codeload tarballs of both Bevy revisions + wgpu `v30.0.1` | downloaded, extracted, diffed/grepped |
| `diff -rq` per touched crate (rc.2 vs 0.20.0) | drift limited to additive changes (details in docs/bevy-audit.md) |
| ad-hoc structural scan of `src/main.rs` (Python mirror of the committed Rust checker) | 0 contract violations; brackets balanced; literals terminated; no `r#`/block comments |
| PyYAML parse of both workflows + matrix comparison against the mandated contract | parse OK; matrix equals the six required entries in order; all required `${{ matrix.* }}` expressions present; no doubled `.exe` |
| `git ls-remote` for the four pinned action tags | SHAs match the pins in the workflows |
| `cargo fmt/check/clippy/test/build`, `cargo metadata`, `cargo tree` | **NOT EXECUTED — no toolchain, no crates.io access** (hard blocker of this sandbox, not a skipped convenience) |

## Next steps (exact commands, networked machine with a GPU for the visual step)

```sh
git switch arena/699178e0-subsystem-for-arxumbra
cargo generate-lockfile          # regenerates Cargo.lock through Cargo (never by hand)
git add Cargo.lock && git commit -m "Regenerate Cargo.lock for Bevy v0.20.0"
cargo fmt --all                  # normalize any formatting drift, then re-run the checks
cargo fmt --all -- --check
cargo metadata --locked --format-version 1
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked              # behavior, syntax-contract, policy, and matrix tests
cargo build --locked --release
cargo tree --locked -d
./target/release/subsystem-for-arxumbra   # from the repository root (assets/ is cwd-relative)
```

If `cargo fmt` reformats `src/main.rs`, re-run `cargo test --locked`
afterwards — the syntax-contract test scans the formatted source too.

## Decisions log

1. Tag pinning (`tag = "v0.20.0"`) over `rev`: manifest readability; the
   resolved commit is recorded in `Cargo.lock` once regenerated (and is
   documented here: `b3bdd75b…`). Never both keys at once.
2. `Cargo.lock` deleted instead of kept stale: the old lock pins rc.2 +
   direct wgpu 30 and would contradict the new manifest under `--locked`.
   Workflows bootstrap it via `cargo generate-lockfile` when absent and use
   `--locked` everywhere else.
3. `WgpuLimits` → engine default (documented deviation, rationale in
   docs/architecture.md §5); all other baseline renderer settings preserved
   through public re-exports.
4. Cargo `version` bumped `0.0.0-s0` → `0.0.0-s1` to track the mandated
   distribution label's `s1` while staying valid semver; the label itself is
   never written into `Cargo.toml`.
5. Nightly removed everywhere (profile gate, `-Z` rustflags, `build-std`,
   `target-cpu=native`, clang/lld linker overrides); stable 1.99.1 pinned.
6. Helper-function extraction limited to pure numeric/logic functions that
   the mandated regression tests need; application assembly remains 100% in
   the `main` chain.
