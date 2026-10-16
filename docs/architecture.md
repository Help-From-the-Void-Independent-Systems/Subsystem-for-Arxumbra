# Architecture, control map, and preserved behavior

## 1. Revisions

| item | value |
| --- | --- |
| foundation branch | `main` @ `82bccf5a3ea5369fac45f1d799e3ee3c4e5c60a2` ("Add files via upload") |
| session branch | `arena/699178e0-subsystem-for-arxumbra`, branched from exactly `82bccf5a` — the `main` baseline is incorporated verbatim as the branch point |
| `arena/fae34dcf-subsystem-for-arxumbra` | **unavailable**: `git ls-remote origin 'refs/heads/arena/*'` returns nothing; the branch no longer exists on the remote, so no diff against it was possible (see §2) |
| remote tags | `v0.0.0.0.0.0-s0` @ `78c1b3a1ef28e25456bc9fae5297efb3308e36ac` |
| Bevy | tag `v0.20.0` @ `b3bdd75becaffe9a16677cf5bae5bd6d198f052b` (previous pin: `v0.20.0-rc.2` @ `343a0233d6788a49f224a3684c8995e246663df3`) |
| wgpu (transitive, via Bevy) | 30.x (`bevy_render` requires `wgpu = "30"`; default-semantics questions verified against `v30.0.1` source) |
| toolchain | stable `1.99.1` pinned in `rust-toolchain.toml` (Bevy MSRV `1.97.1`) |

## 2. Baseline comparison

`area | main behavior | arena changes | chosen behavior | verification`

The `arena/fae34dcf` branch could not be fetched (deleted on the remote), so
"arena changes" below reflects only what the rewrite mandate recorded about a
previously inspected arena revision; every row falls back to the `main`
baseline except where a mandate requirement forces a change.

| area | main behavior (82bccf5a) | arena changes (as recorded in the mandate; branch unavailable) | chosen behavior | verification |
| --- | --- | --- | --- | --- |
| Bevy pin | git tag `v0.20.0-rc.2` | — | git tag `v0.20.0` (final) | `Cargo.toml`; drift audit in docs/bevy-audit.md (rc.2→0.20.0 diffs are additive in all used APIs) |
| direct wgpu dep | `wgpu = { version = "30", default-features = false }`, used once for `wgpu::MemoryBudgetThresholds` | same field observed (`instance_memory_budget_thresholds: wgpu::MemoryBudgetThresholds { … }`) | **removed**; field covered by `..WgpuSettings::default()` | `MemoryBudgetThresholds::default() == { None, None }` proven from wgpu 30.0.1 source — identical to both baseline values; unit test `manifest_pins_git_bevy_v0_20_0_without_direct_wgpu` |
| GPU limits | hand-enumerated `WgpuLimits` with many zero fields | — | engine default limits under unchanged `WgpuSettingsPriority::Functionality` | bevy settings.rs#L91-L111 (defaults); rationale §5 below. Not compile-verified here (blocker, §9) |
| window/HUD/launch/exit behavior | borderless fullscreen "Arxumbra", F1 toggle, Launch pill click, 7 s Escape-hold exit, gated update pass | Dashboard resource, per-frame `Vec<Entity>::clone()` (recorded observations) | main baseline preserved exactly | pure-logic regression tests in `src/main.rs` (`launch_hit`, `next_hold`, `exit_reached`, `hud_gate`, `hud_visibility`, `slot_tint`) |
| per-frame allocation on paint path | none in main (no `Dashboard.ids`; state read back from ECS each gated frame) | `Vec<Entity>::clone()` per frame | main behavior kept (no clone exists to remove) | source inspection; `hud_gate` test documents the no-input-no-write property |
| monitor lookups | two `world.query::<&Monitor>()` folds per read site (width, height separately) | — | one fold returning `(width, height)` per site | Startup/Update source; semantics identical (last-monitor-wins fold, 1920×1080 fallback) |
| exit-hold accumulation | `&mut [bool; 1]` cell hack + per-entity `scale.z >= 7.0` re-check | — | computed from the uniform hold: `exit_reached(next_hold(hold, delta, escape))` | algebraically identical given the uniform-`scale.z` invariant (all entities spawn at 0.0 and are updated by the same expression); tests `escape_hold_accumulates_resets_and_crosses_the_exit_boundary` |
| toolchain | nightly (`cargo-features panic-immediate-abort`, `-Z` rustflags, `build-std`, `target-cpu=native`) | — | stable 1.99.1 pinned; `panic = "abort"`; portable codegen | `Cargo.toml`, `rust-toolchain.toml`; `.cargo/config.toml` deleted; policy test asserts it stays deleted |
| release naming | artifacts `subsystem-for-arxumbra-<platform>-<arch>`, executable + separate `-assets` folder | — | mandated six-entry matrix, artifact `…-v0.0.0.0.0.0-s1`, one archive per platform containing renamed executable + `assets/` | `.github/workflows/release.yml`; Rust tests parse the matrix structurally and assert the six exact executable filenames |
| Cargo version | `0.0.0-s0` | — | `0.0.0-s1` (valid semver) — the fixed label `v0.0.0.0.0.0-s1` is an external contract, *not* a Cargo version, and never appears in `Cargo.toml` | test `distribution_label_stays_separate_from_the_cargo_version` |
| assets | six external PNGs under `assets/`, procedural textures at startup, `include_bytes!` explicitly forbidden by the old workflow check | — | unchanged (six PNGs verified by magic bytes in tests and workflow; procedural textures generated once at startup) | test `external_assets_exist_and_are_png_files`; release workflow asset validation step |

