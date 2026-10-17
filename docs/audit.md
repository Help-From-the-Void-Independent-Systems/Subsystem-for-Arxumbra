# Subsystem-for-Arxumbra rewrite audit

## 1. Revisions and environment

| item | value |
| --- | --- |
| repository | Help-From-the-Void-Independent-Systems/Subsystem-for-Arxumbra |
| session branch | arena/03d209d9-subsystem-for-arxumbra |
| pre-edit base commit | 82bccf5a3ea5369fac45f1d799e3ee3c4e5c60a2 (main, "Add files via upload", 2026-10-16, clean tree, no uncommitted differences) |
| required engine | Bevy git tag v0.20.0, lightweight tag resolving to commit b3bdd75becaffe9a16677cf5bae5bd6d198f052b (verified via api.github.com repos/bevyengine/bevy git ref tags v0.20.0) |
| pre-edit engine | Bevy git tag v0.20.0-rc.2 resolved at 343a0233d6788a49f224a3684c8995e246663df3 (Cargo.lock), rust-version 1.97.1 required by Bevy 0.20.0 |
| toolchain | pinned stable 1.97.1 (rust-toolchain.toml); pre-edit used unpinned nightly with build-std and -Z flags |
| sandbox execution limits | no rustc/cargo installed; rustup and crates.io hosts (static.rust-lang.org, sh.rustup.rs, index.crates.io, static.crates.io) and apt mirrors are unreachable from this environment (verified with curl and apt-get update attempts). Local compilation is environmentally impossible. Compilation, tests, clippy, packaging, and Wayland smoke therefore run as real GitHub Actions executions on this repository, reported with run IDs. |
| workflow log retrieval | download endpoints for historical runs resolve to results-receiver.actions.githubusercontent.com, which is also unreachable from this sandbox (verified: EOF on run 39025388413 log request). Historical diagnostics stay unavailable; new runs are diagnosed from fetched step logs via gh api where possible. |

Run 39025388413 (job 117375254417) cited in the task belongs to revision 31b25ee324afb4c651b1a1e2658dc2dcc73c4042, which is not an ancestor of this repository (main history starts at 82bccf5). Its only retrievable annotation is "Process completed with exit code 101." plus a missing-smoke-artifact warning. The underlying lint diagnostic was not obtainable; no claim is made about which lint failed there.

No workflow run exists for revision 82bccf5. The newest runs observed at inspection time (39041652953, 39041650016, 39039761084) belong to a different session branch and are not evidence for this baseline.

## 2. Pre-edit state (source-derived, revision 82bccf5)

Files: Cargo.toml, Cargo.lock (tracked, 512 packages), .cargo/config.toml, .github/workflows/release.yml (single workflow, 124 lines), src/main.rs (1745 lines, exactly one explicit fn main), assets/app-icon.png, assets/icons/{house,registry,flask,rocket,rocket-pill}.png.

Pre-edit behavior, established by full source reading (no runtime was executable in this environment, so these are source-derived facts, labeled as such):

- window: borderless fullscreen on primary monitor, fallback resolution 1920x1080, AutoVsync, not resizable, no decorations, skip taskbar, title "Arxumbra", exit condition OnAllClosed.
- cursor: CursorOptions visible false (hidden cursor defect present in this revision), grab none, hit test true.
- construction: one App chain; Startup uses one exclusive `&mut World` closure; all UI spawns hidden; procedural textures (divider glow, core, separator, pill, menu card, lab card, lab well, badge) generated inline at startup; PNG icons loaded via AssetServer; Update uses one exclusive `&mut World` closure.
- state encoding defects in this revision: no selected-destination state (all content shows together); hidden flag derived from the visibility of the last unnamed entity scanned; Escape hold timer stored in Transform.scale.z of the last scanned entity; launched flag derived from slot sprite alpha; entities located by Name string matching; no hover, no focus, no armed-press semantics.
- update loop: F1 toggles all visibility (edge); Escape accumulates delta into scale.z while held and resets to 0 on release; accumulated hold >= 7.0 writes AppExit::Success; left click inside a fixed rectangle (derived from monitor physical pixels) latches launched, which reveals slot entities and tints them.
- layout defects in this revision: positions derived from Monitor physical dimensions folded over all monitors (physical/logical mixing); oversized bright divider glow band; fixed offsets from height/2; title text unbounded (can spill across the sidebar at narrow widths); Laboratory card overlaps neighbors at some sizes; both tabs render identically with no selected style.
- release workflow defects in this revision: artifact labels do not match the contract names (subsystem-for-arxumbra-linux-x86_64 style instead of the six exact labels); no validation job, no formatting/lint/test evidence, no smoke, no checksums, no pull_request trigger; stale policy assertions grep src/main.rs for include_bytes!/AssetPlugin/AssetServer/"png",; nightly toolchain with build-std and -Z flags; target-cpu=native for distributed binaries.
- direct wgpu = "30" dependency present; instance_memory_budget_thresholds set through the non-public wgpu::MemoryBudgetThresholds path.

