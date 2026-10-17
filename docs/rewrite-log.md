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
- Spec revision 2: arrived after the first implementation pass (identical
  Sections 1–16; **new Sections 17–25**: low-level Bevy control contract,
  construction/frame-execution separation, ownership & state representation,
  change-detection/invalidation protocol, rendering & asset-lifetime
  invariants, performance budgets, checker hardening with fixtures,
  verification traceability, completion gates). Implemented in commit
  `11a4044`.
- Spec revision 3 (current): a restructured mandate (828 lines). Binding
  deltas implemented in this pass: exactly one `fn` item in maintained Rust
  (no helpers, no `#[test]` functions); no explanatory comments in any
  maintained `.rs`/`.toml`/`.yml` (explanation belongs in documentation);
  flattened closure nesting (each level must have a distinct role); a
  combined feasibility gate executed outside the maintained tree; error
  transparency (no `unwrap`/`expect`/`panic!` shortcuts added, workflow exit
  statuses propagate); regression-controlled optimization with evidence
  classification; a final whole-tree re-scan; and a ten-item final report.
  The checker moved out of the (now prohibited) Rust test module into a
  Python policy job embedded in CI, with fixtures.
- Spec revision 4 (current): rev-3 plus the mandatory UI correction contract
  (§§A–I) and four one-line amendments (core requirement #10, §12, the
  completion criterion list, §28 item 6). Binding deltas implemented in this
  pass: explicit `Destination` selection (Home initial, Registry on
  activation); visible native ungrabbed cursor; optional cursor position
  never fabricated; primary-`Window` metrics in logical pixels (no `Monitor`
  fold); hit areas derived from the same layout model that renders;
  press-arms / release-inside-activates / cancel-on-(outside, focus loss,
  hide, removal); authoritative `UiState`/`Role` state with verified derives
  now permitted; one scale-factor-aware layout model with real text
  measurement; monochrome six-state styling with a visible keyboard focus
  ring and Tab/Shift+Tab/Enter/Space semantics; committed Cargo-resolved
  `Cargo.lock` validated with `--locked`; `x11` backend enabled to match the
  Xvfb smoke environment; readiness/interaction signals replacing
  timeout-survival proof; policy checker de-contradicted (attribute
  allowlist, generalized lint rule, exact `&mut` inventory).

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
  relaxation is `&mut` in signature positions (documented, machine-checked,
  nine occurrences inventoried in docs/architecture.md §13.1). The third
  revision's combined feasibility gate (compile + run a minimal
  closure-conversion experiment outside the maintained tree) is **BLOCKED**
  in this sandbox: no Rust toolchain is installable, so no experiment binary
  could be compiled or executed anywhere; the experiment was kept out of the
  repository (it existed only as `/tmp` analysis scripts) and the blockage is
  reported rather than papered over with a source-only claim.
- **C — minimal vertical slice**: not executed as a separate compile step —
  **blocked**: no Rust toolchain installable in the sandbox (rustup's
  distribution server `static.rust-lang.org` and crates.io are outside the
  network allowlist). Mitigation: every symbol verified against the pinned
  source trees instead of the compiler; first compile must happen downstream.
- **D — complete functional migration**: done (source-complete), and
  re-done for the third revision as a single-`fn` transformation:
  `src/main.rs` is now 1516 lines containing exactly one `fn` item (line 1)
  — the second revision's 13 pure helpers were inlined byte-identically at
  their call sites (pixel formulas as immediately-applied closure
  expressions, logic expressions folded into the Startup/Update chains) and
  the 13-test module was deleted (test functions are prohibited absent
  explicit approval; the approval question is recorded as an open decision).
  `Cargo.toml`, `rust-toolchain.toml`, and both workflows were rewritten
  comment-free; all explanation moved into `docs/`. Behavior preserved per
  docs/architecture.md §2–§3.
- **E — evidence-based optimization**: partially done. Statically-justified
  work removal only (monitor fold merge, exit-check algebra, kept update
  gate, coherent keyboard snapshot, and — after verifying v0.20.0's
  `Mut::into_inner` change-tick semantics from source — the guarded write
  pass required by spec §20); **no measurements possible** (see
  docs/performance.md — everything honestly labeled `not measured`, with the
  §22 acceptance-property table marked PASS (static) or BLOCKED).
- **F — CI and release packaging**: done (source-complete), restructured for
  the third revision: `ci.yml` now runs a Python **policy job** (the whole
  former Rust test suite's static scope: syntax contract + fixtures, comment
  policy per file type, manifest/wgpu identity, matrix + naming, assets) and
  a **toolchain job** (fmt/metadata/tree/clippy/check/test/build, all
  `--locked`); `release.yml` gates the build matrix behind `ci.yml` via a
  reusable-workflow call, and each build job now verifies the binary header
  against its target architecture, the archive layout and permission bits,
  extraction into a different working directory, and the recorded checksum;
  the publication job asserts set-equality of the twelve downloaded files
  against the parsed matrix. Per the requester's 2026-10-16 decision, the
  toolchain job also ends with a headless process smoke check (Xvfb +
  software Vulkan, 12 s survival, no panic, no early exit) as black-box
  compensation for the prohibited Rust test functions. Exact six-entry
  matrix, SHA-pinned actions (see
  the version table below), least-privilege permissions, guarded publication
  kept. YAML parse-validated locally; every embedded Python block executed
  locally against the real tree and synthetic fixtures (commands table).
  Remote execution not performed.
