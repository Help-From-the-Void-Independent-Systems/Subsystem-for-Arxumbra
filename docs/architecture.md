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
| spec revision | third revision of the rewrite mandate (restructured; adds: exactly one `fn` item — no helpers, no `#[test]` functions; no explanatory comments in any maintained `.rs`/`.toml`/`.yml` file, with explanation moved to documentation; flattened closure nesting; a combined feasibility gate executed outside the maintained tree; error transparency in workflows; regression-controlled optimization; a final whole-tree re-scan; a ten-item final report) — implemented by the single-`fn` transformation of `src/main.rs` (2798 → 1516 lines), comment-free manifests and workflows, and the CI policy job |

## 2. Baseline comparison

`area | main behavior | arena changes | chosen behavior | verification`

The `arena/fae34dcf` branch could not be fetched (deleted on the remote), so
"arena changes" below reflects only what the rewrite mandate recorded about a
previously inspected arena revision; every row falls back to the `main`
baseline except where a mandate requirement forces a change.

| area | main behavior (82bccf5a) | arena changes (as recorded in the mandate; branch unavailable) | chosen behavior | verification |
| --- | --- | --- | --- | --- |
| Bevy pin | git tag `v0.20.0-rc.2` | — | git tag `v0.20.0` (final) | `Cargo.toml`; drift audit in docs/bevy-audit.md (rc.2→0.20.0 diffs are additive in all used APIs) |
| direct wgpu dep | `wgpu = { version = "30", default-features = false }`, used once for `wgpu::MemoryBudgetThresholds` | same field observed (`instance_memory_budget_thresholds: wgpu::MemoryBudgetThresholds { … }`) | **removed**; field covered by `..WgpuSettings::default()` | `MemoryBudgetThresholds::default() == { None, None }` proven from wgpu 30.0.1 source — identical to both baseline values; enforced on every CI run by the policy job's manifest scan (package-identity check across all dependency sections, including renamed deps) |
| GPU limits | hand-enumerated `WgpuLimits` with many zero fields | — | engine default limits under unchanged `WgpuSettingsPriority::Functionality` | bevy settings.rs#L91-L111 (defaults); rationale §10 below. Not compile-verified here (blocker, §14) |
| window/HUD/launch/exit behavior | borderless fullscreen "Arxumbra", F1 toggle, Launch pill click, 7 s Escape-hold exit, gated update pass | Dashboard resource, per-frame `Vec<Entity>::clone()` (recorded observations) | main baseline preserved exactly | every transition expression is byte-identical to the baseline (inlined; §3/§5.2) and was re-verified by Python simulation of the boundary truth tables (7/7); Rust test functions are prohibited by the third-revision spec and were removed — runtime verification remains NOT RUN (no toolchain) |
| write-path change detection | `Mut::into_inner()` consumed unconditionally per gated frame, then assigned — in v0.20.0 `into_inner` **sets the CHANGED tick itself**, so every gated frame flagged `Visibility`/`Transform`/`Sprite` on all ~26 entities even when values were equal | — | guarded writes: values compared through non-flagging `Deref` reads; the `Mut` wrapper is consumed only when the next value differs (§8) | engine semantics read from `change_detection/traits.rs#L516-L523`; consumer gates read from `bevy_camera` visibility propagation and `bevy_sprite_render` mesh preparation; final values and frame timing identical to baseline |
| input lookups | `ButtonInput<KeyCode>` resource fetched twice per frame (F1 edge, Escape level) | — | one coherent keyboard snapshot: single borrow yields both readings (§22 property) | Update closure source |
| launch transition | inline `launched \|\| (pressed && hidden == f1 && launch_hit(…))` | — | the same expression, inlined into the Update chain (§3); same-frame toggle+click boundary defined against post-toggle visibility and documented (§5.2) | Python simulation of the inlined expression's truth table (7/7 boundary assertions pass) |
| per-frame allocation on paint path | none in main (no `Dashboard.ids`; state read back from ECS each gated frame) | `Vec<Entity>::clone()` per frame | main behavior kept (no clone exists to remove) | source inspection; the gate expression admits no write path on no-input frames |
| monitor lookups | two `world.query::<&Monitor>()` folds per read site (width, height separately) | — | one fold returning `(width, height)` per site | Startup/Update source; semantics identical (last-monitor-wins fold, 1920×1080 fallback) |
| exit-hold accumulation | `&mut [bool; 1]` cell hack + per-entity `scale.z >= 7.0` re-check | — | computed from the uniform hold: the inlined next-hold expression feeds the `>= 7.0` exit check once per gated frame | algebraically identical given the uniform-`scale.z` invariant (all entities spawn at 0.0 and are updated by the same expression); accumulate/reset/boundary cases simulated in Python |
| toolchain | nightly (`cargo-features panic-immediate-abort`, `-Z` rustflags, `build-std`, `target-cpu=native`) | — | stable 1.99.1 pinned; `panic = "abort"`; portable codegen | `Cargo.toml`, `rust-toolchain.toml`; `.cargo/config.toml` deleted; the CI policy job asserts it stays deleted and that no nightly manifest gate exists |
| release naming | artifacts `subsystem-for-arxumbra-<platform>-<arch>`, executable + separate `-assets` folder | — | mandated six-entry matrix, artifact `…-v0.0.0.0.0.0-s1`, one archive per platform containing renamed executable + `assets/` | `.github/workflows/release.yml`; the CI policy job (PyYAML parse) **and** the build job's independent line-scanner self-check both assert the six exact entries and executable filenames on every run |
| Cargo version | `0.0.0-s0` | — | `0.0.0-s1` (valid semver) — the fixed label `v0.0.0.0.0.0-s1` is an external contract, *not* a Cargo version, and never appears in `Cargo.toml` | CI policy job: semver-shape validation of the manifest version + literal-absence check of the label |
| assets | six external PNGs under `assets/`, procedural textures at startup, `include_bytes!` explicitly forbidden by the old workflow check | — | unchanged (six PNGs verified by magic bytes; procedural textures generated once at startup) | CI policy job asset scan (exact tree + PNG magic bytes) |