## 3. Behavior decision table

| area | pre-edit behavior | required change | preserved invariant | verification |
| --- | --- | --- | --- | --- |
| window | borderless fullscreen primary, 1920x1080 fallback, AutoVsync, not resizable, no decorations | keep configuration; derive layout from window logical size instead of monitor physical size | window mode, title, vsync, decorations off, resize constraints | policy script, Wayland smoke fills the 1280x720 headless monitor |
| cursor | visible false, grab none | visible true, ungrabbed | hit test true | source assertion, smoke readiness; headless capture cannot show the native cursor and is labeled as such |
| navigation | none; all UI shown together once HUD on | authoritative Destination { Home, Extension Registry }, initial Home; Registry content exists and is interactive only while selected | F1 edge toggle preserves selection | smoke keyboard scenarios + screenshots |
| tabs | two identical pills, no states | selected = light with dark content; unselected = dark with light content; distinct hover/pressed/keyboard-focus; selection activation-driven (pointer press+release or keyboard) | monochrome palette | smoke screenshots (Home, Registry, focus), pointer scenarios NOT RUN in CI (no weston pointer injection), verified by keyboard path |
| launch | click rectangle latches launched state, reveals rocket slot sprites | honest disabled Coming Soon action: excluded from activation and focus order, disabled styling | Coming Soon caption | policy source assertions, screenshots |
| escape exit | hold 7.0s accumulated from deltas, reset on release, stored in Transform.scale.z, writes AppExit::Success | same 7.0s threshold and release reset held in the UiState resource; exit by despawning the primary window, which bevy_winit converts to AppExit::Success via exit_on_all_closed | threshold value and reset semantics | smoke: held Escape via wtype, expect process exit 0 |
| HUD toggle | F1 edge-flips everything | same edge toggle; hidden controls are not hit-testable or focusable; restores selected destination | F1 edge semantics | smoke: F1 toggle screenshots |
| layout | monitor physical pixels, width/3 sidebar, bright glow band, fixed Y offsets, unbounded title | logical-window-derived layout model shared by rendering and hit testing; bounded padded sidebar; restrained divider; responsive margins; title wrapped with TextBounds | sprite/text rendering semantics | screenshots at headless resolution; other viewports NOT RUN in CI, structural derivation documented |
| assets | procedural textures generated once at startup; icons loaded from assets | same one-shot generation, strong handles held in a resource; assets staged beside executable | startup-only generation, external assets/ layout | package verification, smoke run from installed copy outside the checkout |
| exit | AppExit::Success written from exclusive world scan | despawn primary window -> OnAllClosed -> AppExit::Success (verified in bevy source) | success exit code | smoke exit status 0 |
| update systems | exclusive world scans of every entity per event; state read back from visibility/scale/alpha/Name | typed systems, typed markers, resource-owned state, change-gated presentation writes | single Update schedule work | source policy scan, remote clippy/build |
| packaging | six targets, wrong names, no validation evidence | exact six contract labels and executable names, single canonical build per target reused for smoke/package/publish, checksums, permissions preserved | six-target matrix and runner labels as contracted | policy script matrix equality, dispatch run artifact inspection |

## 4. Combined-syntax feasibility: mutation conflict (BLOCKED pending decision)

The binding no-mut/no-&mut requirement conflicts with Bevy 0.20.0 public APIs. Evidence, all at commit b3bdd75becaffe9a16677cf5bae5bd6d198f052b:

