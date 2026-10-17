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

| change | rationale | evidence class (see classification below) |
| --- | --- | --- |
| **Guarded component writes**: `Visibility`/`Transform.scale.z`/slot `Sprite.color` are written (and their `Mut` wrappers consumed) only when an explicit equality comparison against the derived next value differs | Verified v0.20.0 semantics: `Mut::into_inner` sets the CHANGED tick unconditionally (`change_detection/traits.rs#L516-L523`), and the engine consumers are change-gated — visibility propagation on `Or<(Changed<Visibility>, Changed<ChildOf>)>` (`bevy_camera`), sprite mesh preparation on `Or<(Changed<Sprite>, …)>` (`bevy_sprite_render`). The baseline flagged all ~26 entities' components on every gated frame (e.g. every frame of a 7 s Escape hold) even when values were equal; now hold-only frames flag only the genuinely progressing `scale.z`, and settled frames flag nothing | unverified hypothesis until measured (no toolchain here); final values and frame timing are identical to baseline by construction |
| One coherent keyboard snapshot: F1 edge + Escape level from a single `ButtonInput<KeyCode>` borrow | spec §22: no redundant input/window lookup when a coherent snapshot suffices (baseline fetched the resource twice per frame) | unverified hypothesis; zero behavior change |
| Monitor geometry read once per site (`(width, height)` fold) instead of two `world.query::<&Monitor>()` constructions per site | duplicate `QueryState` construction + iteration for one coherent value pair | unverified hypothesis until measured; zero behavior change |
| Exit detection computed from the uniform hold value instead of a `&mut [bool;1]` cell re-checked per entity | removes mutable-cell plumbing and a per-entity OR from the write pass | unverified hypothesis; algebraically identical |
| Update gate preserved (inline `f1 \|\| escape \|\| pressed \|\| hold != 0.0`): idle frames with no input and zero hold perform only the input reads + one read-only fold | baseline optimization kept; expression byte-identical | STRUCTURAL |
| Launch transition: the identical boolean expression, inlined at its single call site (rev 3 dissolved the named helper) | same-frame toggle+click boundary defined in docs/architecture.md §5.2, not speed | STRUCTURAL |
| Per-frame `Vec<Entity>::clone()` (recorded arena-branch issue) | does not exist in the chosen `main` baseline; nothing to remove | n/a |
| Procedural textures generated exactly once in `Startup` | baseline behavior kept | n/a |

No floating-point expression, branch structure, or iterator chain was
replaced by a "supposedly faster" alternative; the pixel formulas are the
baseline bodies inlined as immediately-applied closure expressions, with the
twelve clippy-mandated `f32::min(f32::max(v, 0.0), 1.0)` →
`v.clamp(0.0, 1.0)` normalizations proven output-identical on the reachable
domain (docs/architecture.md §3/§10), and the hold accumulator preserves the
baseline's exact operation order (spec §19: no accidental floating-point
changes through reordering).

## Evidence classification (third-revision rule)

Every performance-relevant statement in this document is classified; no
statement is labeled MEASURED, because nothing was measured anywhere (no
toolchain, no GPU, no display — the workload definitions below exist so the
first networked machine can produce MEASURED rows, not to substitute for
them):

| class | meaning | where it applies here |
| --- | --- | --- |
| STRUCTURAL | verifiable by source inspection alone; no runtime claim | all rows of "Work removed or avoided"; the acceptance-property table's PASS (static) rows |
| HYPOTHESIS | expected runtime effect, unverified until measured | every "unverified hypothesis until measured" row above |
| MEASURED | real measurement with recorded conditions | **none — nothing was measured** |
| REGRESSION | before/after comparison under a fixed workload | **none possible here**; the rev-3 transformation was regression-controlled structurally instead (checklist below) |

The third-revision transformation (helpers → inlined expressions, merged
Startup IIFEs, flattened Update chain) was checked against the required
regression dimensions before acceptance: correctness (byte-identical
expression bodies; boundary truth tables re-simulated 7/7), visual
equivalence (same formulas, same constants, same spawn order — nothing
pixel-affecting changed), state freshness (write-pass guards and change
detection untouched), allocations (Startup: one 15-parameter IIFE replaces
two IIFEs — weakly fewer closure allocations; Update: unchanged), frame cost
(no per-frame expression changed semantically; the monitor/snapshot folds
are the same single folds), startup cost (weakly lower: one fewer closure
construction and one fewer argument-tuple evaluation), portability
(unchanged — no codegen/profile/flag change), package integrity (unchanged
matrix, names, layout; new verification steps only *check* more). No change
in this pass increases complexity for an unmeasurable benefit; the merged
IIFE and the flattening both strictly reduce closure machinery, which the
spec's elimination list names as a target (allocation/dynamic-dispatch/
repeated-evaluation machinery).

## Acceptance properties of application-owned code (spec §22)

Investigation targets for the application's own code — **not** claims that
all transitive engine code allocates nothing. "Static" = verified by source
inspection/structural scan in this sandbox; dynamic confirmation requires the
blocked toolchain/GPU environment.