No compatible arena improvement could be preserved beyond what `main` already
contained, because the arena branch is not retrievable; this is recorded as a
blocker-free fact, not an assumption. The main baseline already contains the
"gated update pass" optimization the mandate asked to keep.

## 3. Linear composition in `main`

`src/main.rs` has exactly one runtime entry point and one dominant expression:

```text
bevy::app::App::new()
  → .add_plugins((TaskPoolPlugin{…}, FrameCountPlugin, TimePlugin,
                  TransformPlugin, InputPlugin, AccessibilityPlugin,
                  WindowPlugin{…}, AssetPlugin{…}, WinitPlugin{…}))
  → .add_plugins((RenderPlugin{…}, ImagePlugin{…}, MeshPlugin, CameraPlugin,
                  PipelinedRenderingPlugin, CorePipelinePlugin, SpritePlugin,
                  SpriteRenderPlugin, TextPlugin))
  → .add_systems(Startup, |world: &mut World| { clear color; camera;
                  (|(width, height), world| { (13 handles, world) => 24 spawns;
                                              (badge, world) => 2 spawns; })(monitor fold, world) })
  → .add_systems(Update,  |world: &mut World| { (inputs…, (width,height), world) =>
                  ((hidden, launched, hold), world) => if hud_gate { exit check; uniform write pass } })
  → .run();
```

Plugin configuration literals sit structurally next to the plugin they
configure. There is no `build_app`-style wrapper, no custom plugin, no
extension trait, no macro, and no second execution loop. Named functions
outside `main` are pure numeric/logic helpers with two sanctioned purposes
(reuse in the chain + regression tests); each is documented at its definition.

## 4. Control map

`subsystem | owner | configuration location | execution order | project-controlled behavior | engine/OS/driver-controlled behavior`

