fn main() {
    bevy::app::App::new()
        .add_plugins(
            bevy::app::PluginGroup::set(
                bevy::DefaultPlugins,
                bevy::window::WindowPlugin {
                    primary_window: Some(bevy::window::Window {
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        mode: bevy::window::WindowMode::BorderlessFullscreen(
                            bevy::window::MonitorSelection::Primary,
                        ),
                        position: bevy::window::WindowPosition::Automatic,
                        resolution: bevy::window::WindowResolution::new(1280, 720),
                        title: String::from("Subsystem for Arxumbra"),
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
                        has_shadow: false,
                        titlebar_shown: true,
                        titlebar_transparent: false,
                        titlebar_show_title: true,
                        titlebar_show_buttons: true,
                        borderless_game: true,
                        prefers_home_indicator_hidden: false,
                        prefers_status_bar_hidden: false,
                        preferred_screen_edges_deferring_system_gestures: bevy::window::ScreenEdge::None,
                    }),
                    primary_cursor_options: Some(bevy::window::CursorOptions {
                        visible: false,
                        grab_mode: bevy::window::CursorGrabMode::None,
                        hit_test: true,
                    }),
                    exit_condition: bevy::window::ExitCondition::OnAllClosed,
                    close_when_requested: true,
                },
            )
            .set(bevy::render::RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(Box::new(
                    bevy::render::settings::WgpuSettings {
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
                        gles3_minor_version: bevy::render::settings::Gles3MinorVersion::Automatic,
                        instance_flags: bevy::render::settings::InstanceFlags::VALIDATION_INDIRECT_CALL,
                        memory_hints: bevy::render::settings::MemoryHints::Performance,
                        instance_memory_budget_thresholds: wgpu::MemoryBudgetThresholds {
                            for_resource_creation: None,
                            for_device_loss: None,
                        },
                        force_fallback_adapter: false,
                        adapter_name: None,
                    },
                )),
                synchronous_pipeline_compilation: false,
                debug_flags: bevy::render::RenderDebugFlags::empty(),
            }),
        )
        .add_systems(
            bevy::app::Startup,
            |world: &mut bevy::ecs::world::World| {
                world.spawn(bevy::camera::Camera2d);
                (|formats: bevy::image::CompressedImageFormats,
                  width: f32,
                  height: f32,
                  world: &mut bevy::ecs::world::World|
                 {
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
                    world
                        .resource_mut::<bevy::asset::Assets<bevy::image::Image>>()
                        .insert(
                            bevy::asset::AssetId::Uuid {
                                uuid: bevy::asset::uuid::Uuid::from_u128(1),
                            },
                            bevy::image::Image::from_buffer(
                                include_bytes!("../assets/001.ktx2"),
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
                    world.spawn((
                            bevy::sprite::Sprite {
                                image: bevy::asset::Handle::Uuid(
                                    bevy::asset::uuid::Uuid::from_u128(1),
                                    core::marker::PhantomData,
                                ),
                                texture_atlas: None,
                                color: bevy::color::Color::WHITE,
                                flip_x: false,
                                flip_y: false,
                                custom_size: Some(bevy::math::Vec2::new(
                                    (width / 3.0).max(height * 9.0 / 16.0),
                                    height.max(width * 16.0 / 27.0),
                                )),
                                rect: None,
                                image_mode: bevy::sprite::SpriteImageMode::Auto,
                                alpha_mode: bevy::sprite::SpriteAlphaMode::Opaque,
                            },
                            bevy::transform::components::Transform {
                                translation: bevy::math::Vec3::new(-width / 3.0, 0.0, 1.0),
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
            },
        )
        .run();
}
