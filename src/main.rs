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
                      badge: bevy::asset::Handle<bevy::image::Image>,
                      world: &mut bevy::ecs::world::World| {
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
                                    (0..24576u32)
                                        .flat_map(|index: u32| {
                                            (|x: f32, y: f32| {
                                                [
                                                    255u8,
                                                    255u8,
                                                    255u8,
                                                    ((0.48 * (x / 47.5).powf(2.0) * f32::min(f32::min(y, 511.0 - y) / 12.0, 1.0)).clamp(0.0, 1.0)
                                                        * 255.0) as u8,
                                                ]
                                            })(
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
                                            (|x: f32, y: f32| {
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
                                            })(
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
                                            (|x: f32, y: f32| {
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
                                            })(
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
                                            (|x: f32, y: f32| {
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
                                            })(
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
                                            (|x: f32, y: f32, height: f32| {
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
                                            })(
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
                                            (|x: f32, y: f32| {
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
                                            })(
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
                                            (|x: f32, y: f32| {
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
                                            })(
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
                        world
                            .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                            .add(bevy::image::Image {
                                data: Some(
                                    (0..3136u32)
                                        .flat_map(|index: u32| {
                                            (|x: f32, y: f32| {
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
                                            })(
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
        .add_systems(
            bevy::app::Update,
            |world: &mut bevy::ecs::world::World| {
                (|(f1, escape): (bool, bool),
                  delta: f32,
                  pressed: bool,
                  cursor: bevy::math::Vec2,
                  (width, height): (f32, f32),
                  (hidden, launched, hold): (bool, bool, f32),
                  world: &mut bevy::ecs::world::World| {
                    if f1 || escape || pressed || hold != 0.0 {
                        (|hidden_next: bool,
                          launched_next: bool,
                          hold_next: f32,
                          world: &mut bevy::ecs::world::World| {
                            if hold_next >= 7.0 {
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
                                .for_each(|(visibility, transform, name, sprite)| {
                                    if transform.scale.z != hold_next {
                                        transform.into_inner().scale.z = hold_next;
                                    }
                                    (|slot: bool, named: bool| {
                                        (|vis_next: bevy::camera::visibility::Visibility| {
                                            if *visibility != vis_next {
                                                *visibility.into_inner() = vis_next;
                                            }
                                        })(
                                            [
                                                bevy::camera::visibility::Visibility::Hidden,
                                                bevy::camera::visibility::Visibility::Visible,
                                            ][(!hidden_next && (!named || launched_next == slot))
                                                as usize],
                                        );
                                        sprite.into_iter().for_each(
                                            |sprite: bevy::ecs::change_detection::Mut<
                                                bevy::sprite::Sprite,
                                            >| {
                                                (|current: bevy::color::Color,
                                                  sprite: bevy::ecs::change_detection::Mut<
                                                      bevy::sprite::Sprite,
                                                  >| {
                                                    (|next: bevy::color::Color| {
                                                        if current != next {
                                                            sprite.into_inner().color = next;
                                                        }
                                                    })(
                                                        [
                                                            current,
                                                            [
                                                                bevy::color::Color::srgba(
                                                                    1.0, 1.0, 1.0, 0.0,
                                                                ),
                                                                bevy::color::Color::WHITE,
                                                            ][launched_next as usize],
                                                        ][slot as usize],
                                                    );
                                                })(sprite.color, sprite);
                                            },
                                        );
                                    })(
                                        name.into_iter().fold(
                                            false,
                                            |_, name: &bevy::ecs::name::Name| {
                                                name.as_str() == "slot"
                                            },
                                        ),
                                        name.is_some(),
                                    );
                                });
                        })(
                            hidden != f1,
                            launched
                                || (pressed
                                    && hidden == f1
                                    && cursor.x >= width / 3.0 + 81.0
                                    && cursor.x <= width / 3.0 + 655.0
                                    && cursor.y >= height / 2.0 - 192.0
                                    && cursor.y <= height / 2.0 - 139.0),
                            (hold + delta * escape as u8 as f32) * escape as u8 as f32,
                            world,
                        );
                    }
                })(
                    (|keys: &bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>| {
                        (
                            keys.just_pressed(bevy::input::keyboard::KeyCode::F1),
                            keys.pressed(bevy::input::keyboard::KeyCode::Escape),
                        )
                    })(
                        world
                            .resource::<
                                bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>,
                            >(),
                    ),
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
                                })(
                                    name.into_iter().fold(
                                        false,
                                        |_, name: &bevy::ecs::name::Name| name.as_str() == "slot",
                                    ),
                                )
                            },
                        ),
                    world,
                );
            },
        )
        .run();
}