- crates/bevy_ecs/src/system/commands/mod.rs: all 84 mutator methods of Commands and EntityCommands (spawn, entity, insert, despawn, insert_resource, write_message, queue, trigger, ...) take &mut self; the only &self method is EntityCommands::id (read-only).
- crates/bevy_ecs/src/system/commands/parallel_scope.rs:81: ParallelCommands::command_scope(&self, f: impl FnOnce(Commands) -> R) passes an owned Commands; the closure parameter is then an immutable named binding, and calling any &mut self method on an immutable named binding is error E0596. The Bevy doc example itself writes |mut commands|.
- crates/bevy_ecs/src/query/fetch.rs: Mut<T> is a valid QueryData, but every Query accessor that yields Mut (iter_mut, get_mut, single_mut, ...) takes &mut self on the Query binding.
- crates/bevy_ecs/src/message/messages.rs:125 and message/message_writer.rs: Messages::write and MessageWriter::write take &mut self.
- Exclusive world access is typed &mut World.
- The one &self mutator is crates/bevy_asset/src/server/mod.rs:832 AssetServer::add(&self, asset) -> Handle<A>, which keeps procedural asset creation compliant.
- No Cell/RefCell/lock/atomic workaround is permitted by the contract itself, and none exists in Bevy public system params.

Smallest demonstration: a system closure `|commands: Commands| commands.spawn_empty();` fails with E0596 (cannot borrow `commands` as mutable) under the exact required revision; `|mut commands: Commands|` compiles and is the only syntactic difference.

Requested narrowly scoped decision: permit the `mut` keyword exclusively on parameters inside system/observer closure signatures (Commands, ResMut, Query, Local, MessageWriter params), with no mutable let locals, no written &mut types, no exclusive &mut World parameters, and no interior-mutability substitutes. Startup uses Commands only. All other syntax restrictions stay strict. Until a decision arrives, main.rs is not rewritten and the old source stays in place; independent work (manifest, toolchain, workflow, policy scripts, audit) proceeded.

## 5. Bevy API evidence (commit-pinned)

| mechanism | source symbol | location at b3bdd75 | verified contract | application decision |
| --- | --- | --- | --- | --- |
| asset creation without mut | AssetServer::add(&self) | crates/bevy_asset/src/server/mod.rs:832 | returns strong Handle, registers into Assets | all procedural textures created through it; handles stored in a resource to keep assets alive |
| wgpu budget thresholds | instance_memory_budget_thresholds: MemoryBudgetThresholds | crates/bevy_render/src/settings.rs:12,63,159 | type privately imported from wgpu, not in the pub use list (settings.rs:13); field default = MemoryBudgetThresholds::default() | value expressed as Default::default() via inference; old explicit value (None, None) equals the engine default |
| wgpu limits | WgpuLimits = wgpu::Limits re-export | settings.rs:13-16 | old 60-field literal copies wgpu default limits of the rc.2 wgpu version | replaced by Default::default() to avoid freezing stale field shapes; effective values unchanged (they were the defaults); recorded as structural simplification, not a measured win |
| exit on window despawn | despawn_windows / exit_on_all_closed | crates/bevy_winit/src/system.rs:254 (RemovedComponents<Window> destroys the winit window), crates/bevy_window/src/system.rs:18 (writes AppExit::Success when no windows remain, added to Last by WindowPlugin for ExitCondition::OnAllClosed) | despawning the primary window entity exits with code 0 | Escape-hold exit path |
| deferred visibility | ApplyDeferred | crates/bevy_ecs/src/schedule/executor/mod.rs:159, re-exported at bevy::ecs::schedule | explicit sync point between state writer and readers | one ApplyDeferred between input-state system and presentation system in Update |
| closure system arity | impl_system_function all_tuples 0..=16 | crates/bevy_ecs/src/system/function_system.rs:995 | closures up to 16 system params | systems kept under 16 params |
| text wrapping | TextBounds { width, height } with new_horizontal | crates/bevy_text/src/bounds.rs | bounds Text2d layout | title wrapped to the sidebar content width |
| default font | FontSource::Handle, Handle::Uuid(AssetId::<A>::DEFAULT_UUID, PhantomData) | crates/bevy_text/src/text.rs:284, crates/bevy_asset/src/handle.rs:213, id.rs:49 | default_font feature maps DEFAULT_UUID to bundled FiraMono subset | unchanged text font source; reading dependency-associated constants is permitted |
| wayland-only backend | wayland = ["bevy_internal/wayland"] -> bevy_winit/wayland -> winit/wayland | Cargo.toml:563, crates/bevy_internal/Cargo.toml:152,608 (bevy_winit default-features = false), crates/bevy_winit/Cargo.toml (default = ["x11"], winit default-features = false, features = ["rwh_06"]) | with default-features = false and explicit wayland (no x11, no default_platform), no winit X11 backend is activated | feature list keeps wayland only; CI asserts per-Linux-target that x11rb/x11-dl match no packages and winit/x11 never appears |
| clipboard crate | bevy_clipboard only enabled via default_platform/system_clipboard | Cargo.toml:191,416,467 | not enabled by our feature list, so arboard and its X11 backend never enter the Linux graph | CI asserts arboard matches no packages |
| Window/CursorOptions/WindowPlugin/AssetPlugin/ImagePlugin/WgpuSettings defaults | impl Default present | crates/bevy_window/src/window.rs:480,780, crates/bevy_image/src/image.rs, crates/bevy_render/src/settings.rs | struct-update syntax available | only consequential fields written explicitly; noise copies of defaults removed deliberately |
| winit version | winit 0.30, default-features = false, features = ["rwh_06"] | crates/bevy_winit/Cargo.toml | wayland feature adds winit/wayland and winit/wayland-csd-adwaita only | accepted; csd-adwaita is inert for a decorations-free window |

