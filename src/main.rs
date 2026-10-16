//! Subsystem for Arxumbra — a single-window 2D dashboard interface built on
//! Bevy v0.20.0 (Git tag, commit `b3bdd75becaffe9a16677cf5bae5bd6d198f052b`).
//!
//! Architecture contract for this file (enforced by `cargo test`, see the
//! `tests` module at the bottom):
//!
//! - one runtime entry point, `fn main`, containing one dominant, fully
//!   qualified `App` construction chain that ends in `.run()`;
//! - no `use`/`pub use`/`extern crate`, no `let`, no `mut` bindings, no
//!   author-defined `const`/`static` items, no `unsafe`;
//! - the only `&mut` occurrences are signature positions mandated by Bevy's
//!   public API contracts (exclusive systems take `&mut World`, mutable
//!   query items hand out `&mut T` through `Mut::into_inner`, and `scan`
//!   hands the test scanner its state); they are never local `mut` bindings;
//! - named pure functions below exist for exactly two sanctioned reasons:
//!   reusable numeric logic shared by the chain and targeted regression
//!   tests. Application assembly itself stays inside `main`.
//!
//! Behavior baseline (main @ 82bccf5a), preserved exactly:
//!
//! - borderless fullscreen window titled "Arxumbra" (1920×1080 fallback
//!   resolution, cursor hidden, layout taken from the primary monitor);
//! - F1 toggles the HUD (divider glow/core, header separator, tab pills,
//!   Home / Extension Registry entries, badge, well, OS title);
//! - clicking the Launch pill (only while the HUD is visible and F1 is not
//!   toggling in the same frame) switches from the Laboratory menu to the
//!   launched state, tinting the rocket slot opaque;
//! - holding Escape accumulates `scale.z` on every non-camera entity;
//!   releasing resets it to zero; reaching 7.0 writes `AppExit::Success`;
//! - the per-frame update path runs only when F1/Escape/click input arrives
//!   or an Escape hold is still accumulating;
//! - eight procedural RGBA textures are generated once at startup; icons and
//!   the app icon load from the external `assets/` folder via `AssetServer`.

/// RGBA pixel of the 48×512 `divider-glow` texture: white rgb, alpha rises
/// with `x²` across the 48 px width and fades near the top/bottom 12 px.
fn divider_glow_pixel(x: f32, y: f32) -> [u8; 4] {
    [
        255u8,
        255u8,
        255u8,
        ((0.48 * (x / 47.5).powf(2.0) * f32::min(f32::min(y, 511.0 - y) / 12.0, 1.0)).clamp(0.0, 1.0)
            * 255.0) as u8,
    ]
}

/// RGBA pixel of the 4×512 `divider-core` texture: opaque white line whose
/// alpha softens within 2 px of the corners.
fn divider_core_pixel(x: f32, y: f32) -> [u8; 4] {
    [
        255u8,
        255u8,
        255u8,
        ((0.5
            - (f32::hypot(
                f32::max(2.0 - x, 0.0),
                f32::max(f32::max(2.0 - y, y - 510.0), 0.0),
            ) - 2.0))
            .clamp(0.0, 1.0)
            * 255.0) as u8,
    ]
}

/// RGBA pixel of the 592×3 `header-separator` texture: a soft-edged white
/// rule (round end caps of radius 1.5 px).
fn header_separator_pixel(x: f32, y: f32) -> [u8; 4] {
    [
        255u8,
        255u8,
        255u8,
        ((0.5
            - (f32::hypot(
                f32::max(f32::max(1.5 - x, x - 590.5), 0.0),
                f32::abs(y - 1.5),
            ) - 1.5))
            .clamp(0.0, 1.0)
            * 255.0) as u8,
    ]
}

/// RGBA pixel of the 76×64 `pill` texture: a rounded capsule (tab/button
/// background) with a vertical luminance ramp and a darker band below
/// `y = 58`.
fn pill_pixel(x: f32, y: f32) -> [u8; 4] {
    (|d: f32, lum: f32| {
        [
            lum as u8,
            lum as u8,
            lum as u8,
            ((f32::max(
                f32::min(f32::max(0.5 - d, 0.0), 1.0),
                0.35 * f32::max(1.0 - (d - 0.5) / 4.0, 0.0).powf(1.4),
            )) * 255.0) as u8,
        ]
    })(
        f32::hypot(
            f32::max(f32::max(32.0 - x, x - 43.0), 0.0),
            f32::abs(y - 31.5),
        ) - 28.0,
        255.0 - 22.0 * f32::min(f32::max((y - 4.0) / 56.0, 0.0), 1.0).powi(2)
            - 18.0 * ((y >= 58.0) as u8 as f32),
    )
}

/// RGBA pixel of the 96×(`height` − 48) `menu-card` texture: rounded card
/// with a brightened rim, a highlight along the top edge, and a faint
/// golden-ratio jittered vertical gradient.
fn menu_card_pixel(x: f32, y: f32, height: f32) -> [u8; 4] {
    (|d: f32, lum: f32| {
        [
            (lum + 0.22 * (1.0 + (d + 0.5) / 3.0).clamp(0.0, 1.0).powf(1.4) * 255.0) as u8,
            (lum + 0.22 * (1.0 + (d + 0.5) / 3.0).clamp(0.0, 1.0).powf(1.4) * 255.0) as u8,
            (lum + 0.22 * (1.0 + (d + 0.5) / 3.0).clamp(0.0, 1.0).powf(1.4) * 255.0) as u8,
            (f32::max(
                f32::min(f32::max(0.5 - d, 0.0), 1.0),
                0.22 * (1.0 + (d + 0.5) / 3.0).clamp(0.0, 1.0).powf(1.4),
            ) * 255.0) as u8,
        ]
    })(
        f32::hypot(
            f32::max(f32::max(28.0 - x, x - 68.0), 0.0),
            f32::max(f32::max(28.0 - y, y - (height - 48.0 - 28.0)), 0.0),
        ) - 28.0,
        12.0 + 8.0 * f32::min(f32::max((6.0 - y) / 3.0, 0.0), 1.0)
            - 7.0
                * f32::min(
                    f32::max(
                        (y + ((x * 0.618).fract() * 2.0 - 1.0) * 0.95) / (height - 48.0),
                        0.0,
                    ),
                    1.0,
                ),
    )
}

/// RGBA pixel of the 96×288 `lab-tile` texture: the Laboratory card —
/// rounded rim, bright top edge, body darkening toward the bottom.
fn lab_tile_pixel(x: f32, y: f32) -> [u8; 4] {
    (|d: f32, lum: f32| {
        [
            (lum + 0.22 * (1.0 + (d + 0.5) / 2.5).clamp(0.0, 1.0).powf(1.4) * 255.0) as u8,
            (lum + 0.22 * (1.0 + (d + 0.5) / 2.5).clamp(0.0, 1.0).powf(1.4) * 255.0) as u8,
            (lum + 0.22 * (1.0 + (d + 0.5) / 2.5).clamp(0.0, 1.0).powf(1.4) * 255.0) as u8,
            (f32::max(
                f32::min(f32::max(0.5 - d, 0.0), 1.0),
                0.22 * (1.0 + (d + 0.5) / 2.5).clamp(0.0, 1.0).powf(1.4),
            ) * 255.0) as u8,
        ]
    })(
        f32::hypot(
            f32::max(f32::max(20.0 - x, x - 76.0), 0.0),
            f32::max(f32::max(20.0 - y, y - (288.0 - 20.0)), 0.0),
        ) - 20.0,
        24.0 + 10.0 * f32::min(f32::max((6.0 - y) / 3.0, 0.0), 1.0)
            - 4.0 * f32::min(f32::max(y / 288.0, 0.0), 1.0)
            - 5.0 * f32::min(f32::max((y - 284.0) / 4.0, 0.0), 1.0),
    )
}

/// RGBA pixel of the 72×72 `lab-well` texture: a circular well — opaque in
/// the middle, fading to a faint rim highlight at the rounded edge.
fn lab_well_pixel(x: f32, y: f32) -> [u8; 4] {
    (|d: f32| {
        (|rim: f32| {
            [
                (rim * 255.0) as u8,
                (rim * 255.0) as u8,
                (rim * 255.0) as u8,
                ((f32::min(f32::max(0.5 - d, 0.0), 1.0)) * 255.0) as u8,
            ]
        })((0.30 * (1.0 + d / 2.5).clamp(0.0, 1.0).powf(1.4)).clamp(0.0, 1.0))
    })(
        f32::hypot(
            f32::max(f32::max(12.0 - x, x - 60.0), 0.0),
            f32::max(f32::max(12.0 - y, y - 60.0), 0.0),
        ) - 12.0,
    )
}

/// RGBA pixel of the 56×56 `badge` texture: app-badge plate behind the
/// 26×26 app icon — rounded, slightly darkening toward the bottom.
fn badge_pixel(x: f32, y: f32) -> [u8; 4] {
    (|d: f32, lum: f32| {
        [
            lum as u8,
            lum as u8,
            lum as u8,
            ((f32::max(
                f32::min(f32::max(0.5 - d, 0.0), 1.0),
                0.30 * f32::max(1.0 - (d - 0.5) / 3.5, 0.0).powf(1.4),
            )) * 255.0) as u8,
        ]
    })(
        f32::hypot(
            f32::max(f32::max(18.0 - x, x - 38.0), 0.0),
            f32::max(f32::max(18.0 - y, y - 38.0), 0.0),
        ) - 14.0,
        255.0 - 26.0 * f32::min(f32::max((y - 4.0) / 48.0, 0.0), 1.0).powi(2),
    )
}