No compatible arena improvement could be preserved beyond what `main` already
contained, because the arena branch is not retrievable; this is recorded as a
blocker-free fact, not an assumption. The main baseline already contains the
"gated update pass" optimization the mandate asked to keep.

## 3. Linear composition in `main` (third-revision structure)

`src/main.rs` contains **exactly one `fn` item** — `fn main` on line 1 — and
one dominant expression. There are no helper functions, no `impl` blocks, no
macros, no attributes, and no comments anywhere in the file (the third
revision prohibits all of these; explanation lives in this document):

```text
bevy::app::App::new()
  → .add_plugins((TaskPoolPlugin{…}, FrameCountPlugin, TimePlugin,
                  TransformPlugin, InputPlugin, AccessibilityPlugin,
                  WindowPlugin{…}, AssetPlugin{…}, WinitPlugin{…}))
  → .add_plugins((RenderPlugin{…}, ImagePlugin{…}, MeshPlugin, CameraPlugin,
                  PipelinedRenderingPlugin, CorePipelinePlugin, SpritePlugin,
                  SpriteRenderPlugin, TextPlugin))
  → .add_systems(Startup, |world: &mut World| {
        clear color; camera spawn;
        (|(width, height), world| {                       monitor IIFE
            (|13 handles, badge, world| {                 handles IIFE
                24 HUD spawns; 2 badge spawns
            })(13 image add/load expressions, badge add, world)
        })(monitor fold, world) })
  → .add_systems(Update, |world: &mut World| {
        (|f1, escape, pressed, delta, cursor, (width, height), world| {
                                                              snapshot IIFE
            if f1 || escape || pressed || hold != 0.0 {        gate
                (|hidden, launched, hold_next, world| {        next-values IIFE
                    exit check at hold_next >= 7.0;
                    guarded write pass: query_filtered iter_mut for_each
                      → classification IIFE (Name fold)
                      → visibility IIFE
                      → tint IIFE
                })(derived expressions, world)
            }
        })(input reads, monitor fold, world) })
  → .run();
```

Structural rules this shape follows:

- **One nesting level per distinct role.** Startup reaches 3 closure levels
  (exclusive system → monitor IIFE → handles IIFE); Update reaches 7 (system
  → snapshot → gate block → next-values → per-entity `for_each` →
  classification → visibility → tint IIFEs). No level exists merely to avoid
  a keyword: each binds a coherent value set (monitor size, asset handles,
  input snapshot, derived next-values, per-entity classification) that the
  next level consumes, and each IIFE's argument list is evaluated in the
  enclosing scope — which is how `world` threads through as the final
  argument without any `let`/`mut` binding.
- **Pixel formulas are inlined, byte-identical.** The procedural-texture
  generators (divider glow/core, header separator, pill, menu card, lab tile,
  lab well, badge) appear as immediately-applied closure expressions
  `(|x, y, …| { body })(args)` at their single call site inside the image
  literals. The bodies are the baseline formulas character-for-character
  (f32 constants re-verified), so no visual value changed; the second
  revision's named helpers were dissolved into these expressions because the
  third revision admits no second `fn` item.
- **Badge composition.** The badge's two spawns live inside the merged
  handles IIFE (one 15-parameter closure: 13 shared handles + badge handle +
  world) instead of the second revision's separate badge IIFE — fewer
  closures, identical spawn order and values.
- **Flattened Update chain.** The second revision's four nested read-back
  IIFEs collapsed into one snapshot IIFE (single keyboard borrow, cursor,
  monitor, state fold) whose body holds the gate, and the write pass's
  per-entity logic reads as three small same-level IIFEs rather than a
  deeply-nested expression tower.

Plugin configuration literals sit structurally next to the plugin they
configure. There is no `build_app`-style wrapper, no custom plugin, no
extension trait, and no second execution loop. The whole file is 1516 lines;
the CI policy job re-verifies the single-`fn`, no-comment, keyword, `&mut`-inventory,
and bracket-balance properties on every push, pull request, and tag (see §13).

## 4. Control map

`subsystem | owner | configuration location | execution order | project-controlled behavior | engine/OS/driver-controlled behavior`

