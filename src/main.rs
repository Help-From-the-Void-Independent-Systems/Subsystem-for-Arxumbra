fn main() {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Destination {
        Home,
        ExtensionRegistry,
    }

    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    struct TabFrame {
        center_x: f32,
        center_y: f32,
        half_width: f32,
        half_height: f32,
    }

    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    struct CardFrame {
        center_x: f32,
        center_y: f32,
        half_width: f32,
        half_height: f32,
    }

    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    struct LayoutFrame {
        width: f32,
        height: f32,
        scale_factor: f32,
        margin: f32,
        sidebar_width: f32,
        tab_width: f32,
        tab_icon_x: f32,
        tab_label_x: f32,
        tab_label_bounds: f32,
        title_x: f32,
        title_y: f32,
        title_bounds: f32,
        home_tab: TabFrame,
        registry_tab: TabFrame,
        content: CardFrame,
        laboratory: CardFrame,
        header_x: f32,
        header_y: f32,
        laboratory_title_x: f32,
        laboratory_title_bounds: f32,
        separator_y: f32,
        caption_x: f32,
        caption_y: f32,
        caption_bounds: f32,
        action: CardFrame,
        action_icon_x: f32,
        action_label_x: f32,
        action_label_bounds: f32,
    }

    #[derive(bevy::ecs::resource::Resource)]
    struct UiState {
        selected: Destination,
        hud_visible: bool,
        hover: Option<Destination>,
        focus: Option<Destination>,
        focus_from_keyboard: bool,
        armed: Option<Destination>,
        escape_hold_secs: f32,
        exit_requested: bool,
        cursor: Option<bevy::math::Vec2>,
        layout: LayoutFrame,
    }

    #[derive(bevy::ecs::resource::Resource)]
    struct UiImages {
        pill: bevy::asset::Handle<bevy::image::Image>,
        panel: bevy::asset::Handle<bevy::image::Image>,
        ring: bevy::asset::Handle<bevy::image::Image>,
    }

    #[derive(bevy::ecs::component::Component)]
    struct SidebarPanel;

    #[derive(bevy::ecs::component::Component)]
    struct Divider;

    #[derive(bevy::ecs::component::Component)]
    struct AppIcon;

    #[derive(bevy::ecs::component::Component)]
    struct Title;

    #[derive(bevy::ecs::component::Component)]
    struct HomePill;

    #[derive(bevy::ecs::component::Component)]
    struct HomeIcon;

    #[derive(bevy::ecs::component::Component)]
    struct HomeLabel;

    #[derive(bevy::ecs::component::Component)]
    struct RegistryPill;

    #[derive(bevy::ecs::component::Component)]
    struct RegistryIcon;

    #[derive(bevy::ecs::component::Component)]
    struct RegistryLabel;

    #[derive(bevy::ecs::component::Component)]
    struct RingHome;

    #[derive(bevy::ecs::component::Component)]
    struct RingRegistry;

    #[derive(bevy::ecs::component::Component)]
    struct ContentPanel;

    #[derive(bevy::ecs::component::Component)]
    struct LaboratoryCard;

    #[derive(bevy::ecs::component::Component)]
    struct LaboratoryWell;

    #[derive(bevy::ecs::component::Component)]
    struct Flask;

    #[derive(bevy::ecs::component::Component)]
    struct LaboratoryTitle;

    #[derive(bevy::ecs::component::Component)]
    struct Separator;

    #[derive(bevy::ecs::component::Component)]
    struct Caption;

    #[derive(bevy::ecs::component::Component)]
    struct ActionPill;

    #[derive(bevy::ecs::component::Component)]
    struct ActionIcon;

    #[derive(bevy::ecs::component::Component)]
    struct ActionLabel;

    type SpriteSlot<'a, 'w, 's, M> = bevy::ecs::system::Query<
        'w,
        's,
        (
            bevy::ecs::entity::Entity,
            &'a bevy::transform::components::Transform,
            &'a bevy::camera::visibility::Visibility,
            &'a bevy::sprite::Sprite,
        ),
        bevy::ecs::query::With<M>,
    >;

    type TextSlot<'a, 'w, 's, M> = bevy::ecs::system::Query<
        'w,
        's,
        (
            bevy::ecs::entity::Entity,
            &'a bevy::transform::components::Transform,
            &'a bevy::camera::visibility::Visibility,
            &'a bevy::text::TextColor,
            &'a bevy::text::TextBounds,
        ),
        bevy::ecs::query::With<M>,
    >;

    bevy::app::App::new()
        .insert_resource(UiState {
            selected: Destination::Home,
            hud_visible: true,
            hover: None,
            focus: None,
            focus_from_keyboard: false,
            armed: None,
            escape_hold_secs: 0.0,
            exit_requested: false,
            cursor: None,
            layout: LayoutFrame::default(),
        })
        .insert_resource(bevy::camera::ClearColor(bevy::color::Color::srgb(0.03, 0.03, 0.03)))
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
            bevy::log::LogPlugin::default(),
            bevy::window::WindowPlugin {
                primary_window: Some(bevy::window::Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::BorderlessFullscreen(
                        bevy::window::MonitorSelection::Primary,
                    ),
                    resolution: bevy::window::WindowResolution::new(1920, 1080),
                    title: std::string::String::from("Arxumbra"),
                    resizable: false,
                    decorations: false,
                    focused: true,
                    visible: true,
                    skip_taskbar: true,
                    borderless_game: true,
                    ..bevy::window::Window::default()
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
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..bevy::asset::AssetPlugin::default()
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
                        limits: bevy::render::settings::WgpuLimits {
                            max_texture_dimension_2d: 16384,
                            ..bevy::render::settings::WgpuLimits::default()
                        },
                        dx12_shader_compiler: bevy::render::settings::Dx12Compiler::Fxc,
                        instance_flags:
                            bevy::render::settings::InstanceFlags::VALIDATION_INDIRECT_CALL,
                        memory_hints: bevy::render::settings::MemoryHints::Performance,
                        instance_memory_budget_thresholds: std::default::Default::default(),
                        ..bevy::render::settings::WgpuSettings::default()
                    }),
                ),
                synchronous_pipeline_compilation: false,
                debug_flags: bevy::render::RenderDebugFlags::default(),
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
            bevy::ecs::schedule::IntoScheduleConfigs::chain((
                |mut commands: bevy::ecs::system::Commands, server: bevy::ecs::system::Res<bevy::asset::AssetServer>| {
                    commands.spawn((bevy::camera::Camera2d, bevy::render::view::Msaa::Off));
                    commands.insert_resource(UiImages {
                        pill: server.add(bevy::image::Image {
                            data: Some(
                                (0..4096u32)
                                    .flat_map(|index: u32| {
                                        (|x: f32, y: f32| {
                                            (|offset_x: f32, offset_y: f32| {
                                                (|distance: f32| {
                                                    [
                                                        255u8,
                                                        255,
                                                        255,
                                                        (f32::clamp(
                                                            0.5 - distance,
                                                            0.0,
                                                            1.0,
                                                        ) * 255.0)
                                                            as u8,
                                                    ]
                                                })(f32::hypot(
                                                    f32::max(offset_x, 0.0),
                                                    f32::max(offset_y, 0.0),
                                                ) - 16.0)
                                            })(
                                                f32::abs(x - 32.0) - 16.0,
                                                f32::abs(y - 32.0) - 16.0,
                                            )
                                        })(
                                            (index % 64) as f32 + 0.5,
                                            (index / 64) as f32 + 0.5,
                                        )
                                    })
                                    .collect::<std::vec::Vec<u8>>(),
                            ),
                            data_order: bevy::render::render_resource::TextureDataOrder::LayerMajor,
                            texture_descriptor:
                                bevy::render::render_resource::TextureDescriptor {
                                    label: Some("arxumbra-pill"),
                                    size: bevy::render::render_resource::Extent3d {
                                        width: 64,
                                        height: 64,
                                        depth_or_array_layers: 1,
                                    },
                                    mip_level_count: 1,
                                    sample_count: 1,
                                    dimension: bevy::render::render_resource::TextureDimension::D2,
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
                        panel: server.add(bevy::image::Image {
                            data: Some(
                                (0..9216u32)
                                    .flat_map(|index: u32| {
                                        (|x: f32, y: f32| {
                                            (|offset_x: f32, offset_y: f32| {
                                                (|distance: f32| {
                                                    [
                                                        255u8,
                                                        255,
                                                        255,
                                                        (f32::clamp(
                                                            0.5 - distance,
                                                            0.0,
                                                            1.0,
                                                        ) * 255.0)
                                                            as u8,
                                                    ]
                                                })(f32::hypot(
                                                    f32::max(offset_x, 0.0),
                                                    f32::max(offset_y, 0.0),
                                                ) - 24.0)
                                            })(
                                                f32::abs(x - 48.0) - 24.0,
                                                f32::abs(y - 48.0) - 24.0,
                                            )
                                        })(
                                            (index % 96) as f32 + 0.5,
                                            (index / 96) as f32 + 0.5,
                                        )
                                    })
                                    .collect::<std::vec::Vec<u8>>(),
                            ),
                            data_order: bevy::render::render_resource::TextureDataOrder::LayerMajor,
                            texture_descriptor:
                                bevy::render::render_resource::TextureDescriptor {
                                    label: Some("arxumbra-panel"),
                                    size: bevy::render::render_resource::Extent3d {
                                        width: 96,
                                        height: 96,
                                        depth_or_array_layers: 1,
                                    },
                                    mip_level_count: 1,
                                    sample_count: 1,
                                    dimension: bevy::render::render_resource::TextureDimension::D2,
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
                        ring: server.add(bevy::image::Image {
                            data: Some(
                                (0..4096u32)
                                    .flat_map(|index: u32| {
                                        (|x: f32, y: f32| {
                                            (|offset_x: f32, offset_y: f32| {
                                                (|distance: f32| {
                                                    [
                                                        255u8,
                                                        255,
                                                        255,
                                                        (f32::clamp(
                                                            2.0 - f32::abs(distance),
                                                            0.0,
                                                            1.0,
                                                        ) * 255.0)
                                                            as u8,
                                                    ]
                                                })(f32::hypot(
                                                    f32::max(offset_x, 0.0),
                                                    f32::max(offset_y, 0.0),
                                                ) - 16.0)
                                            })(
                                                f32::abs(x - 32.0) - 16.0,
                                                f32::abs(y - 32.0) - 16.0,
                                            )
                                        })(
                                            (index % 64) as f32 + 0.5,
                                            (index / 64) as f32 + 0.5,
                                        )
                                    })
                                    .collect::<std::vec::Vec<u8>>(),
                            ),
                            data_order: bevy::render::render_resource::TextureDataOrder::LayerMajor,
                            texture_descriptor:
                                bevy::render::render_resource::TextureDescriptor {
                                    label: Some("arxumbra-ring"),
                                    size: bevy::render::render_resource::Extent3d {
                                        width: 64,
                                        height: 64,
                                        depth_or_array_layers: 1,
                                    },
                                    mip_level_count: 1,
                                    sample_count: 1,
                                    dimension: bevy::render::render_resource::TextureDimension::D2,
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
                    });
                },
                bevy::ecs::schedule::ApplyDeferred,
                |mut commands: bevy::ecs::system::Commands, images: bevy::ecs::system::Res<UiImages>, server: bevy::ecs::system::Res<bevy::asset::AssetServer>| {
                    commands.spawn((
                        SidebarPanel,
                            bevy::sprite::Sprite {
                                image: images.panel.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.07, 0.07, 0.07),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::ZERO),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(34.0, 34.0),
                                            max_inset: bevy::math::Vec2::new(34.0, 34.0),
                                        },
                                        center_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        Divider,
                            bevy::sprite::Sprite {
                                image: bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::image::Image>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                ),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.16, 0.16, 0.16),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::ZERO),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Opaque,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        AppIcon,
                            bevy::sprite::Sprite {
                                image: server
                                    .load::<bevy::image::Image>("app-icon.png"),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.92, 0.92, 0.92),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(28.0, 28.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        Title,
                            bevy::sprite::Text2d(std::string::String::from(
                                [
                                    "Linux-Subsystem-for-Arxumbra",
                                    "macOS-Subsystem-for-Arxumbra",
                                    "Windows-Subsystem-for-Arxumbra",
                                ][(cfg!(target_os = "windows") as usize) * 2
                                    + (cfg!(target_os = "macos") as usize)],
                            )),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(
                                    bevy::asset::Handle::Uuid(
                                        bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                        core::marker::PhantomData,
                                    ),
                                ),
                                font_size: bevy::text::FontSize::Px(24.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(0.92, 0.92, 0.92)),
                            bevy::text::TextBounds::default(),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        HomePill,
                            bevy::sprite::Sprite {
                                image: images.pill.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.12, 0.12, 0.12),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::ZERO),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(32.0, 32.0),
                                            max_inset: bevy::math::Vec2::new(32.0, 32.0),
                                        },
                                        center_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        HomeIcon,
                            bevy::sprite::Sprite {
                                image: server
                                    .load::<bevy::image::Image>("icons/house.png"),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.88, 0.88, 0.88),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(26.0, 22.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        HomeLabel,
                            bevy::sprite::Text2d(std::string::String::from("Home")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(
                                    bevy::asset::Handle::Uuid(
                                        bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                        core::marker::PhantomData,
                                    ),
                                ),
                                font_size: bevy::text::FontSize::Px(24.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(0.88, 0.88, 0.88)),
                            bevy::text::TextBounds::default(),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        RegistryPill,
                            bevy::sprite::Sprite {
                                image: images.pill.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.12, 0.12, 0.12),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::ZERO),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(32.0, 32.0),
                                            max_inset: bevy::math::Vec2::new(32.0, 32.0),
                                        },
                                        center_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        RegistryIcon,
                            bevy::sprite::Sprite {
                                image: server
                                    .load::<bevy::image::Image>("icons/registry.png"),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.88, 0.88, 0.88),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(26.0, 22.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        RegistryLabel,
                            bevy::sprite::Text2d(std::string::String::from(
                                "Extension Registry",
                            )),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(
                                    bevy::asset::Handle::Uuid(
                                        bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                        core::marker::PhantomData,
                                    ),
                                ),
                                font_size: bevy::text::FontSize::Px(24.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(0.88, 0.88, 0.88)),
                            bevy::text::TextBounds::default(),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        RingHome,
                            bevy::sprite::Sprite {
                                image: images.ring.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(1.0, 1.0, 1.0),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::ZERO),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(26.0, 26.0),
                                            max_inset: bevy::math::Vec2::new(26.0, 26.0),
                                        },
                                        center_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        RingRegistry,
                            bevy::sprite::Sprite {
                                image: images.ring.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(1.0, 1.0, 1.0),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::ZERO),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(26.0, 26.0),
                                            max_inset: bevy::math::Vec2::new(26.0, 26.0),
                                        },
                                        center_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        ContentPanel,
                            bevy::sprite::Sprite {
                                image: images.panel.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.09, 0.09, 0.09),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::ZERO),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(34.0, 34.0),
                                            max_inset: bevy::math::Vec2::new(34.0, 34.0),
                                        },
                                        center_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        LaboratoryCard,
                            bevy::sprite::Sprite {
                                image: images.panel.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.13, 0.13, 0.13),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::ZERO),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(34.0, 34.0),
                                            max_inset: bevy::math::Vec2::new(34.0, 34.0),
                                        },
                                        center_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        LaboratoryWell,
                            bevy::sprite::Sprite {
                                image: images.panel.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.18, 0.18, 0.18),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(56.0, 56.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(34.0, 34.0),
                                            max_inset: bevy::math::Vec2::new(34.0, 34.0),
                                        },
                                        center_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        Flask,
                            bevy::sprite::Sprite {
                                image: server
                                    .load::<bevy::image::Image>("icons/flask.png"),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.92, 0.92, 0.92),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(48.0, 40.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        LaboratoryTitle,
                            bevy::sprite::Text2d(std::string::String::from("Laboratory")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(
                                    bevy::asset::Handle::Uuid(
                                        bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                        core::marker::PhantomData,
                                    ),
                                ),
                                font_size: bevy::text::FontSize::Px(30.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(0.92, 0.92, 0.92)),
                            bevy::text::TextBounds::default(),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        Separator,
                            bevy::sprite::Sprite {
                                image: bevy::asset::Handle::Uuid(
                                    bevy::asset::AssetId::<bevy::image::Image>::DEFAULT_UUID,
                                    core::marker::PhantomData,
                                ),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.22, 0.22, 0.22),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::ZERO),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Opaque,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        Caption,
                            bevy::sprite::Text2d(std::string::String::from("Coming Soon")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(
                                    bevy::asset::Handle::Uuid(
                                        bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                        core::marker::PhantomData,
                                    ),
                                ),
                                font_size: bevy::text::FontSize::Px(20.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(0.58, 0.58, 0.58)),
                            bevy::text::TextBounds::default(),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        ActionPill,
                            bevy::sprite::Sprite {
                                image: images.pill.clone(),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.10, 0.10, 0.10),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::ZERO),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                    bevy::sprite::TextureSlicer {
                                        border: bevy::sprite::BorderRect {
                                            min_inset: bevy::math::Vec2::new(32.0, 32.0),
                                            max_inset: bevy::math::Vec2::new(32.0, 32.0),
                                        },
                                        center_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        sides_scale_mode:
                                            bevy::sprite::SliceScaleMode::Stretch,
                                        max_corner_scale: 1.0,
                                    },
                                ),
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        ActionIcon,
                            bevy::sprite::Sprite {
                                image: server
                                    .load::<bevy::image::Image>("icons/rocket.png"),
                                texture_atlas: None,
                                color: bevy::color::Color::srgb(0.38, 0.38, 0.38),
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(26.0, 26.0)),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Blend,
                            },
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                    commands.spawn((
                        ActionLabel,
                            bevy::sprite::Text2d(std::string::String::from("Launch")),
                            bevy::text::TextFont {
                                font: bevy::text::FontSource::Handle(
                                    bevy::asset::Handle::Uuid(
                                        bevy::asset::AssetId::<bevy::text::Font>::DEFAULT_UUID,
                                        core::marker::PhantomData,
                                    ),
                                ),
                                font_size: bevy::text::FontSize::Px(24.0),
                                weight: bevy::text::FontWeight::NORMAL,
                                width: bevy::text::FontWidth::NORMAL,
                                style: bevy::text::FontStyle::Normal,
                                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                                font_features: bevy::text::FontFeatures::builder().build(),
                                font_variations: bevy::text::FontVariations::builder().build(),
                            },
                            bevy::text::TextColor(bevy::color::Color::srgb(0.38, 0.38, 0.38)),
                            bevy::text::TextBounds::default(),
                            bevy::sprite::Anchor(bevy::math::Vec2::new(-0.5, 0.0)),
                            bevy::transform::components::Transform::IDENTITY,
                            bevy::camera::visibility::Visibility::Hidden,
                    ));
                },
            )),
        )
        .add_systems(
            bevy::app::Update,
            bevy::ecs::schedule::IntoScheduleConfigs::chain((
                |mut state: bevy::ecs::system::ResMut<UiState>,
                 keys: bevy::ecs::system::Res<bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>,
                 mouse: bevy::ecs::system::Res<bevy::input::ButtonInput<bevy::input::mouse::MouseButton>>,
                 time: bevy::ecs::system::Res<bevy::time::Time>,
                 mut commands: bevy::ecs::system::Commands,
                 windows: bevy::ecs::system::Query<(bevy::ecs::entity::Entity, &bevy::window::Window), bevy::ecs::query::With<bevy::window::PrimaryWindow>>| {
                    windows.iter().next().into_iter().for_each(|(entity, window)| {
                        if !window.focused {
                            if state.hover.is_some() {
                                state.hover = None;
                            }
                            if state.armed.is_some() {
                                state.armed = None;
                            }
                            if state.focus.is_some() {
                                state.focus = None;
                                state.focus_from_keyboard = false;
                            }
                        }
                        if keys.just_pressed(bevy::input::keyboard::KeyCode::F1) {
                            state.hud_visible = !state.hud_visible;
                            state.hover = None;
                            state.armed = None;
                            state.focus = None;
                            state.focus_from_keyboard = false;
                        }
                        (|pressed: bool, delta: f32| {
                            (|hold: f32| {
                                if state.escape_hold_secs != hold {
                                    state.escape_hold_secs = hold;
                                }
                                if hold >= 7.0 && !state.exit_requested {
                                    state.exit_requested = true;
                                    commands.entity(entity).despawn();
                                }
                            })(if keys.pressed(bevy::input::keyboard::KeyCode::Escape) {
                                state.escape_hold_secs + delta
                            } else {
                                0.0
                            });
                        })(
                            keys.pressed(bevy::input::keyboard::KeyCode::Escape),
                            time.delta_secs(),
                        );
                        (|layout: LayoutFrame| {
                            if state.layout != layout {
                                state.layout = layout;
                            }
                        })(
                            (|width: f32,
                              height: f32,
                              scale_factor: f32,
                              margin: f32,
                              sidebar_width: f32| {
                                LayoutFrame {
                                    width,
                                    height,
                                    scale_factor,
                                    margin,
                                    sidebar_width,
                                    tab_width: sidebar_width - 2.0 * margin,
                                    tab_icon_x: margin + 25.0,
                                    tab_label_x: margin + 50.0,
                                    tab_label_bounds: sidebar_width - 2.0 * margin - 62.0,
                                    title_x: margin + 40.0,
                                    title_y: height / 2.0 - margin - 14.0,
                                    title_bounds: sidebar_width - margin - 52.0,
                                    home_tab: TabFrame {
                                        center_x: margin
                                            + (sidebar_width - 2.0 * margin) / 2.0,
                                        center_y: height / 2.0 - margin - 100.0,
                                        half_width: (sidebar_width - 2.0 * margin) / 2.0,
                                        half_height: 28.0,
                                    },
                                    registry_tab: TabFrame {
                                        center_x: margin
                                            + (sidebar_width - 2.0 * margin) / 2.0,
                                        center_y: height / 2.0 - margin - 168.0,
                                        half_width: (sidebar_width - 2.0 * margin) / 2.0,
                                        half_height: 28.0,
                                    },
                                    content: CardFrame {
                                        center_x: (sidebar_width + 14.0 + width - margin)
                                            / 2.0,
                                        center_y: height / 2.0,
                                        half_width: (width - margin - sidebar_width - 14.0)
                                            / 2.0,
                                        half_height: height / 2.0 - margin,
                                    },
                                    laboratory: CardFrame {
                                        center_x: (sidebar_width + 14.0 + width - margin)
                                            / 2.0,
                                        center_y: height / 2.0,
                                        half_width: (width - margin - sidebar_width - 14.0)
                                            / 2.0
                                            - 24.0,
                                        half_height: height / 2.0 - margin - 24.0,
                                    },
                                    header_x: sidebar_width + 90.0,
                                    header_y: height - margin - 52.0,
                                    laboratory_title_x: sidebar_width + 134.0,
                                    laboratory_title_bounds: width
                                        - margin
                                        - sidebar_width
                                        - 210.0,
                                    separator_y: height - margin - 104.0,
                                    caption_x: sidebar_width + 62.0,
                                    caption_y: height - margin - 132.0,
                                    caption_bounds: width - margin - sidebar_width - 62.0,
                                    action: CardFrame {
                                        center_x: sidebar_width
                                            + 62.0
                                            + (width - margin - sidebar_width - 62.0)
                                                .min(420.0)
                                            / 2.0,
                                        center_y: margin + 80.0,
                                        half_width: (width - margin - sidebar_width - 62.0)
                                            .min(420.0)
                                            / 2.0,
                                        half_height: 32.0,
                                    },
                                    action_icon_x: sidebar_width + 99.0,
                                    action_label_x: sidebar_width + 134.0,
                                    action_label_bounds: (width
                                        - margin
                                        - sidebar_width
                                        - 62.0)
                                        .min(420.0)
                                        - 96.0,
                                }
                            })(
                                window.width(),
                                window.height(),
                                window.scale_factor(),
                                (window.width() * 0.02).clamp(12.0, 24.0),
                                (window.width() / 3.0).clamp(232.0, 560.0),
                            ),
                        );
                        (|cursor: Option<bevy::math::Vec2>| {
                            if state.cursor != cursor {
                                state.cursor = cursor;
                            }
                        })(window.cursor_position());
                        if window.focused && state.hud_visible {
                            state.cursor.into_iter().for_each(|point| {
                                (|point: bevy::math::Vec2| {
                                    (|hover: Option<Destination>| {
                                        if state.hover != hover {
                                            state.hover = hover;
                                        }
                                    })(if f32::abs(
                                        point.x - state.layout.home_tab.center_x,
                                    ) <= state.layout.home_tab.half_width
                                        && f32::abs(
                                            point.y - state.layout.home_tab.center_y,
                                        ) <= state.layout.home_tab.half_height
                                    {
                                        Some(Destination::Home)
                                    } else if f32::abs(
                                        point.x - state.layout.registry_tab.center_x,
                                    ) <= state.layout.registry_tab.half_width
                                        && f32::abs(
                                            point.y - state.layout.registry_tab.center_y,
                                        ) <= state.layout.registry_tab.half_height
                                    {
                                        Some(Destination::ExtensionRegistry)
                                    } else {
                                        None
                                    });
                                })(point);
                                if mouse.just_pressed(bevy::input::mouse::MouseButton::Left) {
                                    state.focus = None;
                                    state.focus_from_keyboard = false;
                                    state.armed = state.hover;
                                }
                                if mouse.just_released(bevy::input::mouse::MouseButton::Left) {
                                    state.armed.into_iter().for_each(|armed| {
                                        if state.hover == Some(armed) {
                                            state.selected = armed;
                                        }
                                        state.armed = None;
                                    });
                                }
                            });
                            if state.cursor.is_none() {
                                if state.hover.is_some() {
                                    state.hover = None;
                                }
                                if state.armed.is_some() {
                                    state.armed = None;
                                }
                            }
                        }
                        if keys.just_pressed(bevy::input::keyboard::KeyCode::Tab) {
                            state.focus_from_keyboard = true;
                            state.focus = Some(match state.focus {
                                Some(Destination::ExtensionRegistry) => Destination::Home,
                                Some(Destination::Home) | None => {
                                    Destination::ExtensionRegistry
                                }
                            });
                        }
                        if keys.just_pressed(bevy::input::keyboard::KeyCode::Enter) || keys.just_pressed(bevy::input::keyboard::KeyCode::Space)
                        {
                            state.focus.into_iter().for_each(|destination| {
                                state.selected = destination;
                            });
                        }
                    });
                },
                |state: bevy::ecs::system::Res<UiState>,
                 mut commands: bevy::ecs::system::Commands,
                 sidebar_panel_sprites: SpriteSlot<'_, '_, '_, SidebarPanel>,
                 divider_sprites: SpriteSlot<'_, '_, '_, Divider>,
                 app_icon_sprites: SpriteSlot<'_, '_, '_, AppIcon>,
                 title_texts: TextSlot<'_, '_, '_, Title>,
                 home_pill_sprites: SpriteSlot<'_, '_, '_, HomePill>,
                 home_icon_sprites: SpriteSlot<'_, '_, '_, HomeIcon>,
                 home_label_texts: TextSlot<'_, '_, '_, HomeLabel>,
                 registry_pill_sprites: SpriteSlot<'_, '_, '_, RegistryPill>,
                 registry_icon_sprites: SpriteSlot<'_, '_, '_, RegistryIcon>,
                 registry_label_texts: TextSlot<'_, '_, '_, RegistryLabel>,
                 ring_home_sprites: SpriteSlot<'_, '_, '_, RingHome>| {
                    sidebar_panel_sprites
                        .iter()
                        .for_each(|(entity, transform, visibility, sprite)| {
                            (|desired_visibility, desired_size, desired_translation| {
                                if *visibility != desired_visibility {
                                    commands.entity(entity).insert(desired_visibility);
                                }
                                if sprite.custom_size != Some(desired_size) {
                                    commands.entity(entity).insert(bevy::sprite::Sprite {
                                        custom_size: Some(desired_size),
                                        ..sprite.clone()
                                    });
                                }
                                if transform.translation != desired_translation
                                    || transform.scale != bevy::math::Vec3::ONE
                                {
                                    commands.entity(entity).insert(
                                        bevy::transform::components::Transform {
                                            translation: desired_translation,
                                            rotation: bevy::math::Quat::IDENTITY,
                                            scale: bevy::math::Vec3::ONE,
                                        },
                                    );
                                }
                            })(
                                if state.hud_visible {
                                    bevy::camera::visibility::Visibility::Visible
                                } else {
                                    bevy::camera::visibility::Visibility::Hidden
                                },
                                bevy::math::Vec2::new(
                                    state.layout.sidebar_width,
                                    state.layout.height,
                                ),
                                bevy::math::Vec3::new(
                                    state.layout.sidebar_width / 2.0,
                                    0.0,
                                    0.0,
                                ),
                            );
                        });
                    divider_sprites
                        .iter()
                        .for_each(|(entity, transform, visibility, sprite)| {
                            (|desired_visibility, desired_size, desired_translation| {
                                if *visibility != desired_visibility {
                                    commands.entity(entity).insert(desired_visibility);
                                }
                                if sprite.custom_size != Some(desired_size) {
                                    commands.entity(entity).insert(bevy::sprite::Sprite {
                                        custom_size: Some(desired_size),
                                        ..sprite.clone()
                                    });
                                }
                                if transform.translation != desired_translation
                                    || transform.scale != bevy::math::Vec3::ONE
                                {
                                    commands.entity(entity).insert(
                                        bevy::transform::components::Transform {
                                            translation: desired_translation,
                                            rotation: bevy::math::Quat::IDENTITY,
                                            scale: bevy::math::Vec3::ONE,
                                        },
                                    );
                                }
                            })(
                                if state.hud_visible {
                                    bevy::camera::visibility::Visibility::Visible
                                } else {
                                    bevy::camera::visibility::Visibility::Hidden
                                },
                                bevy::math::Vec2::new(2.0, state.layout.height),
                                bevy::math::Vec3::new(
                                    state.layout.sidebar_width + 1.0,
                                    0.0,
                                    1.0,
                                ),
                            );
                        });
                    app_icon_sprites
                        .iter()
                        .for_each(|(entity, transform, visibility, sprite)| {
                            (|desired_visibility, desired_translation| {
                                if *visibility != desired_visibility {
                                    commands.entity(entity).insert(desired_visibility);
                                }
                                if transform.translation != desired_translation
                                    || transform.scale != bevy::math::Vec3::ONE
                                {
                                    commands.entity(entity).insert(
                                        bevy::transform::components::Transform {
                                            translation: desired_translation,
                                            rotation: bevy::math::Quat::IDENTITY,
                                            scale: bevy::math::Vec3::ONE,
                                        },
                                    );
                                }
                            })(
                                if state.hud_visible {
                                    bevy::camera::visibility::Visibility::Visible
                                } else {
                                    bevy::camera::visibility::Visibility::Hidden
                                },
                                bevy::math::Vec3::new(
                                    state.layout.margin + 14.0,
                                    state.layout.title_y,
                                    1.0,
                                ),
                            );
                        });
                    title_texts
                        .iter()
                        .for_each(|(entity, transform, visibility, color, bounds)| {
                            (|desired_visibility: bevy::camera::visibility::Visibility,
  desired_bounds: bevy::text::TextBounds,
  desired_translation: bevy::math::Vec3| {
                                if *visibility != desired_visibility {
                                    commands.entity(entity).insert(desired_visibility);
                                }
                                if bounds.width != desired_bounds.width || bounds.height != desired_bounds.height {
                                    commands.entity(entity).insert(desired_bounds);
                                }
                                if transform.translation != desired_translation
                                    || transform.scale != bevy::math::Vec3::ONE
                                {
                                    commands.entity(entity).insert(
                                        bevy::transform::components::Transform {
                                            translation: desired_translation,
                                            rotation: bevy::math::Quat::IDENTITY,
                                            scale: bevy::math::Vec3::ONE,
                                        },
                                    );
                                }
                            })(
                                if state.hud_visible {
                                    bevy::camera::visibility::Visibility::Visible
                                } else {
                                    bevy::camera::visibility::Visibility::Hidden
                                },
                                bevy::text::TextBounds::new_horizontal(state.layout.title_bounds),
                                bevy::math::Vec3::new(
                                    state.layout.title_x,
                                    state.layout.title_y,
                                    7.0,
                                ),
                            );
                            if color.0 != bevy::color::Color::srgb(0.92, 0.92, 0.92) {
                                commands
                                    .entity(entity)
                                    .insert(bevy::text::TextColor(bevy::color::Color::srgb(
                                        0.92, 0.92, 0.92,
                                    )));
                            }
                        });
                    (|home_selected: bool, home_hovered: bool, home_pressed: bool| {
                        home_pill_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_color, desired_size, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if sprite.color != desired_color
                                        || sprite.custom_size != Some(desired_size)
                                    {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            color: desired_color,
                                            custom_size: Some(desired_size),
                                            ..sprite.clone()
                                        });
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if state.hud_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::color::Color::srgb(
                                        if home_selected {
                                            if home_pressed {
                                                1.0
                                            } else if home_hovered {
                                                0.96
                                            } else {
                                                0.92
                                            }
                                        } else if home_pressed {
                                            0.26
                                        } else if home_hovered {
                                            0.18
                                        } else {
                                            0.12
                                        },
                                        if home_selected {
                                            if home_pressed {
                                                1.0
                                            } else if home_hovered {
                                                0.96
                                            } else {
                                                0.92
                                            }
                                        } else if home_pressed {
                                            0.26
                                        } else if home_hovered {
                                            0.18
                                        } else {
                                            0.12
                                        },
                                        if home_selected {
                                            if home_pressed {
                                                1.0
                                            } else if home_hovered {
                                                0.96
                                            } else {
                                                0.92
                                            }
                                        } else if home_pressed {
                                            0.26
                                        } else if home_hovered {
                                            0.18
                                        } else {
                                            0.12
                                        },
                                    ),
                                    bevy::math::Vec2::new(
                                        state.layout.tab_width,
                                        56.0,
                                    ),
                                    bevy::math::Vec3::new(
                                        state.layout.home_tab.center_x,
                                        state.layout.home_tab.center_y,
                                        5.0,
                                    ),
                                );
                            });
                        home_icon_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_color, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if sprite.color != desired_color {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            color: desired_color,
                                            ..sprite.clone()
                                        });
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if state.hud_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::color::Color::srgb(
                                        if home_selected {
                                            0.05
                                        } else if home_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                        if home_selected {
                                            0.05
                                        } else if home_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                        if home_selected {
                                            0.05
                                        } else if home_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                    ),
                                    bevy::math::Vec3::new(
                                        state.layout.tab_icon_x,
                                        state.layout.home_tab.center_y,
                                        6.0,
                                    ),
                                );
                            });
                        home_label_texts
                            .iter()
                            .for_each(|(entity, transform, visibility, color, bounds)| {
                                (|desired_visibility: bevy::camera::visibility::Visibility,
  desired_color: bevy::color::Color,
  desired_bounds: bevy::text::TextBounds,
  desired_translation: bevy::math::Vec3| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if color.0 != desired_color {
                                        commands
                                            .entity(entity)
                                            .insert(bevy::text::TextColor(desired_color));
                                    }
                                    if bounds.width != desired_bounds.width || bounds.height != desired_bounds.height {
                                        commands.entity(entity).insert(desired_bounds);
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if state.hud_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::color::Color::srgb(
                                        if home_selected {
                                            0.05
                                        } else if home_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                        if home_selected {
                                            0.05
                                        } else if home_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                        if home_selected {
                                            0.05
                                        } else if home_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                    ),
                                    bevy::text::TextBounds::new_horizontal(state.layout.tab_label_bounds),
                                    bevy::math::Vec3::new(
                                        state.layout.tab_label_x,
                                        state.layout.home_tab.center_y,
                                        7.0,
                                    ),
                                );
                            });
                    })(
                        state.selected == Destination::Home,
                        state.hover == Some(Destination::Home),
                        state.armed == Some(Destination::Home),
                    );
                    (|registry_selected: bool,
                      registry_hovered: bool,
                      registry_pressed: bool| {
                        registry_pill_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_color, desired_size, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if sprite.color != desired_color
                                        || sprite.custom_size != Some(desired_size)
                                    {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            color: desired_color,
                                            custom_size: Some(desired_size),
                                            ..sprite.clone()
                                        });
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if state.hud_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::color::Color::srgb(
                                        if registry_selected {
                                            if registry_pressed {
                                                1.0
                                            } else if registry_hovered {
                                                0.96
                                            } else {
                                                0.92
                                            }
                                        } else if registry_pressed {
                                            0.26
                                        } else if registry_hovered {
                                            0.18
                                        } else {
                                            0.12
                                        },
                                        if registry_selected {
                                            if registry_pressed {
                                                1.0
                                            } else if registry_hovered {
                                                0.96
                                            } else {
                                                0.92
                                            }
                                        } else if registry_pressed {
                                            0.26
                                        } else if registry_hovered {
                                            0.18
                                        } else {
                                            0.12
                                        },
                                        if registry_selected {
                                            if registry_pressed {
                                                1.0
                                            } else if registry_hovered {
                                                0.96
                                            } else {
                                                0.92
                                            }
                                        } else if registry_pressed {
                                            0.26
                                        } else if registry_hovered {
                                            0.18
                                        } else {
                                            0.12
                                        },
                                    ),
                                    bevy::math::Vec2::new(
                                        state.layout.tab_width,
                                        56.0,
                                    ),
                                    bevy::math::Vec3::new(
                                        state.layout.registry_tab.center_x,
                                        state.layout.registry_tab.center_y,
                                        5.0,
                                    ),
                                );
                            });
                        registry_icon_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_color, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if sprite.color != desired_color {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            color: desired_color,
                                            ..sprite.clone()
                                        });
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if state.hud_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::color::Color::srgb(
                                        if registry_selected {
                                            0.05
                                        } else if registry_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                        if registry_selected {
                                            0.05
                                        } else if registry_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                        if registry_selected {
                                            0.05
                                        } else if registry_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                    ),
                                    bevy::math::Vec3::new(
                                        state.layout.tab_icon_x,
                                        state.layout.registry_tab.center_y,
                                        6.0,
                                    ),
                                );
                            });
                        registry_label_texts
                            .iter()
                            .for_each(|(entity, transform, visibility, color, bounds)| {
                                (|desired_visibility: bevy::camera::visibility::Visibility,
  desired_color: bevy::color::Color,
  desired_bounds: bevy::text::TextBounds,
  desired_translation: bevy::math::Vec3| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if color.0 != desired_color {
                                        commands
                                            .entity(entity)
                                            .insert(bevy::text::TextColor(desired_color));
                                    }
                                    if bounds.width != desired_bounds.width || bounds.height != desired_bounds.height {
                                        commands.entity(entity).insert(desired_bounds);
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if state.hud_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::color::Color::srgb(
                                        if registry_selected {
                                            0.05
                                        } else if registry_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                        if registry_selected {
                                            0.05
                                        } else if registry_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                        if registry_selected {
                                            0.05
                                        } else if registry_hovered {
                                            0.98
                                        } else {
                                            0.88
                                        },
                                    ),
                                    bevy::text::TextBounds::new_horizontal(state.layout.tab_label_bounds),
                                    bevy::math::Vec3::new(
                                        state.layout.tab_label_x,
                                        state.layout.registry_tab.center_y,
                                        7.0,
                                    ),
                                );
                            });
                    })(
                        state.selected == Destination::ExtensionRegistry,
                        state.hover == Some(Destination::ExtensionRegistry),
                        state.armed == Some(Destination::ExtensionRegistry),
                    );
                    (|home_focused: bool| {
                        ring_home_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_color, desired_size, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if sprite.color != desired_color {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            color: desired_color,
                                            ..sprite.clone()
                                        });
                                    }
                                    if sprite.custom_size != Some(desired_size) {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            custom_size: Some(desired_size),
                                            ..sprite.clone()
                                        });
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if home_focused && state.hud_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::color::Color::srgb(
                                        if state.selected == Destination::Home {
                                            0.08
                                        } else {
                                            1.0
                                        },
                                        if state.selected == Destination::Home {
                                            0.08
                                        } else {
                                            1.0
                                        },
                                        if state.selected == Destination::Home {
                                            0.08
                                        } else {
                                            1.0
                                        },
                                    ),
                                    bevy::math::Vec2::new(
                                        state.layout.tab_width + 8.0,
                                        64.0,
                                    ),
                                    bevy::math::Vec3::new(
                                        state.layout.home_tab.center_x,
                                        state.layout.home_tab.center_y,
                                        9.0,
                                    ),
                                );
                            });
                    })(state.focus == Some(Destination::Home));

                },
                |state: bevy::ecs::system::Res<UiState>,
                 mut commands: bevy::ecs::system::Commands,
                 ring_registry_sprites: SpriteSlot<'_, '_, '_, RingRegistry>,
                 content_panel_sprites: SpriteSlot<'_, '_, '_, ContentPanel>,
                 laboratory_card_sprites: SpriteSlot<'_, '_, '_, LaboratoryCard>,
                 laboratory_well_sprites: SpriteSlot<'_, '_, '_, LaboratoryWell>,
                 flask_sprites: SpriteSlot<'_, '_, '_, Flask>,
                 laboratory_title_texts: TextSlot<'_, '_, '_, LaboratoryTitle>,
                 separator_sprites: SpriteSlot<'_, '_, '_, Separator>,
                 caption_texts: TextSlot<'_, '_, '_, Caption>,
                 action_pill_sprites: SpriteSlot<'_, '_, '_, ActionPill>,
                 action_icon_sprites: SpriteSlot<'_, '_, '_, ActionIcon>,
                 action_label_texts: TextSlot<'_, '_, '_, ActionLabel>| {
                    (|registry_focused: bool| {
                        ring_registry_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_color, desired_size, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if sprite.color != desired_color {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            color: desired_color,
                                            ..sprite.clone()
                                        });
                                    }
                                    if sprite.custom_size != Some(desired_size) {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            custom_size: Some(desired_size),
                                            ..sprite.clone()
                                        });
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_focused && state.hud_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::color::Color::srgb(
                                        if state.selected == Destination::ExtensionRegistry {
                                            0.08
                                        } else {
                                            1.0
                                        },
                                        if state.selected == Destination::ExtensionRegistry {
                                            0.08
                                        } else {
                                            1.0
                                        },
                                        if state.selected == Destination::ExtensionRegistry {
                                            0.08
                                        } else {
                                            1.0
                                        },
                                    ),
                                    bevy::math::Vec2::new(
                                        state.layout.tab_width + 8.0,
                                        64.0,
                                    ),
                                    bevy::math::Vec3::new(
                                        state.layout.registry_tab.center_x,
                                        state.layout.registry_tab.center_y,
                                        9.0,
                                    ),
                                );
                            });
                    })(state.focus == Some(Destination::ExtensionRegistry));
                    (|registry_visible: bool| {
                        content_panel_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_size, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if sprite.custom_size != Some(desired_size) {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            custom_size: Some(desired_size),
                                            ..sprite.clone()
                                        });
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::math::Vec2::new(
                                        state.layout.content.half_width * 2.0,
                                        state.layout.content.half_height * 2.0,
                                    ),
                                    bevy::math::Vec3::new(
                                        state.layout.content.center_x,
                                        state.layout.content.center_y,
                                        0.0,
                                    ),
                                );
                            });
                        laboratory_card_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_size, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if sprite.custom_size != Some(desired_size) {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            custom_size: Some(desired_size),
                                            ..sprite.clone()
                                        });
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::math::Vec2::new(
                                        state.layout.laboratory.half_width * 2.0,
                                        state.layout.laboratory.half_height * 2.0,
                                    ),
                                    bevy::math::Vec3::new(
                                        state.layout.laboratory.center_x,
                                        state.layout.laboratory.center_y,
                                        1.0,
                                    ),
                                );
                            });
                        laboratory_well_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::math::Vec3::new(
                                        state.layout.header_x,
                                        state.layout.header_y,
                                        2.0,
                                    ),
                                );
                            });
                        flask_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::math::Vec3::new(
                                        state.layout.header_x,
                                        state.layout.header_y,
                                        3.0,
                                    ),
                                );
                            });
                        laboratory_title_texts
                            .iter()
                            .for_each(|(entity, transform, visibility, _color, bounds)| {
                                (|desired_visibility: bevy::camera::visibility::Visibility,
  desired_bounds: bevy::text::TextBounds,
  desired_translation: bevy::math::Vec3| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if bounds.width != desired_bounds.width || bounds.height != desired_bounds.height {
                                        commands.entity(entity).insert(desired_bounds);
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::text::TextBounds {
                                        width: Some(
                                            state.layout.laboratory_title_bounds,
                                        ),
                                        height: None,
                                    },
                                    bevy::math::Vec3::new(
                                        state.layout.laboratory_title_x,
                                        state.layout.header_y,
                                        7.0,
                                    ),
                                );
                            });
                        separator_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_size, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if sprite.custom_size != Some(desired_size) {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            custom_size: Some(desired_size),
                                            ..sprite.clone()
                                        });
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::math::Vec2::new(
                                        state.layout.laboratory.half_width * 2.0 - 48.0,
                                        2.0,
                                    ),
                                    bevy::math::Vec3::new(
                                        state.layout.laboratory.center_x,
                                        state.layout.separator_y,
                                        8.0,
                                    ),
                                );
                            });
                        caption_texts
                            .iter()
                            .for_each(|(entity, transform, visibility, _color, bounds)| {
                                (|desired_visibility: bevy::camera::visibility::Visibility,
  desired_bounds: bevy::text::TextBounds,
  desired_translation: bevy::math::Vec3| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if bounds.width != desired_bounds.width || bounds.height != desired_bounds.height {
                                        commands.entity(entity).insert(desired_bounds);
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::text::TextBounds::new_horizontal(state.layout.caption_bounds),
                                    bevy::math::Vec3::new(
                                        state.layout.caption_x,
                                        state.layout.caption_y,
                                        4.0,
                                    ),
                                );
                            });
                        action_pill_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_size, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if sprite.custom_size != Some(desired_size) {
                                        commands.entity(entity).insert(bevy::sprite::Sprite {
                                            custom_size: Some(desired_size),
                                            ..sprite.clone()
                                        });
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::math::Vec2::new(
                                        state.layout.action.half_width * 2.0,
                                        64.0,
                                    ),
                                    bevy::math::Vec3::new(
                                        state.layout.action.center_x,
                                        state.layout.action.center_y,
                                        5.0,
                                    ),
                                );
                            });
                        action_icon_sprites
                            .iter()
                            .for_each(|(entity, transform, visibility, sprite)| {
                                (|desired_visibility, desired_translation| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::math::Vec3::new(
                                        state.layout.action_icon_x,
                                        state.layout.action.center_y,
                                        6.0,
                                    ),
                                );
                            });
                        action_label_texts
                            .iter()
                            .for_each(|(entity, transform, visibility, _color, bounds)| {
                                (|desired_visibility: bevy::camera::visibility::Visibility,
  desired_bounds: bevy::text::TextBounds,
  desired_translation: bevy::math::Vec3| {
                                    if *visibility != desired_visibility {
                                        commands.entity(entity).insert(desired_visibility);
                                    }
                                    if bounds.width != desired_bounds.width || bounds.height != desired_bounds.height {
                                        commands.entity(entity).insert(desired_bounds);
                                    }
                                    if transform.translation != desired_translation
                                        || transform.scale != bevy::math::Vec3::ONE
                                    {
                                        commands.entity(entity).insert(
                                            bevy::transform::components::Transform {
                                                translation: desired_translation,
                                                rotation: bevy::math::Quat::IDENTITY,
                                                scale: bevy::math::Vec3::ONE,
                                            },
                                        );
                                    }
                                })(
                                    if registry_visible {
                                        bevy::camera::visibility::Visibility::Visible
                                    } else {
                                        bevy::camera::visibility::Visibility::Hidden
                                    },
                                    bevy::text::TextBounds::new_horizontal(state.layout.action_label_bounds),
                                    bevy::math::Vec3::new(
                                        state.layout.action_label_x,
                                        state.layout.action.center_y,
                                        7.0,
                                    ),
                                );
                            });
                    })(
                        state.hud_visible
                            && state.selected == Destination::ExtensionRegistry,
                    );
                },
            )),
        )

        .run();
}