/// `true` when the entity name marks the launch-state rocket slot.
fn slot_name(name: Option<&bevy::ecs::name::Name>) -> bool {
    name.into_iter()
        .fold(false, |_, name: &bevy::ecs::name::Name| name.as_str() == "slot")
}

/// The update path runs only for frames that can change something: an F1
/// edge, a held Escape, a fresh click, or an Escape hold still accumulating
/// (`hold != 0.0`). Unchanged-input frames touch no component.
fn hud_gate(f1_pressed: bool, escape_held: bool, click_pressed: bool, hold: f32) -> bool {
    f1_pressed || escape_held || click_pressed || hold != 0.0
}

/// Escape-hold accumulator, stored in the `scale.z` of every non-camera
/// entity (uniform across entities by construction). While Escape is held it
/// grows by `delta_secs`; on release it resets to zero in the same frame.
fn next_hold(hold: f32, delta_secs: f32, escape_held: bool) -> f32 {
    (hold + delta_secs * escape_held as u8 as f32) * escape_held as u8 as f32
}

/// Exit boundary: holding Escape for 7 accumulated seconds terminates the
/// application with `AppExit::Success`.
fn exit_reached(hold: f32) -> bool {
    hold >= 7.0
}

/// Launch-pill hit test in window cursor coordinates (origin top-left, y
/// down). The region is the 584×64 pill at world `(368 − width/6, 164)`
/// mapped to cursor space and inset by ~5 px horizontally / ~4 px vertically,
/// exactly as in the behavior baseline.
fn launch_hit(cursor: bevy::math::Vec2, width: f32, height: f32) -> bool {
    cursor.x >= width / 3.0 + 81.0
        && cursor.x <= width / 3.0 + 655.0
        && cursor.y >= height / 2.0 - 192.0
        && cursor.y <= height / 2.0 - 139.0
}

/// Visibility for one entity of the uniform update pass:
/// - unnamed entities are the HUD (visible unless `hidden`);
/// - named non-slot entities are the Laboratory menu (visible only while not
///   launched);
/// - the slot rocket is visible only once launched.
fn hud_visibility(
    hidden: bool,
    named: bool,
    launched: bool,
    slot: bool,
) -> bevy::camera::visibility::Visibility {
    [
        bevy::camera::visibility::Visibility::Hidden,
        bevy::camera::visibility::Visibility::Visible,
    ][(!hidden && (!named || launched == slot)) as usize]
}

/// Tint for one sprite of the uniform update pass: only the slot rocket
/// changes color (fully transparent while idle, opaque white once launched);
/// every other sprite keeps its authored color.
fn slot_tint(color: bevy::color::Color, slot: bool, launched: bool) -> bevy::color::Color {
    [
        color,
        [
            bevy::color::Color::srgba(1.0, 1.0, 1.0, 0.0),
            bevy::color::Color::WHITE,
        ][launched as usize],
    ][slot as usize]
}

