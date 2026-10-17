fn main() {
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Destination {
        Home,
        Registry,
    }
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Control {
        HomeTab,
        RegistryTab,
        Launch,
    }
    #[derive(bevy::ecs::component::Component, Debug, Clone, Copy, PartialEq)]
    enum Role {
        SidebarPanel,
        Divider,
        AppIcon,
        AppTitle,
        TabBody(Control),
        TabIcon(Control),
        TabLabel(Control),
        TabRing(Control),
        RegistryRoot,
        CardBackground,
        CardIcon,
        CardTitle,
        CardDescription,
        CardSeparator,
        LaunchBody,
        LaunchIcon,
        LaunchLabel,
        Badge,
        BadgeLabel,
        Well,
        Rocket,
    }
    #[derive(Clone, Copy, PartialEq)]
    struct Layout {
        width: f32,
        height: f32,
        scale: f32,
        sidebar_w: f32,
        icon: bevy::math::Rect,
        title: bevy::math::Rect,
        home_tab: bevy::math::Rect,
        registry_tab: bevy::math::Rect,
        card: bevy::math::Rect,
        card_icon: bevy::math::Rect,
        card_title: bevy::math::Rect,
        desc: bevy::math::Rect,
        sep: bevy::math::Rect,
        launch: bevy::math::Rect,
        badge: bevy::math::Rect,
        well: bevy::math::Rect,
    }
    #[derive(bevy::ecs::resource::Resource, Clone, Copy, PartialEq)]
    struct UiState {
        selected: Destination,
        hud_visible: bool,
        hovered: Option<Control>,
        focused: Option<Control>,
        focus_keyboard: bool,
        armed: Option<Control>,
        launched: bool,
        hold: f32,
        window_focused: bool,
        last_cursor: Option<bevy::math::Vec2>,
        layout: Layout,
        title_h: f32,
        desc_h: f32,
        measured_settled: bool,
        interactive_logged: bool,
    }
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
            bevy::log::LogPlugin::default(),
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
                        min_width: 640.0,
                        min_height: 480.0,
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
                    visible: true,
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
                bevy::log::info!("ARXUMBRA-STARTUP images=9 entities=26");
                (|solid: bevy::asset::Handle<bevy::image::Image>,
                  pill: bevy::asset::Handle<bevy::image::Image>,
                  ring: bevy::asset::Handle<bevy::image::Image>,
                  house: bevy::asset::Handle<bevy::image::Image>,
                  registry: bevy::asset::Handle<bevy::image::Image>,
                  app_icon: bevy::asset::Handle<bevy::image::Image>,
                  flask: bevy::asset::Handle<bevy::image::Image>,
                  rocket_pill: bevy::asset::Handle<bevy::image::Image>,
                  rocket: bevy::asset::Handle<bevy::image::Image>,
                  world: &mut bevy::ecs::world::World| {
                    world.insert_resource(UiState {
                        selected: Destination::Home,
                        hud_visible: false,
                        hovered: None,
                        focused: None,
                        focus_keyboard: false,
                        armed: None,
                        launched: false,
                        hold: 0.0,
                        window_focused: true,
                        last_cursor: None,
                        layout: Layout {
                            width: 0.0,
                            height: 0.0,
                            scale: 0.0,
                            sidebar_w: 0.0,
                            icon: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            title: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            home_tab: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            registry_tab: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            card: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            card_icon: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            card_title: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            desc: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            sep: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            launch: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            badge: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                            well: bevy::math::Rect::new(0.0, 0.0, 0.0, 0.0),
                        },
                        title_h: 104.0,
                        desc_h: 72.0,
                        measured_settled: false,
                        interactive_logged: false,
                    });
                    world.spawn((
                        Role::SidebarPanel,
                        bevy::sprite::Sprite {
                            image: solid.clone(),
                            texture_atlas: None,
                            color: bevy::color::Color::srgb(
                                12.0 / 255.0,
                                12.0 / 255.0,
                                12.0 / 255.0,
                            ),
                            flip_x: false,
                            flip_y: false,
                            custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                            rect: None,
                            image_mode: bevy::sprite::SpriteImageMode::Auto,
                            alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                        },
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 0.0),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::Divider,
                        bevy::sprite::Sprite {
                            image: solid.clone(),
                            texture_atlas: None,
                            color: bevy::color::Color::srgb(
                                56.0 / 255.0,
                                56.0 / 255.0,
                                56.0 / 255.0,
                            ),
                            flip_x: false,
                            flip_y: false,
                            custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                            rect: None,
                            image_mode: bevy::sprite::SpriteImageMode::Auto,
                            alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                        },
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 1.0),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::AppIcon,
                        bevy::sprite::Sprite {
                            image: app_icon.clone(),
                            texture_atlas: None,
                            color: bevy::color::Color::WHITE,
                            flip_x: false,
                            flip_y: false,
                            custom_size: Some(bevy::math::Vec2::new(32.0, 32.0)),
                            rect: None,
                            image_mode: bevy::sprite::SpriteImageMode::Auto,
                            alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                        },
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 2.2),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::AppTitle,
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
                            font_size: bevy::text::FontSize::Px(20.0),
                            weight: bevy::text::FontWeight::NORMAL,
                            width: bevy::text::FontWidth::NORMAL,
                            style: bevy::text::FontStyle::Normal,
                            font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                            font_features: bevy::text::FontFeatures::builder().build(),
                            font_variations: bevy::text::FontVariations::builder().build(),
                        },
                        bevy::text::TextColor(bevy::color::Color::srgb(
                            240.0 / 255.0,
                            240.0 / 255.0,
                            240.0 / 255.0,
                        )),
                        bevy::text::TextLayout {
                            justify: bevy::text::Justify::Left,
                            linebreak: bevy::text::LineBreak::WordBoundary,
                        },
                        bevy::text::TextBounds {
                            width: None,
                            height: None,
                        },
                        bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.5)),
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 2.4),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::TabBody(Control::HomeTab),
                        bevy::sprite::Sprite {
                            image: pill.clone(),
                            texture_atlas: None,
                            color: bevy::color::Color::srgb(
                                28.0 / 255.0,
                                28.0 / 255.0,
                                28.0 / 255.0,
                            ),
                            flip_x: false,
                            flip_y: false,
                            custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                            rect: None,
                            image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                bevy::sprite::TextureSlicer {
                                    border: bevy::sprite::BorderRect {
                                        min_inset: bevy::math::Vec2::new(16.0, 14.0),
                                        max_inset: bevy::math::Vec2::new(16.0, 14.0),
                                    },
                                    center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                    sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                    max_corner_scale: 1.0,
                                },
                            ),
                            alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                        },
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 3.0),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::TabIcon(Control::HomeTab),
                        bevy::sprite::Sprite {
                            image: house.clone(),
                            texture_atlas: None,
                            color: bevy::color::Color::srgb(
                                200.0 / 255.0,
                                200.0 / 255.0,
                                200.0 / 255.0,
                            ),
                            flip_x: false,
                            flip_y: false,
                            custom_size: Some(bevy::math::Vec2::new(24.0, 24.0)),
                            rect: None,
                            image_mode: bevy::sprite::SpriteImageMode::Auto,
                            alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                        },
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 3.2),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::TabLabel(Control::HomeTab),
                        bevy::sprite::Text2d(std::string::String::from("Home")),
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
                        bevy::text::TextColor(bevy::color::Color::srgb(
                            225.0 / 255.0,
                            225.0 / 255.0,
                            225.0 / 255.0,
                        )),
                        bevy::text::TextLayout {
                            justify: bevy::text::Justify::Left,
                            linebreak: bevy::text::LineBreak::WordBoundary,
                        },
                        bevy::text::TextBounds {
                            width: None,
                            height: None,
                        },
                        bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 3.3),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::TabRing(Control::HomeTab),
                        bevy::sprite::Sprite {
                            image: ring.clone(),
                            texture_atlas: None,
                            color: bevy::color::Color::WHITE,
                            flip_x: false,
                            flip_y: false,
                            custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                            rect: None,
                            image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                bevy::sprite::TextureSlicer {
                                    border: bevy::sprite::BorderRect {
                                        min_inset: bevy::math::Vec2::new(16.0, 14.0),
                                        max_inset: bevy::math::Vec2::new(16.0, 14.0),
                                    },
                                    center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                    sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                    max_corner_scale: 1.0,
                                },
                            ),
                            alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                        },
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 3.4),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::TabBody(Control::RegistryTab),
                        bevy::sprite::Sprite {
                            image: pill.clone(),
                            texture_atlas: None,
                            color: bevy::color::Color::srgb(
                                28.0 / 255.0,
                                28.0 / 255.0,
                                28.0 / 255.0,
                            ),
                            flip_x: false,
                            flip_y: false,
                            custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                            rect: None,
                            image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                bevy::sprite::TextureSlicer {
                                    border: bevy::sprite::BorderRect {
                                        min_inset: bevy::math::Vec2::new(16.0, 14.0),
                                        max_inset: bevy::math::Vec2::new(16.0, 14.0),
                                    },
                                    center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                    sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                    max_corner_scale: 1.0,
                                },
                            ),
                            alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                        },
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 3.0),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::TabIcon(Control::RegistryTab),
                        bevy::sprite::Sprite {
                            image: registry.clone(),
                            texture_atlas: None,
                            color: bevy::color::Color::srgb(
                                200.0 / 255.0,
                                200.0 / 255.0,
                                200.0 / 255.0,
                            ),
                            flip_x: false,
                            flip_y: false,
                            custom_size: Some(bevy::math::Vec2::new(24.0, 24.0)),
                            rect: None,
                            image_mode: bevy::sprite::SpriteImageMode::Auto,
                            alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                        },
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 3.2),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::TabLabel(Control::RegistryTab),
                        bevy::sprite::Text2d(std::string::String::from("Extension Registry")),
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
                        bevy::text::TextColor(bevy::color::Color::srgb(
                            225.0 / 255.0,
                            225.0 / 255.0,
                            225.0 / 255.0,
                        )),
                        bevy::text::TextLayout {
                            justify: bevy::text::Justify::Left,
                            linebreak: bevy::text::LineBreak::WordBoundary,
                        },
                        bevy::text::TextBounds {
                            width: None,
                            height: None,
                        },
                        bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 3.3),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    world.spawn((
                        Role::TabRing(Control::RegistryTab),
                        bevy::sprite::Sprite {
                            image: ring,
                            texture_atlas: None,
                            color: bevy::color::Color::WHITE,
                            flip_x: false,
                            flip_y: false,
                            custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                            rect: None,
                            image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                bevy::sprite::TextureSlicer {
                                    border: bevy::sprite::BorderRect {
                                        min_inset: bevy::math::Vec2::new(16.0, 14.0),
                                        max_inset: bevy::math::Vec2::new(16.0, 14.0),
                                    },
                                    center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                    sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                    max_corner_scale: 1.0,
                                },
                            ),
                            alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                        },
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::new(0.0, 0.0, 3.4),
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                        },
                        bevy::camera::visibility::Visibility::Hidden,
                    ));
                    (|group_root: bevy::ecs::entity::Entity,
                      world: &mut bevy::ecs::world::World| {
                        world.spawn((
                            Role::CardBackground,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Sprite {
                                image: pill.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(
                                    18.0 / 255.0,
                                    18.0 / 255.0,
                                    18.0 / 255.0,
                                ),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(16.0, 14.0),
                                            max_inset: bevy::math::Vec2::new(16.0, 14.0),
                                        },
                                        center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.0),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::CardIcon,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Sprite {
                                image: flask,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(28.0, 28.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.4),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::CardTitle,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Text2d(std::string::String::from("Laboratory")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                )),
                                font_size: bevy::text::FontSize::Px(26.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(
                                240.0 / 255.0,
                                240.0 / 255.0,
                                240.0 / 255.0,
                            )),
                            bevy::text::TextLayout {
                                justify: bevy::text::Justify::Left,
                                linebreak: bevy::text::LineBreak::WordBoundary,
                            },
                            bevy::text::TextBounds {
                                width: None,
                                height: None,
                            },
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.5)),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.5),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::CardDescription,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Text2d(std::string::String::from(
                                "Manage the installed Arxumbra subsystem extensions.",
                            )),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                )),
                                font_size: bevy::text::FontSize::Px(18.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(
                                170.0 / 255.0,
                                170.0 / 255.0,
                                170.0 / 255.0,
                            )),
                            bevy::text::TextLayout {
                                justify: bevy::text::Justify::Left,
                                linebreak: bevy::text::LineBreak::WordBoundary,
                            },
                            bevy::text::TextBounds {
                                width: None,
                                height: None,
                            },
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.5)),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.5),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::CardSeparator,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Sprite {
                                image: solid,
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(
                                    56.0 / 255.0,
                                    56.0 / 255.0,
                                    56.0 / 255.0,
                                ),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.4),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::LaunchBody,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Sprite {
                                image: pill.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(
                                    235.0 / 255.0,
                                    235.0 / 255.0,
                                    235.0 / 255.0,
                                ),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(16.0, 14.0),
                                            max_inset: bevy::math::Vec2::new(16.0, 14.0),
                                        },
                                        center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.6),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::LaunchIcon,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Sprite {
                                image: rocket_pill,
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(
                                    20.0 / 255.0,
                                    20.0 / 255.0,
                                    20.0 / 255.0,
                                ),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(24.0, 24.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.7),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::LaunchLabel,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Text2d(std::string::String::from("Launch")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                )),
                                font_size: bevy::text::FontSize::Px(22.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(
                                20.0 / 255.0,
                                20.0 / 255.0,
                                20.0 / 255.0,
                            )),
                            bevy::text::TextLayout {
                                justify: bevy::text::Justify::Left,
                                linebreak: bevy::text::LineBreak::WordBoundary,
                            },
                            bevy::text::TextBounds {
                                width: None,
                                height: None,
                            },
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.7),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::Badge,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Sprite {
                                image: pill.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(
                                    30.0 / 255.0,
                                    30.0 / 255.0,
                                    30.0 / 255.0,
                                ),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(16.0, 14.0),
                                            max_inset: bevy::math::Vec2::new(16.0, 14.0),
                                        },
                                        center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.4),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::BadgeLabel,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Text2d(std::string::String::from("Coming Soon")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                )),
                                font_size: bevy::text::FontSize::Px(14.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(
                                160.0 / 255.0,
                                160.0 / 255.0,
                                160.0 / 255.0,
                            )),
                            bevy::text::TextLayout {
                                justify: bevy::text::Justify::Center,
                                linebreak: bevy::text::LineBreak::WordBoundary,
                            },
                            bevy::text::TextBounds {
                                width: None,
                                height: None,
                            },
                            bevy::sprite::Anchor(bevy::math::Vec2::ZERO),
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.5),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::Well,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Sprite {
                                image: pill.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(
                                    10.0 / 255.0,
                                    10.0 / 255.0,
                                    10.0 / 255.0,
                                ),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(1.0, 1.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(16.0, 14.0),
                                            max_inset: bevy::math::Vec2::new(16.0, 14.0),
                                        },
                                        center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.4),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                        world.spawn((
                            Role::Rocket,
                            bevy::ecs::hierarchy::ChildOf(group_root),
                            bevy::sprite::Sprite {
                                image: rocket,
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(96.0, 96.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(0.0, 0.0, 2.5),
                                rotation: bevy::math::Quat::IDENTITY,
                                scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                            },
                            bevy::camera::visibility::Visibility::Hidden,
                        ));
                    })(
                        world
                            .spawn((
                                Role::RegistryRoot,
                                bevy::transform::components::Transform {
                                    translation: bevy::math::Vec3::new(0.0, 0.0, 0.0),
                                    rotation: bevy::math::Quat::IDENTITY,
                                    scale: bevy::math::Vec3::new(1.0, 1.0, 1.0),
                                },
                                bevy::camera::visibility::Visibility::Hidden,
                            ))
                            .id(),
                        world,
                    );
                })(
                    world
                        .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                        .add(bevy::image::Image {
                            data: Some(
                                (0..16u32)
                                    .flat_map(|_index: u32| [255u8, 255u8, 255u8, 255u8])
                                    .collect::<std::vec::Vec<u8>>(),
                            ),
                            data_order:
                                bevy::render::render_resource::TextureDataOrder::LayerMajor,
                            texture_descriptor:
                                bevy::render::render_resource::TextureDescriptor {
                                    label: Some("ui-solid"),
                                    size: bevy::render::render_resource::Extent3d {
                                        width: 4,
                                        height: 4,
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
                                (0..4096u32)
                                    .flat_map(|index: u32| {
                                        (|x: f32, y: f32| {
                                            (|d: f32| {
                                                [
                                                    255u8,
                                                    255u8,
                                                    255u8,
                                                    ((14.5 - d).clamp(0.0, 1.0) * 255.0) as u8,
                                                ]
                                            })(f32::hypot(
                                                f32::max(f32::max(14.0 - x, x - 50.0), 0.0),
                                                f32::max(f32::max(14.0 - y, y - 50.0), 0.0),
                                            ))
                                        })(
                                            (index % 64) as f32 + 0.5,
                                            (index / 64) as f32 + 0.5,
                                        )
                                    })
                                    .collect::<std::vec::Vec<u8>>(),
                            ),
                            data_order:
                                bevy::render::render_resource::TextureDataOrder::LayerMajor,
                            texture_descriptor:
                                bevy::render::render_resource::TextureDescriptor {
                                    label: Some("ui-pill"),
                                    size: bevy::render::render_resource::Extent3d {
                                        width: 64,
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
                        .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                        .add(bevy::image::Image {
                            data: Some(
                                (0..4096u32)
                                    .flat_map(|index: u32| {
                                        (|x: f32, y: f32| {
                                            (|d: f32| {
                                                [
                                                    255u8,
                                                    255u8,
                                                    255u8,
                                                    (((14.5 - d).clamp(0.0, 1.0)
                                                        * (1.0 - (11.5 - d).clamp(0.0, 1.0)))
                                                        * 255.0) as u8,
                                                ]
                                            })(f32::hypot(
                                                f32::max(f32::max(14.0 - x, x - 50.0), 0.0),
                                                f32::max(f32::max(14.0 - y, y - 50.0), 0.0),
                                            ))
                                        })(
                                            (index % 64) as f32 + 0.5,
                                            (index / 64) as f32 + 0.5,
                                        )
                                    })
                                    .collect::<std::vec::Vec<u8>>(),
                            ),
                            data_order:
                                bevy::render::render_resource::TextureDataOrder::LayerMajor,
                            texture_descriptor:
                                bevy::render::render_resource::TextureDescriptor {
                                    label: Some("ui-ring"),
                                    size: bevy::render::render_resource::Extent3d {
                                        width: 64,
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
                        .resource::<bevy::asset::AssetServer>()
                        .load::<bevy::image::Image>("icons/flask.png"),
                    world
                        .resource::<bevy::asset::AssetServer>()
                        .load::<bevy::image::Image>("icons/rocket-pill.png"),
                    world
                        .resource::<bevy::asset::AssetServer>()
                        .load::<bevy::image::Image>("icons/rocket.png"),
                    world,
                );
            },
        )
        .add_systems(
            bevy::app::Update,
            |keys: bevy::ecs::system::Res<
                 bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>,
             >,
             mouse: bevy::ecs::system::Res<
                 bevy::input::ButtonInput<bevy::input::mouse::MouseButton>,
             >,
             windows: bevy::ecs::system::Query<
                 &bevy::window::Window,
                 bevy::ecs::query::With<bevy::window::PrimaryWindow>,
             >,
             time: bevy::ecs::system::Res<bevy::time::Time>,
             state: bevy::ecs::system::ResMut<UiState>,
             exit: bevy::ecs::system::ResMut<
                 bevy::ecs::message::Messages<bevy::app::AppExit>,
             >,
             entities: bevy::ecs::system::Query<
                 (
                     &Role,
                     &mut bevy::camera::visibility::Visibility,
                     &mut bevy::transform::components::Transform,
                     Option<&mut bevy::sprite::Sprite>,
                     Option<&mut bevy::text::TextColor>,
                     Option<&mut bevy::text::TextBounds>,
                     Option<&bevy::text::TextLayoutInfo>,
                 ),
                 bevy::ecs::query::Without<bevy::camera::Camera2d>,
             >| {
                (|(width_now, height_now, scale_now, window_focused, cursor_now): (
                     f32,
                     f32,
                     f32,
                     bool,
                     Option<bevy::math::Vec2>,
                 )| {
                    (if keys.just_pressed(bevy::input::keyboard::KeyCode::F1)
                        || keys.just_pressed(bevy::input::keyboard::KeyCode::Tab)
                        || keys.just_pressed(bevy::input::keyboard::KeyCode::Enter)
                        || keys.just_pressed(bevy::input::keyboard::KeyCode::Space)
                        || keys.pressed(bevy::input::keyboard::KeyCode::Escape)
                        || mouse.just_pressed(bevy::input::mouse::MouseButton::Left)
                        || mouse.just_released(bevy::input::mouse::MouseButton::Left)
                        || state.hold != 0.0
                        || state.armed.is_some()
                        || !state.interactive_logged
                        || !state.measured_settled
                        || window_focused != state.window_focused
                        || cursor_now != state.last_cursor
                        || width_now != state.layout.width
                        || height_now != state.layout.height
                        || scale_now != state.layout.scale
                    {
                        Some((|current: UiState,
                          title_measured: Option<f32>,
                          desc_measured: Option<f32>,
                          delta: f32,
                          elapsed: f32,
                          width_now: f32,
                          height_now: f32,
                          scale_now: f32,
                          f1_edge: bool,
                          tab_edge: bool,
                          shift_held: bool,
                          enter_edge: bool,
                          space_edge: bool,
                          escape_held: bool,
                          down_edge: bool,
                          up_edge: bool,
                          window_focused: bool,
                          cursor_now: Option<bevy::math::Vec2>| {
                            (|title_h_next: f32, desc_h_next: f32| {
                                (|sidebar_w: f32,
                                  content_w: f32,
                                  card_w: f32,
                                  tabs_top: f32,
                                  title_w: f32| {
                                    (|card_x: f32, card_h: f32, launch_w: f32| {
                                        (|layout_next: Layout,
                                          layout_dirty: bool,
                                          settled_next: bool| {
                                            (|hud_next: bool, hold_next: f32| {
                                                (|hover_next: Option<Control>| {
                                                    (|armed_mid: Option<Control>,
                                                      focused_mid: Option<Control>,
                                                      fk_mid: bool| {
                                                        (|selected_r: Destination,
                                                          launched_r: bool,
                                                          armed_mid2: Option<Control>,
                                                          focused_mid2: Option<Control>,
                                                          fk_mid2: bool| {
                                                            (|selected_next: Destination,
                                                              launched_next: bool,
                                                              armed_next: Option<Control>,
                                                              focused_next: Option<Control>,
                                                              fk_next: bool| {
                                                                (UiState {
                                                                    selected: selected_next,
                                                                    hud_visible: hud_next,
                                                                    hovered: hover_next,
                                                                    focused: focused_next,
                                                                    focus_keyboard: fk_next,
                                                                    armed: armed_next,
                                                                    launched: launched_next,
                                                                    hold: hold_next,
                                                                    window_focused,
                                                                    last_cursor: cursor_now,
                                                                    layout: layout_next,
                                                                    title_h: title_h_next,
                                                                    desc_h: desc_h_next,
                                                                    measured_settled: settled_next,
                                                                    interactive_logged:
                                                                        current.interactive_logged
                                                                            || elapsed >= 2.0,
                                                                }, layout_dirty)
                                                            })(
                                                                if (enter_edge || space_edge)
                                                                    && hud_next
                                                                {
                                                                    match focused_mid2 {
                                                                        Some(Control::HomeTab) => {
                                                                            Destination::Home
                                                                        }
                                                                        Some(Control::RegistryTab) => {
                                                                            Destination::Registry
                                                                        }
                                                                        _ => selected_r,
                                                                    }
                                                                } else {
                                                                    selected_r
                                                                },
                                                                launched_r
                                                                    || ((enter_edge || space_edge)
                                                                        && hud_next
                                                                        && focused_mid2
                                                                            == Some(
                                                                                Control::Launch,
                                                                            )
                                                                        && selected_r
                                                                            == Destination::Registry
                                                                        && !launched_r),
                                                                match armed_mid2 {
                                                                    Some(Control::Launch) => [
                                                                        None,
                                                                        Some(Control::Launch),
                                                                    ][(hud_next
                                                                        && selected_r
                                                                            == Destination::Registry
                                                                        && !launched_r)
                                                                        as usize],
                                                                    other => other,
                                                                },
                                                                match focused_mid2 {
                                                                    Some(Control::Launch) => [
                                                                        None,
                                                                        Some(Control::Launch),
                                                                    ][(hud_next
                                                                        && selected_r
                                                                            == Destination::Registry
                                                                        && !launched_r)
                                                                        as usize],
                                                                    Some(Control::HomeTab) => [
                                                                        None,
                                                                        Some(Control::HomeTab),
                                                                    ][hud_next as usize],
                                                                    Some(Control::RegistryTab) => [
                                                                        None,
                                                                        Some(
                                                                            Control::RegistryTab,
                                                                        ),
                                                                    ][hud_next as usize],
                                                                    None => None,
                                                                },
                                                                fk_mid2,
                                                            )
                                                        })(
                                                            if up_edge
                                                                && armed_mid.is_some()
                                                                && hover_next == armed_mid
                                                                && window_focused
                                                            {
                                                                match armed_mid {
                                                                    Some(Control::HomeTab) => {
                                                                        Destination::Home
                                                                    }
                                                                    Some(Control::RegistryTab) => {
                                                                        Destination::Registry
                                                                    }
                                                                    _ => current.selected,
                                                                }
                                                            } else {
                                                                current.selected
                                                            },
                                                            current.launched
                                                                || (up_edge
                                                                    && armed_mid
                                                                        == Some(Control::Launch)
                                                                    && hover_next == armed_mid
                                                                    && window_focused),
                                                            [armed_mid, None][up_edge as usize],
                                                            [
                                                                focused_mid,
                                                                (|result: (usize, usize)| {
                                                                    if result.1 == 99 {
                                                                        None
                                                                    } else {
                                                                        Some(
                                                                            [
                                                                                Control::HomeTab,
                                                                                Control::RegistryTab,
                                                                                Control::Launch,
                                                                            ][result.1],
                                                                        )
                                                                    }
                                                                })(
                                                                    (1..4usize).fold(
                                                                        (99usize, 99usize),
                                                                        |acc: (usize, usize),
                                                                         step: usize| {
                                                                            (|eligible: [bool; 3]| {
                                                                                (|index: usize| {
                                                                                    if acc.0 == 99
                                                                                        && eligible
                                                                                            [index]
                                                                                    {
                                                                                        (step, index)
                                                                                    } else {
                                                                                        acc
                                                                                    }
                                                                                })(
                                                                                    if focused_mid
                                                                                        .is_none()
                                                                                    {
                                                                                        step - 1
                                                                                    } else if shift_held
                                                                                    {
                                                                                        ([
                                                                                            0usize,
                                                                                            1usize,
                                                                                            2usize,
                                                                                        ][match focused_mid
                                                                                        {
                                                                                            Some(
                                                                                                Control::HomeTab,
                                                                                            ) => 0,
                                                                                            Some(
                                                                                                Control::RegistryTab,
                                                                                            ) => 1,
                                                                                            _ => 2,
                                                                                        }]
                                                                                            + 3
                                                                                            - step)
                                                                                            % 3
                                                                                    } else {
                                                                                        ([
                                                                                            0usize,
                                                                                            1usize,
                                                                                            2usize,
                                                                                        ][match focused_mid
                                                                                        {
                                                                                            Some(
                                                                                                Control::HomeTab,
                                                                                            ) => 0,
                                                                                            Some(
                                                                                                Control::RegistryTab,
                                                                                            ) => 1,
                                                                                            _ => 2,
                                                                                        }]
                                                                                            + step)
                                                                                            % 3
                                                                                    },
                                                                                )
                                                                            })([
                                                                                hud_next,
                                                                                hud_next,
                                                                                hud_next
                                                                                    && (if up_edge
                                                                                        && armed_mid
                                                                                            .is_some()
                                                                                        && hover_next
                                                                                            == armed_mid
                                                                                        && window_focused
                                                                                    {
                                                                                        match armed_mid
                                                                                        {
                                                                                            Some(
                                                                                                Control::HomeTab,
                                                                                            ) => {
                                                                                                Destination::Home
                                                                                            }
                                                                                            Some(
                                                                                                Control::RegistryTab,
                                                                                            ) => {
                                                                                                Destination::Registry
                                                                                            }
                                                                                            _ => {
                                                                                                current
                                                                                                    .selected
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        current
                                                                                            .selected
                                                                                    })
                                                                                        == Destination::Registry
                                                                                    && !(current
                                                                                        .launched
                                                                                        || (up_edge
                                                                                            && armed_mid
                                                                                                == Some(
                                                                                                    Control::Launch,
                                                                                                )
                                                                                            && hover_next
                                                                                                == armed_mid
                                                                                            && window_focused)),
                                                                            ])
                                                                        },
                                                                    ),
                                                                ),
                                                                focused_mid,
                                                            ][tab_edge as usize],
                                                            [fk_mid, true][tab_edge as usize],
                                                        )
                                                    })(
                                                        if !window_focused || !hud_next {
                                                            None
                                                        } else if down_edge {
                                                            hover_next
                                                        } else {
                                                            match current.armed {
                                                                Some(Control::Launch) => [
                                                                    None,
                                                                    Some(Control::Launch),
                                                                ][(current.selected
                                                                    == Destination::Registry
                                                                    && !current.launched)
                                                                    as usize],
                                                                other => other,
                                                            }
                                                        },
                                                        if down_edge
                                                            && hover_next.is_some()
                                                            && window_focused
                                                            && hud_next
                                                        {
                                                            hover_next
                                                        } else {
                                                            current.focused
                                                        },
                                                        if down_edge
                                                            && hover_next.is_some()
                                                            && window_focused
                                                            && hud_next
                                                        {
                                                            false
                                                        } else {
                                                            current.focus_keyboard
                                                        },
                                                    )
                                                })(
                                                    if !window_focused || !hud_next {
                                                        None
                                                    } else {
                                                        cursor_now.into_iter().fold(
                                                            None,
                                                            |_: Option<Control>,
                                                             point: bevy::math::Vec2| {
                                                                [
                                                                    Control::HomeTab,
                                                                    Control::RegistryTab,
                                                                    Control::Launch,
                                                                ]
                                                                .into_iter()
                                                                .fold(
                                                                    None,
                                                                    |acc: Option<Control>,
                                                                     control: Control| {
                                                                        (|eligible: bool,
                                                                          rect: bevy::math::Rect| {
                                                                            if acc.is_none()
                                                                                && eligible
                                                                                && rect
                                                                                    .contains(
                                                                                        point,
                                                                                    )
                                                                            {
                                                                                Some(control)
                                                                            } else {
                                                                                acc
                                                                            }
                                                                        })(
                                                                            match control {
                                                                                Control::HomeTab => {
                                                                                    true
                                                                                }
                                                                                Control::RegistryTab => {
                                                                                    true
                                                                                }
                                                                                Control::Launch => {
                                                                                    current.selected == Destination::Registry && !current.launched
                                                                                }
                                                                            },
                                                                            match control {
                                                                                Control::HomeTab => {
                                                                                    layout_next
                                                                                        .home_tab
                                                                                }
                                                                                Control::RegistryTab => {
                                                                                    layout_next
                                                                                        .registry_tab
                                                                                }
                                                                                Control::Launch => {
                                                                                    layout_next
                                                                                        .launch
                                                                                }
                                                                            },
                                                                        )
                                                                    },
                                                                )
                                                            },
                                                        )
                                                    },
                                                )
                                            })(
                                                if f1_edge {
                                                    !current.hud_visible
                                                } else {
                                                    current.hud_visible
                                                },
                                                (current.hold + delta * escape_held as u8 as f32)
                                                    * escape_held as u8 as f32,
                                            )
                                        })(
                                            Layout {
                                                width: width_now,
                                                height: height_now,
                                                scale: scale_now,
                                                sidebar_w,
                                                icon: bevy::math::Rect::new(16.0, 16.0, 48.0, 48.0),
                                                title: bevy::math::Rect::new(58.0, 16.0, 58.0 + title_w, 16.0 + title_h_next),
                                                home_tab: bevy::math::Rect::new(16.0, tabs_top, sidebar_w - 16.0, tabs_top + 56.0),
                                                registry_tab: bevy::math::Rect::new(16.0, tabs_top + 64.0, sidebar_w - 16.0, tabs_top + 120.0),
                                                card: bevy::math::Rect::new(card_x, 24.0, card_x + card_w, 24.0 + card_h),
                                                card_icon: bevy::math::Rect::new(card_x + 20.0, 44.0, card_x + 48.0, 72.0),
                                                card_title: bevy::math::Rect::new(card_x + 60.0, 44.0, card_x + card_w - 160.0, 72.0),
                                                desc: bevy::math::Rect::new(card_x + 20.0, 88.0, card_x + card_w - 20.0, 88.0 + desc_h_next),
                                                sep: bevy::math::Rect::new(card_x + 20.0, 104.0 + desc_h_next, card_x + card_w - 20.0, 105.0 + desc_h_next),
                                                launch: bevy::math::Rect::new(card_x + 20.0, 121.0 + desc_h_next, card_x + 20.0 + launch_w, 177.0 + desc_h_next),
                                                badge: bevy::math::Rect::new(card_x + card_w - 136.0, 44.0, card_x + card_w - 20.0, 72.0),
                                                well: bevy::math::Rect::new(card_x + 20.0, 88.0, card_x + card_w - 20.0, 24.0 + card_h - 20.0),
                                            },
                                            width_now != current.layout.width
                                                || height_now != current.layout.height
                                                || scale_now != current.layout.scale
                                                || title_h_next != current.title_h
                                                || desc_h_next != current.desc_h,
                                            title_measured == Some(title_h_next)
                                                && desc_measured == Some(desc_h_next)
                                                && !(width_now != current.layout.width
                                                    || height_now != current.layout.height
                                                    || scale_now != current.layout.scale
                                                    || title_h_next != current.title_h
                                                    || desc_h_next != current.desc_h),
                                        )
                                    })(
                                        sidebar_w + 24.0 + (content_w - card_w) / 2.0,
                                        (173.0 + desc_h_next).min(height_now - 48.0),
                                        (card_w - 40.0).min(240.0),
                                    )
                                })(
                                    (width_now * 0.26).clamp(180.0, 340.0),
                                    (width_now - (width_now * 0.26).clamp(180.0, 340.0) - 48.0)
                                        .max(240.0),
                                    (width_now - (width_now * 0.26).clamp(180.0, 340.0) - 48.0)
                                        .clamp(240.0, 560.0),
                                    title_h_next.max(32.0) + 32.0,
                                    ((width_now * 0.26).clamp(180.0, 340.0) - 74.0).max(60.0),
                                )
                            })(
                                title_measured.unwrap_or(current.title_h),
                                desc_measured.unwrap_or(current.desc_h),
                            )
                        })(
                            *state,
                            entities.iter().fold(
                                None,
                                |acc: Option<f32>,
                                 (role, _visibility, _transform, _sprite, _text_color, _text_bounds, layout_info)| {
                                    match (role, layout_info) {
                                        (Role::AppTitle, Some(layout_info)) => [
                                            acc,
                                            Some(layout_info.size.y / layout_info.scale_factor),
                                        ][(layout_info.scale_factor > 0.0) as usize],
                                        _ => acc,
                                    }
                                },
                            ),
                            entities.iter().fold(
                                None,
                                |acc: Option<f32>,
                                 (role, _visibility, _transform, _sprite, _text_color, _text_bounds, layout_info)| {
                                    match (role, layout_info) {
                                        (Role::CardDescription, Some(layout_info)) => [
                                            acc,
                                            Some(layout_info.size.y / layout_info.scale_factor),
                                        ][(layout_info.scale_factor > 0.0) as usize],
                                        _ => acc,
                                    }
                                },
                            ),
                            time.delta_secs(),
                            time.elapsed_secs(),
                            width_now,
                            height_now,
                            scale_now,
                            keys.just_pressed(bevy::input::keyboard::KeyCode::F1),
                            keys.just_pressed(bevy::input::keyboard::KeyCode::Tab),
                            keys.pressed(bevy::input::keyboard::KeyCode::ShiftLeft)
                                || keys.pressed(bevy::input::keyboard::KeyCode::ShiftRight),
                            keys.just_pressed(bevy::input::keyboard::KeyCode::Enter),
                            keys.just_pressed(bevy::input::keyboard::KeyCode::Space),
                            keys.pressed(bevy::input::keyboard::KeyCode::Escape),
                            mouse.just_pressed(bevy::input::mouse::MouseButton::Left),
                            mouse.just_released(bevy::input::mouse::MouseButton::Left),
                            window_focused,
                            cursor_now,
                        ))
                    } else {
                        None
                    })
                        .map_or((), |(next_state, layout_dirty_now): (UiState, bool)| {
                        if next_state.hold >= 7.0 {
                            exit.into_inner().write(
                                bevy::app::AppExit::Success,
                            );
                        }
                        if next_state.interactive_logged && !state.interactive_logged {
                            bevy::log::info!(
                                "ARXUMBRA-INTERACTIVE selected={:?} hud={} launched={} w={} h={} s={} home={},{},{},{} registry={},{},{},{} launch={},{},{},{}",
                                next_state.selected,
                                next_state.hud_visible,
                                next_state.launched,
                                next_state.layout.width,
                                next_state.layout.height,
                                next_state.layout.scale,
                                next_state.layout.home_tab.min.x,
                                next_state.layout.home_tab.min.y,
                                next_state.layout.home_tab.size().x,
                                next_state.layout.home_tab.size().y,
                                next_state.layout.registry_tab.min.x,
                                next_state.layout.registry_tab.min.y,
                                next_state.layout.registry_tab.size().x,
                                next_state.layout.registry_tab.size().y,
                                next_state.layout.launch.min.x,
                                next_state.layout.launch.min.y,
                                next_state.layout.launch.size().x,
                                next_state.layout.launch.size().y,
                            );
                        }
                        if layout_dirty_now {
                            bevy::log::info!(
                                "ARXUMBRA-LAYOUT w={} h={} s={} home={},{},{},{} registry={},{},{},{} launch={},{},{},{}",
                                next_state.layout.width,
                                next_state.layout.height,
                                next_state.layout.scale,
                                next_state.layout.home_tab.min.x,
                                next_state.layout.home_tab.min.y,
                                next_state.layout.home_tab.size().x,
                                next_state.layout.home_tab.size().y,
                                next_state.layout.registry_tab.min.x,
                                next_state.layout.registry_tab.min.y,
                                next_state.layout.registry_tab.size().x,
                                next_state.layout.registry_tab.size().y,
                                next_state.layout.launch.min.x,
                                next_state.layout.launch.min.y,
                                next_state.layout.launch.size().x,
                                next_state.layout.launch.size().y,
                            );
                        }
                        if *state != next_state {
                            *state.into_inner() = next_state;
                        }
                        for (
                            role,
                            visibility,
                            transform,
                            sprite,
                            text_color,
                            text_bounds,
                            _layout_info,
                        ) in entities
                        {
                            (|(
                                 rect,
                                 is_text,
                                 shown,
                                 tint,
                                 ink,
                                 wrap,
                             ): (
                                 Option<bevy::math::Rect>,
                                 bool,
                                 bool,
                                 Option<bevy::color::Color>,
                                 Option<bevy::color::Color>,
                                 Option<f32>,
                             )| {
                                (|vis_next: bevy::camera::visibility::Visibility| {
                                    if *visibility
                                        != vis_next
                                    {
                                        *visibility
                                            .into_inner(
                                            ) = vis_next;
                                    }
                                })([
                                    bevy::camera::visibility::Visibility::Hidden,
                                    bevy::camera::visibility::Visibility::Visible,
                                ][shown as usize]);
                                rect.map_or((), |rect: bevy::math::Rect| {
                                    if layout_dirty_now {
                                        (|kept_z: f32, transform: bevy::ecs::change_detection::Mut<bevy::transform::components::Transform>| {
                                        (|next: bevy::transform::components::Transform| {
                                            if *transform
                                                != next
                                            {
                                                *transform
                                                    .into_inner(
                                                    ) = next;
                                            }
                                        })(bevy::transform::components::Transform {
                                            translation: bevy::math::Vec3::new(
                                                [
                                                    (rect.min.x
                                                        + rect
                                                            .size(
                                                            )
                                                            .x
                                                            / 2.0
                                                        - next_state.layout
                                                            .width
                                                            / 2.0)
                                                        * next_state.layout
                                                            .scale,
                                                    (rect.min.x
                                                        - next_state.layout
                                                            .width
                                                            / 2.0)
                                                        * next_state.layout
                                                            .scale,
                                                ][is_text
                                                    as usize],
                                                [
                                                    (next_state.layout
                                                        .height
                                                        / 2.0
                                                        - (rect.min.y
                                                            + rect
                                                                .size(
                                                                )
                                                                .y
                                                                / 2.0))
                                                        * next_state.layout
                                                            .scale,
                                                    (next_state.layout
                                                        .height
                                                        / 2.0
                                                        - rect.min.y)
                                                        * next_state.layout
                                                            .scale,
                                                ][is_text
                                                    as usize],
                                                kept_z,
                                            ),
                                            rotation: bevy::math::Quat::IDENTITY,
                                            scale: [
                                                bevy::math::Vec3::new(
                                                    next_state.layout
                                                        .scale,
                                                    next_state.layout
                                                        .scale,
                                                    1.0,
                                                ),
                                                bevy::math::Vec3::new(
                                                    1.0,
                                                    1.0,
                                                    1.0,
                                                ),
                                            ][is_text
                                                as usize],
                                        });
                                        })(transform.translation.z, transform);
                                    }
                                    sprite.into_iter().for_each(|sprite: bevy::ecs::change_detection::Mut<bevy::sprite::Sprite>| {
                                        (|current_sprite: bevy::sprite::Sprite, sprite: bevy::ecs::change_detection::Mut<bevy::sprite::Sprite>| {
                                            (|next: bevy::sprite::Sprite| {
                                                if current_sprite.color != next.color || current_sprite.custom_size != next.custom_size {
                                                    *sprite.into_inner() = next;
                                                }
                                            })(bevy::sprite::Sprite {
                                                color: tint.unwrap_or(current_sprite.color),
                                                custom_size: [current_sprite.custom_size, Some(rect.size())][layout_dirty_now as usize],
                                                ..current_sprite
                                            });
                                        })(sprite.clone(), sprite);
                                    });
                                    text_color.into_iter().for_each(|text_color: bevy::ecs::change_detection::Mut<bevy::text::TextColor>| {
                                        (|current_ink: bevy::color::Color, text_color: bevy::ecs::change_detection::Mut<bevy::text::TextColor>| {
                                            (|next: bevy::text::TextColor| {
                                                if current_ink != next.0 {
                                                    *text_color.into_inner() = next;
                                                }
                                            })(bevy::text::TextColor(ink.unwrap_or(current_ink)));
                                        })(text_color.0, text_color);
                                    });
                                    text_bounds.into_iter().for_each(|text_bounds: bevy::ecs::change_detection::Mut<bevy::text::TextBounds>| {
                                        if layout_dirty_now {
                                            (|next: Option<f32>| {
                                                if text_bounds.width != next {
                                                    text_bounds.into_inner().width = next;
                                                }
                                            })(wrap);
                                        }
                                    });
                                })
                            })(
                                match role {
                                    Role::SidebarPanel => (
                                        Some(bevy::math::Rect::new(0.0, 0.0, next_state.layout.sidebar_w, next_state.layout.height)),
                                        false,
                                        next_state.hud_visible,
                                        Some(bevy::color::Color::srgb(12.0 / 255.0, 12.0 / 255.0, 12.0 / 255.0)),
                                        None,
                                        None,
                                    ),
                                    Role::Divider => (
                                        Some(bevy::math::Rect::new(next_state.layout.sidebar_w - 2.0, 0.0, next_state.layout.sidebar_w, next_state.layout.height)),
                                        false,
                                        next_state.hud_visible,
                                        Some(bevy::color::Color::srgb(56.0 / 255.0, 56.0 / 255.0, 56.0 / 255.0)),
                                        None,
                                        None,
                                    ),
                                    Role::AppIcon => (
                                        Some(next_state.layout.icon),
                                        false,
                                        next_state.hud_visible,
                                        Some(bevy::color::Color::WHITE),
                                        None,
                                        None,
                                    ),
                                    Role::AppTitle => (
                                        Some(next_state.layout.title),
                                        true,
                                        next_state.hud_visible,
                                        None,
                                        Some(bevy::color::Color::srgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0)),
                                        Some(next_state.layout.title.size().x),
                                    ),
                                    Role::TabBody(control) => (
                                        Some(match *control {
                                            Control::HomeTab => next_state.layout.home_tab,
                                            Control::RegistryTab => next_state.layout.registry_tab,
                                            Control::Launch => next_state.layout.launch,
                                        }),
                                        false,
                                        next_state.hud_visible,
                                        Some((|gray: u8| {
                                            bevy::color::Color::srgb(gray as f32 / 255.0, gray as f32 / 255.0, gray as f32 / 255.0)
                                        })(match (
                                            next_state.selected == match *control {
                                                Control::HomeTab => Destination::Home,
                                                Control::RegistryTab => Destination::Registry,
                                                Control::Launch => Destination::Home,
                                            },
                                            next_state.armed == Some(*control),
                                            next_state.hovered == Some(*control),
                                        ) {
                                            (true, true, _) => 215,
                                            (true, _, true) => 245,
                                            (true, _, _) => 235,
                                            (false, true, _) => 20,
                                            (false, _, true) => 44,
                                            (false, _, _) => 28,
                                        })),
                                        None,
                                        None,
                                    ),
                                    Role::TabIcon(control) => (
                                        Some((|tab: bevy::math::Rect| {
                                            bevy::math::Rect::new(tab.min.x + 14.0, tab.min.y + 16.0, tab.min.x + 38.0, tab.min.y + 40.0)
                                        })(match *control {
                                            Control::HomeTab => next_state.layout.home_tab,
                                            Control::RegistryTab => next_state.layout.registry_tab,
                                            Control::Launch => next_state.layout.launch,
                                        })),
                                        false,
                                        next_state.hud_visible,
                                        Some((|gray: u8| {
                                            bevy::color::Color::srgb(gray as f32 / 255.0, gray as f32 / 255.0, gray as f32 / 255.0)
                                        })([
                                            225u8,
                                            20u8,
                                        ][(next_state.selected == match *control {
                                            Control::HomeTab => Destination::Home,
                                            Control::RegistryTab => Destination::Registry,
                                            Control::Launch => Destination::Home,
                                        }) as usize])),
                                        None,
                                        None,
                                    ),
                                    Role::TabLabel(control) => (
                                        Some((|tab: bevy::math::Rect| {
                                            bevy::math::Rect::new(tab.min.x + 50.0, tab.min.y + 28.0, tab.max.x - 10.0, tab.min.y + 52.0)
                                        })(match *control {
                                            Control::HomeTab => next_state.layout.home_tab,
                                            Control::RegistryTab => next_state.layout.registry_tab,
                                            Control::Launch => next_state.layout.launch,
                                        })),
                                        true,
                                        next_state.hud_visible,
                                        None,
                                        Some((|gray: u8| {
                                            bevy::color::Color::srgb(gray as f32 / 255.0, gray as f32 / 255.0, gray as f32 / 255.0)
                                        })([
                                            225u8,
                                            20u8,
                                        ][(next_state.selected == match *control {
                                            Control::HomeTab => Destination::Home,
                                            Control::RegistryTab => Destination::Registry,
                                            Control::Launch => Destination::Home,
                                        }) as usize])),
                                        Some((|tab: bevy::math::Rect| {
                                            tab.size().x - 60.0
                                        })(match *control {
                                            Control::HomeTab => next_state.layout.home_tab,
                                            Control::RegistryTab => next_state.layout.registry_tab,
                                            Control::Launch => next_state.layout.launch,
                                        })),
                                    ),
                                    Role::TabRing(control) => (
                                        Some((|tab: bevy::math::Rect| {
                                            bevy::math::Rect::new(tab.min.x + 3.0, tab.min.y + 3.0, tab.max.x - 3.0, tab.max.y - 3.0)
                                        })(match *control {
                                            Control::HomeTab => next_state.layout.home_tab,
                                            Control::RegistryTab => next_state.layout.registry_tab,
                                            Control::Launch => next_state.layout.launch,
                                        })),
                                        false,
                                        next_state.hud_visible
                                            && next_state.focus_keyboard
                                            && next_state.focused == Some(*control),
                                        Some((|gray: u8| {
                                            bevy::color::Color::srgb(gray as f32 / 255.0, gray as f32 / 255.0, gray as f32 / 255.0)
                                        })([
                                            250u8,
                                            15u8,
                                        ][(next_state.selected == match *control {
                                            Control::HomeTab => Destination::Home,
                                            Control::RegistryTab => Destination::Registry,
                                            Control::Launch => Destination::Home,
                                        }) as usize])),
                                        None,
                                        None,
                                    ),
                                    Role::RegistryRoot => (
                                        None,
                                        false,
                                        next_state.hud_visible
                                            && next_state.selected == Destination::Registry,
                                        None,
                                        None,
                                        None,
                                    ),
                                    Role::CardBackground => (
                                        Some(next_state.layout.card),
                                        false,
                                        true,
                                        Some(bevy::color::Color::srgb(18.0 / 255.0, 18.0 / 255.0, 18.0 / 255.0)),
                                        None,
                                        None,
                                    ),
                                    Role::CardIcon => (
                                        Some(next_state.layout.card_icon),
                                        false,
                                        true,
                                        Some(bevy::color::Color::WHITE),
                                        None,
                                        None,
                                    ),
                                    Role::CardTitle => (
                                        Some(next_state.layout.card_title),
                                        true,
                                        true,
                                        None,
                                        Some(bevy::color::Color::srgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0)),
                                        Some(next_state.layout.card_title.size().x),
                                    ),
                                    Role::CardDescription => (
                                        Some(next_state.layout.desc),
                                        true,
                                        !next_state.launched,
                                        None,
                                        Some(bevy::color::Color::srgb(170.0 / 255.0, 170.0 / 255.0, 170.0 / 255.0)),
                                        Some(next_state.layout.desc.size().x),
                                    ),
                                    Role::CardSeparator => (
                                        Some(next_state.layout.sep),
                                        false,
                                        !next_state.launched,
                                        Some(bevy::color::Color::srgb(56.0 / 255.0, 56.0 / 255.0, 56.0 / 255.0)),
                                        None,
                                        None,
                                    ),
                                    Role::LaunchBody => (
                                        Some(next_state.layout.launch),
                                        false,
                                        !next_state.launched,
                                        Some((|gray: u8| {
                                            bevy::color::Color::srgb(gray as f32 / 255.0, gray as f32 / 255.0, gray as f32 / 255.0)
                                        })(match (
                                            next_state.armed == Some(Control::Launch),
                                            next_state.hovered == Some(Control::Launch),
                                        ) {
                                            (true, _) => 215,
                                            (_, true) => 250,
                                            (_, _) => 235,
                                        })),
                                        None,
                                        None,
                                    ),
                                    Role::LaunchIcon => (
                                        Some(bevy::math::Rect::new(next_state.layout.launch.min.x + 16.0, next_state.layout.launch.min.y + 16.0, next_state.layout.launch.min.x + 40.0, next_state.layout.launch.min.y + 40.0)),
                                        false,
                                        !next_state.launched,
                                        Some(bevy::color::Color::srgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0)),
                                        None,
                                        None,
                                    ),
                                    Role::LaunchLabel => (
                                        Some(bevy::math::Rect::new(next_state.layout.launch.min.x + 52.0, next_state.layout.launch.min.y + 28.0, next_state.layout.launch.max.x - 12.0, next_state.layout.launch.min.y + 52.0)),
                                        true,
                                        !next_state.launched,
                                        None,
                                        Some(bevy::color::Color::srgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0)),
                                        Some(next_state.layout.launch.size().x - 64.0),
                                    ),
                                    Role::Badge => (
                                        Some(next_state.layout.badge),
                                        false,
                                        true,
                                        Some(bevy::color::Color::srgb(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0)),
                                        None,
                                        None,
                                    ),
                                    Role::BadgeLabel => (
                                        Some(bevy::math::Rect::new((next_state.layout.badge.min.x + next_state.layout.badge.max.x) / 2.0, (next_state.layout.badge.min.y + next_state.layout.badge.max.y) / 2.0, (next_state.layout.badge.min.x + next_state.layout.badge.max.x) / 2.0, (next_state.layout.badge.min.y + next_state.layout.badge.max.y) / 2.0)),
                                        true,
                                        true,
                                        None,
                                        Some(bevy::color::Color::srgb(160.0 / 255.0, 160.0 / 255.0, 160.0 / 255.0)),
                                        Some(next_state.layout.badge.size().x - 16.0),
                                    ),
                                    Role::Well => (
                                        Some(next_state.layout.well),
                                        false,
                                        next_state.launched,
                                        Some(bevy::color::Color::srgb(10.0 / 255.0, 10.0 / 255.0, 10.0 / 255.0)),
                                        None,
                                        None,
                                    ),
                                    Role::Rocket => (
                                        Some((|well: bevy::math::Rect| {
                                            bevy::math::Rect::new(well.min.x + (well.size().x - 96.0) / 2.0, well.min.y + (well.size().y - 96.0) / 2.0, well.min.x + (well.size().x + 96.0) / 2.0, well.min.y + (well.size().y + 96.0) / 2.0)
                                        })(next_state.layout.well)),
                                        false,
                                        next_state.launched,
                                        Some(bevy::color::Color::WHITE),
                                        None,
                                        None,
                                    ),
                                },
                            );
                        }
                    })
                })(windows.iter().fold(
                    (0.0, 0.0, 1.0, false, None),
                    |_: (f32, f32, f32, bool, Option<bevy::math::Vec2>),
                     window: &bevy::window::Window| {
                        (
                            window.width(),
                            window.height(),
                            window.scale_factor(),
                            window.focused,
                            window.cursor_position(),
                        )
                    },
                ))
            },
        )
        .run();
}