| subsystem | owner | configuration location | execution order | project-controlled | engine/OS/driver-controlled |
| --- | --- | --- | --- | --- | --- |
| window & presentation | `WindowPlugin` + winit | `main` chain: `Window` literal (borderless fullscreen on primary monitor, title, hidden cursor, min 180×120, non-resizable, `prevent_default_event_handling: true`) | before `Startup` systems; window events drive the `Update` schedule via `WinitPlugin` (`run_on_any_thread: false` → main thread owns the event loop) | mode, title, constraints, cursor visibility/grab, exit condition `OnAllClosed` | actual monitor resolution, DPI scaling, WM compositing, vsync behavior (`PresentMode::AutoVsync` is a request) |
| time | `TimePlugin` | engine defaults | `Time` updated before `Update` | only consumption (`delta_secs` for the hold accumulator) | clock source, delta smoothing/maximum |
| input | `InputPlugin` + winit | engine defaults | `ButtonInput` resources refreshed per frame before `Update` | F1 edge, Escape level, left-click edge, cursor position read — as one coherent keyboard snapshot plus per-resource reads | key repeat, device mapping, focus semantics |
| assets | `AssetPlugin` | `main` chain: `assets/` relative folder, `AssetMode::Unprocessed`, `AssetMetaCheck::Never`, `UnapprovedPathMode::Forbid` | `AssetServer` streams IO on the io task pool (1–4 threads); handles resolve asynchronously | paths, unprocessed mode, load sites (six PNGs at startup), failure policy (Forbid) | filesystem IO timing, PNG decode threads, load-failure propagation via asset events |
| ECS scheduling | `App` schedules | `Startup` (one exclusive closure), `Update` (one exclusive closure) | `Startup` once after plugin `finish/cleanup`; `Update` every frame; deferred commands none (exclusive world access applies changes immediately) | the two systems' entire logic; gate that suppresses writes on no-input frames; equality guards that suppress no-op writes (§8) | task-pool thread counts (explicit policy: io/async ≤ 4, compute = all cores), system execution across frames |
| rendering | `RenderPlugin` + core pipeline + sprite/text plugins | `main` chain: backends VULKAN∥METAL∥DX12, `HighPerformance`, `Functionality` priority, `TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`, FXC on DX12, `VALIDATION_INDIRECT_CALL`, `MemoryHints::Performance`, rest = `WgpuSettings::default()`; `PipelinedRenderingPlugin` keeps extraction on the render sub-app | extraction → preparation → render schedules, pipelined against the next main-world update | backend set, requested features/priority, MSAA off, clear color black, sprite/text layering via `Transform.translation.z` and 2D draw order | adapter/device selection within the backend set, actual limits (Functionality priority negotiates with the adapter), presentation, driver behavior |
| GPU resource management | Bevy render world | engine defaults | per-frame extraction of changed sprites/text | nothing below the `Image`/`Sprite` component level (deliberate: standard sprite/text rendering satisfies this app; no custom node/pipeline — §6) | buffer/texture allocation, memory budgets (`MemoryBudgetThresholds` defaults), shader compilation (async) |
| startup/shutdown | `App::run` | terminal `.run()` in `main` | startup → frame loop → `AppExit` message or last-window-closed → plugin cleanup | writing `AppExit::Success` after the 7 s Escape hold; `ExitCondition::OnAllClosed` | process exit code plumbing, winit loop teardown |

Exclusive `&mut World` systems are retained deliberately (baseline
architecture): the app is one tiny HUD; typed query items
(`Mut<Visibility>`, `Mut<Transform>`, `Option<Mut<Sprite>>`) still scope
access inside the query, and there is no parallel system work to serialize.
No `Cell`/`RefCell`/locks/atomics were introduced anywhere.

## 5. Construction sequence vs runtime dependency graph (spec §18)

Two distinct structures, documented separately. The builder chain in §3 is
the **construction sequence** (registration order). It is *not* the frame
schedule: nothing below infers execution order from the position of a plugin
or system in a tuple.

### 5.1 Runtime dependency graph / frame dataflow

Conceptual phases `input observation → state transition → invalidation
decision → layout/visual derivation → ECS changes → extraction/preparation/
rendering`, mapped to the real engine structures of the pinned revision:

| phase | inputs | outputs | resource/component access | prerequisites | run condition | deferred effects | verification |
| --- | --- | --- | --- | --- | --- | --- | --- |
| input observation | OS events (winit loop, engine-owned) | `ButtonInput<KeyCode>` (F1 edge, Escape level), `ButtonInput<MouseButton>` (click edge), `Time::delta_secs`, `Window::cursor_position` | reads: engine input/time/window resources | winit drained OS events for this frame iteration (engine-owned; exact internal placement not relied upon — the exclusive system reads resources, it does not order against engine systems) | every `Update` | none | resource types/paths verified in pinned source; runtime NOT RUN |
| state transition (read-back) | `Visibility`/`Transform`/`Sprite.color`/`Name` of ~26 non-camera entities, `Monitor` | `(hidden, launched, hold)` frame state; `(width, height)` | one read-only `query_filtered` fold (`Without<Camera2d>`), one `Monitor` fold | input snapshot taken | every `Update` (read-only; cheap) | none | fold semantics transcribed from baseline; boundary truth tables simulated in Python |
| invalidation decision | inline gate `f1 \|\| escape \|\| pressed \|\| hold != 0.0` | whether the write pass runs at all | none | state read-back | gate passes only on F1 edge, Escape held, click edge, or active hold | none | gate expression identical to baseline; no-input-no-write property visible in source |
| layout/visual derivation | gate inputs, `(hidden, launched, hold)`, cursor, monitor size | `hidden_next`, `launched_next`, `hold_next`; per-entity visibility/tint values | pure inlined expressions; no ECS access | invalidation decision | inside gated block; per entity inside the write pass | none | expressions byte-identical to baseline helpers; same-frame boundary simulated (§5.2) |
| ECS changes | derived next-values | guarded component writes (`scale.z`, `Visibility`, slot `Sprite.color`); `Messages<AppExit>::write(AppExit::Success)` at the 7 s boundary | `Mut<T>` items consumed **only** when the compared value differs (§8); exit check hoisted before the write pass | derivation | gated; per-entity equality guards | none — exclusive system, all changes immediate and visible in the same frame | engine `into_inner`/`Deref` tick semantics verified from source; runtime NOT RUN |
| extraction/preparation/rendering | changed components | extracted sprites/text, prepared meshes, render passes, presentation | engine-owned (render sub-app) | main-world `Update` for the frame completed (pipelined rendering overlaps extraction with the *next* main-world update) | engine schedules; sprite mesh preparation gated on `Changed<Sprite>` etc. (verified) | engine-internal | gates read from `bevy_sprite_render`/`bevy_camera` source; runtime/GPU NOT RUN (blocker) |