| subsystem | owner | configuration location | execution order | project-controlled | engine/OS/driver-controlled |
| --- | --- | --- | --- | --- | --- |
| window & presentation | `WindowPlugin` + winit | `main` chain: `Window` literal (borderless fullscreen on primary monitor, title, hidden cursor, min 180×120, non-resizable, `prevent_default_event_handling: true`) | before `Startup` systems; window events drive the `Update` schedule via `WinitPlugin` (`run_on_any_thread: false` → main thread owns the event loop) | mode, title, constraints, cursor visibility/grab, exit condition `OnAllClosed` | actual monitor resolution, DPI scaling, WM compositing, vsync behavior (`PresentMode::AutoVsync` is a request) |
| time | `TimePlugin` | engine defaults | `Time` updated before `Update` | only consumption (`delta_secs` for the hold accumulator) | clock source, delta smoothing/maximum |
| input | `InputPlugin` + winit | engine defaults | `ButtonInput` resources refreshed per frame before `Update` | F1 edge, Escape level, left-click edge, cursor position read | key repeat, device mapping, focus semantics |
| assets | `AssetPlugin` | `main` chain: `assets/` relative folder, `AssetMode::Unprocessed`, `AssetMetaCheck::Never`, `UnapprovedPathMode::Forbid` | `AssetServer` streams IO on the io task pool (1–4 threads); handles resolve asynchronously | paths, unprocessed mode, load sites (six PNGs at startup), failure policy (Forbid) | filesystem IO timing, PNG decode threads, load-failure propagation via asset events |
| ECS scheduling | `App` schedules | `Startup` (one exclusive closure), `Update` (one exclusive closure) | `Startup` once after plugin `finish/cleanup`; `Update` every frame; deferred commands none (exclusive world access applies changes immediately) | the two systems' entire logic; gate that suppresses writes on no-input frames | task-pool thread counts (explicit policy: io/async ≤ 4, compute = all cores), system execution across frames |
| rendering | `RenderPlugin` + core pipeline + sprite/text plugins | `main` chain: backends VULKAN∥METAL∥DX12, `HighPerformance`, `Functionality` priority, `TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`, FXC on DX12, `VALIDATION_INDIRECT_CALL`, `MemoryHints::Performance`, rest = `WgpuSettings::default()`; `PipelinedRenderingPlugin` keeps extraction on the render sub-app | extraction → preparation → render schedules, pipelined against the next main-world update | backend set, requested features/priority, MSAA off, clear color black, sprite/text layering via `Transform.translation.z` and 2D draw order | adapter/device selection within the backend set, actual limits (Functionality priority negotiates with the adapter), presentation, driver behavior |
| GPU resource management | Bevy render world | engine defaults | per-frame extraction of changed sprites/text | nothing below the `Image`/`Sprite` component level (deliberate: standard sprite/text rendering satisfies this app; no custom node/pipeline) | buffer/texture allocation, memory budgets (`MemoryBudgetThresholds` defaults), shader compilation (async) |
| startup/shutdown | `App::run` | terminal `.run()` in `main` | startup → frame loop → `AppExit` message or last-window-closed → plugin cleanup | writing `AppExit::Success` after the 7 s Escape hold; `ExitCondition::OnAllClosed` | process exit code plumbing, winit loop teardown |

Exclusive `&mut World` systems are retained deliberately (baseline
architecture): the app is one tiny HUD; typed query items
(`&mut Visibility`, `&mut Transform`, `Option<&mut Sprite>`) still scope
access inside the query, and there is no parallel system work to serialize.
No `Cell`/`RefCell`/locks/atomics were introduced anywhere.

## 5. Notable decisions

- **GPU limits**: the baseline enumerated every `wgpu::Limits` field by hand,
  with dozens of zeros (e.g. `max_color_attachment_bytes_per_sample: 32`,
  `max_uniform_buffer_binding_size: 65536`). Those are *requests below* what
  Bevy's own 2D pipelines generally require, risk adapter-request failure, and
  cannot be expressed at all without naming `wgpu::Limits` fields through a
  forbidden direct dependency. The rewrite requests nothing beyond
  `WgpuSettings::default()` (which under `Functionality` priority is
  `wgpu::Limits::default()` — conservative, widely supported values), while
  keeping every baseline setting that *is* expressible through public Bevy
  re-exports. This follows the mandate's "request only capabilities needed"
  and is flagged as an intentional, documented deviation — not a silent one.
- **`instance_memory_budget_thresholds`**: not publicly nameable in 0.20.0
  (private `use wgpu::MemoryBudgetThresholds` in settings.rs). The struct
  update `..WgpuSettings::default()` yields `{ None, None }` — proven equal to
  the baseline's explicit value from the wgpu 30.0.1 source.
- **Monitor reads**: one `(width, height)` fold per read site instead of two
  `QueryState` constructions (Startup and Update), same last-wins semantics
  and same 1920×1080 fallback.
- **Exit detection**: algebraically identical replacement of the
  `&mut [bool; 1]` cell + per-entity re-check by `exit_reached(next_hold(…))`,
  valid because `scale.z` is uniform across all non-camera entities by
  construction (spawned 0.0, updated by one expression). Removes one mutable
  cell plumbing from the hot path without changing observable behavior.
