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
            bevy::diagnostic::DiagnosticsPlugin,
            bevy::input::InputPlugin,
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
            bevy::a11y::AccessibilityPlugin,
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
                        device_label: None,
                        backends: Some(
                            bevy::render::settings::Backends::VULKAN
                                | bevy::render::settings::Backends::METAL
                                | bevy::render::settings::Backends::DX12,
                        ),
                        power_preference: bevy::render::settings::PowerPreference::HighPerformance,
                        priority: bevy::render::settings::WgpuSettingsPriority::Functionality,
                        features: bevy::render::settings::WgpuFeatures::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                        disabled_features: None,
                        limits: bevy::render::settings::WgpuLimits {
                            max_texture_dimension_1d: 8192,
                            max_texture_dimension_2d: 16384,
                            max_texture_dimension_3d: 2048,
                            max_texture_array_layers: 256,
                            max_bind_groups: 4,
                            max_bind_groups_plus_vertex_buffers: 24,
                            max_bindings_per_bind_group: 1000,
                            max_dynamic_uniform_buffers_per_pipeline_layout: 8,
                            max_dynamic_storage_buffers_per_pipeline_layout: 4,
                            max_sampled_textures_per_shader_stage: 16,
                            max_samplers_per_shader_stage: 16,
                            max_storage_buffers_per_shader_stage: 8,
                            max_storage_textures_per_shader_stage: 4,
                            max_uniform_buffers_per_shader_stage: 12,
                            max_binding_array_elements_per_shader_stage: 0,
                            max_binding_array_acceleration_structure_elements_per_shader_stage: 0,
                            max_binding_array_sampler_elements_per_shader_stage: 0,
                            max_uniform_buffer_binding_size: 65536,
                            max_storage_buffer_binding_size: 134217728,
                            max_vertex_buffers: 8,
                            max_buffer_size: 268435456,
                            max_vertex_attributes: 16,
                            max_vertex_buffer_array_stride: 2048,
                            max_inter_stage_shader_variables: 16,
                            min_uniform_buffer_offset_alignment: 256,
                            min_storage_buffer_offset_alignment: 256,
                            max_color_attachments: 8,
                            max_color_attachment_bytes_per_sample: 32,
                            max_compute_workgroup_storage_size: 16384,
                            max_compute_invocations_per_workgroup: 256,
                            max_compute_workgroup_size_x: 256,
                            max_compute_workgroup_size_y: 256,
                            max_compute_workgroup_size_z: 64,
                            max_compute_workgroups_per_dimension: 65535,
                            max_immediate_size: 0,
                            max_non_sampler_bindings: 1000000,
                            max_task_workgroup_total_count: 0,
                            max_task_workgroups_per_dimension: 0,
                            max_mesh_workgroup_total_count: 0,
                            max_mesh_workgroups_per_dimension: 0,
                            max_task_invocations_per_workgroup: 0,
                            max_task_invocations_per_dimension: 0,
                            max_mesh_invocations_per_workgroup: 0,
                            max_mesh_invocations_per_dimension: 0,
                            max_task_payload_size: 0,
                            max_mesh_output_vertices: 0,
                            max_mesh_output_primitives: 0,
                            max_mesh_output_layers: 0,
                            max_mesh_multiview_view_count: 0,
                            max_blas_primitive_count: 0,
                            max_blas_geometry_count: 0,
                            max_tlas_instance_count: 0,
                            max_acceleration_structures_per_shader_stage: 0,
                            max_buffers_and_acceleration_structures_per_shader_stage: 28,
                            max_multiview_view_count: 0,
                            max_ray_dispatch_count: 0,
                            max_ray_recursion_depth: 0,
                        },
                        constrained_limits: None,
                        dx12_shader_compiler: bevy::render::settings::Dx12Compiler::Fxc,
                        gles3_minor_version:
                            bevy::render::settings::Gles3MinorVersion::Automatic,
                        instance_flags:
                            bevy::render::settings::InstanceFlags::VALIDATION_INDIRECT_CALL,
                        memory_hints: bevy::render::settings::MemoryHints::Performance,
                        instance_memory_budget_thresholds:
                            wgpu::MemoryBudgetThresholds {
                                for_resource_creation: None,
                                for_device_loss: None,
                            },
                        force_fallback_adapter: false,
                        adapter_name: None,
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
            bevy::ui::UiPlugin,
            bevy::ui_render::UiRenderPlugin,
        ))
        .add_systems(
            bevy::app::Startup,
            |world: &mut bevy::ecs::world::World| {
                world.spawn(bevy::camera::Camera2d);
                (|formats: bevy::image::CompressedImageFormats,
                  width: f32,
                  height: f32,
                  world: &mut bevy::ecs::world::World| {
                    world
                        .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                        .insert(
                            bevy::asset::AssetId::Uuid {
                                uuid: bevy::asset::uuid::Uuid::from_u128(0),
                            },
                            bevy::image::Image::from_buffer(
                                include_bytes!("../assets/000.ktx2"),
                                bevy::image::ImageType::Extension("ktx2"),
                                formats,
                                true,
                                bevy::image::ImageSampler::Default,
                                bevy::asset::RenderAssetUsages::RENDER_WORLD,
                            )
                            .unwrap(),
                        )
                        .unwrap();
                    world.spawn((
                        bevy::sprite::Sprite {
                            image: bevy::asset::Handle::Uuid(
                                bevy::asset::uuid::Uuid::from_u128(0),
                                core::marker::PhantomData,
                            ),
                            texture_atlas: None,
                            color: bevy::color::Color::WHITE,
                            flip_x: false,
                            flip_y: false,
                            custom_size: Some(bevy::math::Vec2::new(
                                width.max(height * 16.0 / 9.0),
                                height.max(width * 9.0 / 16.0),
                            )),
                            rect: None,
                            image_mode: bevy::sprite::SpriteImageMode::Auto,
                            alpha_mode: bevy::sprite::SpriteAlphaMode::Opaque,
                        },
                        bevy::transform::components::Transform {
                            translation: bevy::math::Vec3::ZERO,
                            rotation: bevy::math::Quat::IDENTITY,
                            scale: bevy::math::Vec3::ONE,
                        },
                    ));
                })(
                    world.resource::<bevy::image::CompressedImageFormatSupport>().0,
                    world
                        .query::<&bevy::window::Monitor>()
                        .iter(world)
                        .next()
                        .unwrap()
                        .physical_width as f32,
                    world
                        .query::<&bevy::window::Monitor>()
                        .iter(world)
                        .next()
                        .unwrap()
                        .physical_height as f32,
                    world,
                );
                world
                    .spawn((
                        bevy::ui::Node {
                            position_type: bevy::ui::PositionType::Absolute,
                            left: bevy::ui::Val::Px(0.0),
                            top: bevy::ui::Val::Px(0.0),
                            width: bevy::ui::Val::Percent(33.333),
                            height: bevy::ui::Val::Percent(100.0),
                            flex_direction: bevy::ui::FlexDirection::Column,
                            padding: bevy::ui::UiRect::all(bevy::ui::Val::Px(24.0)),
                            border: bevy::ui::UiRect {
                                top: bevy::ui::Val::Px(0.0),
                                right: bevy::ui::Val::Px(1.0),
                                bottom: bevy::ui::Val::Px(0.0),
                                left: bevy::ui::Val::Px(0.0),
                            },
                            border_radius: bevy::ui::BorderRadius::px(0.0, 24.0, 24.0, 0.0),
                            ..bevy::ui::Node::DEFAULT
                        },
                        bevy::ui::BackgroundColor(bevy::color::Color::srgba(
                            10.0 / 255.0,
                            11.0 / 255.0,
                            15.0 / 255.0,
                            0.96,
                        )),
                        bevy::ui::BorderColor {
                            top: bevy::color::Color::srgba(1.0, 1.0, 1.0, 0.05),
                            right: bevy::color::Color::srgba(1.0, 1.0, 1.0, 0.10),
                            bottom: bevy::color::Color::srgba(1.0, 1.0, 1.0, 0.05),
                            left: bevy::color::Color::srgba(1.0, 1.0, 1.0, 0.05),
                        },
                        bevy::ui::BoxShadow::new(
                            bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.95),
                            bevy::ui::Val::Px(0.0),
                            bevy::ui::Val::Px(0.0),
                            bevy::ui::Val::Px(0.0),
                            bevy::ui::Val::Px(60.0),
                        ),
                        bevy::ui::ZIndex(10),
                    ))
                    .with_children(|panel| {
                        panel
                            .spawn(bevy::ui::Node {
                                width: bevy::ui::Val::Percent(100.0),
                                flex_direction: bevy::ui::FlexDirection::Column,
                                row_gap: bevy::ui::Val::Px(16.0),
                                ..bevy::ui::Node::DEFAULT
                            })
                            .with_children(|stack| {
                                stack
                                    .spawn((
                                        bevy::ui::Node {
                                            width: bevy::ui::Val::Percent(100.0),
                                            flex_direction: bevy::ui::FlexDirection::Row,
                                            align_items: bevy::ui::AlignItems::Center,
                                            column_gap: bevy::ui::Val::Px(12.0),
                                            padding: bevy::ui::UiRect {
                                                bottom: bevy::ui::Val::Px(14.0),
                                                ..bevy::ui::UiRect::ZERO
                                            },
                                            border: bevy::ui::UiRect {
                                                bottom: bevy::ui::Val::Px(1.0),
                                                ..bevy::ui::UiRect::ZERO
                                            },
                                            ..bevy::ui::Node::DEFAULT
                                        },
                                        bevy::ui::BorderColor {
                                            top: bevy::color::Color::NONE,
                                            right: bevy::color::Color::NONE,
                                            bottom: bevy::color::Color::srgba(1.0, 1.0, 1.0, 0.10),
                                            left: bevy::color::Color::NONE,
                                        },
                                    ))
                                    .with_children(|header| {
                                        header
                                            .spawn((
                                                bevy::ui::Node {
                                                    width: bevy::ui::Val::Px(40.0),
                                                    height: bevy::ui::Val::Px(40.0),
                                                    flex_shrink: 0.0,
                                                    align_items: bevy::ui::AlignItems::Center,
                                                    justify_content: bevy::ui::JustifyContent::Center,
                                                    border: bevy::ui::UiRect::all(
                                                        bevy::ui::Val::Px(1.0),
                                                    ),
                                                    border_radius: bevy::ui::BorderRadius::all(
                                                        bevy::ui::Val::Px(12.0),
                                                    ),
                                                    ..bevy::ui::Node::DEFAULT
                                                },
                                                bevy::ui::BackgroundColor(
                                                    bevy::color::Color::srgba(
                                                        34.0 / 255.0,
                                                        35.0 / 255.0,
                                                        43.0 / 255.0,
                                                        1.0,
                                                    ),
                                                ),
                                                bevy::ui::BorderColor::all(
                                                    bevy::color::Color::srgba(1.0, 1.0, 1.0, 0.20),
                                                ),
                                            ))
                                            .with_children(|logo| {
                                                logo.spawn((
                                                    bevy::ui::widget::Text::new(">_"),
                                                    bevy::text::TextFont::from_font_size(
                                                        bevy::text::FontSize::Px(16.0),
                                                    )
                                                    .with_font_weight(
                                                        bevy::text::FontWeight::BOLD,
                                                    ),
                                                    bevy::text::TextColor(
                                                        bevy::color::Color::WHITE,
                                                    ),
                                                    bevy::ui::Node {
                                                        width: bevy::ui::Val::Percent(100.0),
                                                        height: bevy::ui::Val::Percent(100.0),
                                                        align_items: bevy::ui::AlignItems::Center,
                                                        justify_content:
                                                            bevy::ui::JustifyContent::Center,
                                                        ..bevy::ui::Node::DEFAULT
                                                    },
                                                ));
                                            });
                                        header
                                            .spawn(bevy::ui::Node {
                                                flex_grow: 1.0,
                                                min_width: bevy::ui::Val::Px(0.0),
                                                flex_direction: bevy::ui::FlexDirection::Row,
                                                align_items: bevy::ui::AlignItems::Center,
                                                column_gap: bevy::ui::Val::Px(8.0),
                                                ..bevy::ui::Node::DEFAULT
                                            })
                                            .with_children(|identity| {
                                                identity.spawn((
                                                    bevy::ui::widget::Text::new(
                                                        "LINUX SUBSYSTEM FOR ARXUMBRA",
                                                    ),
                                                    bevy::text::TextFont::from_font_size(
                                                        bevy::text::FontSize::Px(12.0),
                                                    )
                                                    .with_font_weight(
                                                        bevy::text::FontWeight::BOLD,
                                                    ),
                                                    bevy::text::TextColor(
                                                        bevy::color::Color::WHITE,
                                                    ),
                                                    bevy::text::TextLayout::no_wrap(),
                                                    bevy::ui::Node {
                                                        min_width: bevy::ui::Val::Px(0.0),
                                                        ..bevy::ui::Node::DEFAULT
                                                    },
                                                ));
                                                identity
                                                    .spawn((
                                                        bevy::ui::Node {
                                                            padding: bevy::ui::UiRect {
                                                                top: bevy::ui::Val::Px(2.0),
                                                                right: bevy::ui::Val::Px(6.0),
                                                                bottom: bevy::ui::Val::Px(2.0),
                                                                left: bevy::ui::Val::Px(6.0),
                                                            },
                                                            border: bevy::ui::UiRect::all(
                                                                bevy::ui::Val::Px(1.0),
                                                            ),
                                                            border_radius:
                                                                bevy::ui::BorderRadius::all(
                                                                    bevy::ui::Val::Px(4.0),
                                                                ),
                                                            flex_shrink: 0.0,
                                                            align_items:
                                                                bevy::ui::AlignItems::Center,
                                                            justify_content:
                                                                bevy::ui::JustifyContent::Center,
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.10,
                                                            ),
                                                        ),
                                                        bevy::ui::BorderColor::all(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.25,
                                                            ),
                                                        ),
                                                    ))
                                                    .with_children(|badge| {
                                                        badge.spawn((
                                                            bevy::ui::widget::Text::new(
                                                                "v0.0.0.0.0.0-s1",
                                                            ),
                                                            bevy::text::TextFont::from_font_size(
                                                                bevy::text::FontSize::Px(9.0),
                                                            )
                                                            .with_font_weight(
                                                                bevy::text::FontWeight::BOLD,
                                                            ),
                                                            bevy::text::TextColor(
                                                                bevy::color::Color::WHITE,
                                                            ),
                                                            bevy::text::TextLayout::no_wrap(),
                                                        ));
                                                    });
                                            });
                                    });
                                stack
                                    .spawn(bevy::ui::Node {
                                        width: bevy::ui::Val::Percent(100.0),
                                        padding: bevy::ui::UiRect {
                                            top: bevy::ui::Val::Px(4.0),
                                            ..bevy::ui::UiRect::ZERO
                                        },
                                        flex_direction: bevy::ui::FlexDirection::Column,
                                        row_gap: bevy::ui::Val::Px(8.0),
                                        ..bevy::ui::Node::DEFAULT
                                    })
                                    .with_children(|nav| {
                                        nav.spawn((
                                            bevy::ui::Node {
                                                width: bevy::ui::Val::Percent(100.0),
                                                height: bevy::ui::Val::Px(50.0),
                                                flex_direction: bevy::ui::FlexDirection::Row,
                                                align_items: bevy::ui::AlignItems::Center,
                                                column_gap: bevy::ui::Val::Px(12.0),
                                                padding: bevy::ui::UiRect {
                                                    top: bevy::ui::Val::Px(12.0),
                                                    right: bevy::ui::Val::Px(16.0),
                                                    bottom: bevy::ui::Val::Px(12.0),
                                                    left: bevy::ui::Val::Px(16.0),
                                                },
                                                border: bevy::ui::UiRect::all(
                                                    bevy::ui::Val::Px(1.0),
                                                ),
                                                border_radius: bevy::ui::BorderRadius::all(
                                                    bevy::ui::Val::Px(12.0),
                                                ),
                                                ..bevy::ui::Node::DEFAULT
                                            },
                                            bevy::ui::BackgroundColor(
                                                bevy::color::Color::srgba(
                                                    24.0 / 255.0,
                                                    25.0 / 255.0,
                                                    32.0 / 255.0,
                                                    0.90,
                                                ),
                                            ),
                                            bevy::ui::BorderColor::all(
                                                bevy::color::Color::srgba(1.0, 1.0, 1.0, 0.40),
                                            ),
                                            bevy::ui::BoxShadow::new(
                                                bevy::color::Color::srgba(
                                                    1.0, 1.0, 1.0, 0.07,
                                                ),
                                                bevy::ui::Val::Px(0.0),
                                                bevy::ui::Val::Px(0.0),
                                                bevy::ui::Val::Px(0.0),
                                                bevy::ui::Val::Px(14.0),
                                            ),
                                        ))
                                        .with_children(|sanctuary| {
                                            sanctuary
                                                .spawn(bevy::ui::Node {
                                                    position_type:
                                                        bevy::ui::PositionType::Relative,
                                                    width: bevy::ui::Val::Px(24.0),
                                                    height: bevy::ui::Val::Px(24.0),
                                                    flex_shrink: 0.0,
                                                    ..bevy::ui::Node::DEFAULT
                                                })
                                                .with_children(|icon| {
                                                    icon.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(4.0),
                                                            top: bevy::ui::Val::Px(4.0),
                                                            width: bevy::ui::Val::Px(16.0),
                                                            height: bevy::ui::Val::Px(16.0),
                                                            border: bevy::ui::UiRect::all(
                                                                bevy::ui::Val::Px(1.0),
                                                            ),
                                                            border_radius:
                                                                bevy::ui::BorderRadius::all(
                                                                    bevy::ui::Val::Px(8.0),
                                                                ),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::NONE,
                                                        ),
                                                        bevy::ui::BorderColor::all(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.85,
                                                            ),
                                                        ),
                                                    ));
                                                    icon.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(3.0),
                                                            top: bevy::ui::Val::Px(11.0),
                                                            width: bevy::ui::Val::Px(18.0),
                                                            height: bevy::ui::Val::Px(2.0),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.85,
                                                            ),
                                                        ),
                                                    ));
                                                    icon.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(11.0),
                                                            top: bevy::ui::Val::Px(3.0),
                                                            width: bevy::ui::Val::Px(2.0),
                                                            height: bevy::ui::Val::Px(18.0),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.85,
                                                            ),
                                                        ),
                                                    ));
                                                    icon.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(9.0),
                                                            top: bevy::ui::Val::Px(9.0),
                                                            width: bevy::ui::Val::Px(6.0),
                                                            height: bevy::ui::Val::Px(6.0),
                                                            border_radius:
                                                                bevy::ui::BorderRadius::all(
                                                                    bevy::ui::Val::Px(3.0),
                                                                ),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::WHITE,
                                                        ),
                                                    ));
                                                });
                                            sanctuary.spawn((
                                                bevy::ui::widget::Text::new("Sanctuary"),
                                                bevy::text::TextFont::from_font_size(
                                                    bevy::text::FontSize::Px(12.0),
                                                )
                                                .with_font_weight(bevy::text::FontWeight::BOLD),
                                                bevy::text::TextColor(
                                                    bevy::color::Color::WHITE,
                                                ),
                                                bevy::text::TextLayout::no_wrap(),
                                            ));
                                        });
                                        nav.spawn((
                                            bevy::ui::Node {
                                                width: bevy::ui::Val::Percent(100.0),
                                                height: bevy::ui::Val::Px(50.0),
                                                flex_direction: bevy::ui::FlexDirection::Row,
                                                align_items: bevy::ui::AlignItems::Center,
                                                column_gap: bevy::ui::Val::Px(12.0),
                                                padding: bevy::ui::UiRect {
                                                    top: bevy::ui::Val::Px(12.0),
                                                    right: bevy::ui::Val::Px(16.0),
                                                    bottom: bevy::ui::Val::Px(12.0),
                                                    left: bevy::ui::Val::Px(16.0),
                                                },
                                                border: bevy::ui::UiRect::all(
                                                    bevy::ui::Val::Px(1.0),
                                                ),
                                                border_radius: bevy::ui::BorderRadius::all(
                                                    bevy::ui::Val::Px(12.0),
                                                ),
                                                ..bevy::ui::Node::DEFAULT
                                            },
                                            bevy::ui::BackgroundColor(
                                                bevy::color::Color::srgba(
                                                    6.0 / 255.0,
                                                    7.0 / 255.0,
                                                    10.0 / 255.0,
                                                    0.40,
                                                ),
                                            ),
                                            bevy::ui::BorderColor::all(
                                                bevy::color::Color::srgba(
                                                    35.0 / 255.0,
                                                    37.0 / 255.0,
                                                    46.0 / 255.0,
                                                    0.30,
                                                ),
                                            ),
                                        ))
                                        .with_children(|extensions| {
                                            extensions
                                                .spawn(bevy::ui::Node {
                                                    position_type:
                                                        bevy::ui::PositionType::Relative,
                                                    width: bevy::ui::Val::Px(24.0),
                                                    height: bevy::ui::Val::Px(24.0),
                                                    flex_shrink: 0.0,
                                                    ..bevy::ui::Node::DEFAULT
                                                })
                                                .with_children(|chip| {
                                                    chip.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(6.0),
                                                            top: bevy::ui::Val::Px(6.0),
                                                            width: bevy::ui::Val::Px(12.0),
                                                            height: bevy::ui::Val::Px(12.0),
                                                            border: bevy::ui::UiRect::all(
                                                                bevy::ui::Val::Px(1.0),
                                                            ),
                                                            border_radius:
                                                                bevy::ui::BorderRadius::all(
                                                                    bevy::ui::Val::Px(2.0),
                                                                ),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::NONE,
                                                        ),
                                                        bevy::ui::BorderColor::all(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.80,
                                                            ),
                                                        ),
                                                    ));
                                                    chip.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(8.0),
                                                            top: bevy::ui::Val::Px(1.0),
                                                            width: bevy::ui::Val::Px(2.0),
                                                            height: bevy::ui::Val::Px(4.0),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.80,
                                                            ),
                                                        ),
                                                    ));
                                                    chip.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(14.0),
                                                            top: bevy::ui::Val::Px(1.0),
                                                            width: bevy::ui::Val::Px(2.0),
                                                            height: bevy::ui::Val::Px(4.0),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.80,
                                                            ),
                                                        ),
                                                    ));
                                                    chip.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(8.0),
                                                            top: bevy::ui::Val::Px(19.0),
                                                            width: bevy::ui::Val::Px(2.0),
                                                            height: bevy::ui::Val::Px(4.0),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.80,
                                                            ),
                                                        ),
                                                    ));
                                                    chip.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(14.0),
                                                            top: bevy::ui::Val::Px(19.0),
                                                            width: bevy::ui::Val::Px(2.0),
                                                            height: bevy::ui::Val::Px(4.0),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.80,
                                                            ),
                                                        ),
                                                    ));
                                                    chip.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(1.0),
                                                            top: bevy::ui::Val::Px(8.0),
                                                            width: bevy::ui::Val::Px(4.0),
                                                            height: bevy::ui::Val::Px(2.0),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.80,
                                                            ),
                                                        ),
                                                    ));
                                                    chip.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(1.0),
                                                            top: bevy::ui::Val::Px(14.0),
                                                            width: bevy::ui::Val::Px(4.0),
                                                            height: bevy::ui::Val::Px(2.0),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.80,
                                                            ),
                                                        ),
                                                    ));
                                                    chip.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(19.0),
                                                            top: bevy::ui::Val::Px(8.0),
                                                            width: bevy::ui::Val::Px(4.0),
                                                            height: bevy::ui::Val::Px(2.0),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.80,
                                                            ),
                                                        ),
                                                    ));
                                                    chip.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(19.0),
                                                            top: bevy::ui::Val::Px(14.0),
                                                            width: bevy::ui::Val::Px(4.0),
                                                            height: bevy::ui::Val::Px(2.0),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.80,
                                                            ),
                                                        ),
                                                    ));
                                                    chip.spawn((
                                                        bevy::ui::Node {
                                                            position_type:
                                                                bevy::ui::PositionType::Absolute,
                                                            left: bevy::ui::Val::Px(10.0),
                                                            top: bevy::ui::Val::Px(10.0),
                                                            width: bevy::ui::Val::Px(4.0),
                                                            height: bevy::ui::Val::Px(4.0),
                                                            border_radius:
                                                                bevy::ui::BorderRadius::all(
                                                                    bevy::ui::Val::Px(2.0),
                                                                ),
                                                            ..bevy::ui::Node::DEFAULT
                                                        },
                                                        bevy::ui::BackgroundColor(
                                                            bevy::color::Color::srgba(
                                                                1.0, 1.0, 1.0, 0.30,
                                                            ),
                                                        ),
                                                    ));
                                                });
                                            extensions.spawn((
                                                bevy::ui::widget::Text::new(
                                                    "Extensions Registry",
                                                ),
                                                bevy::text::TextFont::from_font_size(
                                                    bevy::text::FontSize::Px(12.0),
                                                ),
                                                bevy::text::TextColor(
                                                    bevy::color::Color::srgba(
                                                        155.0 / 255.0,
                                                        160.0 / 255.0,
                                                        173.0 / 255.0,
                                                        1.0,
                                                    ),
                                                ),
                                                bevy::text::TextLayout::no_wrap(),
                                            ));
                                        });
                                    });
                            });
                    });
            },
        )
        .run();
}