| property | status | how established |
| --- | --- | --- |
| no avoidable entity-list clone in the steady-state frame path | PASS (static) | no `Vec<Entity>`/entity collection exists; state is a `(bool, bool, f32)` fold |
| no procedural texture regeneration without an explicit trigger | PASS (static) | generation exists only in the Startup closure; no runtime call site |
| no string formatting or font-size assignment per frame when inputs unchanged | PASS (static) | zero `format!`/text writes in the Update path; all text static after Startup |
| no unbounded growth of application-owned collections | PASS (static) | application owns no collections at all |
| no repeated plugin registration / schedule construction / resource setup after initialization | PASS (static) | single construction chain; systems registered once; no `World`-mutating setup at runtime beyond component writes |
| no custom locks/atomics introduced to work around syntax restrictions | PASS (static + machine-checked) | scanner forbids `Mutex`/`RwLock`/`Cell`/`RefCell`/atomics; zero occurrences |
| no extra dynamic-dispatch or heap-allocation layer to create a fluent chain | PASS (static) | chain is direct method calls; inner closures are monomorphic IIFEs; no `dyn`, no `Box` beyond the `RenderCreation::Automatic(Box<WgpuSettings>)` the engine API itself requires (once, at construction) |
| no redundant input/window lookup when a coherent snapshot suffices | PASS (static) | one keyboard borrow; one monitor fold per site; one cursor fold; one window-independent state fold |
| idle vs active workloads measured separately | BLOCKED | no toolchain/GPU — procedures below; idle = gate-closed frames, active = hold/transition frames |
| micro-optimization discipline (`observed cost → hypothesis → isolated change → correctness check → repeated measurement → keep/revert`) | recorded | no micro-optimization was applied on speculation; every change above is either correctness-mandated (§20 guarded writes) or work-removal with identical output, each labeled "unverified hypothesis until measured" — none is claimed as a measured win |

No hardware-specific intrinsics, custom allocators, manual SIMD, aggressive
inlining hints, or backend-specific tuning were added: nothing is measured,
so nothing is justified under §22's rule; `target-cpu=native` was removed for
portability of distributed binaries.


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

## Revision-4 honest workload comparison (§I)

No runtime measurements exist in this environment (no toolchain, no GPU, no
display); every row below is classified STRUCTURAL (verifiable by source
inspection) or NOT MEASURED. The rev-3 rows above compared a hidden-HUD idle
scene against active frames; per §I that comparison is superseded by the
like-for-like table below (before = rev-4 base `64e92baa`, after = rev-4
implementation):

| workload | before (rev-3, structural) | after (rev-4, structural) | classification |
| --- | --- | --- | --- |
| Home idle (HUD visible, no input) | gate closed on most frames; when open: world-wide query re-creation + full traversal with unconditional `into_inner()` on changed ticks | gate closed: 16 scalar comparisons, zero writes, zero iteration | STRUCTURAL; frame timings NOT MEASURED |
| Registry idle (no input) | same as Home idle plus a larger visible set | identical to Home idle (gate closed; visibility state static) | STRUCTURAL |
| hover change (pointer moves across one control) | no hover concept existed | gate opens (cursor delta): one layout-model hit fold (≤3 rect contains), guarded color writes on ≤2 entities (tab body + previous hover), no texture work, no text work, no geometry writes (layout branch skipped) | STRUCTURAL |
| keyboard focus change (Tab) | no keyboard focus existed | gate opens: one ≤3-step cycle fold, guarded visibility write on 2 ring entities + guarded color writes, no layout writes | STRUCTURAL |
| tab switch (activation) | fixed-rect press check; state re-derived from sprite alpha/`Name` strings each gated frame; geometry constants re-applied | one state-layer evaluation; guarded visibility write on the Registry root (subtree propagation by the engine) + guarded color writes on 2 tab bodies; zero spawns, zero asset work, zero layout writes | STRUCTURAL |
| launch transition | alpha threshold read-back from slot sprites | `launched` flag flip; guarded visibility writes on 8 card entities; well/rocket appear via hierarchy propagation | STRUCTURAL |
| resize / DPI change | not handled (fixed world constants; `Monitor` fold at startup only) | gate opens: layout recompute (one pass of rect arithmetic), guarded transform/`custom_size`/`TextBounds` writes on all placed entities, text re-wrap by the engine, measurement refinement ≤3 frames | STRUCTURAL; NOT MEASURED |
| startup | 8 procedural textures incl. a monitor-height-dependent card texture; 26 spawns | 3 procedural textures (4×4, 64×64, 64×64) + 6 PNG loads; 26 spawns; layout performed once on the first Update frame (no duplicated layout math anywhere in the tree, including CI — the harness parses application-logged rects) | STRUCTURAL |
| per-frame state reconstruction | state read back from `Visibility`/alpha/`scale.z` on gated frames | eliminated: authoritative `UiState` snapshot; presentation is write-only downstream | STRUCTURAL |

A gated frame is split into a pure transition (no system parameter is
touched, nothing is allocated, the result is one `Copy` `(UiState, bool)`
value) and one effect pass that runs only when the gate opened: at most one
equality-guarded resource assignment, at most one exit message, at most two
log lines, and the guarded component writes listed above.

Assets are built once (each spawned sprite retains its own `Handle<Image>`;
no separate image resource is kept, because nothing would read it) and never
regenerated; hover/focus changes touch only color/visibility; a settled
focus ring performs zero allocations and zero geometry updates per frame.
Optional interaction transitions were deliberately not implemented
(immediate state changes), so no persistent animation loop exists (§F
permits but does not require transitions).