Ordering is explicit only where a phase consumes another phase's results:
within the single exclusive Update system, ordering is program order
(snapshot → fold → gate → derive → guarded write → exit message), which is
guaranteed by the language, not by scheduler hints. There are **no** ordering
edges between independent systems to get wrong, because the application owns
exactly one Update system. No tuple-position inference is used anywhere:
plugin registration order matters only where Bevy documents it (e.g.
`RenderPlugin` before pipeline plugins that extend it — the chain keeps the
baseline's verified order), and per-frame behavior depends only on the
schedule model (Startup once before the first Update; Update before
PostUpdate/extraction boundaries of the same frame).

### 5.2 Deferred mutations and the same-frame boundary

The application uses **no `Commands` and no deferred operations**: both
systems take `&mut World` exclusively, so every spawn, insert, and component
write is visible immediately and no flush points or schedule barriers exist —
none are needed, and none were added (spec §18.2: no arbitrary flushes; each
would need a concrete producer→consumer dependency, and there is none).

The one genuine same-frame boundary — a state transition (F1 toggle) and a
click arriving in the same frame — is **defined**, not left to ordering:
the inlined launch expression evaluates the click against the *post-toggle*
visibility (`hidden == f1_pressed` is exactly "the HUD is visible after this
frame's toggle", since `hidden_next = hidden != f1_pressed`). Consequences,
re-verified by Python simulation of the inlined expression (7/7 assertions):

- click on the frame whose F1 press *hides* the HUD → dropped;
- click on the frame whose F1 press *reveals* the HUD → registers;
- click while the HUD is already visible, no toggle → registers;
- click while the HUD stays hidden → dropped;
- already-launched state is absorbing.

This is the baseline semantics, preserved exactly (same boolean expression,
now inlined into the Update chain — §3).

### 5.3 Startup and shutdown states

Distinct states, in order, with what each depends on:

1. **Application construction** — `App::new()` + plugin registration (the §3
   chain). No window, no device yet.
2. **Plugin initialization** — engine `build`/`finish`/`cleanup` run inside
   `App::run`; the render sub-app and schedules are constructed. Not
   duplicated or bypassed: the standard runner is used (§6), so lifecycle
   handling is entirely the engine's supported path.
3. **Startup schedule** — the exclusive Startup closure inserts the clear
   color, spawns the camera, reads the monitor fold, creates the eight
   procedural images (pixel formulas inlined at their image literals),
   requests the six external PNG loads, and spawns all 26 HUD entities (all
   `Visibility::Hidden`, `scale (1,1,0)` — the HUD starts hidden exactly as
   in the baseline; F1 reveals it).
4. **Renderer readiness** — device/adapter selection at first extraction
   (engine-owned; `Functionality` priority + default limits maximize
   compatibility). Not awaited or polled by application code.
5. **Asset loading** — external PNGs stream in asynchronously on the io task
   pool. The interface is **not** marked ready by asset completion and the
   event loop is never blocked on assets: procedural textures (divider,
   pills, cards, wells, badge) are synchronous `Image` assets available at
   the first frame; icons/title font appear when their loads resolve —
   baseline behavior. Missing assets surface as engine asset errors (§9.3).
6. **Interactive readiness / normal running** — from the first `Update`
   frame: input snapshot → gated transitions (§5.1).
7. **Termination** — either `Messages<AppExit>::write(AppExit::Success)` when
   the Escape hold crosses 7.0 s, or the engine's `ExitCondition::OnAllClosed`
   path. Both stay inside the supported lifecycle: `App::run` observes
   `AppExit`, runs plugin cleanup, and returns normally. No `process::exit`,
   no abort-to-avoid-shutdown; `panic = "abort"` affects only panic paths
   (unrecoverable by definition), never normal termination.

## 6. Integration level selection (spec §17)

### 6.1 Lowest sufficient public level per subsystem

Levels per §17.1: (1) plugin/resource configuration, (2) typed systems +
schedules + component/resource ownership, (3) public render-world
extraction/preparation integration, (4) custom pipeline/graph nodes,
(5) custom runner / manual renderer init.

| subsystem | level chosen | capability gained | complexity introduced | public API used | verification needed |
| --- | --- | --- | --- | --- | --- |
| window/presentation | 1 | exact borderless-fullscreen config, cursor, constraints, exit condition | none beyond the literal | `WindowPlugin`, `Window`, `CursorOptions`, `ExitCondition` | fields verified in pinned source; visual check BLOCKED (no display) |
| renderer configuration | 1 | backend set, power/priority, features, validation flag, memory hints — all baseline settings expressible without wgpu | struct-update defaulting for the one non-nameable field (proven equivalent) | `RenderPlugin`, `RenderCreation::Automatic(Box<WgpuSettings>)`, `bevy::render::settings::*` re-exports | `WgpuSettings` + wgpu 30.0.1 defaults read line-by-line (bevy-audit.md) |
| frame logic (input→state→writes) | 2 | full per-frame control with immediate (non-deferred) mutation and read-back state; single system = no ordering ambiguity | exclusive-world closures are less idiomatic than typed params — accepted as the baseline architecture, kept; typed alternatives are impossible under the no-`mut`-binding rule (see §13.1) | `add_systems(Startup/Update, FnMut(&mut World))`, `World::query_filtered`, `Mut`, `Messages::write` | tick semantics of `Mut` verified; boundary semantics simulated in Python (runtime NOT RUN) |
| sprites/text rendering | **1–2 (standard rendering retained)** | everything the UI needs: textured quads, per-sprite color/alpha, 2D text, z-layering | none | `SpritePlugin`, `SpriteRenderPlugin`, `TextPlugin`, `Sprite`, `Text2d`, `TextFont`, `TextColor` | §17.1 explicitly allows this: no measured requirement or bottleneck justifies level 3–4; none used |
| render-world integration | **not used (levels 3–4)** | — | — | — | justified absence: no application requirement needs custom extraction, nodes, or pipelines; introducing them would duplicate engine functionality (§17.1 rule) |
| runner/lifecycle | **not used (level 5)** | — | — | — | `App::run` provides every needed lifecycle state (§5.3); a custom runner would risk duplicating plugin `finish`/`cleanup` |

No deeper level was selected for appearance; the recorded decision for this
UI application is that standard sprite/text rendering **is** the correct
low-level integration, with explicit engine-configuration control (level 1)
and explicit ECS ownership (level 2) as the depth actually required.

### 6.2 No private-engine coupling (§17.2)

- No private fields, internal ordering assumptions, memory-layout
  assumptions, or source patches: every symbol used is public in the pinned
  revision (bevy-audit.md table, commit-pinned permalinks).
- No transmutation of engine handles (`transmute` is machine-checked
  forbidden), no manufactured `Entity` identifiers (entities are only used
  as spawned/queried; identity is the `Name` component, §7), no pointer
  stability assumptions (`from_raw_parts`/`as_mut_ptr` machine-checked
  forbidden), no backend-native handle access.
- No direct backend dependency, second graphics device, second queue, or
  competing surface lifecycle: exactly one `RenderPlugin`, one window, one
  camera; wgpu is transitive-only (policy-tested).
- **Hints vs guarantees**, distinguished explicitly: `PowerPreference::HighPerformance`,
  `MemoryHints::Performance`, `PresentMode::AutoVsync` (engine default),
  task-pool thread counts, and `WgpuSettingsPriority::Functionality` are
  *requests*; the OS/driver/adapter decide the actual device, limits,
  present mode, and scheduling. Nothing in the docs or CI checks claims these
  as guarantees.

## 7. Ownership, state transitions, and data representation (spec §19)

Per application-relevant value: `authoritative owner | initialization |
mutation authority | readers | invalidation triggers | lifetime`.

| value | owner (single source of truth) | initialization | mutation authority | readers | invalidation triggers | lifetime |
| --- | --- | --- | --- | --- | --- | --- |
| HUD hidden/shown | `Visibility` of the unnamed HUD entities (ECS) | spawned `Hidden` in Startup | Update write pass only (guarded) | Update read-back fold; engine visibility propagation | F1 edge | process |
| launched state | slot entity's `Sprite.color` alpha (> 0.5 ⇔ launched) | spawned transparent (`srgba(1,1,1,0)`) | Update write pass only (guarded) | read-back fold; sprite extraction | Launch click while post-toggle-visible | process |
| Escape hold accumulator | `Transform.scale.z`, uniform across all non-camera entities | spawned `0.0` | Update write pass only (guarded) | read-back fold; exit check | Escape held/released | process |
| entity role identity | `Name` component (`"menu"`, `"menu-ink"`, `"slot"`; unnamed = HUD chrome) | Startup spawns | never mutated | fold classification (`slot_name`, `named`) | none | process |
| layout geometry | `Transform.translation`/`scale`, `Sprite.custom_size`, text anchors | Startup, from one `Monitor` fold (1920×1080 fallback) | never mutated afterwards | renderer | none at runtime — window is borderless fullscreen **and non-resizable**, so no resize path exists; scale-factor changes are engine-handled for the projection (verified: `camera_system` listens to `WindowScaleFactorChanged`) while the baseline layout is not recomputed — a documented baseline limitation (§9.1), not an oversight | process |
| input observations | engine resources (`ButtonInput`, `Time`, `Window`) | engine | engine (winit/input systems) | Update snapshot (one keyboard borrow + per-resource reads + cursor/monitor folds) | every frame | engine-owned |
| procedural images | `Assets<Image>` via `AssetServer`; strong `Handle`s stored in the `Sprite` components that use them | Startup, generated once | never (no regeneration trigger exists) | sprite/text extraction | none | process (handles held by components ⇒ no accidental unload, §9.2) |
| external PNG assets | same (`Assets<Image>`, handles in components) | Startup load requests; async resolution | engine asset pipeline | extraction | none (no reload support in project scope) | process |
| application `Resource`s | **none exist** | — | — | — | — | — |

Representation decisions, per §19's rules:

- **One authoritative representation**: state lives only in ECS components;
  derived values (`hidden_next`, `launched_next`, `hold_next`) exist solely
  as closure parameters within one frame and are never stored — there is no
  second, independently mutable source of truth (the arena branch's recorded
  `Dashboard` resource pattern is exactly what this avoids).
- **No oversized Resource, no fragmentation**: no resource at all; components
  used (`Visibility`, `Transform`, `Sprite`, `Name`, `Text2d`/`TextFont`/
  `TextColor`) are the engine's own — no application-defined component
  fragmentation, because the read-back fold over ~26 entities is the access
  pattern and a query iterator serves it directly.
- **No `Entity` numeric identity**: roles are `Name`-based; entities are
  never indexed by hand, never stored in lists; despawned/missing entities
  cannot occur (nothing despawns), and the fold handles the empty case
  gracefully by construction (fold identity `(false, false, 0.0)`).
- **Fixed-size storage only where the size is a true invariant**: `[u8; 4]`
  pixels; texture buffers sized by closed formulas that appear inlined next
  to their texture descriptors (the same expression authors both, so a
  mismatch is structurally visible); the visibility/tint decisions are
  two-element lookup expressions indexed by `bool as usize` — total by
  construction.
- **No speculative containers**: no `Vec<Entity>`, no `HashMap`/`BTreeMap`/
  registry introduced — the access pattern is "iterate all, classify by
  `Name`", which a query fold serves with zero allocations; replacing it with
  a map would add complexity without measured need (§19, §22).
- **No state cloning for ownership**: the frame state is a `(bool, bool, f32)`
  tuple passed by value; handles are moved into the spawns that own them
  (`.clone()` only where two spawns genuinely share one image asset, plus one
  redundant clone removed relative to baseline).
- **Configuration vs dynamic state** stays distinguishable without `const`/
  `static` items: immutable configuration exists as literals inside the
  construction chain (window fields, plugin settings); dynamic state exists
  only in ECS components.
- **Floating-point discipline**: the hold accumulator keeps the baseline
  expression byte-for-byte — `(hold + delta_secs * escape_held as u8 as f32) *
  escape_held as u8 as f32` — no reordering, so no accidental FP behavior
  change. Exact comparisons are retained where they encode discrete state:
  `alpha > 0.5` (launched detection), `scale.z >= 7.0` (exit boundary),
  `hold != 0.0` (gate), and the §8 write guards (`!=` on `f32`/`Visibility`/
  `Color` where both sides come from the same expression, so equality is
  exact and meaningful; no tolerance is appropriate for a discrete reset to
  `0.0`).
- **No replay infrastructure, no state-machine framework**: the transitions
  are three booleans and one float; their truth tables are documented (§5.2)
  and were simulated in Python. Rust test functions are prohibited by the
  third-revision spec (§8/§20 of the spec: exactly one `fn` item, tests only
  by explicit approval), so no `#[test]` module exists — `cargo test`
  honestly reports 0 tests in CI.

## 8. Change detection and invalidation protocol (spec §20)

### 8.1 Verified engine semantics (v0.20.0, commit-pinned in bevy-audit.md)

- `Mut::into_inner(self)` → `&mut T` **marks the component changed
  unconditionally** (`change_detection/traits.rs`: "Consume self … while
  marking self as 'changed'"; it calls `set_changed()`). Merely obtaining
  mutable access this way *is* a write as far as change detection is
  concerned — the baseline pattern flagged every touched component on every
  gated frame.
- Reading through `Deref` (`&*m`, field reads) never flags.
- Consumers are change-gated: visibility propagation runs from
  `Or<(Changed<Visibility>, Changed<ChildOf>)>` sources; sprite mesh
  preparation runs from `Or<(Changed<Sprite>, Changed<Anchor>, …)>`.
- The engine does **not** compare old/new values on assignment; equal-value
  writes through `Mut` still flag.

### 8.2 Protocol implemented

What causes each expensive operation to run:

| operation | trigger | non-triggers (guarded out) |
| --- | --- | --- |
| procedural texture generation | Startup, once — no regeneration trigger exists | anything at runtime |
| write pass (query iteration) | inline gate: F1 edge ∥ Escape held ∥ click edge ∥ hold ≠ 0 | idle frames (no input, settled hold) |
| `Transform.scale.z` write + CHANGED flag | hold value actually changes (accumulating or resetting) | gated frames where the inlined next-hold expression returns the current value (e.g. F1-only frame with settled hold) |
| `Visibility` write + CHANGED flag → propagation re-run | F1 toggle or launch transition changes an entity's computed visibility | hold-only frames, repeat-input frames with unchanged visibility |
| `Sprite.color` write + CHANGED flag → sprite mesh preparation | launch transition (slot tint transparent ↔ opaque) | all frames where the tint is unchanged |
| exit message write | the next-hold value crosses 7.0 exactly once (hold resets/exit terminates immediately after) | every other gated frame |
| layout recomputation | never at runtime (window non-resizable; §7 layout row) | resize/scale-factor events (documented baseline limitation, §9.1) |
| text content / font / font-size changes | never after Startup (all text static) | every frame |

Structural layout invalidation and color/opacity animation are separate by
construction: the Escape-hold "fade" only ever touches `scale.z` (progress)
and, at transitions, `Visibility`/`Sprite.color` — it never recomputes
geometry or rewrites font sizes. Conversely, a visibility toggle never moves
anything.

Settled animations stop producing application-owned updates: once the hold
resets to `0.0` and no input arrives, the gate closes the write pass
entirely; while the gate is open on repeat input, the equality guards ensure
only genuinely changed components are written and flagged. Hidden UI can
never go stale: visibility is a pure function of the read-back state
(`hidden_next` + role classification + `launched_next` + slot membership),
recomputed from the authoritative ECS values every time the gate opens —
there is no cached layout or shadow state that could drift.

**Honest scope**: this eliminates *application-owned* spurious invalidation.
It is explicitly **not** a claim of zero renderer work on idle frames —
extraction of the (unchanged) visible set, presentation, and OS compositing
continue every frame, engine-owned.

No new generic change-detection layer was built: the guards are three direct
explicit comparisons using the components' own `PartialEq` (verified derived
for `Visibility` and `Color`), which §20 names as the sufficient approach.

## 9. Coordinate spaces, rendering and asset-lifetime invariants (spec §21)

### 9.1 Coordinate and visual correctness

| space | convention | source of truth (verified) |
| --- | --- | --- |
| cursor input | **logical** pixels (`physical / scale_factor`), origin top-left of the window, y down | `Window::cursor_position` (window.rs#L618-L623); `physical_cursor_position` also exists but the baseline used the logical variant — preserved |
| layout constants | monitor **physical** pixels (`Monitor::physical_width/height`), 1920×1080 fallback | monitor fold in Startup/Update |
| world / camera projection | centered orthographic, y up, 1 world unit ↔ 1 pixel at scale factor 1; runtime projection updated from the **physical** target size | `Camera2d` `#[require(Projection::Orthographic(default_2d()))]`; `ScalingMode::WindowSize`; `camera_system` → `projection.update(physical size)` |
| hit testing | baseline formula maps the pill's world rect into cursor space assuming the two coincide: `x ∈ [width/3+81, width/3+655]`, `y ∈ [height/2−192, height/2−139]` (inclusive) | inlined expression, byte-identical to the baseline; region arithmetic re-checked against the baseline at 1920×1080 and at the 180×120 minimum by Python simulation |

At scale factor 1.0 — the fullscreen 1920×1080 target environment — logical,
physical, and world pixels coincide exactly and the hit test is exact. At
other scale factors the preserved baseline mixes logical cursor coordinates
with a physical-pixel layout (hit region effectively scaled by the factor).
Per the spec's conflict rule (baseline preservation vs §21.1 "avoid mixing
physical and logical pixels") this is **reported as an open decision** rather
than silently changed: switching to `physical_cursor_position()` (or laying
out in logical pixels) would alter baseline behavior at scale ≠ 1 and needs a
product decision. Recorded in bevy-audit.md "Unresolved questions" #3.

Camera orientation/origin were verified through the actual APIs above, not
assumed. Alpha conventions are untouched: `Rgba8UnormSrgb` textures,
`Color::srgba` authoring, default sprite alpha blending, `alpha > 0.5` as the
discrete launched test — no sRGB/linear reinterpretation or blending change
was made to obtain similar screenshots. Text uses fixed pixel font sizes as
in the baseline; no character-count heuristic exists or is claimed as glyph
measurement (all text is static; no runtime measurement is needed).

### 9.2 CPU and GPU resource lifetime

- Procedural image data is generated **once per process** (Startup); the
  regeneration trigger set is empty by construction (§8.2).
- Render-asset usage follows the baseline's explicit descriptors
  (`TEXTURE_BINDING | COPY_DST`, `asset_usage` main+render worlds,
  `copy_on_resize: false`): CPU-side data is not discarded blindly — the
  baseline fields are kept verbatim.
- Strong handles live **inside the components that render them**
  (`Sprite.image`, `TextFont.font`): assets cannot unload accidentally, and
  the style-driven rewrite moved no handle out of a component (the one
  redundant `.clone()` removed was on a handle whose final use immediately
  follows).
- Steady state performs no repeated GPU uploads, pipeline creation, bind-group
  rebuilds, or shader specialization from application code: images are static
  after Startup, and §8's guards minimize `Changed<Sprite>` churn that would
  drive mesh re-preparation. Engine-side costs were verified as gated
  (bevy-audit.md) rather than assumed.
- No custom render-world data exists, so no main-world/render-world
  synchronization boundary was needed; no shared mutable state or manual
  blocking sync was introduced.
- Device-loss recovery and backend fallback remain the engine's supported
  behavior — no custom initialization alters it (level 5 integration not
  used, §6.1).

### 9.3 Unsupported environments

- **Missing/corrupt assets**: `AssetMode::Unprocessed` + engine asset error
  events → Bevy logs load errors with paths; the HUD still renders with
  procedural textures (no blank window, no silent exit).
  `UnapprovedPathMode::Forbid` blocks path-escape attempts loudly.
- **Unsupported graphics capabilities**: adapter/device request failures
  surface the engine's wgpu error output; `Functionality` priority + default
  limits (replacing the baseline's suspect zero-valued hand limits) maximize
  the chance of a working device.
- **No display access**: winit/Bevy fail at window creation with the engine's
  error message — again no silent exit; this is also why every headless check
  in CI avoids window creation entirely (the policy job is pure Python; the
  toolchain job compiles and runs an empty test harness; no step executes the
  binary).
- **Backends**: VULKAN ∥ METAL ∥ DX12 preserved per the baseline — no single
  backend forced across platforms; `VALIDATION_INDIRECT_CALL` kept explicit
  (note: `WgpuSettings::default()` would drop it in release builds without
  DX12 — keeping it explicit preserves the baseline's validation intent).

## 10. Notable decisions

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
- **Guarded write pass** (§8): the single behavioral-mechanics change vs the
  baseline — identical values, identical frame timing, strictly fewer
  spurious CHANGED flags. Motivated by verified v0.20.0 `into_inner`
  semantics; §20 requires exactly this treatment.
- **Monitor reads**: one `(width, height)` fold per read site instead of two
  `QueryState` constructions (Startup and Update), same last-wins semantics
  and same 1920×1080 fallback.
- **Exit detection**: algebraically identical replacement of the
  `&mut [bool; 1]` cell + per-entity re-check by the inlined next-hold
  expression feeding one `>= 7.0` comparison per gated frame, valid because
  `scale.z` is uniform across all non-camera entities by construction
  (spawned 0.0, updated by one expression). Removes one mutable cell plumbing
  from the hot path without changing observable behavior.
- **Keyboard snapshot**: F1 edge and Escape level read from one
  `ButtonInput` borrow (§22: no redundant input lookups when a coherent
  snapshot suffices).
- **Profiles**: `panic = "abort"` (stable) replaces nightly
  `immediate-abort`; fat LTO/1 codegen unit/strip kept; `target-cpu=native`,
  `-Z location-detail=none`, `-Z trap-unreachable`, and `build-std` removed —
  they made release binaries non-portable (native ISA) and required nightly.
  Diagnostic trade-off: `abort` loses unwinding but keeps the panic message;
  asset-load failures still surface through Bevy's asset error handling.

## 11. Naming identities (Section 11.4 separation)

| identity | value |
| --- | --- |
| Cargo package name | `subsystem-for-arxumbra` |
| Cargo binary target name | `subsystem-for-arxumbra` (verified from `cargo metadata` in the release workflow, not assumed) |
| Cargo package version | `0.0.0-s1` (valid semver) |
| fixed distribution label | `v0.0.0.0.0.0-s1` — external naming contract only; six numeric segments make it invalid as a Cargo version, so it never appears in `Cargo.toml` |
| build job display name | `${{ matrix.artifact }}` (e.g. `x86_64-Linux-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1`) |
| uploaded artifact name | `${{ matrix.artifact }}` |
| archive name | `${{ matrix.artifact }}.tar.gz` (+ `.sha256` sibling, combined `SHA256SUMS` at publish time) |
| actual executable filename inside the archive | `${{ matrix.artifact }}${{ matrix.extension }}` — the six exact filenames are asserted by the CI policy job and by the release build job's own matrix self-check |
| release tag | whatever `v*` tag triggers the workflow; if it differs from the fixed label, the label still governs file names (distinction documented here, per mandate) |

## 12. Package layout and asset resolution

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
behavior and is validated in the release workflow against the real staged
files. Unix executable bits are verified twice per non-Windows target: inside
the archive (tarfile mode check) and after extraction into a fresh temporary
directory (a different working directory), alongside the extracted layout
(executable + `assets/` with exactly six PNGs) and the recorded SHA-256
checksum; the binary header (ELF/Mach-O/PE machine field) is inspected
against the matrix target architecture on every runner.

## 13. Strict-syntax exceptions, checker boundary, and prelude policy

### 13.1 The relaxation, formally requested (nine `&mut` spellings)

`&mut` appears in project source **only** where a public library signature
mandates it — exactly nine occurrences, inventoried and enforced by the CI
policy job (any count or spelling drift fails CI). Exact APIs (all verified
in the pinned revision):

1. **Exclusive systems — six occurrences of
   `&mut bevy::ecs::world::World`.** `App::add_systems(Startup/Update,
   closure)` with `FnMut(&mut World)` closures is the exclusive-system
   contract of `bevy_app`/`bevy_ecs` (`impl IntoSystem<…>` for
   `FnMut(&mut World)`); there is no `&mut`-free spelling of an exclusive
   system. Two occurrences are the system closure parameters themselves
   (lines 148 and 1345); four are IIFE parameters that thread the world
   inward (Startup: monitor + handles IIFEs; Update: snapshot + next-values
   IIFEs) — necessary because IIFE arguments are evaluated in the enclosing
   scope, which is the only `let`-free way to move `world` past a closure
   boundary. Typed-system alternatives were evaluated and rejected with
   source evidence: `Commands::spawn(&mut self, …)`,
   `Query::iter_mut/single_mut(&mut self)`, and `ResMut` methods all take
   `&mut self`, so a typed system parameter would have to be a `mut`
   *binding* — prohibited by the contract, while `&mut` in a signature
   position is exactly the relaxation the original spec's petition clause
   contemplated. The baseline `main` itself uses `|world: &mut World|`.
2. **Mutable query data types — three occurrences:
   `&mut bevy::camera::visibility::Visibility`,
   `&mut bevy::transform::components::Transform`,
   `Option<&mut bevy::sprite::Sprite>`.** In
   `World::query_filtered::<(…), Without<Camera2d>>` these are the
   *type-level* spelling required by the `QueryData` trait — the only way to
   name mutable query access in a type position. The items handed out are
   `Mut<T>` wrappers, consumed through `into_inner` only under equality
   guards (§8); the write-pass closure parameters themselves take `Mut<T>`
   directly and are not spelled `&mut T`.

Compiler diagnostics demonstrating the absence of alternatives are
**unavailable in this sandbox (no Rust toolchain — blocker)**; the evidence
substituted is the pinned-source signature reading above
(docs/bevy-audit.md). No `mut` binding, no `mut` parameter, no raw pointer,
no `Cell`/`RefCell`/`Mutex`/`RwLock`/atomic, no `unsafe`, no
`transmute`/`from_raw_parts`/`as_mut_ptr`, and no codegen escape hatch
exists anywhere; the checker enforces exactly this rule set, token-boundary
aware, over every `.rs` file in the tree.

`'static` lifetime: unused in project source today; permitted by contract if
a future Bevy signature requires it (the checker whitelists `'static`
precisely, via the preceding-apostrophe exception, and a fixture proves it).

### 13.2 Inspection boundary and checker location (honest scope)

The checker lives **in the CI policy job** (`.github/workflows/ci.yml`,
`python3`, runs on every push/PR and — through the reusable-workflow call —
before every release build): a tokenizer-level text scanner (comments and
literals masked, word-boundary matching, the two narrow exceptions above),
**not** an AST/macro-expansion analysis, and it does not claim to prove
anything about arbitrary macro expansion. The third revision prohibits a
second Rust entry point or a `#[cfg(test)]`-hosted checker, and prohibits
checker tooling in runtime dependencies — hence Python embedded in the
workflow, with positive/negative fixtures (keyword, comment-in-URL,
block-comment, attribute, lifetime, identifier-substring, local-import,
cfg-disabled-fn, fn-count, TOML-comment, YAML-comment incl. block-scalar and
quoted-`#` cases) executed before the repository scan; a fixture failure
aborts the job. Comment scanning is syntax-aware per file type: Rust
tokenizer for `.rs`, string-aware for TOML, block-scalar/quote-aware for
YAML — never a global `//`/`#` deletion or match.

The boundary is acceptable here because the audited surface is small and
fully enumerated: `src/main.rs` is the only Rust file (asserted by the scan),
contains **zero** macro invocations, zero attributes, no project-defined
macros, no `include_bytes!`/`include_str!` (machine-checked forbidden), and
the tree has no `build.rs`, no `examples/`, `benches/`, `tests/`, or
`.cargo/` (all asserted). There is no `no_implicit_prelude`, no
`#[macro_use]`, and no hidden prelude module anywhere: the crate is a single
file whose every engine/std integration point is spelled with fully qualified
paths, so the no-`use` rule is satisfied by explicitness, not by a smuggled
prelude.

## 14. Known limitations of this rewrite environment

- No Rust toolchain and no crates.io access existed in the rewrite sandbox
  (network allowlist: github.com/codeload/api.github/npm/pypi). Consequently
  `cargo fmt/check/clippy/test/build` and `cargo generate-lockfile` were
  **not executed**; verification was source-level (pinned Bevy/wgpu trees,
  drift diff, structural scans, parse-level simulations, and local execution
  of every Python block embedded in the workflows). `Cargo.lock` was
  removed rather than left stale — regenerate and commit it as the first step
  on a networked machine (exact commands in docs/rewrite-log.md §Next steps).
- The third revision's combined feasibility gate (compile + run a minimal
  closure-conversion experiment before the transformation) is **BLOCKED** in
  this sandbox for the same reason; the transformation was instead
  mirror-verified structurally (balance, junctions, counts) and every
  embedded CI check was executed locally against the real tree.
- GPU/visual checks were not executed (no display). Reproducible scenario
  list in docs/performance.md.
- The scale-factor question (§9.1) is an open product decision, reported
  rather than resolved unilaterally.
