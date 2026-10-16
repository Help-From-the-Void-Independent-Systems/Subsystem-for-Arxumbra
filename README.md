# Subsystem for Arxumbra

A minimal 2D dashboard interface built on
[Bevy](https://github.com/bevyengine/bevy) **v0.20.0** (official Git tag,
commit `b3bdd75becaffe9a16677cf5bae5bd6d198f052b`), written under a strict
source-syntax contract: exactly one function — `fn main` — containing one
dominant, fully qualified application-construction chain; no
`use`/`let`/`mut` bindings/author-defined `const`/`static`/`unsafe`; no
comments in any maintained Rust, manifest, or workflow file (explanation
lives in `docs/`); and no direct `wgpu` dependency (renderer settings go
through verified `bevy::render::settings` re-exports).

## Behavior

- Borderless fullscreen window titled **Arxumbra** on the primary monitor,
  hidden cursor, black background; layout derives from the monitor size
  (1920×1080 fallback).
- **F1** toggles the HUD (divider, tab pills, Home / Extension Registry
  entries, badge, OS title, launch well).
- Clicking the **Launch** pill (only while the HUD is visible) switches from
  the Laboratory menu to the launched state (rocket slot).
- Holding **Escape** for 7 accumulated seconds exits with success; releasing
  resets the hold.
- Idle frames perform no component writes (the update pass is gated on
  relevant input), and every write that does happen is guarded by an
  equality comparison, so settled frames flag nothing as changed — engine
  change detection (visibility propagation, sprite mesh preparation) only
  runs for real transitions.

Icons and the app icon load from the external `assets/` folder; eight RGBA
textures (divider glow/core, header separator, pill, menu card, lab tile,
lab well, badge) are generated procedurally once at startup.

## Build & run

Requires the pinned stable toolchain (`rust-toolchain.toml` → 1.99.1; rustup
installs it automatically) and, on Linux, `pkg-config libwayland-dev
libxkbcommon-dev`.

```sh
cargo generate-lockfile   # only needed until Cargo.lock is committed
cargo build --release
./target/release/subsystem-for-arxumbra   # run from the repository root
```

`assets/` is resolved relative to the current working directory, so packaged
builds must be launched from the package root (executable next to `assets/`).

## Checks

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked    # honestly reports 0 tests — see below
```

The crate deliberately contains **no Rust test functions**: the governing
specification admits exactly one `fn` item (`main`) and prohibits `#[test]`
functions absent explicit approval, so `cargo test` reports zero tests and
CI records that count honestly. All static enforcement lives in the CI
**policy job** (`.github/workflows/ci.yml`): a syntax-aware Python scanner —
validated against embedded positive/negative fixtures before every run —
enforces the single-`fn` rule, the prohibited-keyword contract, the exact
nine-spelling `&mut` inventory, zero comments in maintained
Rust/TOML/YAML files, the dependency policy (Git Bevy v0.20.0, no direct or
renamed wgpu), the six-entry release matrix and exact executable filenames,
and the asset tree. The release workflow additionally verifies binary
headers against each target architecture, archive layout, executable-bit
preservation, extraction from a different working directory, and checksums,
and its publication job checks the downloaded file set against the matrix.
As black-box compensation for the absent Rust tests, the CI toolchain job
ends with a **headless process smoke check**: the release binary must run
Startup plus 12 seconds of Update frames under Xvfb with software Vulkan
(lavapipe) — no physical display — without panicking or exiting early.
Behavioral boundaries (F1/click same-frame semantics, Escape-hold exit) are
documented with truth tables in `docs/architecture.md` §5.2.

## Release packaging

`.github/workflows/release.yml` builds six platform packages on tag pushes
(`v*`). Each package is one archive named `<artifact>.tar.gz` containing
exactly the executable `<artifact><extension>` and `assets/`, with a
`.sha256` sibling; publication runs in a separate `contents: write` job only
for tag events. The artifact/executable label `…-v0.0.0.0.0.0-s1` is a fixed
external naming contract — it is intentionally **not** the Cargo package
version (`0.0.0-s1`), see `docs/architecture.md` §11.

## Documentation

- `docs/bevy-audit.md` — commit-pinned Bevy v0.20.0 source audit.
- `docs/architecture.md` — control map, baseline comparison, decisions.
- `docs/performance.md` — measurement record (honest `not measured` where
  nothing was measured) and reproducible procedures.
- `docs/rewrite-log.md` — revisions, stage status, executed commands.
