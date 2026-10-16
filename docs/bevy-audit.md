# Bevy v0.20.0 source audit

Engine revision audited: tag `v0.20.0` → commit
[`b3bdd75becaffe9a16677cf5bae5bd6d198f052b`](https://github.com/bevyengine/bevy/tree/b3bdd75becaffe9a16677cf5bae5bd6d198f052b)
(lightweight tag; `git ls-remote` and the GitHub refs API both resolve the tag
directly to that commit). The previous manifest pin, `v0.20.0-rc.2`, resolves
to `343a0233d6788a49f224a3684c8995e246663df3`. Both trees were downloaded and
diffed crate-by-crate for every crate this application touches; the drift
between rc.2 and the final tag is additive (new methods, internal refactors)
in all files relevant here — no struct field this application constructs
changed shape.

Every permalink below pins the audited commit. "Verified fact" means the
symbol/signature was read in that revision, not recalled from another version.

| mechanism | source file and symbol | commit-pinned permalink | verified fact | project decision |
| --- | --- | --- | --- | --- |
| App construction / plugin tuples / exclusive-system closures | `bevy_app::App`, `add_plugins`, `add_systems` | [crates/bevy_app/src/app.rs](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_app/src/app.rs) | `App::new()`, tuple plugin groups, `add_systems(Startup/Update, closure)` with `&mut World` closures (exclusive systems) | Single dominant chain in `main`, one `run()` terminal |
| Exit handling | `AppExit` enum | [app.rs#L1547-L1550](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_app/src/app.rs#L1547-L1550) | `AppExit` derives `Message` (`reflect(Debug, PartialEq, Clone, Message)`), variants `Success`/`Error` | Escape-hold boundary writes `Messages<AppExit>::write(AppExit::Success)` |
| Message bus (0.20 rename of events) | `Messages<M: Message>`, `write` | [crates/bevy_ecs/src/message/messages.rs#L95](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_ecs/src/message/messages.rs#L95), [#L125](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_ecs/src/message/messages.rs#L125) | `pub struct Messages<M: Message>`; `pub fn write(&mut self, message: M) -> MessageId<M>`; re-exported as `bevy::ecs::message` (`pub mod message`, `pub use messages::*`) | Used for `AppExit`; no event reader needed |
| Task pools | `TaskPoolPlugin`, `TaskPoolOptions`, `TaskPoolThreadAssignmentPolicy` | [crates/bevy_app/src/task_pool_options.rs](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_app/src/task_pool_options.rs) | Fields `min_threads`/`max_threads`/`percent`/`on_thread_spawn`/`on_thread_destroy` unchanged since rc.2 (crate not in drift diff) | Baseline explicit pool sizing preserved (io/async capped at 4 threads) |
| Window configuration | `WindowPlugin`, `Window` (all fields constructed), `CursorOptions`, `ExitCondition` | [crates/bevy_window/src/window.rs](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_window/src/window.rs) | bevy_window has **zero** file drift rc.2→0.20.0; `WindowResolution::new(u32, u32)` at [window.rs#L921-L923](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_window/src/window.rs#L921-L923); `cursor_position() -> Option<Vec2>` at [#L620](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_window/src/window.rs#L620) | Full explicit `Window` literal preserved (borderless fullscreen, hidden cursor, 180×120 min constraints, `resizable: false`) |
| Monitor geometry | `Monitor::physical_width/height` | [crates/bevy_window/src/monitor.rs#L36-L38](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_window/src/monitor.rs#L36-L38) | `pub physical_height: u32`, `pub physical_width: u32` | One fold per read site returns the whole `(width, height)` pair (baseline queried `Monitor` twice per site) |
| Winit event loop ownership | `WinitPlugin { run_on_any_thread }` | [crates/bevy_winit/src/lib.rs#L76](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_winit/src/lib.rs#L76) | Field `run_on_any_thread: bool` | Kept `false`: the event loop owns the main thread (required on macOS/Windows) |
| Accessibility | `AccessibilityPlugin`, root re-export | [crates/bevy_internal/src/lib.rs#L17-L18](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_internal/src/lib.rs#L17-L18) | `#[cfg(feature = "bevy_window")] pub use bevy_a11y as a11y;` and `bevy_window = ["dep:bevy_window", "dep:bevy_a11y", "bevy_image"]` ([bevy_internal/Cargo.toml#L249](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_internal/Cargo.toml#L249)) | `bevy::a11y::AccessibilityPlugin` resolves with the enabled `bevy_window` feature; kept in the plugin list as in the baseline |
| Asset pipeline | `AssetPlugin` fields, `AssetMode::Unprocessed`, `AssetMetaCheck::Never`, `UnapprovedPathMode::Forbid`, `AssetServer::load::<A>` | [crates/bevy_asset/src/lib.rs#L277-L284](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_asset/src/lib.rs#L277-L284), [crates/bevy_asset/src/server/mod.rs#L364](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_asset/src/server/mod.rs#L364) | `pub fn load<'a, A: Asset>(&self, path: impl Into<AssetPath<'a>>) -> Handle<A>`; bevy_asset has zero file drift rc.2→0.20.0 | Unprocessed mode, `assets/` relative folder, six external PNGs loaded once at startup |
| Stable-UUID handles | `Handle::Uuid(Uuid, PhantomData)`, `AssetId::<A>::DEFAULT_UUID` | [crates/bevy_asset/src/handle.rs#L115-L122](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_asset/src/handle.rs#L115-L122), [crates/bevy_asset/src/id.rs#L49](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_asset/src/id.rs#L49) | `Uuid(Uuid, #[reflect(ignore, clone)] PhantomData<fn() -> A>)`; `pub const DEFAULT_UUID: Uuid` on `impl<A: Asset> AssetId<A>` | Default-image and default-font handles constructed exactly as the baseline (reading a dependency-provided associated constant is permitted by the syntax contract) |
| Renderer settings | `RenderPlugin { render_creation, synchronous_pipeline_compilation, debug_flags }`, `RenderCreation::Automatic(Box<WgpuSettings>)`, `RenderDebugFlags` | [crates/bevy_render/src/lib.rs#L160-L167](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/lib.rs#L160-L167) | Exactly three public fields; `RenderDebugFlags` is a bitflags type deriving `Default` (= empty) | Baseline values preserved: async pipeline compilation on, debug flags empty |
| **wgpu-free settings** | `WgpuSettings` struct + `impl Default` | [crates/bevy_render/src/settings.rs#L39-L63](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/settings.rs#L39-L63), [#L70-L163](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/settings.rs#L70-L163) | Field `instance_memory_budget_thresholds: MemoryBudgetThresholds` exists, but `MemoryBudgetThresholds` is imported **privately** ([settings.rs#L12](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/settings.rs#L12)) and is **not** in the public `pub use wgpu::{…}` list ([settings.rs#L13-L16](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/settings.rs#L13-L16)) nor in the `render_resource` re-export list ([render_resource/mod.rs#L37-L70](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/render_resource/mod.rs#L37-L70)) | The type cannot be named through any public Bevy path in 0.20.0 → the field is covered by `..WgpuSettings::default()` struct-update syntax; `wgpu::` paths removed from project source; no direct `wgpu` dependency |
| Default semantics of the budget thresholds | `MemoryBudgetThresholds` (wgpu 30.0.1) | [gfx-rs/wgpu v30.0.1 wgpu-types/src/instance.rs#L335-L351](https://github.com/gfx-rs/wgpu/blob/v30.0.1/wgpu-types/src/instance.rs#L335-L351) | `#[derive(Default, Clone, Debug, Copy)]`, fields `for_resource_creation: Option<u8>`, `for_device_loss: Option<u8>` → `default() == { None, None }` | **Proven equivalent** to the baseline's explicit `{ None, None }` — struct-update defaulting is behavior-preserving here, not an assumption |
| Default limits/features | `WgpuSettings::default()` body | [settings.rs#L84-L163](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/settings.rs#L84-L163) | Defaults: `backends = Some(from_env ∥ all())`, `power_preference = HighPerformance`, `priority = Functionality`, `features = TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`, `limits = wgpu::Limits::default()` (non-WebGL), `memory_hints = MemoryHints::default()`, release builds drop `VALIDATION_INDIRECT_CALL` unless DX12 is in backends | The seven baseline-configured fields stay explicit (including `backends = VULKAN∥METAL∥DX12`); the baseline's hand-enumerated `WgpuLimits` (many zero-valued fields below any plausible adapter minimum, e.g. `max_color_attachment_bytes_per_sample: 32`) is replaced by the engine default under unchanged `Functionality` priority — see docs/architecture.md §"GPU limits" |
| `MemoryHints::Performance` | wgpu 30.0.1 `MemoryHints` | [wgpu-types/src/device.rs#L57-L60](https://github.com/gfx-rs/wgpu/blob/v30.0.1/wgpu-types/src/device.rs#L57-L60) | `#[default] Performance` | Baseline value kept explicitly (identical to default; kept for visibility) |
| Public GPU-type re-exports | `bevy::render::settings::{Backends, Dx12Compiler, WgpuFeatures, Gles3MinorVersion, InstanceFlags, WgpuLimits, MemoryHints, PowerPreference}`; `bevy::render::render_resource::{TextureDescriptor, Extent3d, TextureDimension, TextureFormat, TextureUsages, TextureDataOrder, …}` | [settings.rs#L13-L16](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/settings.rs#L13-L16), [render_resource/mod.rs#L37-L70](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/render_resource/mod.rs#L37-L70) | All texture-descriptor types used by the procedural images are re-exported; `MemoryBudgetThresholds` is not | Every GPU-facing type in project source resolves through Bevy re-exports only |
| Procedural images | `Image` fields `data: Option<Vec<u8>>`, `data_order`, `texture_descriptor`, `sampler`, `texture_view_descriptor`, `asset_usage`, `copy_on_resize`; `ImageSamplerDescriptor::linear()` | [crates/bevy_image/src/image.rs#L639-L666](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_image/src/image.rs#L639-L666), [#L903](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_image/src/image.rs#L903) | image.rs drift rc.2→0.20.0 is internal (`from_buffer_inner`, grayscale expansion) — constructed fields unchanged; `TextureDescriptor<Option<&'static str>, &'static [TextureFormat]>` | Eight RGBA8UnormSrgb textures generated once at startup; full-field literals kept |
| Sprites | `Sprite` fields (`image, texture_atlas, color, flip_x, flip_y, custom_size, rect, image_mode, alpha_mode`), `SpriteImageMode`, `SpriteAlphaMode`, `Anchor(pub Vec2)`, slicing types | [crates/bevy_sprite/src/sprite.rs#L19-L41](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_sprite/src/sprite.rs#L19-L41), [#L180-L202](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_sprite/src/sprite.rs#L180-L202), [#L303](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_sprite/src/sprite.rs#L303); [texture_slice/slicer.rs#L14-L28](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_sprite/src/texture_slice/slicer.rs#L14-L28), [border_rect.rs#L10](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_sprite/src/texture_slice/border_rect.rs#L10) | sprite.rs drift is one added method (`slice_border_out_of_bounds`) — fields unchanged; `TextureSlicer`/`BorderRect`/`SliceScaleMode` re-exported at crate root ([lib.rs#L54](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_sprite/src/lib.rs#L54)) | Baseline sprite construction preserved verbatim |
| 2D text | `Text2d(pub String)`, `TextFont` fields, `FontSize::Px(f32)`, `FontSource::Handle`, `TextColor` | [crates/bevy_sprite/src/text2d.rs#L102](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_sprite/src/text2d.rs#L102), [crates/bevy_text/src/text.rs#L783-L789](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_text/src/text.rs#L783-L789) | `FontSize` enum with `Px(f32)`; bevy_text has zero file drift rc.2→0.20.0 | Baseline text construction preserved (default font via `DEFAULT_UUID` handle, `default_font` feature) |
| Camera / clear color / MSAA | `Camera2d`, `ClearColor(pub Color)`, `Msaa::Off`, `Visibility` | [crates/bevy_camera/src/clear_color.rs#L56](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_camera/src/clear_color.rs#L56), [crates/bevy_render/src/view/mod.rs#L248-L249](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/view/mod.rs#L248-L249) | `pub enum Msaa { Off = 1, … }`; bevy_camera has zero drift | Single `Camera2d` + `Msaa::Off`; black clear color inserted as a resource in `Startup` |
| World access & change detection | `World::query/query_filtered/resource/resource_mut/insert_resource/spawn`, `QueryState::iter/iter_mut`, `Mut::into_inner`, `Without` | [crates/bevy_ecs/src/world/mod.rs](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_ecs/src/world/mod.rs), [query/iter.rs](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_ecs/src/query/iter.rs) | bevy_ecs drift rc.2→0.20.0: `#[inline(always)]` additions, `#[repr(align(8))]` on an internal cell type — no API change to the methods used | Exclusive-world systems (baseline architecture) with typed query items; no `&World`-wide system param downgrade needed |
| Entity naming | `Name::new`, `Name::as_str` | [crates/bevy_ecs/src/name.rs](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_ecs/src/name.rs) | Not in drift diff; `Name::new(impl Into<Box<str>>)` | `"menu"`/`"menu-ink"`/`"slot"` groups drive visibility, exactly as the baseline |
| Input & time | `ButtonInput<KeyCode>::just_pressed/pressed`, `KeyCode::{F1, Escape}`, `MouseButton::Left`, `Time::delta_secs` | [crates/bevy_input/src/button_input.rs](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_input/src/button_input.rs), [crates/bevy_time/src/time.rs](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_time/src/time.rs) | bevy_input/bevy_time have zero drift rc.2→0.20.0 | Edge-triggered F1/click, level-triggered Escape, delta-seconds accumulation |
| Pipelined rendering | `PipelinedRenderingPlugin` | [crates/bevy_render/src/pipelined_rendering.rs#L140](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/pipelined_rendering.rs#L140) | Unit struct, kept with `multi_threaded` | Kept as in the baseline (render extraction overlaps main-world updates) |
| Feature gates for every `bevy::*` module used | bevy_internal re-exports | [crates/bevy_internal/src/lib.rs#L17-L115](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_internal/src/lib.rs#L17-L115), [Cargo.toml#L237-L263](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_internal/Cargo.toml#L237-L263) | `app/ecs/math/input/time/transform/diagnostic/tasks` ungated; `a11y` rides on `bevy_window`; `color` rides on `bevy_image`; `mesh` rides on `bevy_camera`; all 16 manifest features exist verbatim in [bevy/Cargo.toml](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/Cargo.toml) | The rc.2 feature list carries over unchanged to v0.20.0; every module path used in `main.rs` resolves |
| **`Mut::into_inner` change semantics** | `Mut`/`ResMut` wrapper methods | [crates/bevy_ecs/src/change_detection/traits.rs#L516-L523](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_ecs/src/change_detection/traits.rs#L516-L523) | Doc-verified: "Consume `self` and return a mutable reference to the contained value **while marking `self` as 'changed'**" — `into_inner` calls `set_changed()` unconditionally; `Deref` reads never flag; `DerefMut` flags | Spec §20's warning is real in this revision: the baseline pattern (`into_inner()` then assign) flags every touched component on every gated frame. The rewrite reads through `Deref`, compares, and consumes the `Mut` only when the value actually differs |
| Visibility propagation gate | `visibility_propagate_system` | [crates/bevy_camera/src/visibility/mod.rs#L638-L648](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_camera/src/visibility/mod.rs#L638-L648) | Changed-source query is filtered `Or<(Changed<Visibility>, Changed<ChildOf>)>`; recursion updates `InheritedVisibility`/`ViewVisibility` downstream | Skipping no-op `Visibility` writes (guarded pattern above) prevents whole-subtree re-propagation on hold-only/repeat-input frames |
| Sprite mesh preparation gate | `bevy_sprite_render` mesh preparation | [crates/bevy_sprite_render/src/sprite_mesh/mod.rs#L180-L184](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_sprite_render/src/sprite_mesh/mod.rs#L180-L184) | Query filter `Or<(Changed<Sprite>, Changed<Anchor>, …, Changed<SpriteMaterialCount>)>` — sprite mesh work is change-detection driven | Guarded `Sprite.color` writes mean settled UI triggers zero mesh preparation; a launch transition still flags exactly once |
| Camera2d composition | `Camera2d` component | [crates/bevy_camera/src/components.rs#L9-L16](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_camera/src/components.rs#L9-L16) | `#[require(Camera, Projection::Orthographic(OrthographicProjection::default_2d()), Frustum = …)]` — spawning `Camera2d` guarantees a centered orthographic projection; not assumed, read | `(Camera2d, Msaa::Off)` spawn is sufficient; no manual projection setup |
| Projection size semantics | `ScalingMode::WindowSize`, runtime projection update | [crates/bevy_camera/src/projection.rs#L523-L534](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_camera/src/projection.rs#L523-L534), [#L692-L695](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_camera/src/projection.rs#L692-L695); [crates/bevy_render/src/camera.rs#L432](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_render/src/camera.rs#L432) | `WindowSize` maps 1 world unit ↔ 1 pixel at scale factor 1; the runtime camera system calls `projection.update(size.x, size.y)` with the **physical** target size (`RenderTargetInfo.physical_size`, scale_factor at camera.rs#L277-L307) | World units = physical pixels of the fullscreen target; layout from `Monitor::physical_*` lives in the same space **at scale factor 1.0** — see coordinate-space record in docs/architecture.md §9 |
| Cursor coordinate space | `Window::cursor_position` vs `physical_cursor_position` | [crates/bevy_window/src/window.rs#L618-L628](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/crates/bevy_window/src/window.rs#L618-L628) | `cursor_position()` = `physical_cursor_position() / scale_factor` → **logical** pixels, origin top-left; `physical_cursor_position()` also public | Baseline (preserved) hit-tests the logical cursor against a physical-pixel layout: exact at scale factor 1.0 (the 1920×1080 fullscreen target), skewed at other scale factors. Recorded as a baseline limitation + open decision, not silently "fixed" |
| MSRV / toolchain | workspace `rust-version` | [Cargo.toml](https://github.com/bevyengine/bevy/blob/b3bdd75becaffe9a16677cf5bae5bd6d198f052b/Cargo.toml) | `rust-version = "1.97.1"` | Project pins stable 1.99.1 (≥ MSRV) in `rust-toolchain.toml`; nightly-only flags removed |

## Actual coverage

Read directly in the pinned revision: `bevy_render/src/settings.rs` (whole
`WgpuSettings` + `Default` impl), `bevy_render/src/lib.rs` (`RenderPlugin`),
`bevy_render/src/render_resource/mod.rs` (re-export list),
`bevy_asset/src/handle.rs` + `id.rs` + `lib.rs` (asset types used),
`bevy_ecs/src/message/*` (Messages API), `bevy_app/src/app.rs` (AppExit),
`bevy_window/src/window.rs`/`monitor.rs` (all constructed fields),
`bevy_winit/src/lib.rs` (plugin field), `bevy_image/src/image.rs` (struct
fields + drift), `bevy_sprite/src/sprite.rs`/`text2d.rs`/`texture_slice/*`
(all constructed fields + drift), `bevy_text/src/text.rs` (FontSize/TextFont),
`bevy_camera`/`bevy_internal` re-export and feature wiring, plus wgpu 30.0.1
`wgpu-types` for the two default-semantics questions above. Added for the
Section 17–25 requirements: `bevy_ecs/src/change_detection/traits.rs`
(`into_inner`/`DerefMut` tick semantics), `bevy_camera/src/visibility/mod.rs`
(propagation gating), `bevy_sprite_render/src/sprite_mesh/mod.rs` (preparation
gating), `bevy_camera/src/components.rs` + `projection.rs` and
`bevy_render/src/camera.rs` (camera composition and coordinate spaces).

**Not** read line-by-line (relying on wiring identical to the rc.2 baseline):
RenderApp extraction/preparation/render-graph internals, task-pool
implementation details, the winit event-loop internals, text layout
(glyphbrush) internals, and the sprite/mesh2d pipeline shader code. No
project code reaches into those layers.

## Unresolved questions

1. Nothing in this sandbox can compile Rust (no toolchain, and crates.io is
   outside the network allowlist), so every "verified fact" above is
   source-level verification, not compiler confirmation. First compile must
   happen on a networked machine (`cargo check --locked` after
   `cargo generate-lockfile`).
2. Whether the baseline's hand-enumerated `WgpuLimits` ever produced a working
   device on real hardware (several zero-valued fields look below wgpu
   validation minimums). The rewrite uses engine defaults instead; see
   docs/architecture.md.
3. Scale-factor behavior of the preserved baseline hit test
   (`cursor_position()` logical vs physical-pixel layout): exact at scale
   factor 1.0, skewed otherwise. Changing it would alter baseline behavior,
   so per the spec's conflict rule this is reported as an open decision
   rather than resolved unilaterally.
4. Fate of the behavioral regression tests: revision 3 prohibits `#[test]`
   functions by default (exactly one `fn` item), while revision 1 mandated
   regression tests for the behavioral baseline. **Decided by the requester
   on 2026-10-16**: the prohibition stands (no test module is restored) and
   the gap is compensated with black-box checks — the CI toolchain job now
   ends with a headless process smoke check (Xvfb + software Vulkan; the
   release binary must survive 12 s of Startup + Update frames without
   panic or early exit), alongside the already-existing static policy job
   and the release pipeline's package-inspection steps (binary headers,
   archive layout, permission bits, extraction, checksums).

## Revision-3 approval request: the nine `&mut` signature spellings

The no-`mut` contract cannot be satisfied by any typed-system alternative in
Bevy v0.20.0 (all signatures read in the pinned tree):
`Commands::spawn(&mut self, …)`, `QueryState::iter_mut/single_mut/get_mut`
(`&mut self` receivers), and `ResMut`'s methods likewise — a typed system
would need a `mut` *binding*, which the contract forbids outright, whereas
`&mut` in a *signature/type position* is the relaxation the original
mandate's petition clause contemplated (and the baseline `main` itself uses
`|world: &mut World|`). The implementation therefore uses exactly nine
`&mut` occurrences, inventoried in docs/architecture.md §13.1: six
`&mut bevy::ecs::world::World` (two exclusive-system closure parameters +
four IIFE parameters that thread the world, the only `let`-free way to move
it across a closure boundary) and three mutable `QueryData` spellings
(`&mut Visibility`, `&mut Transform`, `Option<&mut Sprite>` — the only
type-level spelling of mutable query access). The CI policy job enforces the
exact inventory: any tenth occurrence, any other spelling, or any drift in
the six/three counts fails the build. Compiler diagnostics demonstrating the
absence of alternatives are unavailable here (no toolchain — the substituted
evidence is the pinned-source signature reading above). **Approval for this
narrow relaxation was GRANTED by the requester on 2026-10-16.**

## Revision-3 feasibility gate status: BLOCKED

The combined feasibility gate (compile and run a minimal
closure-conversion experiment before transforming the maintained source)
could not be executed: no Rust toolchain is installable in this sandbox
(`static.rust-lang.org` and crates.io are outside the network allowlist).
Per the spec, the experiment was kept entirely outside the maintained tree
(analysis scripts lived in `/tmp`; nothing experimental was committed, and
no second Rust entry point exists in the repository). The substituted
verification: (a) the transformation was assembled mechanically from the
already-verified second-revision text with byte-identical expression bodies;
(b) the final tree was re-scanned (structure, balance, counts, policy —
all green); (c) every Python check embedded in the workflows was executed
locally against the real tree and synthetic fixtures. First compilation
remains the downstream gate (commands in docs/rewrite-log.md).

## Acceptance map (spec §24)

One compact map for the whole rewrite. Status values: **PASS** = the named
check was executed in this environment and succeeded; **NOT RUN** = the check
exists but was never executed anywhere; **BLOCKED** = cannot be executed in
this environment (blocker named). A source review is never counted as a
runtime test; a parse-level simulation is never counted as a GitHub run.

| requirement | implementation location | verification method | executed status | evidence / blocker |
| --- | --- | --- | --- | --- |
| Bevy from official Git at exactly v0.20.0; no direct/renamed wgpu | `Cargo.toml`; CI policy job manifest scan (tomllib; package identity across all dependency sections incl. renamed deps) | refs API + `git ls-remote` tag→commit; manifest greps (executed); policy scan executed locally against the final tree | greps/refs/policy scan: PASS · remote CI run: NOT RUN | `v0.20.0` → `b3bdd75b…`; blocker: no Rust toolchain/crates.io in sandbox |
| Dominant linear chain in `main`; no wrappers/hidden config; exactly one `fn` item (rev 3) | `src/main.rs` `fn main` (line 1; 1516 lines, no other `fn`, no `impl`) | structural review; policy scanner (single-`fn` assertion + fixtures; bracket/literal balance) executed locally | PASS (static + local scan) | — |
| Strict syntax contract incl. §23 hardening; comment policy per file type (rev 3) | CI policy job: tokenizer scanner (+`transmute`/`from_raw_parts`/`as_mut_ptr`), syntax-aware TOML/YAML comment scanners, whole-tree scan, fixtures | the committed checker itself executed locally on the final tree: 0 violations, 0 comments in any maintained file; all fixtures (keyword ±, comment/URL, block-comment, attribute, lifetime, fn-count, TOML, YAML) pass before the scan | checker + fixtures: PASS (executed locally) · remote CI run: NOT RUN | macro-expansion boundary recorded in docs/architecture.md §13.2 |
| Plugin & lifecycle ownership; exit through supported lifecycle | chain (18 plugins); Update closure writes `AppExit::Success` | every plugin/field/enum verified against pinned source (table above) | PASS (static) · runtime: NOT RUN | — |
| Construction order vs frame execution documented separately | docs/architecture.md §3 + §5 | documentation authored from verified schedule facts (Startup→Update, pipelined extraction) | PASS (documentation) | — |
| Required runtime phase ordering; no inferred tuple ordering | docs/architecture.md §5.1 phase table | ordering claims cite verified engine behavior, not chain position | PASS (documentation) · runtime: NOT RUN | — |
| Input & state-transition invariants; same-frame toggle+click boundary defined | inlined expressions in the Update chain; truth table in docs/architecture.md §5.2 | all 7 boundary assertions re-simulated in Python against the inlined expression | simulation: PASS · runtime: NOT RUN | blocker: no toolchain; test fns prohibited (open question #4) |
| Invalidation protocol; settled animations stop updating; hidden UI never stale | guarded write pass (equality comparison before `Mut::into_inner`); inline gate | engine facts verified from source (`into_inner` sets CHANGED; `Changed<Visibility>`/`Changed<Sprite>` gates); guard algebra reviewed | engine facts: PASS · runtime: NOT RUN | — |
| Nine `&mut` signature spellings (rev 3 relaxation) | docs/architecture.md §13.1; CI policy job enforces the exact inventory (6 × `&mut World`, 1 each `&mut Visibility`/`&mut Transform`/`&mut Sprite`) | pinned-source signature reading (typed alternatives require forbidden `mut` bindings); inventory asserted locally | inventory: PASS · **approval: GRANTED (requester, 2026-10-16)** | compiler diagnostics unavailable (no toolchain) |
| Black-box compensation for prohibited test fns (requester decision, 2026-10-16) | CI toolchain job: headless process smoke check (Xvfb + lavapipe, 12 s survival, no panic, no early exit); release build jobs: package inspection (headers/layout/permissions/extraction/checksums) | package-inspection blocks executed locally on synthetic fixtures (PASS); smoke check step authored — cannot be executed here (no toolchain) | smoke check: NOT RUN (remote CI) · package inspection: PASS (local fixtures) | first GitHub run will exercise the smoke check; a software-device failure would fail loudly, not silently |
| Combined feasibility gate before transformation (rev 3) | experiment confined to `/tmp` (outside the maintained tree); no second Rust entry point committed | — | **BLOCKED** | no installable Rust toolchain; substituted verification recorded in the feasibility section above |
| No per-frame entity-list clone; no texture regeneration; no per-frame formatting/lookups (§22 properties) | Update/Startup structure | static inspection (no such code path exists); keyboard reads consolidated into one snapshot | PASS (static) · measurement: BLOCKED | no toolchain/GPU for dynamic confirmation |
| Asset lifetime & package path behavior | Startup handles stored in components; `release.yml` `dist/<artifact>/` staging | PNG magic-byte check executed; workflow YAML parsed; staging/extraction steps executed locally on synthetic archives | PASS (static/parse/local) · packaged run: NOT RUN | blocker: cannot build |
| Exact six matrix entries & executable filenames | `release.yml`; CI policy job (PyYAML) **and** the release build job's independent line-scanner self-check | both methods executed locally: each equals the mandated 6-entry contract in order; no doubled extension; all `${{ matrix.* }}` expressions present | PASS (two independent methods, executed) · GitHub run: NOT RUN | a locally linted workflow is not a successful GitHub run |
| Package integrity: binary headers, archive layout, permission bits, extraction from a different cwd, checksums (rev 3) | `release.yml` build-job verification steps | every embedded Python block executed locally against synthetic ELF/Mach-O/PE fixtures (correct arch accepted, mismatch rejected) and synthetic archives (unix + Windows shapes, missing/extra file rejected) | PASS (local execution on fixtures) · real-binary run: NOT RUN | requires a real build; runners execute the same blocks |
| Action pins resolve to exact SHAs; least privilege; version mapping outside comments | `release.yml`, `ci.yml`; mapping table in docs/rewrite-log.md | `git ls-remote` on all four action repos (lightweight tags → pinned SHAs); permissions blocks reviewed and policy-asserted (top-level `contents: read`; write only in the publication job) | PASS | — |
| Cargo version/label separation | `Cargo.toml` `0.0.0-s1`; label only in workflows | executed policy greps + semver validation of `0.0.0-s1`; label absent from manifest (policy-scanned on every CI run) | PASS (static + local scan) | — |
| Performance measurements | docs/performance.md | none executable here | BLOCKED | every metric honestly `not measured`; claims classified STRUCTURAL/HYPOTHESIS per docs/performance.md; reproducible procedures recorded |
| GPU/visual behavior | — | — | BLOCKED | no display/GPU in sandbox; scenario list in docs/performance.md |
| Compile / fmt / clippy / test / build | whole crate (`cargo test` will honestly report 0 tests) | — | BLOCKED → NOT RUN downstream | no toolchain; first downstream step: `cargo generate-lockfile`, `cargo fmt --all`, then the full gate sequence in docs/rewrite-log.md |