## 6. Material settings

| setting | pre-edit value | selected value | reason | portability consequence |
| --- | --- | --- | --- | --- |
| bevy source | git tag v0.20.0-rc.2 | git tag v0.20.0 | contract requires the final tag; resolved commit recorded from the lockfile once regenerated | none |
| direct wgpu | version = "30", default-features = false | removed | contract prohibition; the only use was the non-public MemoryBudgetThresholds literal | none; wgpu stays transitive inside Bevy |
| instance_memory_budget_thresholds | wgpu::MemoryBudgetThresholds { None, None } | Default::default() | type has no public Bevy path; engine default equals the old value | none |
| wgpu limits | 60-field literal | WgpuLimits { max_texture_dimension_2d: 16384, ..Default::default() } | verified against wgpu v30.0.1 limits.rs defaults(): every other field of the old literal equals the default (byte-unit fields confirm: 65536 = 64 KiB, 134217728 = 128 MiB, 268435456 = 256 MiB); the single deliberate deviation (2D texture limit doubled to 16384) is kept explicit | none |
| backends | VULKAN | METAL | DX12 | unchanged | Wayland CI uses lavapipe (Vulkan software) and is reported separately from hardware runs |
| rust toolchain | nightly (unpinned) + rust-src + build-std | stable 1.97.1 pinned via rust-toolchain.toml | Bevy 0.20.0 rust-version 1.97.1; contract prefers pinned stable | removes nightly-only flags from artifacts |
| .cargo/config.toml | target-cpu=native, -Z location-detail=none, -Z trap-unreachable=yes, clang+lld, build-std | file removed | native targeting is prohibited for distributed binaries; -Z flags are nightly-only obfuscation without verified need | default portable codegen baseline |
| profile.release | opt-level 3, fat LTO, 1 codegen unit, strip, immediate-abort, plus redundant explicit false fields | opt-level 3, fat LTO, 1 codegen unit, strip, abort | stable panic="abort"; redundant fields (debug false, debug-assertions false, overflow-checks false, incremental false) equal release defaults and were dropped | none |
| features | rc.2 feature list | same list plus bevy_log | LogPlugin gives actionable startup/render diagnostics required by the error-transparency contract | bevy_log is already in the dependency graph of many features; no new third-party code |
| window literal | ~45 fields spelled | consequential fields explicit + engine defaults | avoids freezing unrelated field shapes across engine revisions | documented per-field in section 5 |

## 7. Verification ledger

Status values: PASS (ran and passed), FAIL (ran and failed), NOT RUN (not executed), BLOCKED (concrete dependency prevents execution), PENDING (defined, not yet reached).