- **Profiles**: `panic = "abort"` (stable) replaces nightly
  `immediate-abort`; fat LTO/1 codegen unit/strip kept; `target-cpu=native`,
  `-Z location-detail=none`, `-Z trap-unreachable`, and `build-std` removed —
  they made release binaries non-portable (native ISA) and required nightly.
  Diagnostic trade-off: `abort` loses unwinding but keeps the panic message;
  asset-load failures still surface through Bevy's asset error handling.

## 6. Naming identities (Section 11.4 separation)

| identity | value |
| --- | --- |
| Cargo package name | `subsystem-for-arxumbra` |
| Cargo binary target name | `subsystem-for-arxumbra` (verified from `cargo metadata` in the release workflow, not assumed) |
| Cargo package version | `0.0.0-s1` (valid semver) |
| fixed distribution label | `v0.0.0.0.0.0-s1` — external naming contract only; six numeric segments make it invalid as a Cargo version, so it never appears in `Cargo.toml` |
| build job display name | `${{ matrix.artifact }}` (e.g. `x86_64-Linux-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1`) |
| uploaded artifact name | `${{ matrix.artifact }}` |
| archive name | `${{ matrix.artifact }}.tar.gz` (+ `.sha256` sibling, combined `SHA256SUMS` at publish time) |
| actual executable filename inside the archive | `${{ matrix.artifact }}${{ matrix.extension }}` — the six exact filenames are asserted by `staged_executable_names_are_the_six_required_filenames` |
| release tag | whatever `v*` tag triggers the workflow; if it differs from the fixed label, the label still governs file names (distinction documented here, per mandate) |

## 7. Package layout and asset resolution

```text
dist/<artifact>/                    → archived as <artifact>.tar.gz (root of archive = package root)
  <artifact><extension>             → the executable, renamed from target/<triple>/release/subsystem-for-arxumbra<extension>
  assets/                           → copy of the repository assets/ tree
    app-icon.png
    icons/{flask,house,registry,rocket,rocket-pill}.png
```

`AssetPlugin.file_path = "assets"` resolves relative to the **current working
directory** (`bevy_asset` uses `std::env::current_dir`-based
`FileAssetReader`), so the package must be launched from its own root (e.g.
`./<artifact>` inside the extracted directory). This matches the baseline
behavior and is validated in the workflow by checking the real staged files.
Unix executable bits are preserved inside `.tar.gz`; the workflow re-applies
`chmod +x` before archiving and verifies `test -x` after staging.

## 8. Strict-syntax exceptions (explicit, narrow, Bevy/std-mandated)

The only relaxation from the literal contract, present already in the `main`
baseline and unavoidable with safe public APIs:

- `&mut` in *signature positions* required by library contracts:
  `|world: &mut bevy::ecs::world::World|` exclusive-system closures,
  `&mut Visibility` / `&mut Transform` / `&mut Sprite` inner-closure
  parameters fed from `Mut::into_inner()`, and
  `state: &mut (…)` parameters of `Iterator::scan` / `Option::last_mut` items
  in the test module. No `mut` binding, `mut` parameter, `Cell`-family
  interior mutability, `unsafe`, raw pointer, or codegen escape hatch exists
  anywhere; the automated checker (`strict_syntax_contract_holds_…`) enforces
  exactly this rule set, token-boundary aware, over every handwritten `.rs`
  file.
- `'static` lifetime: unused in project source today; permitted by contract
  if a future Bevy signature requires it (the checker already whitelists it).

## 9. Known limitations of this rewrite environment

- No Rust toolchain and no crates.io access existed in the rewrite sandbox
  (network allowlist: github.com/codeload/api.github/npm/pypi). Consequently
  `cargo fmt/check/clippy/test/build` and `cargo generate-lockfile` were
  **not executed**; verification was source-level (pinned Bevy/wgpu trees,
  drift diff, structural scans). `Cargo.lock` was removed rather than left
  stale — regenerate and commit it as the first step on a networked machine
  (exact commands in docs/rewrite-log.md §Next steps).
- GPU/visual checks were not executed (no display). Reproducible scenario
  list in docs/performance.md.