fn main() {
    bevy::app::App::new()
        .add_plugins((
            bevy::app::TaskPoolPlugin {
                task_pool_options: bevy::app::TaskPoolOptions {
                    min_total_threads: 1,
                    max_total_threads: usize::MAX,
                    io: bevy::app::TaskPoolThreadAssignmentPolicy {
                        min_threads: 1,
                        max_threads: 4,
                        percent: 0.25,
                        on_thread_spawn: None,
                        on_thread_destroy: None,
                    },
                    async_compute: bevy::app::TaskPoolThreadAssignmentPolicy {
                        min_threads: 1,
                        max_threads: 4,
                        percent: 0.25,
                        on_thread_spawn: None,
                        on_thread_destroy: None,
                    },
                    compute: bevy::app::TaskPoolThreadAssignmentPolicy {
                        min_threads: 1,
                        max_threads: usize::MAX,
                        percent: 1.0,
                        on_thread_spawn: None,
                        on_thread_destroy: None,
                    },
                },
            },
            bevy::diagnostic::FrameCountPlugin,
            bevy::time::TimePlugin,
            bevy::transform::TransformPlugin,
            bevy::input::InputPlugin,
            bevy::a11y::AccessibilityPlugin,
            bevy::window::WindowPlugin {
                primary_window: Some(bevy::window::Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::BorderlessFullscreen(
                        bevy::window::MonitorSelection::Primary,
                    ),
                    position: bevy::window::WindowPosition::Automatic,
                    resolution: bevy::window::WindowResolution::new(1920, 1080),
                    title: std::string::String::from("Arxumbra"),
                    name: None,
                    composite_alpha_mode: bevy::window::CompositeAlphaMode::Auto,
                    resize_constraints: bevy::window::WindowResizeConstraints {
                        min_width: 180.0,
                        min_height: 120.0,
                        max_width: f32::INFINITY,
                        max_height: f32::INFINITY,
                    },
                    resizable: false,
                    enabled_buttons: bevy::window::EnabledButtons {
                        minimize: true,
                        maximize: true,
                        close: true,
                    },
                    decorations: false,
                    transparent: false,
                    focused: true,
                    window_level: bevy::window::WindowLevel::Normal,
                    canvas: None,
                    fit_canvas_to_parent: false,
                    prevent_default_event_handling: true,
                    internal: bevy::window::InternalWindowState::default(),
                    ime_enabled: false,
                    ime_position: bevy::math::Vec2::ZERO,
                    window_theme: None,
                    visible: true,
                    skip_taskbar: true,
                    clip_children: true,
                    desired_maximum_frame_latency: None,
                    recognize_pinch_gesture: false,
                    recognize_rotation_gesture: false,
                    recognize_doubletap_gesture: false,
                    recognize_pan_gesture: None,
                    movable_by_window_background: false,
                    fullsize_content_view: false,
                    has_shadow: true,
                    titlebar_shown: true,
                    titlebar_transparent: false,
                    titlebar_show_title: true,
                    titlebar_show_buttons: true,
                    borderless_game: true,
                    prefers_home_indicator_hidden: false,
                    prefers_status_bar_hidden: false,
                    preferred_screen_edges_deferring_system_gestures:
                        bevy::window::ScreenEdge::None,
                }),
                primary_cursor_options: Some(bevy::window::CursorOptions {
                    visible: false,
                    grab_mode: bevy::window::CursorGrabMode::None,
                    hit_test: true,
                }),
                exit_condition: bevy::window::ExitCondition::OnAllClosed,
                close_when_requested: true,
            },
            bevy::asset::AssetPlugin {
                file_path: std::string::String::from("assets"),
                processed_file_path: std::string::String::from("imported_assets/Default"),
                watch_for_changes_override: None,
                use_asset_processor_override: None,
                mode: bevy::asset::AssetMode::Unprocessed,
                meta_check: bevy::asset::AssetMetaCheck::Never,
                unapproved_path_mode: bevy::asset::UnapprovedPathMode::Forbid,
            },
            bevy::winit::WinitPlugin {
                run_on_any_thread: false,
            },
        ))
        .add_plugins((
            // WgpuSettings: every field the baseline configured explicitly is
            // still configured explicitly; the remaining fields (device_label,
            // disabled_features, limits, constrained_limits,
            // gles3_minor_version, instance_memory_budget_thresholds,
            // force_fallback_adapter, adapter_name) come from
            // `WgpuSettings::default()` — verified against Bevy v0.20.0
            // `crates/bevy_render/src/settings.rs` (lines 70–163) and wgpu
            // 30.0.1 `wgpu-types/src/instance.rs` (lines 335–351):
            // `MemoryBudgetThresholds::default()` is `{ None, None }`,
            // identical to the baseline's explicit values. The baseline's
            // hand-enumerated `WgpuLimits` (many zero-valued fields) is
            // replaced by the engine default under the unchanged
            // `WgpuSettingsPriority::Functionality`; see docs/architecture.md.
            bevy::render::RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(
                    std::boxed::Box::new(bevy::render::settings::WgpuSettings {
                        backends: Some(
                            bevy::render::settings::Backends::VULKAN
                                | bevy::render::settings::Backends::METAL
                                | bevy::render::settings::Backends::DX12,
                        ),
                        power_preference: bevy::render::settings::PowerPreference::HighPerformance,
                        priority: bevy::render::settings::WgpuSettingsPriority::Functionality,
                        features:
                            bevy::render::settings::WgpuFeatures::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                        dx12_shader_compiler: bevy::render::settings::Dx12Compiler::Fxc,
                        instance_flags:
                            bevy::render::settings::InstanceFlags::VALIDATION_INDIRECT_CALL,
                        memory_hints: bevy::render::settings::MemoryHints::Performance,
                        ..bevy::render::settings::WgpuSettings::default()
                    }),
                ),
                synchronous_pipeline_compilation: false,
                debug_flags: bevy::render::RenderDebugFlags::empty(),
            },
            bevy::image::ImagePlugin {
                default_sampler: bevy::image::ImageSamplerDescriptor::linear(),
            },
            bevy::mesh::MeshPlugin,
            bevy::camera::CameraPlugin,
            bevy::render::pipelined_rendering::PipelinedRenderingPlugin,
            bevy::core_pipeline::CorePipelinePlugin,
            bevy::sprite::SpritePlugin,
            bevy::sprite_render::SpriteRenderPlugin,
            bevy::text::TextPlugin,
        ))
        // Startup: exclusive-world closure. One `Monitor` fold now yields the
        // full `(width, height)` pair (the baseline queried `Monitor` twice
        // per dimension — the same data read once, coherently). Inner
        // immediately-invoked closures bind the computed values because the
        // strict contract has no `let`; each binds exactly what the enclosed
        // spawns consume.
        .add_systems(
            bevy::app::Startup,
            |world: &mut bevy::ecs::world::World| {
                world.insert_resource(bevy::camera::ClearColor(bevy::color::Color::BLACK));
                world.spawn((bevy::camera::Camera2d, bevy::render::view::Msaa::Off));
                (|(width, height): (f32, f32), world: &mut bevy::ecs::world::World| {
                    (|glow: bevy::asset::Handle<bevy::image::Image>,
                      core: bevy::asset::Handle<bevy::image::Image>,
                      separator: bevy::asset::Handle<bevy::image::Image>,
                      pill: bevy::asset::Handle<bevy::image::Image>,
                      house: bevy::asset::Handle<bevy::image::Image>,
                      registry: bevy::asset::Handle<bevy::image::Image>,
                      app_icon: bevy::asset::Handle<bevy::image::Image>,
                      menu_card: bevy::asset::Handle<bevy::image::Image>,
                      lab: bevy::asset::Handle<bevy::image::Image>,
                      well: bevy::asset::Handle<bevy::image::Image>,
                      flask: bevy::asset::Handle<bevy::image::Image>,
                      rocket: bevy::asset::Handle<bevy::image::Image>,
                      rocket_pill: bevy::asset::Handle<bevy::image::Image>,
                      world: &mut bevy::ecs::world::World| {
                        // Left divider glow.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: glow,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(48.0, height)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 6.0 - 24.0,
                                    0.0,
                                    3.0,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Left divider core line.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: core,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(4.0, height)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(-width / 6.0 - 2.0, 0.0, 4.0),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Header separator rule above the tab pills.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: separator,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(width / 3.0 - 48.0, 3.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 3.0,
                                    height / 2.0 - 103.5,
                                    6.0,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Home tab pill.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: pill.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(width / 3.0 - 40.0, 64.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(32.0, 0.0),
                                            max_inset: bevy::math::Vec2::new(32.0, 0.0),
                                        },
                                        center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 3.0,
                                    height / 2.0 - 152.0,
                                    5.0,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Home tab icon.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: house,
                                texture_atlas: None,
                                color: bevy::color::Color::BLACK,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(26.0, 22.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 2.0 + 56.0,
                                    height / 2.0 - 152.0,
                                    5.5,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Home tab label.
                        world.spawn((
                            bevy::sprite::Text2d(std::string::String::from("Home")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                )),
                                font_size: bevy::text::FontSize::Px(24.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::BLACK),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 2.0 + 88.0,
                                    height / 2.0 - 152.0,
                                    6.0,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Extension Registry tab pill.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: pill.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(width / 3.0 - 40.0, 64.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(32.0, 0.0),
                                            max_inset: bevy::math::Vec2::new(32.0, 0.0),
                                        },
                                        center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 3.0,
                                    height / 2.0 - 227.0,
                                    5.0,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Extension Registry tab icon.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: registry,
                                texture_atlas: None,
                                color: bevy::color::Color::BLACK,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(26.0, 22.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 2.0 + 56.0,
                                    height / 2.0 - 227.0,
                                    5.5,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // App icon (top-left, over the badge plate).
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: app_icon,
                                texture_atlas: None,
                                color: bevy::color::Color::BLACK,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(26.0, 26.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 2.0 + 52.0,
                                    height / 2.0 - 52.0,
                                    2.5,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Extension Registry tab label.
                        world.spawn((
                            bevy::sprite::Text2d(std::string::String::from("Extension Registry")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                )),
                                font_size: bevy::text::FontSize::Px(24.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::BLACK),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 2.0 + 88.0,
                                    height / 2.0 - 227.0,
                                    6.0,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Launch-slot well (top-right; the rocket slot sits
                        // above it after launch).
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: well.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(72.0, 72.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    84.0 - width / 6.0,
                                    height / 2.0 - 84.0,
                                    2.9,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Laboratory menu card (named "menu": shown only
                        // while the HUD is visible and not launched).
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: menu_card,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(
                                    width - width / 3.0 - 48.0,
                                    height - 48.0,
                                )),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(40.0, 28.0),
                                            max_inset: bevy::math::Vec2::new(40.0, 28.0),
                                        },
                                        center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(width / 6.0, 0.0, 2.0),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Laboratory tile.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: lab,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(640.0, 288.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(40.0, 20.0),
                                            max_inset: bevy::math::Vec2::new(40.0, 20.0),
                                        },
                                        center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(368.0 - width / 6.0, 252.0, 2.5),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Laboratory well.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: well,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(72.0, 72.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    111.5 - width / 6.0,
                                    332.0,
                                    2.6,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Laboratory flask icon.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: flask,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(48.0, 40.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    111.5 - width / 6.0,
                                    332.5,
                                    2.7,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Vertical separator next to the Laboratory label.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::image::Image>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                ),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.36, 0.36, 0.36),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(2.0, 72.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Opaque,
                            },
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    172.5 - width / 6.0,
                                    332.5,
                                    2.7,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Laboratory label.
                        world.spawn((
                            bevy::sprite::Text2d(std::string::String::from("Laboratory")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                )),
                                font_size: bevy::text::FontSize::Px(30.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::WHITE),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    196.0 - width / 6.0,
                                    332.0,
                                    2.8,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Upper rule of the "Coming Soon" row.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::image::Image>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                ),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.36, 0.36, 0.36),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(584.0, 2.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Opaque,
                            },
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    368.0 - width / 6.0,
                                    271.0,
                                    2.7,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // "Coming Soon" row label.
                        world.spawn((
                            bevy::sprite::Text2d(std::string::String::from("Coming Soon")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                )),
                                font_size: bevy::text::FontSize::Px(20.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(0.58, 0.58, 0.58)),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    76.0 - width / 6.0,
                                    240.0,
                                    2.7,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Lower rule of the "Coming Soon" row.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::image::Image>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                ),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.36, 0.36, 0.36),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(584.0, 2.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Opaque,
                            },
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    368.0 - width / 6.0,
                                    207.0,
                                    2.7,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Launch button pill (its inset cursor-space hit
                        // region is `launch_hit` below).
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: pill,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(584.0, 64.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(32.0, 0.0),
                                            max_inset: bevy::math::Vec2::new(32.0, 0.0),
                                        },
                                        center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    368.0 - width / 6.0,
                                    164.0,
                                    2.7,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Launch button rocket icon (ink layer).
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: rocket_pill,
                                texture_atlas: None,
                                color: bevy::color::Color::BLACK,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(26.0, 26.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::ecs::name::Name::new("menu-ink"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    323.0 - width / 6.0,
                                    164.0,
                                    2.8,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Launch button label.
                        world.spawn((
                            bevy::sprite::Text2d(std::string::String::from("Launch")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                )),
                                font_size: bevy::text::FontSize::Px(24.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::BLACK),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(0.0, 0.0)),
                            bevy::ecs::name::Name::new("menu"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    386.0 - width / 6.0,
                                    164.0,
                                    2.8,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // Launch-state rocket (named "slot"): starts fully
                        // transparent; `slot_tint` makes it opaque white once
                        // launched, and its sprite alpha is the authoritative
                        // launched flag read back by the update pass.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: rocket,
                                texture_atlas: None,
                                color: bevy::color::Color::srgba(1.0, 1.0, 1.0, 0.0),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(44.0, 44.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::ecs::name::Name::new("slot"),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    84.0 - width / 6.0,
                                    height / 2.0 - 84.0,
                                    2.95,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                    })(
                        world
                            .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                            .add(bevy::image::Image {
                                data: Some(
                                    (0..24576u32)
                                        .flat_map(|index: u32| {
                                            divider_glow_pixel(
                                                (index % 48) as f32 + 0.5,
                                                (index / 48) as f32 + 0.5,
                                            )
                                        })
                                        .collect::<std::vec::Vec<u8>>(),
                                ),
                                data_order:
                                    bevy::render::render_resource::TextureDataOrder::LayerMajor,
                                texture_descriptor:
                                    bevy::render::render_resource::TextureDescriptor {
                                        label: Some("divider-glow"),
                                        size: bevy::render::render_resource::Extent3d {
                                            width: 48,
                                            height: 512,
                                            depth_or_array_layers: 1,
                                        },
                                        mip_level_count: 1,
                                        sample_count: 1,
                                        dimension:
                                            bevy::render::render_resource::TextureDimension::D2,
                                        format:
                                            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                                        usage: bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
                                            | bevy::render::render_resource::TextureUsages::COPY_DST,
                                        view_formats: &[],
                                    },
                                sampler: bevy::image::ImageSampler::Default,
                                texture_view_descriptor: None,
                                asset_usage: bevy::asset::RenderAssetUsages::MAIN_WORLD
                                    | bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                copy_on_resize: false,
                            }),
                        world
                            .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                            .add(bevy::image::Image {
                                data: Some(
                                    (0..2048u32)
                                        .flat_map(|index: u32| {
                                            divider_core_pixel(
                                                (index % 4) as f32 + 0.5,
                                                (index / 4) as f32 + 0.5,
                                            )
                                        })
                                        .collect::<std::vec::Vec<u8>>(),
                                ),
                                data_order:
                                    bevy::render::render_resource::TextureDataOrder::LayerMajor,
                                texture_descriptor:
                                    bevy::render::render_resource::TextureDescriptor {
                                        label: Some("divider-core"),
                                        size: bevy::render::render_resource::Extent3d {
                                            width: 4,
                                            height: 512,
                                            depth_or_array_layers: 1,
                                        },
                                        mip_level_count: 1,
                                        sample_count: 1,
                                        dimension:
                                            bevy::render::render_resource::TextureDimension::D2,
                                        format:
                                            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                                        usage: bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
                                            | bevy::render::render_resource::TextureUsages::COPY_DST,
                                        view_formats: &[],
                                    },
                                sampler: bevy::image::ImageSampler::Default,
                                texture_view_descriptor: None,
                                asset_usage: bevy::asset::RenderAssetUsages::MAIN_WORLD
                                    | bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                copy_on_resize: false,
                            }),
                        world
                            .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                            .add(bevy::image::Image {
                                data: Some(
                                    (0..1776u32)
                                        .flat_map(|index: u32| {
                                            header_separator_pixel(
                                                (index % 592) as f32 + 0.5,
                                                (index / 592) as f32 + 0.5,
                                            )
                                        })
                                        .collect::<std::vec::Vec<u8>>(),
                                ),
                                data_order:
                                    bevy::render::render_resource::TextureDataOrder::LayerMajor,
                                texture_descriptor:
                                    bevy::render::render_resource::TextureDescriptor {
                                        label: Some("header-separator"),
                                        size: bevy::render::render_resource::Extent3d {
                                            width: 592,
                                            height: 3,
                                            depth_or_array_layers: 1,
                                        },
                                        mip_level_count: 1,
                                        sample_count: 1,
                                        dimension:
                                            bevy::render::render_resource::TextureDimension::D2,
                                        format:
                                            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                                        usage: bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
                                            | bevy::render::render_resource::TextureUsages::COPY_DST,
                                        view_formats: &[],
                                    },
                                sampler: bevy::image::ImageSampler::Default,
                                texture_view_descriptor: None,
                                asset_usage: bevy::asset::RenderAssetUsages::MAIN_WORLD
                                    | bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                copy_on_resize: false,
                            }),
                        world
                            .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                            .add(bevy::image::Image {
                                data: Some(
                                    (0..4864u32)
                                        .flat_map(|index: u32| {
                                            pill_pixel(
                                                (index % 76) as f32 + 0.5,
                                                (index / 76) as f32 + 0.5,
                                            )
                                        })
                                        .collect::<std::vec::Vec<u8>>(),
                                ),
                                data_order:
                                    bevy::render::render_resource::TextureDataOrder::LayerMajor,
                                texture_descriptor:
                                    bevy::render::render_resource::TextureDescriptor {
                                        label: Some("pill"),
                                        size: bevy::render::render_resource::Extent3d {
                                            width: 76,
                                            height: 64,
                                            depth_or_array_layers: 1,
                                        },
                                        mip_level_count: 1,
                                        sample_count: 1,
                                        dimension:
                                            bevy::render::render_resource::TextureDimension::D2,
                                        format:
                                            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                                        usage: bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
                                            | bevy::render::render_resource::TextureUsages::COPY_DST,
                                        view_formats: &[],
                                    },
                                sampler: bevy::image::ImageSampler::Default,
                                texture_view_descriptor: None,
                                asset_usage: bevy::asset::RenderAssetUsages::MAIN_WORLD
                                    | bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                copy_on_resize: false,
                            }),
                        world
                            .resource::<bevy::asset::AssetServer>()
                            .load::<bevy::image::Image>("icons/house.png"),
                        world
                            .resource::<bevy::asset::AssetServer>()
                            .load::<bevy::image::Image>("icons/registry.png"),
                        world
                            .resource::<bevy::asset::AssetServer>()
                            .load::<bevy::image::Image>("app-icon.png"),
                        world
                            .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                            .add(bevy::image::Image {
                                data: Some(
                                    (0..(96 * ((height - 48.0) as u32)))
                                        .flat_map(|index: u32| {
                                            menu_card_pixel(
                                                (index % 96) as f32 + 0.5,
                                                (index / 96) as f32 + 0.5,
                                                height,
                                            )
                                        })
                                        .collect::<std::vec::Vec<u8>>(),
                                ),
                                data_order:
                                    bevy::render::render_resource::TextureDataOrder::LayerMajor,
                                texture_descriptor:
                                    bevy::render::render_resource::TextureDescriptor {
                                        label: Some("menu-card"),
                                        size: bevy::render::render_resource::Extent3d {
                                            width: 96,
                                            height: (height - 48.0) as u32,
                                            depth_or_array_layers: 1,
                                        },
                                        mip_level_count: 1,
                                        sample_count: 1,
                                        dimension:
                                            bevy::render::render_resource::TextureDimension::D2,
                                        format:
                                            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                                        usage: bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
                                            | bevy::render::render_resource::TextureUsages::COPY_DST,
                                        view_formats: &[],
                                    },
                                sampler: bevy::image::ImageSampler::Default,
                                texture_view_descriptor: None,
                                asset_usage: bevy::asset::RenderAssetUsages::MAIN_WORLD
                                    | bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                copy_on_resize: false,
                            }),
                        world
                            .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                            .add(bevy::image::Image {
                                data: Some(
                                    (0..27648u32)
                                        .flat_map(|index: u32| {
                                            lab_tile_pixel(
                                                (index % 96) as f32 + 0.5,
                                                (index / 96) as f32 + 0.5,
                                            )
                                        })
                                        .collect::<std::vec::Vec<u8>>(),
                                ),
                                data_order:
                                    bevy::render::render_resource::TextureDataOrder::LayerMajor,
                                texture_descriptor:
                                    bevy::render::render_resource::TextureDescriptor {
                                        label: Some("lab-tile"),
                                        size: bevy::render::render_resource::Extent3d {
                                            width: 96,
                                            height: 288,
                                            depth_or_array_layers: 1,
                                        },
                                        mip_level_count: 1,
                                        sample_count: 1,
                                        dimension:
                                            bevy::render::render_resource::TextureDimension::D2,
                                        format:
                                            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                                        usage: bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
                                            | bevy::render::render_resource::TextureUsages::COPY_DST,
                                        view_formats: &[],
                                    },
                                sampler: bevy::image::ImageSampler::Default,
                                texture_view_descriptor: None,
                                asset_usage: bevy::asset::RenderAssetUsages::MAIN_WORLD
                                    | bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                copy_on_resize: false,
                            }),
                        world
                            .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                            .add(bevy::image::Image {
                                data: Some(
                                    (0..5184u32)
                                        .flat_map(|index: u32| {
                                            lab_well_pixel(
                                                (index % 72) as f32 + 0.5,
                                                (index / 72) as f32 + 0.5,
                                            )
                                        })
                                        .collect::<std::vec::Vec<u8>>(),
                                ),
                                data_order:
                                    bevy::render::render_resource::TextureDataOrder::LayerMajor,
                                texture_descriptor:
                                    bevy::render::render_resource::TextureDescriptor {
                                        label: Some("lab-well"),
                                        size: bevy::render::render_resource::Extent3d {
                                            width: 72,
                                            height: 72,
                                            depth_or_array_layers: 1,
                                        },
                                        mip_level_count: 1,
                                        sample_count: 1,
                                        dimension:
                                            bevy::render::render_resource::TextureDimension::D2,
                                        format:
                                            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                                        usage: bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
                                            | bevy::render::render_resource::TextureUsages::COPY_DST,
                                        view_formats: &[],
                                    },
                                sampler: bevy::image::ImageSampler::Default,
                                texture_view_descriptor: None,
                                asset_usage: bevy::asset::RenderAssetUsages::MAIN_WORLD
                                    | bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                copy_on_resize: false,
                            }),
                        world
                            .resource::<bevy::asset::AssetServer>()
                            .load::<bevy::image::Image>("icons/flask.png"),
                        world
                            .resource::<bevy::asset::AssetServer>()
                            .load::<bevy::image::Image>("icons/rocket.png"),
                        world
                            .resource::<bevy::asset::AssetServer>()
                            .load::<bevy::image::Image>("icons/rocket-pill.png"),
                        world,
                    );
                    (|badge: bevy::asset::Handle<bevy::image::Image>,
                      world: &mut bevy::ecs::world::World| {
                        // Badge plate behind the app icon.
                        world.spawn((
                            bevy::sprite::Sprite {
                                image: badge,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(56.0, 56.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 2.0 + 52.0,
                                    height / 2.0 - 52.0,
                                    2.0,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        // OS-specific title next to the app icon.
                        world.spawn((
                            bevy::sprite::Text2d(std::string::String::from(
                                [
                                    "Linux-Subsystem-for-Arxumbra",
                                    "macOS-Subsystem-for-Arxumbra",
                                    "Windows-Subsystem-for-Arxumbra",
                                ][(cfg!(target_os = "windows") as usize) * 2
                                    + (cfg!(target_os = "macos") as usize)],
                            )),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                )),
                                font_size: bevy::text::FontSize::Px(24.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::WHITE),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(
                                    -width / 2.0 + 96.0,
                                    height / 2.0 - 52.0,
                                    5.0,
                                ),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 0.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                    })(
                        world
                            .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                            .add(bevy::image::Image {
                                data: Some(
                                    (0..3136u32)
                                        .flat_map(|index: u32| {
                                            badge_pixel(
                                                (index % 56) as f32 + 0.5,
                                                (index / 56) as f32 + 0.5,
                                            )
                                        })
                                        .collect::<std::vec::Vec<u8>>(),
                                ),
                                data_order:
                                    bevy::render::render_resource::TextureDataOrder::LayerMajor,
                                texture_descriptor:
                                    bevy::render::render_resource::TextureDescriptor {
                                        label: Some("badge"),
                                        size: bevy::render::render_resource::Extent3d {
                                            width: 56,
                                            height: 56,
                                            depth_or_array_layers: 1,
                                        },
                                        mip_level_count: 1,
                                        sample_count: 1,
                                        dimension:
                                            bevy::render::render_resource::TextureDimension::D2,
                                        format:
                                            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                                        usage: bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
                                            | bevy::render::render_resource::TextureUsages::COPY_DST,
                                        view_formats: &[],
                                    },
                                sampler: bevy::image::ImageSampler::Default,
                                texture_view_descriptor: None,
                                asset_usage: bevy::asset::RenderAssetUsages::MAIN_WORLD
                                    | bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                copy_on_resize: false,
                            }),
                        world,
                    );
                })(
                    // One coherent read of the monitor size (baseline read it
                    // twice). Falls back to 1920×1080 while no `Monitor`
                    // resource exists yet.
                    world
                        .query::<&bevy::window::Monitor>()
                        .iter(world)
                        .fold((1920.0, 1080.0), |_, monitor| {
                            (monitor.physical_width as f32, monitor.physical_height as f32)
                        }),
                    world,
                );
            },
        )
        // Update: exclusive-world closure. Structure (identical semantics to
        // the baseline, with the frame state read back from the ECS exactly
        // as the baseline does):
        //
        // 1. read F1 edge, Escape hold, delta, click edge, cursor, monitor
        //    size;
        // 2. one read-only fold derives `(hidden, launched, hold)`:
        //    - `hidden`  — visibility of the unnamed HUD entities,
        //    - `launched`— slot-rocket sprite alpha > 0.5,
        //    - `hold`    — Escape-hold accumulator (uniform `scale.z`);
        // 3. only when `hud_gate` passes, one write pass applies the F1
        //    toggle, the Launch click, the Escape accumulation, per-entity
        //    visibility (`hud_visibility`) and slot tint (`slot_tint`);
        // 4. crossing the 7 s hold boundary writes `AppExit::Success`.
        .add_systems(
            bevy::app::Update,
            |world: &mut bevy::ecs::world::World| {
                (|f1: bool,
                  escape: bool,
                  delta: f32,
                  pressed: bool,
                  cursor: bevy::math::Vec2,
                  (width, height): (f32, f32),
                  world: &mut bevy::ecs::world::World| {
                    (|(hidden, launched, hold): (bool, bool, f32),
                      world: &mut bevy::ecs::world::World| {
                        if hud_gate(f1, escape, pressed, hold) {
                            (|hidden_next: bool,
                              launched_next: bool,
                              hold_next: f32,
                              world: &mut bevy::ecs::world::World| {
                                if exit_reached(hold_next) {
                                    world
                                        .resource_mut::<
                                            bevy::ecs::message::Messages<bevy::app::AppExit>,
                                        >()
                                        .write(bevy::app::AppExit::Success);
                                }
                                world
                                    .query_filtered::<
                                        (
                                            &mut bevy::camera::visibility::Visibility,
                                            &mut bevy::transform::components::Transform,
                                            Option<&bevy::ecs::name::Name>,
                                            Option<&mut bevy::sprite::Sprite>,
                                        ),
                                        bevy::ecs::query::Without<bevy::camera::Camera2d>,
                                    >()
                                    .iter_mut(world)
                                    .for_each(
                                        |(
                                            visibility,
                                            transform,
                                            name,
                                            sprite,
                                        )| {
                                            (|slot: bool,
                                              named: bool,
                                              visibility: &mut bevy::camera::visibility::Visibility,
                                              transform: &mut bevy::transform::components::Transform,
                                              sprite: Option<
                                                  bevy::ecs::change_detection::Mut<
                                                      bevy::sprite::Sprite,
                                                  >,
                                              >| {
                                                transform.scale.z = next_hold(
                                                    transform.scale.z,
                                                    delta,
                                                    escape,
                                                );
                                                *visibility = hud_visibility(
                                                    hidden_next, named, launched_next, slot,
                                                );
                                                sprite.into_iter().for_each(
                                                    |sprite: bevy::ecs::change_detection::Mut<
                                                        bevy::sprite::Sprite,
                                                    >| {
                                                        (|sprite: &mut bevy::sprite::Sprite| {
                                                            sprite.color = slot_tint(
                                                                sprite.color,
                                                                slot,
                                                                launched_next,
                                                            );
                                                        })(sprite.into_inner());
                                                    },
                                                );
                                            })(
                                                slot_name(name),
                                                name.is_some(),
                                                visibility.into_inner(),
                                                transform.into_inner(),
                                                sprite,
                                            );
                                        },
                                    );
                            })(
                                hidden != f1,
                                launched
                                    || (pressed && hidden == f1 && launch_hit(cursor, width, height)),
                                next_hold(hold, delta, escape),
                                world,
                            );
                        }
                    })(
                        world
                            .query_filtered::<
                                (
                                    &bevy::camera::visibility::Visibility,
                                    &bevy::transform::components::Transform,
                                    Option<&bevy::sprite::Sprite>,
                                    Option<&bevy::ecs::name::Name>,
                                ),
                                bevy::ecs::query::Without<bevy::camera::Camera2d>,
                            >()
                            .iter(world)
                            .fold(
                                (false, false, 0.0),
                                |state: (bool, bool, f32), (visibility, transform, sprite, name)| {
                                    (|slot: bool| {
                                        (
                                            [
                                                state.0,
                                                *visibility
                                                    == bevy::camera::visibility::Visibility::Hidden,
                                            ][name.is_none() as usize],
                                            sprite.into_iter().fold(
                                                state.1,
                                                |value: bool, sprite: &bevy::sprite::Sprite| {
                                                    value
                                                        || (slot
                                                            && sprite.color.to_srgba().alpha > 0.5)
                                                },
                                            ),
                                            transform.scale.z,
                                        )
                                    })(slot_name(name))
                                },
                            ),
                        world,
                    );
                })(
                    world
                        .resource::<bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>()
                        .just_pressed(bevy::input::keyboard::KeyCode::F1),
                    world
                        .resource::<bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>()
                        .pressed(bevy::input::keyboard::KeyCode::Escape),
                    world.resource::<bevy::time::Time>().delta_secs(),
                    world
                        .resource::<bevy::input::ButtonInput<bevy::input::mouse::MouseButton>>()
                        .just_pressed(bevy::input::mouse::MouseButton::Left),
                    world
                        .query::<&bevy::window::Window>()
                        .iter(world)
                        .fold(bevy::math::Vec2::ZERO, |_, window| {
                            window
                                .cursor_position()
                                .into_iter()
                                .fold(bevy::math::Vec2::ZERO, |_, point| point)
                        }),
                    world
                        .query::<&bevy::window::Monitor>()
                        .iter(world)
                        .fold((1920.0, 1080.0), |_, monitor| {
                            (monitor.physical_width as f32, monitor.physical_height as f32)
                        }),
                    world,
                );
            },
        )
        .run();
}

#[cfg(test)]
mod tests {
    // Behavior regression tests. The expectations below encode the recorded
    // baseline (main @ 82bccf5a) — see the module docs at the top of this
    // file. They are headless: no `App` is built, so they run on CI without a
    // GPU or display server.

    #[test]
    fn launch_button_hit_region_is_the_inset_pill_rectangle() {
        // 1920×1080: x ∈ [721, 1295], y ∈ [348, 401] (inclusive, cursor
        // coordinates with origin at the top-left corner).
        assert!(super::launch_hit(
            bevy::math::Vec2::new(1920.0 / 3.0 + 81.0, 1080.0 / 2.0 - 192.0),
            1920.0,
            1080.0
        ));
        assert!(super::launch_hit(
            bevy::math::Vec2::new(1920.0 / 3.0 + 655.0, 1080.0 / 2.0 - 139.0),
            1920.0,
            1080.0
        ));
        assert!(super::launch_hit(
            bevy::math::Vec2::new(960.0, 375.0),
            1920.0,
            1080.0
        ));
        assert!(!super::launch_hit(
            bevy::math::Vec2::new(1920.0 / 3.0 + 80.5, 1080.0 / 2.0 - 192.0),
            1920.0,
            1080.0
        ));
        assert!(!super::launch_hit(
            bevy::math::Vec2::new(1920.0 / 3.0 + 655.5, 1080.0 / 2.0 - 139.0),
            1920.0,
            1080.0
        ));
        assert!(!super::launch_hit(
            bevy::math::Vec2::new(960.0, 1080.0 / 2.0 - 193.0),
            1920.0,
            1080.0
        ));
        assert!(!super::launch_hit(
            bevy::math::Vec2::new(960.0, 1080.0 / 2.0 - 138.0),
            1920.0,
            1080.0
        ));
    }

    #[test]
    fn launch_button_is_unreachable_at_the_minimum_window_size() {
        // At the 180×120 resize-constraint minimum the hit band maps to
        // y ∈ [−132, −79] in cursor space: clicking can never launch. This is
        // baseline behavior, recorded rather than "fixed".
        assert!(!super::launch_hit(
            bevy::math::Vec2::new(141.0, 0.0),
            180.0,
            120.0
        ));
        assert!(!super::launch_hit(
            bevy::math::Vec2::new(141.0, 120.0),
            180.0,
            120.0
        ));
    }

    #[test]
    fn escape_hold_accumulates_resets_and_crosses_the_exit_boundary() {
        assert_eq!(super::next_hold(0.0, 1.0 / 60.0, true), 1.0 / 60.0);
        assert_eq!(super::next_hold(3.0, 0.5, true), 3.5);
        // Release resets in the same frame.
        assert_eq!(super::next_hold(6.5, 0.5, false), 0.0);
        // Boundary: exactly 7.0 exits, just below does not.
        assert!(super::exit_reached(7.0));
        assert!(!super::exit_reached(6.95));
        assert!(!super::exit_reached(super::next_hold(6.9, 0.05, true)));
        assert!(super::exit_reached(super::next_hold(6.9, 0.2, true)));
        // A long pause produces one large delta; accumulation is linear in
        // delta and crosses the boundary in that single frame.
        assert!(super::exit_reached(super::next_hold(0.0, 120.0, true)));
    }

    #[test]
    fn update_gate_runs_only_for_relevant_input_or_active_hold() {
        assert!(!super::hud_gate(false, false, false, 0.0));
        assert!(super::hud_gate(true, false, false, 0.0));
        assert!(super::hud_gate(false, true, false, 0.0));
        assert!(super::hud_gate(false, false, true, 0.0));
        assert!(super::hud_gate(false, false, false, 0.5));
    }

    #[test]
    fn visibility_follows_hidden_named_launched_and_slot() {
        // HUD (unnamed): visible unless hidden by F1.
        assert_eq!(
            super::hud_visibility(false, false, false, false),
            bevy::camera::visibility::Visibility::Visible
        );
        assert_eq!(
            super::hud_visibility(true, false, false, false),
            bevy::camera::visibility::Visibility::Hidden
        );
        // Laboratory menu (named, not slot): visible only before launch.
        assert_eq!(
            super::hud_visibility(false, true, false, false),
            bevy::camera::visibility::Visibility::Visible
        );
        assert_eq!(
            super::hud_visibility(false, true, true, false),
            bevy::camera::visibility::Visibility::Hidden
        );
        // Rocket slot: visible only after launch.
        assert_eq!(
            super::hud_visibility(false, true, false, true),
            bevy::camera::visibility::Visibility::Hidden
        );
        assert_eq!(
            super::hud_visibility(false, true, true, true),
            bevy::camera::visibility::Visibility::Visible
        );
        // F1-hidden suppresses everything.
        assert_eq!(
            super::hud_visibility(true, true, true, true),
            bevy::camera::visibility::Visibility::Hidden
        );
    }

    #[test]
    fn slot_tint_only_touches_the_slot_sprite() {
        assert_eq!(
            super::slot_tint(bevy::color::Color::BLACK, false, true),
            bevy::color::Color::BLACK
        );
        assert_eq!(
            super::slot_tint(bevy::color::Color::BLACK, false, false),
            bevy::color::Color::BLACK
        );
        assert_eq!(
            super::slot_tint(bevy::color::Color::BLACK, true, true),
            bevy::color::Color::WHITE
        );
        assert_eq!(
            super::slot_tint(bevy::color::Color::BLACK, true, false),
            bevy::color::Color::srgba(1.0, 1.0, 1.0, 0.0)
        );
    }

    #[test]
    fn slot_names_are_recognized_exactly() {
        assert!(super::slot_name(Some(&bevy::ecs::name::Name::new("slot"))));
        assert!(!super::slot_name(Some(&bevy::ecs::name::Name::new("menu"))));
        assert!(!super::slot_name(Some(&bevy::ecs::name::Name::new(
            "menu-ink"
        ))));
        assert!(!super::slot_name(None));
    }

    #[test]
    fn procedural_textures_keep_their_pixel_invariants() {
        // Expected values below were computed under IEEE-754 single-precision
        // emulation of the exact baseline expressions (margins to the next
        // integer are ≥ 0.05, far above f32 rounding differences).
        assert_eq!(super::divider_glow_pixel(47.5, 256.5), [255, 255, 255, 122]);
        assert_eq!(super::divider_glow_pixel(0.5, 0.5)[3], 0);
        assert_eq!(super::divider_core_pixel(2.5, 256.5), [255, 255, 255, 255]);
        assert_eq!(super::divider_core_pixel(0.5, 0.5), [255, 255, 255, 96]);
        assert_eq!(
            super::header_separator_pixel(0.5, 0.5),
            [255, 255, 255, 255]
        );
        assert_eq!(super::pill_pixel(37.5, 31.5)[0], 249);
        assert_eq!(super::pill_pixel(37.5, 60.5)[0], 215);
        assert_eq!(
            super::menu_card_pixel(48.5, 300.5, 1032.0),
            [9, 9, 9, 255]
        );
        assert_eq!(super::lab_tile_pixel(0.5, 0.5), [90, 90, 90, 56]);
        assert_eq!(super::lab_well_pixel(36.5, 36.5), [0, 0, 0, 255]);
        assert_eq!(super::lab_well_pixel(0.5, 0.5), [76, 76, 76, 0]);
        assert_eq!(super::badge_pixel(0.5, 0.5), [255, 255, 255, 0]);
    }

    #[test]
    fn procedural_texture_buffers_match_their_descriptor_dimensions() {
        // Buffer length identity actually used by the startup chain:
        // pixel count × 4 bytes == width × height × 4.
        assert_eq!(
            (0..24576u32)
                .flat_map(|index: u32| {
                    super::divider_glow_pixel(
                        (index % 48) as f32 + 0.5,
                        (index / 48) as f32 + 0.5,
                    )
                })
                .count(),
            48 * 512 * 4
        );
        assert_eq!(
            (0..2048u32)
                .flat_map(|index: u32| {
                    super::divider_core_pixel((index % 4) as f32 + 0.5, (index / 4) as f32 + 0.5)
                })
                .count(),
            4 * 512 * 4
        );
        assert_eq!(
            (0..1776u32)
                .flat_map(|index: u32| {
                    super::header_separator_pixel(
                        (index % 592) as f32 + 0.5,
                        (index / 592) as f32 + 0.5,
                    )
                })
                .count(),
            592 * 3 * 4
        );
        assert_eq!(
            (0..4864u32)
                .flat_map(|index: u32| {
                    super::pill_pixel((index % 76) as f32 + 0.5, (index / 76) as f32 + 0.5)
                })
                .count(),
            76 * 64 * 4
        );
        assert_eq!(
            (0..(96 * ((1032.0 - 48.0) as u32)))
                .flat_map(|index: u32| {
                    super::menu_card_pixel(
                        (index % 96) as f32 + 0.5,
                        (index / 96) as f32 + 0.5,
                        1032.0,
                    )
                })
                .count(),
            96 * (1032 - 48) * 4
        );
        assert_eq!(
            (0..27648u32)
                .flat_map(|index: u32| {
                    super::lab_tile_pixel((index % 96) as f32 + 0.5, (index / 96) as f32 + 0.5)
                })
                .count(),
            96 * 288 * 4
        );
        assert_eq!(
            (0..5184u32)
                .flat_map(|index: u32| {
                    super::lab_well_pixel((index % 72) as f32 + 0.5, (index / 72) as f32 + 0.5)
                })
                .count(),
            72 * 72 * 4
        );
        assert_eq!(
            (0..3136u32)
                .flat_map(|index: u32| {
                    super::badge_pixel((index % 56) as f32 + 0.5, (index / 56) as f32 + 0.5)
                })
                .count(),
            56 * 56 * 4
        );
    }

    #[test]
    fn external_assets_exist_and_are_png_files() {
        [
            "assets/app-icon.png",
            "assets/icons/flask.png",
            "assets/icons/house.png",
            "assets/icons/registry.png",
            "assets/icons/rocket-pill.png",
            "assets/icons/rocket.png",
        ]
        .iter()
        .for_each(|path: &&str| {
            assert!(std::path::Path::new(path).is_file(), "missing asset {path}");
            std::fs::read(path).into_iter().for_each(|bytes: std::vec::Vec<u8>| {
                assert!(
                    bytes.starts_with(&[137u8, 80, 78, 71, 13, 10, 26, 10]),
                    "asset {path} is not a PNG file"
                );
            });
        });
    }

    // Strict source-syntax contract check (Section 4 of the rewrite mandate).
    // This is a syntax-aware scanner, not a plain grep: it first removes
    // comments, string literals, and character literals with a character
    // state machine, then searches for prohibited keyword *tokens* at token
    // boundaries (identifiers such as `resource_mut`, `asset_usage`, or
    // `use_asset_processor_override` cannot trigger it). Documented
    // exceptions, all mandated by public API signatures:
    //
    // - `&mut` (mutable *reference* syntax) where Bevy's exclusive-system
    //   and mutable-query contracts, or `Iterator::scan`/`Option::last_mut`,
    //   require it; never a `mut` binding or `mut` parameter;
    // - `'static` (a lifetime, not a `static` item).
    //
    // Scanned files: every `*.rs` under `src/` (currently exactly
    // `src/main.rs`) plus `build.rs` when present. There are no other
    // handwritten Rust files, no examples, no integration-test directory,
    // and no generated or included source files. Implemented with `std`
    // only — no runtime or dev dependency is added for this check.

    /// Removes comments, string literals, and character literals. Modes:
    /// 0 code, 1 line comment, 2 block comment, 3 string, 4 char-literal
    /// escape body, 5 just after `'`, 6 lifetime or char-literal body.
    /// Rust's nested block comments are treated as non-nested (this file
    /// contains none); raw strings are not handled (this file contains
    /// none) — both limitations are covered by the assertions in
    /// `strict_syntax_contract_holds_for_every_handwritten_rust_file`.
    fn strip_comments_and_literals(source: &str) -> std::string::String {
        source
            .chars()
            .scan((0u8, ' '), |state: &mut (u8, char), c: char| {
                Some(match state.0 {
                    1 => {
                        if c == '\n' {
                            state.0 = 0;
                            std::string::String::from("\n")
                        } else {
                            std::string::String::from(" ")
                        }
                    }
                    2 => {
                        if state.1 == '*' && c == '/' {
                            state.0 = 0;
                        }
                        state.1 = c;
                        std::string::String::from(" ")
                    }
                    3 => {
                        if state.1 == '\\' && c == '\\' {
                            state.1 = ' ';
                        } else if c == '"' && state.1 != '\\' {
                            state.0 = 0;
                            state.1 = c;
                        } else {
                            state.1 = c;
                        }
                        std::string::String::from(" ")
                    }
                    4 => {
                        if state.1 == '\\' && c == '\\' {
                            state.1 = ' ';
                        } else if c == '\'' && state.1 != '\\' {
                            state.0 = 0;
                            state.1 = c;
                        } else {
                            state.1 = c;
                        }
                        std::string::String::from(" ")
                    }
                    5 => {
                        if c == '\\' {
                            state.0 = 4;
                            state.1 = c;
                            std::string::String::from(" ")
                        } else if c.is_alphanumeric() || c == '_' {
                            state.0 = 6;
                            state.1 = c;
                            ['\'', c].into_iter().collect::<std::string::String>()
                        } else {
                            state.0 = 0;
                            state.1 = c;
                            ['\'', c].into_iter().collect::<std::string::String>()
                        }
                    }
                    6 => {
                        if c == '\'' {
                            state.0 = 0;
                            state.1 = c;
                            std::string::String::from(" ")
                        } else if c.is_alphanumeric() || c == '_' {
                            state.1 = c;
                            c.to_string()
                        } else {
                            state.0 = 0;
                            state.1 = c;
                            c.to_string()
                        }
                    }
                    _ => match (state.1, c) {
                        ('/', '/') => {
                            state.0 = 1;
                            state.1 = c;
                            std::string::String::from(" ")
                        }
                        ('/', '*') => {
                            state.0 = 2;
                            state.1 = c;
                            std::string::String::from(" ")
                        }
                        (_, '"') => {
                            state.0 = 3;
                            state.1 = c;
                            std::string::String::from(" ")
                        }
                        (_, '\'') => {
                            state.0 = 5;
                            state.1 = c;
                            std::string::String::from(" ")
                        }
                        (_, '/') => {
                            state.1 = c;
                            c.to_string()
                        }
                        _ => {
                            state.1 = c;
                            c.to_string()
                        }
                    },
                })
            })
            .collect::<std::string::String>()
    }

    /// `true` when the match at `start` spans a whole token (no adjacent
    /// identifier characters on either side).
    fn token_boundary(cleaned: &str, start: usize, len: usize) -> bool {
        cleaned[..start]
            .chars()
            .next_back()
            .into_iter()
            .all(|previous: char| !previous.is_alphanumeric() && previous != '_')
            && cleaned[start + len..]
                .chars()
                .next()
                .into_iter()
                .all(|next: char| !next.is_alphanumeric() && next != '_')
    }

    /// Documented exceptions: 1 = `&mut` reference syntax (previous
    /// non-whitespace character is `&`), 2 = `'static` lifetime (previous
    /// non-whitespace character is `'`).
    fn exception_applies(cleaned: &str, start: usize, exception: u8) -> bool {
        cleaned[..start]
            .chars()
            .rev()
            .find(|previous: &char| !previous.is_whitespace())
            .into_iter()
            .any(|previous: char| {
                (exception == 1 && previous == '&') || (exception == 2 && previous == '\'')
            })
    }

    fn contract_violations(cleaned: &str) -> std::vec::Vec<std::string::String> {
        [
            ("use", 0u8),
            ("let", 0),
            ("mut", 1),
            ("const", 0),
            ("static", 2),
            ("unsafe", 0),
            ("extern", 0),
            ("dyn", 0),
            ("macro_rules", 0),
            ("RefCell", 0),
            ("Cell", 0),
            ("Mutex", 0),
            ("RwLock", 0),
            ("AtomicBool", 0),
            ("AtomicUsize", 0),
            ("include_bytes", 0),
            ("include_str", 0),
            ("no_implicit_prelude", 0),
        ]
        .iter()
        .flat_map(|(keyword, exception): &(&str, u8)| {
            cleaned.match_indices(keyword).filter_map(
                move |(start, found): (usize, &str)| {
                    (token_boundary(cleaned, start, found.len())
                        && !exception_applies(cleaned, start, *exception))
                        .then(|| format!("{found} at byte offset {start}"))
                },
            )
        })
        .collect::<std::vec::Vec<std::string::String>>()
    }

    fn handwritten_rust_files() -> std::vec::Vec<std::path::PathBuf> {
        std::fs::read_dir("src")
            .into_iter()
            .flat_map(|entries: std::fs::ReadDir| {
                entries.flat_map(|entry: std::result::Result<std::fs::DirEntry, std::io::Error>| {
                    entry.into_iter().map(|entry: std::fs::DirEntry| entry.path())
                })
            })
            .filter(|path: &std::path::PathBuf| {
                path.extension()
                    .into_iter()
                    .fold(false, |_, extension| extension == "rs")
            })
            .chain(
                std::iter::once(std::path::PathBuf::from("build.rs"))
                    .filter(|path: &std::path::PathBuf| path.is_file()),
            )
            .collect::<std::vec::Vec<std::path::PathBuf>>()
    }

    #[test]
    fn strict_syntax_contract_holds_for_every_handwritten_rust_file() {
        assert!(
            !handwritten_rust_files().is_empty(),
            "the scanner found no source files; its working directory assumption is broken"
        );
        handwritten_rust_files()
            .iter()
            .for_each(|path: &std::path::PathBuf| {
                std::fs::read_to_string(path).into_iter().for_each(
                    |source: std::string::String| {
                        // Guard the scanner's own limitations: nested block
                        // comments and raw strings are not modeled. The probe
                        // sequences are assembled from chars so that this
                        // guard does not trip on its own source text.
                        assert!(
                            !source.contains(
                                &['/', '*'].into_iter().collect::<std::string::String>()
                            ),
                            "{} uses block comments the scanner models as non-nested",
                            path.display()
                        );
                        assert!(
                            !source.contains(
                                &['r', '#'].into_iter().collect::<std::string::String>()
                            ),
                            "{} uses raw strings the scanner does not model",
                            path.display()
                        );
                        contract_violations(&strip_comments_and_literals(&source))
                            .iter()
                            .for_each(|violation: &std::string::String| {
                                panic!(
                                    "prohibited construct `{violation}` in {}",
                                    path.display()
                                );
                            });
                    },
                );
            });
    }

    // Dependency policy (Section 7): Bevy from the official Git repository at
    // tag v0.20.0, and no direct wgpu dependency of any kind.

    #[test]
    fn manifest_pins_git_bevy_v0_20_0_without_direct_wgpu() {
        std::fs::read_to_string("Cargo.toml")
            .into_iter()
            .for_each(|manifest: std::string::String| {
                assert!(
                    manifest.contains("git = \"https://github.com/bevyengine/bevy\""),
                    "bevy must come from the official Git repository"
                );
                assert!(
                    manifest.contains("tag = \"v0.20.0\""),
                    "bevy must be pinned to tag v0.20.0"
                );
                assert!(
                    !manifest.contains("cargo-features"),
                    "nightly cargo-features gate must stay removed"
                );
                assert!(
                    !manifest.contains("immediate-abort"),
                    "nightly panic strategy must stay removed"
                );
                manifest
                    .lines()
                    .filter(|line: &str| line.trim_start().starts_with("wgpu"))
                    .for_each(|line: &str| {
                        panic!("direct wgpu dependency declaration found: {line}");
                    });
            });
        assert!(
            !std::path::Path::new(".cargo/config.toml").exists(),
            "nightly rustflags and target-cpu=native must stay removed"
        );
    }

    /// `true` for a valid Cargo semantic version: three numeric segments and
    /// an optional alphanumeric pre-release. The fixed distribution label
    /// `v0.0.0.0.0.0-s1` is deliberately *not* one (six segments) and must
    /// never be forced into `Cargo.toml`.
    fn valid_cargo_semver(version: &str) -> bool {
        std::iter::once(version)
            .map(|version: &str| {
                version
                    .split_once('-')
                    .map(|(core_part, pre_release): (&str, &str)| (core_part, Some(pre_release)))
                    .unwrap_or((version, None))
            })
            .all(|(core_part, pre_release): (&str, Option<&str>)| {
                core_part.split('.').count() == 3
                    && core_part.split('.').all(|segment: &str| {
                        !segment.is_empty()
                            && segment
                                .bytes()
                                .all(|digit: u8| digit.is_ascii_digit())
                    })
                    && pre_release.into_iter().all(|pre_release: &str| {
                        !pre_release.is_empty()
                            && pre_release.bytes().all(|symbol: u8| {
                                symbol.is_ascii_alphanumeric()
                                    || symbol == b'.'
                                    || symbol == b'-'
                            })
                    })
            })
    }

    #[test]
    fn valid_cargo_semver_accepts_the_package_version_and_rejects_the_label() {
        assert!(valid_cargo_semver("0.0.0-s1"));
        assert!(valid_cargo_semver("0.0.0"));
        assert!(valid_cargo_semver("1.2.3-rc.1"));
        assert!(!valid_cargo_semver("0.0.0.0.0.0-s1"));
        assert!(!valid_cargo_semver("v0.0.0-s1"));
        assert!(!valid_cargo_semver("0.0-s1"));
        assert!(!valid_cargo_semver("0.0.0-"));
    }

    #[test]
    fn distribution_label_stays_separate_from_the_cargo_version() {
        std::fs::read_to_string("Cargo.toml")
            .into_iter()
            .for_each(|manifest: std::string::String| {
                manifest
                    .lines()
                    .filter(|line: &str| line.trim_start().starts_with("version"))
                    .map(|line: &str| line.split_once('=').map(|(_, value): (&str, &str)| value))
                    .flatten()
                    .map(|value: &str| value.trim().trim_matches('"'))
                    .take(1)
                    .for_each(|version: &str| {
                        assert!(
                            valid_cargo_semver(version),
                            "Cargo package version {version} is not a valid semantic version"
                        );
                        assert_ne!(
                            version, "0.0.0.0.0.0-s1",
                            "the fixed distribution label is not a Cargo version"
                        );
                    });
                assert!(
                    !manifest.contains("0.0.0.0.0.0"),
                    "the six-segment distribution label must not leak into Cargo.toml"
                );
            });
    }

    // Release-matrix regression checks (Section 11.5). The matrix is parsed
    // structurally out of the workflow's `matrix:` block (key/value lines
    // between `matrix:` and the next `steps:`), not substring-searched as a
    // whole, and the parsed entries must equal the six mandated external
    // naming contracts exactly — case, spelling, runners, targets, and
    // extensions included.

    /// One `include:` row of the release matrix.
    #[derive(Debug, PartialEq)]
    struct MatrixEntry {
        artifact: std::string::String,
        runner: std::string::String,
        target: std::string::String,
        extension: std::string::String,
    }

    fn matrix_entry(
        artifact: &str,
        runner: &str,
        target: &str,
        extension: &str,
    ) -> MatrixEntry {
        MatrixEntry {
            artifact: std::string::String::from(artifact),
            runner: std::string::String::from(runner),
            target: std::string::String::from(target),
            extension: std::string::String::from(extension),
        }
    }

    fn release_matrix(yaml: &str) -> std::vec::Vec<MatrixEntry> {
        yaml.lines()
            .scan(
                (std::vec::Vec::<MatrixEntry>::new(), false),
                |state: &mut (std::vec::Vec<MatrixEntry>, bool), line: &str| {
                    (|trimmed: &str| {
                        state.1 = (state.1 && trimmed != "steps:") || trimmed == "matrix:";
                        trimmed
                            .strip_prefix("- artifact:")
                            .into_iter()
                            .for_each(|value: &str| {
                                state.0.push(matrix_entry(value.trim(), "", "", ""));
                            });
                        state
                            .0
                            .last_mut()
                            .into_iter()
                            .for_each(|entry: &mut MatrixEntry| {
                                [
                                    ("runner:", 1u8),
                                    ("target:", 2),
                                    ("extension:", 3),
                                ]
                                .iter()
                                .for_each(|(key, field): &(&str, u8)| {
                                    (state.1 && trimmed.starts_with(*key))
                                        .then(|| {
                                            trimmed.split_once(':').map(
                                                |(_, value): (&str, &str)| {
                                                    value.trim().trim_matches('"')
                                                },
                                            )
                                        })
                                        .flatten()
                                        .into_iter()
                                        .for_each(|value: &str| match field {
                                            1 => {
                                                entry.runner =
                                                    std::string::String::from(value)
                                            }
                                            2 => {
                                                entry.target =
                                                    std::string::String::from(value)
                                            }
                                            _ => {
                                                entry.extension =
                                                    std::string::String::from(value)
                                            }
                                        });
                                });
                            });
                    })(line.trim());
                    Some(state.0.clone())
                },
            )
            .flatten()
            .fold(std::vec::Vec::new(), |_, entries: std::vec::Vec<MatrixEntry>| {
                entries
            })
    }

    #[test]
    fn release_workflow_matrix_matches_the_six_mandated_entries_exactly() {
        std::fs::read_to_string(".github/workflows/release.yml")
            .into_iter()
            .for_each(|yaml: std::string::String| {
                assert_eq!(
                    release_matrix(&yaml),
                    std::vec![
                        matrix_entry(
                            "AArch64-Linux-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
                            "ubuntu-24.04-arm",
                            "aarch64-unknown-linux-gnu",
                            "",
                        ),
                        matrix_entry(
                            "x86_64-Linux-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
                            "ubuntu-24.04",
                            "x86_64-unknown-linux-gnu",
                            "",
                        ),
                        matrix_entry(
                            "AArch64-macOS-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
                            "macos-15",
                            "aarch64-apple-darwin",
                            "",
                        ),
                        matrix_entry(
                            "x86_64-macOS-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
                            "macos-15-intel",
                            "x86_64-apple-darwin",
                            "",
                        ),
                        matrix_entry(
                            "AArch64-Windows-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
                            "windows-11-arm",
                            "aarch64-pc-windows-msvc",
                            ".exe",
                        ),
                        matrix_entry(
                            "x86_64-Windows-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
                            "windows-2025",
                            "x86_64-pc-windows-msvc",
                            ".exe",
                        ),
                    ]
                );
            });
    }

    #[test]
    fn staged_executable_names_are_the_six_required_filenames() {
        std::fs::read_to_string(".github/workflows/release.yml")
            .into_iter()
            .for_each(|yaml: std::string::String| {
                release_matrix(&yaml)
                    .iter()
                    .map(|entry: &MatrixEntry| {
                        assert!(
                            !entry.artifact.ends_with(".exe"),
                            "artifact label {} must not carry the extension itself",
                            entry.artifact
                        );
                        assert!(
                            entry.extension == "" || entry.extension == ".exe",
                            "unexpected extension {} for {}",
                            entry.extension,
                            entry.artifact
                        );
                        format!("{}{}", entry.artifact, entry.extension)
                    })
                    .zip(
                        [
                            "AArch64-Linux-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
                            "x86_64-Linux-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
                            "AArch64-macOS-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
                            "x86_64-macOS-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1",
                            "AArch64-Windows-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1.exe",
                            "x86_64-Windows-Subsystem-for-Arxumbra-v0.0.0.0.0.0-s1.exe",
                        ]
                        .iter(),
                    )
                    .for_each(|(executable, required): (std::string::String, &&str)| {
                        assert_eq!(executable, **required, "executable filename drifted");
                    });
                // Job identity, staging, package layout, and archive naming
                // must all be driven by the matrix expressions themselves.
                [
                    "name: ${{ matrix.artifact }}",
                    "runs-on: ${{ matrix.runner }}",
                    "${{ matrix.artifact }}${{ matrix.extension }}",
                    "dist/${{ matrix.artifact }}/assets",
                    "${{ matrix.artifact }}.tar.gz",
                ]
                .iter()
                .for_each(|expression: &&str| {
                    assert!(
                        yaml.contains(*expression),
                        "release workflow lost the matrix expression {expression}"
                    );
                });
                assert!(
                    !yaml.contains(".exe.exe"),
                    "doubled .exe suffix detected in release workflow"
                );
            });
    }
}