- **G — final verification**: partially done — see commands below.
- **Spec-v2 response pass (Sections 17–25)**: done (source-complete).
  Code: guarded write pass (§20), `next_launched` helper + same-frame
  boundary test (§18.2), coherent keyboard snapshot (§22), checker keywords
  extended with `transmute`/`from_raw_parts`/`as_mut_ptr` (§17.2/§23),
  committed checker fixtures — 6 negative, 28 positive, 1 block-comment
  (§23). Docs: architecture.md restructured (integration-level table §6,
  construction-vs-frame graph §5, ownership §7, invalidation protocol §8,
  coordinate spaces & asset lifetime §9, formal exception request + checker
  boundary §13); bevy-audit.md gained six newly verified engine facts and the
  compact acceptance map (§24); performance.md gained the §22 acceptance
  properties. Completion gates (§25): each gate answered in the acceptance
  map or the sections above; the rustfmt-readability gate and everything
  requiring compilation remain NOT RUN/BLOCKED (no toolchain), reported not
  concealed.
- **Spec-v3 response pass (current)**: done (source-complete).
  Code: single-`fn` transformation of `src/main.rs` (2798 → 1516 lines;
  assembly via a `/tmp` line-surgery script kept outside the repository,
  then helper-call inlining with byte-identical bodies, then syntax-aware
  comment stripping); Startup merged into one handles IIFE (15 parameters:
  13 shared handles + badge + world, 26 spawns); Update flattened to
  snapshot → gate → next-values → guarded write pass with three same-level
  per-entity IIFEs (max 7 closure levels, each with a distinct role).
  Config: `Cargo.toml` + `rust-toolchain.toml` rewritten comment-free
  (content otherwise unchanged). Workflows: full rewrites (see stage F).
  Docs: architecture.md (§1/§2/§3/§5/§6/§7/§8/§9/§10/§11/§12/§13/§14),
  bevy-audit.md (approval request + feasibility status + acceptance map),
  performance.md (evidence classification), README.md; every Rust-test
  reference removed repo-wide. Final whole-tree re-scan: CI policy scan
  executed locally — ALL CHECKS PASSED (see commands).

## Commands executed in this environment and outcomes