| check | status | evidence |
| --- | --- | --- |
| local cargo check/clippy/test/build | BLOCKED | no toolchain; rustup and crates.io unreachable from sandbox (section 1) |
| syntax and comment policy scan (scripts/verify_syntax.py) | PENDING | run in CI validation job and locally where python3 exists |
| fixture selftest of the scanner | PENDING | run in CI validation job |
| workflow inventory exactly release.yml | PENDING | verify_policy.py in CI |
| release matrix equals the six contract entries | PENDING | verify_policy.py in CI |
| manifest policy (no direct wgpu, wayland-only, tag v0.20.0, no cargo-features) | PENDING | verify_policy.py in CI |
| lockfile resolved from tag v0.20.0 and comment-free | PENDING | regenerated by cargo in a dispatched run, normalized, committed, then --locked checks |
| per-Linux-target X11 absence (x11rb, x11-dl, winit/x11, arboard) | PENDING | cargo tree --target assertions in CI |
| cargo fmt --all -- --check | PENDING | CI |
| cargo check --locked --all-targets | PENDING | CI |
| cargo clippy --locked --all-targets -- -D warnings | PENDING | CI; replaces the unreachable historical 101 diagnosis with a real run |
| cargo test --locked | PENDING | CI; zero #[test] functions is reported honestly |
| Wayland smoke on x86_64 and aarch64 Linux (headless Weston, lavapipe) | PENDING | CI build jobs |
| six-target build, stage, package, checksum, machine-type verification | PENDING | CI matrix |
| publication on tag | PENDING | requires tag push; dispatch runs never publish |
| pointer-driven UI scenarios | NOT RUN | no pointer-injection facility exists for Weston in the CI image; keyboard-driven equivalents are run instead, labeled separately |
| screenshots at 1366x768/1920x1080/1280x720/800x600 and high-DPI | NOT RUN | window is fixed borderless fullscreen; CI captures the headless-monitor resolution only; local capture impossible in this sandbox |

## 8. Approved exceptions and open decisions

- event coverage policy: pull_request runs validation plus the full six-target build matrix and Wayland smoke (unprivileged, publication impossible because the publish job is tag-gated); workflow_dispatch runs the same when a human with Actions dispatch rights triggers it (the sandbox integration token lacks Actions write, so automated verification uses pull_request runs); version-tag pushes add publication. This is the disclosed deduplication and coverage choice for the single-workflow event contract.
- sandbox integration token: repository administration works, but Actions administration and workflow dispatch return 403; the workflow_dispatch lockfile-bootstrap input therefore only operates for a human dispatcher. The bootstrap flow actually used is described in section 9.

- no-mut: RESOLVED by owner decision on 2026-10-17. The approved scope is "mut on system-closure parameters only": the `mut` keyword appears exclusively as parameter-binding markers inside system closure signatures (Commands, ResMut, Query params); no mutable let-locals, no written &mut types, no exclusive &mut World parameters, no interior-mutability substitutes. scripts/verify_syntax.py enforces the precise shape: `mut` is accepted only in the parameter form `mut identifier: Type` and never after let/ref/&/*. The approved occurrences are exactly the system parameters of the four application systems.
- launch: RESOLVED by owner decision on 2026-10-17: honest disabled Coming Soon action; the decorative launched-latch is removed; the action is excluded from hover, arming, and focus order; disabled styling is static.
- escape threshold: RESOLVED by owner decision on 2026-10-17: keep 7.0 seconds, now stored in UiState.escape_hold_secs.
- Pointer-injection-free smoke: keyboard scenarios substitute for pointer scenarios where tools allow; both labeled in section 7.
- Workflow rerun policy: publishing to an existing tag replaces assets with the newly verified same-run packages (--clobber) because a tag maps to one commit; disclosed here as the safe-rerun behavior.

## 9. Lockfile bootstrap flow actually used

The committed Cargo.lock still resolves v0.20.0-rc.2 and carries the generated comment header, so --locked commands fail until cargo regenerates it. Because no cargo exists in this environment and workflow dispatch is not executable by the integration token, the bootstrap runs inside the pull_request verification run itself: the validation job regenerates the lockfile when the workflow_dispatch input would have requested it... this cannot fire on pull_request. Instead the pull_request validation performs the regeneration unconditionally while the committed lockfile does not yet resolve from tag v0.20.0, uploads the normalized candidate as an artifact, and fails fast if any later step cannot validate against it; once the candidate is committed to the branch, the regeneration step stops running because the committed lockfile resolves correctly.

## 9. Action pins

| action | version | pinned SHA |
| --- | --- | --- |
| actions/checkout | v7.0.1 | 3d3c42e5aac5ba805825da76410c181273ba90b1 |
| actions/upload-artifact | v7.0.2 | cf430e030ddbb5b0abf93d22962f4752f3646cd9 |
| actions/download-artifact | v8.0.2 | 9000827ccba6bdab643e8b6fd33ac0654aef8333 |
| actions/cache | v6.1.0 | 55cc8345863c7cc4c66a329aec7e433d2d1c52a9 |

Version mapping recorded here per contract; workflow files carry SHAs only.
