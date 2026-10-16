# Performance record

**Honesty rule applied: every row below is `not measured` unless a real
measurement with recorded conditions exists. No numbers are invented.**

The rewrite sandbox had no Rust toolchain, no crates.io access, no GPU, and no
display server (network allowlist: github.com, codeload.github.com,
api.github.com, registry.npmjs.org, pypi.org). Nothing could be compiled or
run, so no before/after measurement was possible here. The baseline itself was
also never compiled in this environment (its `Cargo.lock` existed from a
previous, networked build against `v0.20.0-rc.2`).

| metric | baseline (main @ 82bccf5a) | rewritten result | conditions | interpretation |
| --- | --- | --- | --- | --- |
| startup time (first frame) | not measured | not measured | — | requires a GPU machine; procedure below |
| CPU frame time, warm idle (no input) | not measured | not measured | — | update path is gated off; expected dominated by Bevy schedule + pipelined extraction |
| CPU frame time, active (Escape held) | not measured | not measured | — | gated write pass over ~26 entities |
| GPU frame time | not measured | not measured | — | needs correct instrumentation (e.g. tracing/profiling render pass); vsync-limited FPS alone is explicitly not accepted as the metric |
| median/p95/p99 frame times | not measured | not measured | — | sample ≥ 60 s warm, after ≥ 5 s warmup |
| allocations in application-owned hot path | not measured | not measured | — | static analysis: the gated update pass allocates nothing application-side (no `Vec`/`String` construction; `QueryState` construction per call is engine-arena work). A global allocator counter on a networked machine is the honest way to confirm |
| resident memory / GPU memory | not measured | not measured | — | tooling-dependent |
| executable size | not measured | not measured | — | expected smaller than baseline: `panic="abort"` vs nightly `immediate-abort` differs little, but removal of `build-std` changes std codegen; must be measured per platform |
| package size (archive) | not measured | not measured | — | executable + six PNGs (~few hundred KB of assets) |
| clean build time | not measured | not measured | — | record per runner: Bevy-from-git fat-LTO release builds are long; CI cache separates by OS/target/toolchain/lockfile |
| incremental build time | not measured | not measured | — | `incremental = false` in release (deliberate); debug profile untouched |

## Work removed or avoided (static, verifiable by inspection — not a runtime claim)

| change | rationale | runtime claim status |
| --- | --- | --- |
| Monitor geometry read once per site (`(width, height)` fold) instead of two `world.query::<&Monitor>()` constructions per site | duplicate `QueryState` construction + iteration for one coherent value pair | unverified hypothesis until measured; zero behavior change |
| Exit detection computed from the uniform hold value instead of a `&mut [bool;1]` cell re-checked per entity | removes mutable-cell plumbing and a per-entity OR from the write pass | unverified hypothesis; algebraically identical |
| Update gate preserved (`hud_gate`): idle frames with no input and zero hold perform only the input reads + one read-only fold | baseline optimization kept and regression-tested | behavior encoded in tests |
| Per-frame `Vec<Entity>::clone()` (recorded arena-branch issue) | does not exist in the chosen `main` baseline; nothing to remove | n/a |
| Procedural textures generated exactly once in `Startup` | baseline behavior kept | n/a |

No floating-point expression, branch structure, or iterator chain was
replaced by a "supposedly faster" alternative; the pixel shaders are
byte-for-byte the baseline formulas, now in named functions.

## Reproducible measurement procedure (for a machine with GPU + display)

Record: hardware, OS, `rustc --version`, commit SHA, feature set, profile,
rendering backend (from Bevy's startup log), resolution, present mode,
warmup ≥ 5 s, sample ≥ 60 s, ≥ 3 repetitions; report median/p95/p99.

1. `cargo generate-lockfile && cargo build --release` (rewritten tree).
2. Baseline comparison build: `git worktree add /tmp/baseline 82bccf5a` —
   note the baseline requires **nightly** (`cargo-features
   panic-immediate-abort`, `-Z` rustflags, `build-std`) and its rc.2 pin;
   if it does not build on the current nightly, record the blocker instead of
   silently repairing it.
3. Scenarios: (a) launch → first visible frame; (b) warm idle 60 s (no
   input); (c) Escape held 6 s then released, repeated; (d) F1 toggle burst
   1 Hz for 30 s; (e) Launch click → state transition latency; (f) 7 s Escape
   hold → process exit.
4. Instrument CPU frame time with `bevy::diagnostic::FrameTimeDiagnosticsPlugin`
   added **only in the measurement build** (not in the shipped binary), or an
   external profiler; label GPU numbers with the instrumentation used.
5. Visual smoke scenarios: HUD toggle, tab pills rendering, Laboratory card
   before launch, rocket slot after launch, faded/transparent slot tint at
   idle, black clear color, hidden cursor, OS title string per platform,
   icon loading from an extracted package launched **outside** the repository
   directory (cwd = package root).