| command | outcome |
| --- | --- |
| `git ls-remote origin` | only `main` + tag `v0.0.0.0.0.0-s0`; arena branch absent |
| GitHub refs API for `v0.20.0` / `v0.20.0-rc.2` | commits recorded above |
| codeload tarballs of both Bevy revisions + wgpu `v30.0.1` | downloaded, extracted, diffed/grepped |
| `diff -rq` per touched crate (rc.2 vs 0.20.0) | drift limited to additive changes (details in docs/bevy-audit.md) |
| ad-hoc structural scan of `src/main.rs` (Python mirror of the committed Rust checker) | 0 contract violations; brackets balanced; literals terminated; no `r#`/block comments — re-run after the spec-v2 edits |
| Python simulation of the committed fixtures test and boundary test | all 6 negative fixtures → 0 violations; all 28 positive fixtures → flagged with the expected keyword; block-comment fixture → stripped; all 7 same-frame boundary assertions pass |
| engine-source verification for §20 (change-detection semantics) | `Mut::into_inner` sets CHANGED (`change_detection/traits.rs#L516-L523`); visibility propagation gated on `Changed<Visibility>` (`bevy_camera/visibility/mod.rs#L638-L648`); sprite mesh prep gated on `Changed<Sprite>` (`bevy_sprite_render/sprite_mesh/mod.rs#L180-L184`) |
| engine-source verification for §21 (coordinate spaces) | `cursor_position()` = physical ÷ scale_factor (logical, window.rs#L618-L623); `Camera2d` requires centered orthographic projection (components.rs#L9-L16); runtime projection updated from physical target size (bevy_render/camera.rs#L432) |
| PyYAML parse of both workflows + matrix comparison against the mandated contract | parse OK; matrix equals the six required entries in order; all required `${{ matrix.* }}` expressions present; no doubled `.exe` |
| `git ls-remote` for the four pinned action tags | SHAs match the pins in the workflows |
| (rev 3) transformation script on `src/main.rs` + structural re-scan | 1516 lines; 0 comments; 0 attributes; exactly 1 `fn` (line 1); 0 `impl`; 0 keyword violations; `&mut` inventory exactly the 9 approved spellings; brackets balanced; 27 spawns / 8 image adds / 6 loads / 2 `add_systems` / 2 `add_plugins` |
| (rev 3) local execution of the CI policy job's embedded Python against the real tree | fixtures (keyword ±, comments, TOML, YAML, fn-count, attributes) PASS; rust source policy PASS; manifest policy PASS; workflow comment/pin policy PASS; release matrix + naming PASS; asset policy PASS — "ALL CHECKS PASSED", exit 0 |
| (rev 3) local execution of the release workflow's embedded Python blocks against synthetic fixtures | matrix self-check PASS; binary-header inspection PASS for ELF x86_64/aarch64, Mach-O arm64, PE x86_64/arm64 and correctly REJECTS an architecture mismatch; archive layout + permission verification PASS (unix and Windows shapes); checksum record→verify round-trip PASS; downloaded-set equality PASS (12/12) and correctly REJECTS both a missing and an extra file |
| `cargo fmt/check/clippy/test/build`, `cargo metadata`, `cargo tree` | **NOT EXECUTED in the sandbox** — no toolchain, no crates.io access (hard blocker, not a skipped convenience). **Executed remotely for the first time on 2026-10-16** (GitHub Actions, ubuntu-24.04, run 38969211084, workflow_dispatch): policy job PASS; `cargo generate-lockfile` PASS; `cargo fmt --all -- --check` PASS; `cargo metadata --locked` + `cargo tree --locked -d` PASS; the whole Bevy v0.20.0 dependency graph **and the transformed single-`fn` crate type-checked and borrow-checked successfully** under the clippy driver; the run failed only on 36 lint errors (24 × `redundant_closure_call` on the contract-mandated IIFE pattern, 12 × `manual_clamp`) — both resolved (decision 18); `cargo check --all-targets`, `cargo test` (0 tests), `cargo build --release`, and the headless smoke check had not been reached yet in that run |

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
cargo test --locked              # honestly reports 0 tests (test fns prohibited by spec)
cargo build --locked --release
cargo tree --locked -d
./target/release/subsystem-for-arxumbra   # from the repository root (assets/ is cwd-relative)
```

If `cargo fmt` reformats `src/main.rs`, re-run the CI policy job afterwards —
it scans the formatted source too (formatting does not change tokens, so a
pass before `cargo fmt` implies a pass after, but re-running is free).

## Pinned action versions (mapping recorded outside the workflows, per spec §23)

The workflows pin actions to full commit SHAs and contain no comments, so the
SHA↔version mapping lives here (verified via the GitHub refs API and
`git ls-remote`):

| action | pinned SHA | version tag |
| --- | --- | --- |
| `actions/checkout` | `3d3c42e5aac5ba805825da76410c181273ba90b1` | v7.0.1 |
| `actions/cache` | `55cc8345863c7cc4c66a329aec7e433d2d1c52a9` | v6.1.0 |
| `actions/upload-artifact` | `cf430e030ddbb5b0abf93d22962f4752f3646cd9` | v7.0.2 |
| `actions/download-artifact` | `9000827ccba6bdab643e8b6fd33ac0654aef8333` | v8.0.2 |

## Decisions log

1. Tag pinning (`tag = "v0.20.0"`) over `rev`: manifest readability; the
   resolved commit is recorded in `Cargo.lock` once regenerated (and is
   documented here: `b3bdd75b…`). Never both keys at once.
2. `Cargo.lock` deleted instead of kept stale: the old lock pins rc.2 +
   direct wgpu 30 and would contradict the new manifest under `--locked`.
   Workflows bootstrap it via `cargo generate-lockfile` when absent and use
   `--locked` everywhere else.
3. `WgpuLimits` → engine default (documented deviation, rationale in
   docs/architecture.md §10); all other baseline renderer settings preserved
   through public re-exports.
4. Cargo `version` bumped `0.0.0-s0` → `0.0.0-s1` to track the mandated
   distribution label's `s1` while staying valid semver; the label itself is
   never written into `Cargo.toml`.
5. Nightly removed everywhere (profile gate, `-Z` rustflags, `build-std`,
   `target-cpu=native`, clang/lld linker overrides); stable 1.99.1 pinned.
6. Helper-function extraction limited to pure numeric/logic functions that
   the mandated regression tests need; application assembly remains 100% in
   the `main` chain. — **Superseded by revision 3**: exactly one `fn` item is
   now required, so all helpers were inlined byte-identically (decision 11).
7. (spec v2, §20) Guarded write pass: values are compared through
   non-flagging `Deref` reads and `Mut::into_inner` is consumed only on an
   actual difference — after verifying from the pinned source that
   `into_inner` itself sets the CHANGED tick and that visibility propagation
   and sprite mesh preparation are change-gated. Final values and frame
   timing are identical to the baseline; only spurious invalidation is
   removed. No allow-attributes were added anywhere (`fn_params_excessive_bools`
   is a pedantic-group lint, off under CI's default `-D warnings`).
8. (spec v2, §18.2) The launch transition moved into named `next_launched`
   with the byte-identical boolean expression, so the same-frame
   toggle+click boundary ("click evaluated against post-toggle visibility")
   is defined in one place and regression-tested instead of implicit. —
   **Amended by revision 3**: the same expression is now inlined at its
   single call site (no named helper may exist); the boundary definition and
   its Python-simulated truth table are unchanged (docs/architecture.md §5.2).
9. (spec v2, §21.1) The baseline's mix of logical cursor coordinates with a
   physical-pixel layout at scale factors ≠ 1 is preserved (baseline
   fidelity) and reported as an open decision rather than silently changed —
   per the mandate's conflict rule (bevy-audit.md, Unresolved questions #3).
10. (spec v2, §23) Checker keyword list extended with `transmute`,
    `from_raw_parts`, `as_mut_ptr` (raw-pointer/transmute escape routes of
    §17.2); positive/negative fixtures committed inside `mod tests`; the
    whole-tree scan already covered every handwritten `.rs` file
    (`src/**` + `build.rs` if present) — the inspection boundary (tokenizer,
    not macro-expansion analysis) is documented in architecture.md §13.2. —
    **Relocated by revision 3**: the checker and its fixtures now live in the
    CI policy job (decision 13); the keyword list and boundary are unchanged.
11. (spec v3) Single-`fn` transformation strategy: a `/tmp` line-surgery
    script (kept outside the repository, per the "feasibility work outside
    the maintained tree" rule) assembled the new Startup/Update chains from
    the verified second-revision text, inlined every helper call by
    paren-matched substitution of its byte-identical body as an
    immediately-applied closure expression, and stripped comments with a
    syntax-aware scanner — then the result was re-scanned (final whole-tree
    re-scan requirement). No value, expression, or spawn order changed; the
    diff is structural only.
12. (spec v3) Startup merged the separate handles and badge IIFEs into one
    15-parameter IIFE (13 shared handles + badge handle + world) and kept
    the monitor IIFE outside it, because the menu-card pixel formula takes
    the monitor `height` as a parameter — the nesting preserves that data
    dependency while reducing closure count (regression check: fewer Startup
    allocations, identical spawns).
13. (spec v3) Checker relocation: revision 3 prohibits test functions and a
    second Rust entry point, so the whole static-check scope (syntax
    contract, comment policy, manifest/wgpu policy, matrix/naming policy,
    asset policy) moved into a Python policy job embedded in `ci.yml`,
    executed before any Rust step, with fixtures run first. The release
    workflow calls `ci.yml` as a reusable workflow, so tag pushes enforce the
    identical gate before any target build starts.
14. (spec v3) Comment policy: all comments removed from `src/main.rs`,
    `Cargo.toml`, `rust-toolchain.toml`, and both workflows (YAML step
    *names* and shell `echo` lines are functional data/output, not comments,
    and were kept). Action-version annotations (`# v7.0.1` style) were
    replaced by the SHA↔version table in this file, per the spec's
    "version mapping outside comments" rule. Legal/execution-metadata
    boundaries (shebangs, license headers, lockfile headers) do not occur in
    this tree; none were touched.
15. (spec v3) Packaging verification added to the release build jobs:
    binary-header architecture inspection (ELF/Mach-O/PE machine fields),
    archive layout + permission-bit verification, extraction into a fresh
    temporary directory, checksum record→verify round-trip, and
    publication-time set-equality of the twelve downloaded files against the
    parsed matrix — all cross-platform via `python3` (present on all six
    runner images), avoiding `jq`/`sha256sum` portability gaps.
16. (spec v3) Error transparency: no `unwrap`/`expect`/`panic!` was
    introduced anywhere (none exists — machine-checked keyword-adjacent
    policy via the fixtures); every workflow run block starts with
    `set -euo pipefail`, every Python block fails through assertions or
    non-zero exit, and no step uses `continue-on-error`.
17. (spec v3, RESOLVED 2026-10-16) Two decisions were escalated to the
    requester instead of being taken silently, and both were answered:
    (a) the nine `&mut` signature-position spellings
    (docs/architecture.md §13.1 — narrowest possible relaxation; precedent:
    the original spec's petition clause and the baseline's own
    `|world: &mut World|`; the CI policy job enforces the exact inventory) —
    **approved as implemented**; (b) the fate of behavioral regression tests
    (revision 3 prohibits `#[test]` functions by default while revision 1
    mandated them) — **compensate with black-box checks**: no test module is
    restored; the CI toolchain job gained a headless process smoke check
    (Xvfb + software Vulkan: the release binary must survive 12 s of Startup
    + Update frames with no panic and no early exit — exit status 124 from
    `timeout` is the pass condition), which joins the static policy job and
    the release pipeline's package-inspection steps. The smoke check cannot
    be executed in this sandbox (no toolchain); its first real run is the
    remote CI run, and a software-device failure there would fail loudly
    rather than being concealed (no `continue-on-error` anywhere).
18. (remote CI feedback, 2026-10-16) The first remote run failed only on
    clippy lints (everything else — policy scan, lockfile bootstrap,
    `cargo fmt --all -- --check`, metadata, tree, and the full type/borrow
    check of the transformed crate — passed). Resolution of the two lint
    families:
    (a) `manual_clamp` (12 sites): fixed **in code** —
    `f32::min(f32::max(v, 0.0), 1.0)` → `v.clamp(0.0, 1.0)`. Equivalence:
    identical for all finite non-NaN `v` (verified over 250k sampled values
    incl. ±inf and denormals; at `v == -0.0` the results differ only in the
    sign of zero and both cast to `0u8`); the Rust-level NaN divergence
    (`f32::max(NaN, 0.0) = 0.0` vs `NaN.clamp(..) = NaN`) is unreachable —
    every clamped argument is built from finite pixel coordinates and
    non-zero constant divisors, except `(height - 48.0)`, which cannot be
    zero for a monitor hosting the 120-px-minimum fullscreen window
    (fallback: 1080). Even in that impossible case the saturating `as u8`
    cast keeps direct-channel pixels identical; no panic is possible.
    (b) `redundant_closure_call` (24 sites): the lint targets the
    immediately-applied-closure pattern itself — the only `let`-free value
    binding under the syntax contract — so it is unfixable without violating
    the contract, and source attributes are banned by the comment/attribute
    policy. Allowed via **one targeted manifest entry**
    (`[lints.clippy] redundant_closure_call = "allow"`), and the CI policy
    job now asserts the manifest's entire `[lints]` table equals exactly
    that entry — any additional or blanket suppression fails CI. This is
    neither `allow(warnings)` nor clippy disabling: all other lints stay at
    `-D warnings`.
19. (rev-4) `x11` Bevy feature added alongside `wayland`: the correction
    contract §H forbids labeling an Xvfb probe as a passing application test
    while the manifest selects only Wayland. `x11` resolves through
    `bevy_internal/x11 → bevy_winit/x11 → winit x11` (x11-dl, dlopen-based:
    no new build-time system dependency; runtime needs the standard
    libX11/libxcb/libxkbcommon stack, pulled by the smoke step). Wayland is
    kept so no platform support is removed. Production fullscreen behavior
    is unchanged; the feature only adds a window-system backend.
20. (rev-4) Typed `Update` system with seven annotated parameters instead of
    the rev-3 exclusive `&mut World` update: state flows through
    `ResMut<UiState>` snapshots, never through sprite scanning. Seven keeps
    the closure at the previously remote-proven clippy `too_many_arguments`
    boundary (the old 7-argument system closure and a 15-argument threading
    IIFE both passed `-D warnings`). `FrameCount` was dropped as a parameter
    in favor of `Time::elapsed_secs` for the one-shot readiness signal.
21. (rev-4) Local types inside `fn main` (`Destination`, `Control`, `Role`,
    `Layout`, `UiState`, `UiImages`): contract §8 prefers state type
    declarations local to main; Rust permits item declarations in function
    bodies, and Bevy's `Component`/`Resource` derives expand legally there
    (local types are `'static`). No helper methods are attached.
22. (rev-4) Layout model choice: explicit sprite/text layout (not Bevy UI
    node layout). Rationale: the correction contract requires one model used
    for both placement and hit testing; the rect-based model is that single
    source by construction, needs no extra plugin/features, keeps the
    frame-time cost to guarded writes on change, and its responsive
    behavior is fully simulable (and was simulated at five viewports ×
    five measurement variants). Bevy UI would add a second geometry
    (node layout) that hit testing would have to read back per frame.
23. (rev-4) Text measurement: `TextBounds` wrap widths from the model;
    `TextLayoutInfo.size / scale_factor` read back for the title and
    description heights until stable (`measured_settled`). Converges in
    ~3 frames at startup while the HUD is hidden, so refinement is
    invisible; no fixed character-width estimate is used anywhere.
24. (rev-4) Cargo.lock delivery: a one-shot `lockfile.yml` workflow
    (branch-scoped push trigger, `contents: write` only for that job,
    `hashFiles` guard so it no-ops once the lock exists) ran
    `cargo generate-lockfile` on the runner and committed the result
    (`c2f43178`, `[skip ci]`). The lock is Cargo-resolved, never
    hand-edited; ci.yml/release.yml now hard-fail when it is absent and
    every dependency command runs `--locked`. The bootstrap workflow is
    deleted once the final state is green, and the policy job's workflow
    inventory returns to {ci, release}.
25. (rev-4) Checker de-contradiction: attributes are validated against a
    derive allowlist (verified spellings only; inner attributes and all
    other forms still rejected, fixtures both directions); the lint rule
    generalizes from one hardcoded equality to "named lints only, explicit
    levels, never `all`/`warnings`"; the `&mut` audit keeps exact
    per-spelling counts (8 now, re-requested in architecture.md §13 — the
    prior approval explicitly does not carry over).
26. (rev-4) Smoke redesign: readiness = parsed `ARXUMBRA-INTERACTIVE` line
    (proves ≥2 s of Update frames with renderer up), interaction = xdotool
    keyboard/pointer scenarios driven by the application-logged control
    rects (no duplicated layout math in the harness), assertions = ImageMagick
    AE thresholds between captured states (appearance >4000 px, ring >100 px,
    hover >2000 px, cancelled-press <3000 px while ≠ Registry state >4000
    px), termination = Escape-hold self-exit code 0. Lavapipe identity is
    captured and reported as software-renderer evidence, separately from any
    physical-GPU claim. `cargo fmt --check` and `clippy` diagnostics are
    mirrored into the run summary because this environment cannot fetch job
    logs (results-receiver host unreachable); the strict steps still fail
    the job on any diff/warning.

27. (rev-4) Diagnostics oracle. This environment cannot fetch Actions job
    logs, artifacts, or check-run summaries, so the Lint step strips ANSI
    escapes (`CARGO_TERM_COLOR: always` is workflow-level env, and colored
    output defeated every `^error` pattern — the reason two consecutive
    failures reported nothing but "exit code 101"), re-runs clippy with
    `--message-format short`, and mirrors the located one-line diagnostics
    into annotations carrying `file=`/`line=`, spread over the error,
    warning and notice levels because GitHub retains only ten annotations
    per level per step. The smoke step mirrors the ANSI-stripped
    application-log head and tail the same way, on failure only.
28. (rev-4) Temporary rustfmt bootstrap. `lockfile.yml` (already
    `contents: write`, branch-scoped, and already slated for deletion) also
    runs `cargo fmt --all` and commits any diff, because no Rust toolchain
    can be installed in this sandbox (only github.com, npm and PyPI hosts
    are reachable, so `static.rust-lang.org` is not). It has never had to
    commit: the hand-formatted waterfall has matched rustfmt byte-for-byte
    at every push, which the strict `cargo fmt --check` step independently
    confirms. Both scaffolds — lockfile generation and formatting — are
    deleted before the final state, and the policy workflow inventory
    returns to {ci, release}.
29. (rev-4) Effect-pass separation, forced by compile errors. The
    transition chain now evaluates to `(UiState, layout_dirty)` and touches
    no system parameter; the exit write, the readiness and layout log
    lines, the snapshot write and the visual pass run in a `map_or((), …)`
    closure outside the chain. Reason: threading `state`, `exit` and
    `entities` as closure arguments required annotating them, and the query
    annotation cannot be inferred from use — a query's *item* types
    (`Mut<T>`) differ from its *query-data* spellings (`&mut T`), so rustc
    took the item shape from the `match` patterns, bound `control: Control`
    instead of `&Control`, and reported six "cannot be dereferenced" errors
    plus an argument mismatch and one "type annotations needed". Annotating
    the threaded parameter would have added five more `&mut` spellings for
    the same five accesses (§13); capturing keeps the inventory at 8.
30. (rev-4) Window facts fold. `for window in windows.iter()` became
    `windows.iter().fold(…)` into one `Copy` tuple consumed by a
    destructuring closure parameter, because a loop body may not move
    `ResMut`/`Query` values ("value moved here, in previous iteration of
    loop"). The fold keeps exactly one primary window's facts and needs no
    second world borrow.
31. (rev-4) `map_or` as tail expression. `Option::map_or` is `#[must_use]`,
    so the visual pass's `rect.map_or((), …)` is its closure's tail
    expression instead of a `;`-terminated statement; no discard binding is
    needed (and `let _ =` is banned anyway). `unnecessary_map_or` was
    verified against clippy's source to fire only on boolean-literal
    defaults, so the unit default is safe.
32. (rev-4) Dead code removed on compiler evidence. `UiImages` (nine
    fields never read — each spawned `Sprite` already retains its own
    handle) and `Layout.pad` (never read; the padding constants stay inline
    in the formulas). The policy's struct inventory moved to
    `["Layout", "UiState"]` and the derive count to five in the same
    commit, so the checker still asserts the exact set.
33. (rev-4) Lint dispositions. Fixed in code: `.max(240.0).min(560.0)` →
    `.clamp(240.0, 560.0)` (`manual_clamp`), `focused_mid == None` →
    `focused_mid.is_none()`. Allowed in the manifest: `type_complexity`,
    for the typed query parameter only — scoring, alternatives and the
    approval request are recorded in architecture.md §12.
34. (rev-4) Smoke environment hardening. The smoke step installs the X11
    runtime libraries that winit's x11 backend dlopens (`libx11-6`,
    `libxcursor1`, `libxrandr1`, `libxi6`, `libxkbcommon0`,
    `libxkbcommon-x11-0`) next to Xvfb, xauth, xdotool, imagemagick,
    x11-utils, mesa-vulkan-drivers, libvulkan1 and vulkan-tools. Remote
    run `39033443676` reached the smoke step and died in it with exit
    code `100` — an `apt-get install` failure, not an application failure,
    and it produced no annotations at all because the step aborted before
    any evidence line. Three changes follow from that:
    (a) the install now runs with `-o DPkg::Lock::Timeout=180` inside a
    three-attempt loop that refreshes the package lists between attempts,
    and, if the batch still fails, retries package by package so that one
    bad name cannot hide the rest; only the tools the harness itself needs
    (`xvfb`, `xdotool`, `imagemagick`) are fatal when unavailable, while a
    missing X11 runtime library is reported per package and left to show
    up as a `dlopen` failure in the mirrored application log. Remote run
    `39035672621` proved the value of the mirroring: the annotated apt
    tail read `E: Unable to locate package libxrandr1` — the Debian-style
    name was wrong, Ubuntu noble ships the Xrandr runtime as
    **`libxrandr2`** (SONAME `libXrandr.so.2`), exactly the library winit's
    x11 backend dlopens. Every other name in the list was re-checked
    against the SONAME convention (`libx11-6`, `libxcursor1`, `libxi6`,
    `libxkbcommon0`, `libxkbcommon-x11-0`);
    (b) every failure reason is accumulated into one
    `::error title=smoke-failures::` annotation and the ANSI-stripped
    application log is mirrored (head 4 lines as warnings, tail 9 as
    errors) so that a smoke failure is diagnosable without job-log access,
    within the ~10-annotations-per-level cap;
    (c) the step starts with `set +e`/`set -u` instead of `set -uo
    pipefail`. GitHub's default shell for a Linux `run:` block is
    `bash -e {0}`, so errexit stayed active before and any
    non-zero-but-handled command (notably `wait "$APP_PID"` when the app
    exits on its own, or a `grep` that finds nothing in a pipeline) would
    abort the step mid-evidence. The exit code is now captured with
    `wait "$APP_PID" || APP_RC=$?`.
36. (rev-4) Smoke screen geometry. Xvfb was started as `1366x768x24` while
    the application requests a borderless fullscreen window with a
    `1920x1080` resolution. Xvfb runs no window manager, so
    `_NET_WM_STATE_FULLSCREEN` is not guaranteed to be honoured: the window
    could have been placed at its requested `1920x1080` geometry on a
    smaller screen, i.e. clipped, and the pointer coordinates the harness
    derives from the application's own logical rects would then not map
    onto the visible screen. The screen is now `1920x1080x24`, so the
    requested geometry, the fullscreen geometry and the root window
    coincide on either path and the logical coordinates parsed from
    `ARXUMBRA-INTERACTIVE` are root-screen coordinates. The
    "cursor moved away" step targets `(1880, 1040)`: at that viewport the
    sidebar occupies x 0..340 and the card x 850..1410 / y 24..269, so the
    point is on screen and outside every interactive rect. Two notices were
    added (the `xdpyinfo` dimensions and the application's own `w=/h=/s=`
    metrics) so that a geometry mismatch appears in the annotations instead
    of only in unreadable stdout.
37. (rev-4) The smoke AE thresholds were dead code; the first green run
    exposed it. The scenario loop parsed each entry as
    `name1 name2 THRESHOLD DIRECTION description` but then tested
    `[ "$3" = gt ]` — field 3 is the threshold, field 4 is the direction —
    so the direction test was never true and **no AE threshold was ever
    enforced**; only the separately coded `home2n`/`cancel1` upper bound
    ran. Remote run `39037415773` (the first fully green run of this crate)
    reported the metrics as notices, which is how the defect surfaced:
    `home1~focus1=64` against an intended floor of 100 would have failed
    the step had the check been live. Fixed by naming the fields (`A`, `B`,
    `DIRECTION`, `THRESHOLD`) and handling both directions in one loop. The
    two ring pairs now assert a floor of 500 px: a 3 px stroke around a
    308x56 tab is roughly 2.2 kpx, and the observed `focus1~focus2=2352` is
    consistent with one ring appearing while another disappears, so 500 is
    a floor a real ring clears several times over and a no-op does not.
    Two additions make the next run self-diagnosing instead of ambiguous:
    every capture is repeated until two consecutive captures are
    pixel-identical (`smoke-shots` notice reports the probe count per
    capture), so a screenshot can no longer be taken mid-frame under
    lavapipe, which is the most likely explanation for a 64 px residue on a
    transition that should have moved ~2 kpx; and the focus-ring region is
    cropped using the rect the application itself logged and compared
    separately (`smoke-ring` notice), which distinguishes "the ring did not
    appear" from "something elsewhere on screen changed".
    One negative result is worth keeping: `home2n~cancel1=0` with the
    pointer at (1880, 1040) and (5, 5) respectively proves
    `import -window root` does **not** capture the X cursor, so cursor
    evidence in this harness is behavioral (the hover restyle,
    `registry1~hover1=16798`, is driven by `Window::cursor_position()`) and
    not pixel-level. No screenshot from the run could be retrieved in this
    environment either: the artifact
    `smoke-evidence-linux-x11-lavapipe` (50,081 bytes) exists, but its
    download URL redirects to a blob host outside this sandbox's allowlist.
35. (rev-4) Equality-guarded snapshot write. `ResMut::into_inner` consumes
    the parameter and unconditionally calls `set_changed()`
    (`bevy_ecs/src/change_detection/traits.rs`, `impl_methods!`), so an
    ungated write flags the resource on every gated frame even when the
    transition produced an identical value. `UiState` derives `PartialEq`
    and is `Copy`, so the write-back is now
    `if *state != next_state { *state.into_inner() = next_state; }`. The
    single read of the previous value (`!state.interactive_logged`) still
    precedes the consumption, and nothing reads `state` afterwards.

## Commands executed for revision 4 (this environment) and outcomes

- Re-downloaded and inspected Bevy `v0.20.0` (`b3bdd75b`) source; API
  verification greps recorded in docs/bevy-audit.md (x11 feature chain,
  `CursorOptions`, `TextBounds`/`TextLayoutInfo`/`FontSize`,
  `Rect::{new,from_corners,contains,size}`, `BorderRect`/`TextureSlicer`,
  `ButtonInput::{press,just_released}` repeat semantics,
  `Time::{delta_secs,elapsed_secs}`, `ChildOf`, owned-`Query`
  `IntoIterator`, `LogPlugin::default`, text2d camera `scale_factor`).
- `python3` policy-scan extraction of the updated ci.yml job: ALL CHECKS
  PASSED (locally, before push).
- `python3` verification suite (layout simulation, 12-scenario state
  machine, contrast, structural invariants): ALL CHECKS PASSED; kept at
  `/home/user/arc-parts/verify_rev4.py` (outside the maintained tree).
- `bash -n` on every workflow `run:` block: OK (16/16).
- git: rebased the session onto the true remote tip (`64e92baa`) after
  discovering the sandbox had re-cloned from `main`; pushed `0ad59fe`
  (rev-4 implementation), `c2f43178` (CI-generated Cargo.lock), `975152b`
  (apt retry hardening). Remote results are recorded in the run ledger as
  they land; nothing is claimed before observation.
- Remote iteration against the annotation oracle (decision 27), one push
  per observed failure: `31b25ee` (mirroring added, produced nothing —
  colored logs), `ca0d2f8` (ANSI stripped, short-format pass, located
  annotations — first readable diagnostics: ten findings, four distinct
  error codes), `81da43d` (effect-pass separation, window-facts fold,
  destructured visual tuple — four findings left, all "use of moved
  value"), `a94c6ab` (`map_or` instead of the option loop, `type_complexity`
  allowed in the manifest — four lint findings left), `a0f8646` (all four
  fixed or removed). Remote results at `a0f8646`: policy ✓, formatting ✓,
  lockfile requirement ✓, `cargo metadata`/`tree --duplicates` ✓,
  `cargo clippy --all-targets -- -D warnings` ✓,
  `cargo check --all-targets` ✓, `cargo test` ✓ (zero tests, reported
  honestly), release build and X11 smoke check in flight at the time of
  writing.
- Clippy source read through `api.github.com` (`rust-lang/rust-clippy`,
  `stable`) before pushing, because a blind allow would have been
  unjustifiable: `clippy_lints/src/types/type_complexity.rs` (the scoring
  visitor: `10 × nest` per path/tuple/array node, `1` per reference,
  threshold comparison is strictly greater) and
  `clippy_lints/src/types/mod.rs` (`check_fn` → `check_fn_decl` runs for
  `FnKind::Closure`, so closure parameter annotations are linted), plus
  `clippy_lints/src/methods/unnecessary_map_or.rs` (fires only when the
  default is a boolean literal).
- Bevy `v0.20.0` source verification in the pinned tree already downloaded
  here: `ResMut::into_inner(mut self) -> &'w mut T` generated by
  `impl_methods!` in `bevy_ecs/src/change_detection/traits.rs` (it calls
  `set_changed()`), `Messages::write(&mut self, M)` in
  `bevy_ecs/src/message/messages.rs`, `Sprite`'s exact nine public fields
  and `#[require(Transform, Visibility, VisibilityClass, Anchor)]` in
  `bevy_sprite/src/sprite.rs`, `Msaa` living in `bevy_render::view`, and
  `bevy_log`'s default fmt layer writing ANSI-colored records to stderr
  (`bevy_log/src/lib.rs`, `tracing_subscriber::fmt::Layer::default()
  .with_writer(std::io::stderr)`) — which is why the smoke harness parses
  the log with `.*`-anchored patterns and strips escapes before mirroring.
- Remote CI iteration `a0f8646` (run `39033443676`, job `117398449767`,
  read through `actions/runs/…/attempts/1/jobs` and the check-run
  annotations endpoint): policy ✓, fmt-diff report ✓, `cargo fmt --check`
  ✓ (the hand-formatted deep-nesting waterfall again matched rustfmt
  byte-for-byte), lockfile requirement ✓, `cargo metadata`/`tree` ✓,
  `cargo clippy --all-targets -- -D warnings` ✓, `cargo check
  --all-targets` ✓, `cargo test` ✓ (zero tests), **release build ✓
  (`cargo build --locked --release`)**, X11 smoke check ✗ with `Process
  completed with exit code 100`, artifact upload ✓ but with `No files were
  found with the provided path: /tmp/shots /tmp/smoke.log`, and `Post
  Cache` **skipped** — a failed job does not refresh the `target` cache,
  so the next run rebuilds Bevy from scratch. The first compile-clean,
  lint-clean, link-clean remote run of this crate; the only remaining
  unknown is the smoke gate, which decision 34 addresses.
- Remote CI iteration `9b541bd` (run `39035672621`, job `117404844654`):
  policy ✓, `cargo fmt --check` ✓ (the new equality-guarded write matched
  rustfmt without the bootstrap workflow pushing anything — `git ls-remote`
  still pointed at `9b541bd` after the "Apply rustfmt to the sources" step
  of run `39035669208` succeeded), lockfile requirement ✓, dependency
  metadata ✓, clippy ✓, `cargo check --all-targets` ✓, `cargo test` ✓
  (zero tests), **release build ✓**, smoke ✗. The hardened smoke step
  delivered its first real diagnosis through annotations alone: three apt
  attempts, each warned, then the mirrored apt tail named
  `E: Unable to locate package libxrandr1` (decision 34a). The artifact
  step again reported `No files were found with the provided path:
  /tmp/shots /tmp/smoke.log` because the step aborted before launching the
  application, so no screenshot or application-log evidence exists yet for
  this crate under Xvfb.
- Remote CI iteration `878a3b9` (run `39037415773`, job `117409937279`,
  event `pull_request`, 13:38:49Z → 13:53:09Z): **conclusion `success` —
  every step green**, policy ✓, format diff report ✓, `cargo fmt --check`
  ✓, lockfile requirement ✓, dependency metadata ✓, clippy `-D warnings`
  ✓, `cargo check --all-targets` ✓, `cargo test` ✓ (zero tests), release
  build ✓, X11 smoke check ✓, artifact upload ✓, `Post Cache` ✓ (the
  `target` cache is warm again). Annotation evidence from the smoke step:
  `smoke-vulkan` deviceName/driverName = `llvmpipe (LLVM 20.1.2, 256 bits)`
  / `llvmpipe`; `smoke-screen` dimensions `1920x1080 pixels`, root depth 24;
  `smoke-app-window` `w=1920 h=1080 s=1` (the application's own metrics
  equal the X screen, so logical coordinates are root-screen coordinates);
  `smoke-rects` `home=16,80,308,56 registry=16,144,308,56
  launch=870,165,240,56` (which also proves text measurement settled:
  `tabs_top = 80` implies a measured `title_h` of 48, and the launch pill at
  `y = 165` implies a measured `desc_h` of 44, both below their 104/72
  defaults); `smoke-adapter` `AdapterInfo { name: "llvmpipe (LLVM 20.1.2,
  256 bits)", vendor: 65541, device: 0, device_type: Cpu, driver:
  "llvmpipe", driver_info: "Mesa 25.2.8-0ubuntu0.24.04.4 (LLVM 20.1.2)",
  backend: Vulkan, subgroup_min_size: 8, subgroup_max_size: 8 }` — software
  rendering, reported separately from any physical-GPU claim;
  `smoke-ae` `home0~home1=204000 home1~focus1=64 focus1~focus2=2352
  home1~registry1=34333 registry1~hover1=16798 hover1~home2=34333
  cancel1~registry1=34333 home2n~cancel1=0`; `smoke-exit` exit code 0 after
  the Escape hold. The single anomalous value (`home1~focus1=64`) is
  analysed in decision 37, together with the dead threshold check it
  exposed.
