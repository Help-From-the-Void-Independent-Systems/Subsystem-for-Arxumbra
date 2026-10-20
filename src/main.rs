#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
            bevy::log::LogPlugin {
                level: bevy::log::Level::ERROR,
                filter: std::string::String::from("wgpu=error,naga=error,sctk_adwaita=off"),
                ..Default::default()
            },
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
                    title: std::string::String::from(if cfg!(target_os = "linux") {
                        "Linux-Subsystem-for-Arxumbra"
                    } else if cfg!(target_os = "macos") {
                        "macOS-Subsystem-for-Arxumbra"
                    } else {
                        "Windows-Subsystem-for-Arxumbra"
                    }),
                    resize_constraints: bevy::window::WindowResizeConstraints {
                        min_width: 800.0,
                        min_height: 500.0,
                        ..bevy::window::WindowResizeConstraints::default()
                    },
                    resizable: false,
                    decorations: false,
                    transparent: false,
                    focused: true,
                    internal: bevy::window::InternalWindowState::default(),
                    skip_taskbar: true,
                    desired_maximum_frame_latency: None,
                    ..Default::default()
                }),
                primary_cursor_options: Some(bevy::window::CursorOptions {
                    visible: true,
                    grab_mode: bevy::window::CursorGrabMode::None,
                    hit_test: true,
                }),
                ..Default::default()
            },
            bevy::asset::AssetPlugin::default(),
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
                        ..Default::default()
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
        .insert_resource(bevy::camera::ClearColor(bevy::color::Color::srgb(
            0.047, 0.047, 0.047,
        )))
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::reactive_low_power(
                std::time::Duration::MAX,
            ),
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(
                std::time::Duration::MAX,
            ),
        })
        .add_systems(
            bevy::app::Startup,
            |mut commands: bevy::ecs::system::Commands| {
                commands.spawn((bevy::camera::Camera2d, bevy::render::view::Msaa::Off));
            },
        )
        .add_systems(
            bevy::app::PostUpdate,
            |(keys, mouse): (
                bevy::ecs::system::Res<
                    bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>,
                >,
                bevy::ecs::system::Res<
                    bevy::input::ButtonInput<bevy::input::mouse::MouseButton>,
                >,
            ),
             (time, windows): (
                bevy::ecs::system::Res<bevy::time::Time>,
                bevy::ecs::system::Query<
                    &bevy::window::Window,
                    bevy::ecs::query::With<bevy::window::PrimaryWindow>,
                >,
            ),
             (mut images, mut commands): (
                bevy::ecs::system::ResMut<bevy::asset::Assets<bevy::image::Image>>,
                bevy::ecs::system::Commands<'_, '_>,
            ),
             mut settings: bevy::ecs::system::ResMut<bevy::winit::WinitSettings>,
             sprites: bevy::ecs::system::Query<
                (
                    Option<&bevy::sprite::Sprite>,
                    &bevy::camera::visibility::Visibility,
                ),
                bevy::ecs::query::Without<bevy::text::TextColor>,
            >,
             texts: bevy::ecs::system::Query<
                (
                    Option<&bevy::text::TextColor>,
                    &bevy::camera::visibility::Visibility,
                ),
                bevy::ecs::query::Without<bevy::sprite::Sprite>,
            >,
             layout_info: bevy::ecs::system::Query<&bevy::text::TextLayoutInfo>,
             mut hud: bevy::ecs::system::Local<Option<bool>>,
             mut navigation_state: bevy::ecs::system::Local<(bool, f32)>,
             mut interaction_state: bevy::ecs::system::Local<[Option<u8>; 3]>,
             mut anim: bevy::ecs::system::Local<([[f32; 3]; 3], [f32; 2])>,
             mut viewport: bevy::ecs::system::Local<bevy::math::Vec2>,
             mut title_metrics: bevy::ecs::system::Local<(f32, f32, f32)>,
             mut layout: bevy::ecs::system::Local<
                Option<[bevy::math::Rect; 14]>,
            >,
             mut slots: bevy::ecs::system::Local<
                Option<[bevy::ecs::entity::Entity; 32]>,
            >,
             mut handles: bevy::ecs::system::Local<
                Option<[bevy::asset::Handle<bevy::image::Image>; 7]>,
            >| {
                windows.single().into_iter().for_each(|window| {
                    if hud.is_none() {
                        *hud = Some(true);
                    }

                    title_metrics.1 = if title_metrics.0 > 0.0
                        || (*slots)
                            .and_then(|entities| {
                                layout_info
                                    .get(entities[10])
                                    .ok()
                                    .map(|info| info.size.x / info.scale_factor)
                            })
                            .filter(|width| width.is_finite() && *width > 0.0)
                            .is_some_and(|width: f32| {
                                title_metrics.0 = width;
                                true
                            })
                    {
                        ((title_metrics.0 + 100.0) / 8.0).ceil() * 8.0
                    } else {
                        512.0f32.min(window.width() * 0.70)
                    };

                    title_metrics.2 = if title_metrics.0 > 0.0 {
                        (title_metrics.2 + time.delta_secs()).min(1.0)
                    } else {
                        0.0
                    };

                    if viewport.x != window.width()
                        || viewport.y != window.height()
                        || layout.is_none()
                        || (*layout).is_some_and(
                            |[sidebar, ..]: [bevy::math::Rect; 14]| {
                                sidebar.width() != title_metrics.1
                            },
                        )
                    {
                        *viewport = bevy::math::Vec2::new(window.width(), window.height());
                        *layout = Some([
                            bevy::math::Rect::new(
                                0.0,
                                0.0,
                                title_metrics.1,
                                viewport.y,
                            ),
                            bevy::math::Rect::new(
                                title_metrics.1 + 16.0,
                                16.0,
                                viewport.x - 16.0,
                                viewport.y - 16.0,
                            ),
                            bevy::math::Rect::new(
                                16.0,
                                84.0,
                                title_metrics.1 - 16.0,
                                132.0,
                            ),
                            bevy::math::Rect::new(
                                16.0,
                                140.0,
                                title_metrics.1 - 16.0,
                                188.0,
                            ),
                            bevy::math::Rect::new(
                                title_metrics.1 + 44.0,
                                144.0,
                                (title_metrics.1 + 44.0 + 640.0)
                                    .min(viewport.x - 16.0 - 28.0),
                                352.0,
                            ),
                            bevy::math::Rect::new(
                                title_metrics.1 + 44.0 + 20.0,
                                224.0,
                                (title_metrics.1 + 44.0 + 640.0)
                                    .min(viewport.x - 16.0 - 28.0)
                                    - 20.0,
                                226.0,
                            ),
                            bevy::math::Rect::new(
                                (title_metrics.1 + 44.0 + 640.0)
                                    .min(viewport.x - 16.0 - 28.0)
                                    - 20.0
                                    - 150.0,
                                288.0,
                                (title_metrics.1 + 44.0 + 640.0)
                                    .min(viewport.x - 16.0 - 28.0)
                                    - 20.0,
                                332.0,
                            ),
                            bevy::math::Rect::new(
                                title_metrics.1 + 16.0 + 28.0,
                                112.0,
                                viewport.x - 16.0 - 28.0,
                                114.0,
                            ),
                            bevy::math::Rect::new(20.0, 16.0, 66.0, 62.0),
                            bevy::math::Rect::new(
                                24.0,
                                90.0,
                                60.0,
                                126.0,
                            ),
                            bevy::math::Rect::new(
                                24.0,
                                146.0,
                                60.0,
                                182.0,
                            ),
                            bevy::math::Rect::new(
                                title_metrics.1 + 16.0 + 28.0,
                                40.0,
                                title_metrics.1 + 16.0 + 28.0 + 44.0,
                                84.0,
                            ),
                            bevy::math::Rect::new(
                                title_metrics.1 + 44.0 + 20.0,
                                164.0,
                                title_metrics.1 + 44.0 + 20.0 + 42.0,
                                206.0,
                            ),
                            bevy::math::Rect::new(
                                (title_metrics.1 + 44.0 + 640.0)
                                    .min(viewport.x - 16.0 - 28.0)
                                    - 20.0
                                    - 150.0
                                    + 8.0,
                                295.0,
                                (title_metrics.1 + 44.0 + 640.0)
                                    .min(viewport.x - 16.0 - 28.0)
                                    - 20.0
                                    - 150.0
                                    + 38.0,
                                325.0,
                            ),
                        ]);
                        (*slots).into_iter().flatten().for_each(|entity| {
                            commands.entity(entity).despawn();
                        });
                        *slots = None;
                    }

                    (*layout).into_iter().for_each(
                        |[
                            sidebar,
                            panel,
                            home_tab,
                            registry_tab,
                            card,
                            card_divider,
                            launch_button,
                            section_divider,
                            app_tile,
                            home_tile,
                            registry_tile,
                            registry_header_tile,
                            flask_tile,
                            rocket_tile,
                        ]| {
*hud = Some(if keys.just_pressed(bevy::input::keyboard::KeyCode::F1) {
                                !hud.unwrap_or(false)
                            } else {
                                hud.unwrap_or(false)
                            });

                            interaction_state[0] = match window.cursor_position() {
                                Some(cursor) if hud.unwrap_or(false) && window.focused => {
                                    if home_tab.contains(cursor) {
                                        Some(0u8)
                                    } else if registry_tab.contains(cursor) {
                                        Some(1u8)
                                    } else if navigation_state.0 && launch_button.contains(cursor) {
                                        Some(2u8)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            };

                            interaction_state[1] = if !hud.unwrap_or(false) || !window.focused {
                                None
                            } else if keys.just_pressed(bevy::input::keyboard::KeyCode::Tab) {
                                if keys.pressed(bevy::input::keyboard::KeyCode::ShiftLeft)
                                    || keys.pressed(bevy::input::keyboard::KeyCode::ShiftRight)
                                {
                                    match interaction_state[1] {
                                        Some(2u8) => Some(1u8),
                                        Some(1u8) => Some(0u8),
                                        Some(0u8) => {
                                            if navigation_state.0 { Some(2u8) } else { Some(1u8) }
                                        }
                                        _ => {
                                            if navigation_state.0 { Some(2u8) } else { Some(1u8) }
                                        }
                                    }
                                } else {
                                    match interaction_state[1] {
                                        Some(0u8) => Some(1u8),
                                        Some(1u8) => {
                                            if navigation_state.0 { Some(2u8) } else { Some(0u8) }
                                        }
                                        Some(2u8) => Some(0u8),
                                        _ => Some(0u8),
                                    }
                                }
                            } else if interaction_state[1] == Some(2u8) && !navigation_state.0 {
                                None
                            } else {
                                interaction_state[1]
                            };

                            if hud.unwrap_or(false)
                                && window.focused
                                && interaction_state[2].is_some()
                                && interaction_state[2] == interaction_state[0]
                                && mouse.just_released(bevy::input::mouse::MouseButton::Left)
                            {
                                match interaction_state[2] {
                                    Some(0u8) => navigation_state.0 = false,
                                    Some(1u8) => navigation_state.0 = true,
                                    _ => (),
                                }
                            }
                            if hud.unwrap_or(false)
                                && window.focused
                                && interaction_state[1].is_some()
                                && (keys.just_pressed(bevy::input::keyboard::KeyCode::Enter)
                                    || keys.just_pressed(bevy::input::keyboard::KeyCode::Space))
                            {
                                match interaction_state[1] {
                                    Some(0u8) => navigation_state.0 = false,
                                    Some(1u8) => navigation_state.0 = true,
                                    _ => (),
                                }
                            }

                            interaction_state[2] = if !hud.unwrap_or(false)
                                || !window.focused
                                || (interaction_state[2] == Some(2u8) && !navigation_state.0)
                                || mouse.just_released(bevy::input::mouse::MouseButton::Left)
                            {
                                None
                            } else if mouse.just_pressed(bevy::input::mouse::MouseButton::Left) {
                                interaction_state[0]
                            } else {
                                interaction_state[2]
                            };

                            navigation_state.1 = if keys.pressed(bevy::input::keyboard::KeyCode::Escape) {
                                (navigation_state.1 + time.delta_secs()).min(7.0)
                            } else {
                                0.0
                            };
                            if navigation_state.1 >= 7.0 {
                                commands.write_message(bevy::app::AppExit::Success);
                            }

                            anim.0[0][0] += ((interaction_state[0] == Some(0u8)) as u8 as f32
                                - anim.0[0][0])
                                .clamp(-time.delta_secs() / 0.15, time.delta_secs() / 0.15);
                            anim.0[0][1] += ((interaction_state[0] == Some(1u8)) as u8 as f32
                                - anim.0[0][1])
                                .clamp(-time.delta_secs() / 0.15, time.delta_secs() / 0.15);
                            anim.0[0][2] += ((interaction_state[0] == Some(2u8)) as u8 as f32
                                - anim.0[0][2])
                                .clamp(-time.delta_secs() / 0.15, time.delta_secs() / 0.15);
                            anim.0[1][0] += ((interaction_state[1] == Some(0u8)) as u8 as f32
                                - anim.0[1][0])
                                .clamp(-time.delta_secs() / 0.11, time.delta_secs() / 0.11);
                            anim.0[1][1] += ((interaction_state[1] == Some(1u8)) as u8 as f32
                                - anim.0[1][1])
                                .clamp(-time.delta_secs() / 0.11, time.delta_secs() / 0.11);
                            anim.0[1][2] += ((interaction_state[1] == Some(2u8)) as u8 as f32
                                - anim.0[1][2])
                                .clamp(-time.delta_secs() / 0.11, time.delta_secs() / 0.11);
                            anim.0[2][0] += (((interaction_state[2] == Some(0u8)) && (interaction_state[0] == Some(0u8)))
                                as u8 as f32
                                - anim.0[2][0])
                                .clamp(-time.delta_secs() / 0.06, time.delta_secs() / 0.06);
                            anim.0[2][1] += (((interaction_state[2] == Some(1u8)) && (interaction_state[0] == Some(1u8)))
                                as u8 as f32
                                - anim.0[2][1])
                                .clamp(-time.delta_secs() / 0.06, time.delta_secs() / 0.06);
                            anim.0[2][2] += (((interaction_state[2] == Some(2u8)) && (interaction_state[0] == Some(2u8)))
                                as u8 as f32
                                - anim.0[2][2])
                                .clamp(-time.delta_secs() / 0.06, time.delta_secs() / 0.06);
                            anim.1[0] += (navigation_state.0 as u8 as f32 - anim.1[0])
                                .clamp(-time.delta_secs() / 0.15, time.delta_secs() / 0.15);
                            anim.1[1] = if hud.unwrap_or(false) && navigation_state.0 {
                                anim.1[1]
                                    + (1.0f32 - anim.1[1]).clamp(
                                        -time.delta_secs() / 0.14,
                                        time.delta_secs() / 0.14,
                                    )
                            } else {
                                0.0
                            };

                            if handles.is_none() {
                                *handles = Some([
                                    images.add(
                                        bevy::image::Image::from_buffer(
                                            &[137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 26, 0, 0, 0, 22, 8, 6, 0, 0, 0, 222, 136, 140, 181, 0, 0, 0, 160, 73, 68, 65, 84, 120, 218, 99, 248, 255, 255, 63, 3, 61, 48, 57, 154, 140, 160, 152, 166, 22, 21, 2, 241, 15, 40, 46, 164, 133, 69, 50, 64, 124, 236, 63, 38, 56, 6, 149, 163, 138, 69, 49, 64, 252, 241, 63, 110, 240, 17, 170, 134, 108, 139, 68, 129, 120, 213, 127, 226, 193, 42, 168, 30, 146, 44, 114, 7, 226, 87, 255, 73, 7, 175, 160, 122, 9, 90, 196, 14, 196, 83, 255, 83, 14, 166, 66, 205, 194, 106, 145, 37, 16, 223, 249, 79, 61, 112, 7, 106, 38, 138, 69, 160, 176, 173, 0, 226, 6, 32, 222, 79, 5, 75, 246, 67, 205, 170, 128, 197, 27, 182, 248, 105, 160, 130, 69, 13, 196, 36, 134, 81, 139, 168, 110, 81, 3, 37, 106, 71, 45, 26, 181, 8, 175, 69, 14, 80, 133, 232, 216, 129, 18, 181, 116, 105, 152, 128, 48, 0, 173, 25, 163, 86, 246, 201, 209, 147, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130],
                                            bevy::image::ImageType::Extension("png"),
                                            bevy::image::CompressedImageFormats::NONE,
                                            true,
                                            bevy::image::ImageSampler::Default,
                                            bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                        )
                                        .expect("embedded home icon must decode"),
                                    ),
                                    images.add(
                                        bevy::image::Image::from_buffer(
                                            &[137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 26, 0, 0, 0, 22, 8, 6, 0, 0, 0, 222, 136, 140, 181, 0, 0, 0, 85, 73, 68, 65, 84, 120, 218, 99, 248, 255, 255, 63, 3, 61, 48, 140, 193, 14, 196, 139, 129, 248, 199, 127, 236, 224, 7, 84, 158, 29, 73, 51, 73, 122, 96, 154, 98, 254, 19, 7, 98, 144, 44, 34, 73, 15, 76, 83, 3, 145, 154, 26, 144, 44, 34, 73, 207, 168, 69, 67, 199, 34, 186, 165, 58, 186, 229, 35, 186, 149, 12, 163, 69, 208, 104, 17, 52, 106, 209, 104, 17, 52, 0, 69, 16, 0, 172, 77, 4, 129, 121, 239, 65, 171, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130],
                                            bevy::image::ImageType::Extension("png"),
                                            bevy::image::CompressedImageFormats::NONE,
                                            true,
                                            bevy::image::ImageSampler::Default,
                                            bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                        )
                                        .expect("embedded registry icon must decode"),
                                    ),
                                    images.add(
                                        bevy::image::Image::from_buffer(
                                            &[137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 26, 0, 0, 0, 26, 8, 6, 0, 0, 0, 169, 74, 76, 206, 0, 0, 0, 225, 73, 68, 65, 84, 120, 218, 99, 248, 255, 255, 63, 3, 61, 48, 195, 168, 69, 180, 176, 136, 29, 136, 207, 254, 135, 128, 43, 64, 236, 135, 69, 141, 31, 84, 238, 63, 84, 45, 59, 57, 22, 249, 33, 25, 176, 10, 136, 95, 65, 105, 118, 40, 70, 22, 131, 57, 200, 143, 28, 139, 10, 161, 6, 177, 35, 249, 176, 15, 136, 119, 64, 113, 31, 154, 220, 43, 168, 30, 146, 45, 10, 5, 226, 99, 88, 130, 243, 10, 20, 163, 7, 211, 49, 168, 30, 146, 45, 210, 130, 26, 136, 44, 54, 7, 136, 179, 160, 120, 14, 154, 220, 21, 168, 30, 178, 82, 221, 99, 36, 151, 135, 66, 131, 11, 38, 215, 135, 228, 3, 118, 168, 90, 178, 147, 247, 84, 104, 4, 203, 0, 241, 126, 32, 230, 131, 242, 253, 160, 236, 253, 80, 57, 63, 168, 90, 178, 45, 210, 130, 166, 170, 197, 64, 108, 137, 148, 148, 97, 73, 222, 18, 42, 183, 10, 95, 176, 17, 155, 97, 119, 32, 249, 12, 29, 192, 124, 178, 131, 26, 37, 3, 93, 44, 162, 91, 208, 209, 45, 49, 208, 37, 121, 211, 45, 195, 210, 173, 8, 162, 91, 161, 74, 183, 106, 130, 110, 21, 223, 104, 227, 100, 112, 88, 4, 0, 80, 10, 72, 141, 156, 49, 192, 226, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130],
                                            bevy::image::ImageType::Extension("png"),
                                            bevy::image::CompressedImageFormats::NONE,
                                            true,
                                            bevy::image::ImageSampler::Default,
                                            bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                        )
                                        .expect("embedded application icon must decode"),
                                    ),
                                    images.add(
                                        bevy::image::Image::from_buffer(
                                            &[137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 26, 0, 0, 0, 26, 8, 6, 0, 0, 0, 169, 74, 76, 206, 0, 0, 2, 65, 73, 68, 65, 84, 120, 156, 189, 150, 189, 106, 21, 81, 16, 128, 191, 185, 55, 49, 4, 99, 162, 68, 208, 70, 33, 32, 8, 42, 130, 16, 68, 252, 41, 4, 81, 176, 177, 18, 11, 125, 3, 75, 193, 198, 206, 214, 87, 8, 121, 5, 27, 219, 96, 165, 77, 42, 27, 3, 65, 20, 193, 42, 38, 138, 177, 72, 52, 201, 253, 44, 238, 172, 217, 172, 187, 119, 239, 109, 28, 24, 230, 236, 153, 191, 51, 115, 102, 230, 108, 208, 2, 106, 7, 232, 182, 136, 245, 34, 98, 175, 205, 214, 127, 129, 104, 98, 168, 157, 136, 232, 169, 151, 128, 123, 64, 15, 232, 84, 196, 138, 189, 119, 17, 241, 82, 141, 136, 112, 104, 239, 106, 36, 78, 171, 27, 14, 7, 87, 82, 183, 54, 205, 213, 19, 86, 97, 7, 88, 5, 4, 126, 55, 240, 123, 192, 58, 240, 93, 141, 148, 29, 30, 82, 9, 245, 136, 186, 172, 246, 212, 69, 245, 102, 226, 98, 238, 45, 171, 115, 101, 157, 58, 24, 107, 98, 68, 132, 121, 79, 63, 213, 53, 250, 247, 185, 18, 17, 175, 211, 232, 124, 238, 173, 69, 196, 39, 181, 59, 168, 242, 26, 29, 85, 34, 27, 207, 207, 201, 210, 29, 76, 38, 29, 207, 22, 24, 152, 178, 182, 59, 34, 171, 168, 108, 36, 56, 88, 173, 70, 68, 175, 205, 78, 107, 68, 21, 216, 140, 136, 93, 0, 117, 115, 20, 197, 97, 83, 215, 165, 31, 213, 3, 245, 84, 178, 174, 50, 66, 133, 181, 58, 202, 162, 144, 126, 186, 166, 128, 91, 201, 234, 228, 222, 177, 97, 157, 213, 130, 218, 201, 166, 157, 83, 215, 212, 213, 26, 153, 21, 117, 87, 189, 145, 223, 109, 51, 177, 214, 81, 55, 233, 98, 118, 254, 195, 116, 60, 145, 24, 234, 163, 228, 45, 169, 221, 145, 29, 165, 82, 168, 215, 178, 41, 223, 102, 9, 215, 69, 253, 166, 56, 200, 200, 81, 149, 162, 89, 82, 127, 169, 183, 213, 41, 117, 38, 105, 177, 62, 172, 222, 81, 183, 213, 247, 234, 88, 145, 242, 81, 156, 220, 31, 110, 150, 30, 128, 199, 77, 81, 69, 197, 73, 209, 140, 93, 96, 5, 56, 13, 188, 40, 216, 13, 103, 43, 108, 60, 1, 182, 128, 51, 192, 55, 248, 219, 236, 3, 163, 121, 154, 39, 92, 104, 77, 193, 190, 238, 66, 234, 60, 107, 138, 170, 28, 13, 234, 73, 117, 93, 221, 82, 207, 101, 222, 39, 146, 214, 97, 193, 59, 155, 58, 95, 213, 19, 101, 155, 112, 112, 214, 21, 235, 243, 192, 44, 253, 65, 58, 67, 63, 101, 189, 164, 117, 184, 151, 244, 11, 176, 9, 28, 7, 46, 86, 237, 215, 13, 213, 40, 209, 163, 17, 177, 23, 17, 59, 73, 235, 112, 55, 159, 135, 105, 246, 39, 205, 245, 138, 173, 253, 69, 169, 16, 102, 129, 87, 192, 101, 224, 3, 240, 28, 248, 81, 136, 53, 28, 172, 155, 114, 23, 128, 13, 224, 46, 176, 12, 68, 49, 217, 255, 169, 186, 156, 109, 211, 192, 60, 48, 1, 28, 202, 212, 13, 130, 160, 255, 172, 111, 3, 159, 35, 226, 163, 109, 63, 42, 67, 53, 91, 11, 212, 77, 145, 90, 163, 233, 172, 245, 81, 108, 242, 83, 247, 16, 254, 1, 138, 35, 55, 114, 200, 252, 29, 22, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130],
                                            bevy::image::ImageType::Extension("png"),
                                            bevy::image::CompressedImageFormats::NONE,
                                            true,
                                            bevy::image::ImageSampler::Default,
                                            bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                        )
                                        .expect("embedded flask icon must decode"),
                                    ),
                                    images.add(
                                        bevy::image::Image::from_buffer(
                                            &[137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 26, 0, 0, 0, 26, 8, 6, 0, 0, 0, 169, 74, 76, 206, 0, 0, 1, 111, 73, 68, 65, 84, 120, 156, 237, 148, 63, 47, 131, 81, 20, 198, 159, 251, 182, 108, 42, 37, 210, 197, 32, 49, 48, 25, 208, 8, 73, 135, 198, 32, 190, 140, 207, 96, 17, 187, 81, 24, 172, 34, 38, 22, 131, 193, 38, 26, 11, 139, 79, 96, 208, 133, 52, 42, 254, 244, 103, 112, 222, 244, 208, 146, 247, 109, 223, 193, 208, 39, 185, 57, 247, 222, 220, 251, 60, 247, 220, 243, 71, 26, 224, 191, 35, 100, 65, 2, 4, 73, 145, 164, 86, 8, 129, 44, 56, 59, 4, 128, 124, 230, 196, 78, 32, 2, 114, 110, 61, 1, 84, 129, 209, 248, 1, 153, 136, 184, 249, 36, 176, 9, 220, 243, 133, 53, 219, 207, 253, 206, 144, 76, 36, 103, 118, 26, 216, 5, 30, 105, 227, 25, 152, 249, 249, 152, 94, 68, 242, 102, 203, 192, 155, 19, 120, 49, 123, 144, 218, 27, 11, 178, 31, 177, 200, 34, 240, 4, 28, 1, 219, 192, 59, 240, 97, 98, 253, 123, 99, 4, 243, 38, 82, 115, 123, 91, 230, 205, 126, 106, 111, 236, 66, 193, 70, 17, 40, 1, 21, 160, 110, 164, 183, 192, 184, 157, 59, 182, 56, 77, 89, 38, 38, 243, 198, 5, 187, 2, 52, 129, 6, 80, 179, 47, 194, 217, 59, 224, 210, 206, 148, 237, 78, 186, 47, 139, 107, 0, 56, 4, 54, 128, 27, 160, 213, 69, 236, 21, 88, 240, 15, 236, 11, 192, 181, 35, 142, 179, 237, 193, 137, 244, 222, 25, 220, 23, 174, 2, 87, 192, 153, 75, 231, 38, 176, 156, 86, 164, 163, 85, 208, 110, 31, 35, 146, 78, 36, 45, 73, 58, 183, 189, 33, 73, 69, 73, 85, 73, 13, 73, 36, 109, 162, 221, 2, 24, 11, 149, 36, 213, 37, 93, 72, 218, 9, 33, 172, 75, 58, 149, 52, 43, 41, 10, 33, 180, 18, 185, 242, 23, 92, 66, 204, 185, 249, 48, 176, 103, 113, 26, 243, 231, 50, 3, 223, 59, 245, 10, 80, 200, 84, 200, 138, 48, 248, 117, 38, 196, 9, 197, 251, 175, 153, 1, 210, 224, 19, 115, 55, 200, 122, 225, 199, 109, 89, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130],
                                            bevy::image::ImageType::Extension("png"),
                                            bevy::image::CompressedImageFormats::NONE,
                                            true,
                                            bevy::image::ImageSampler::Default,
                                            bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                        )
                                        .expect("embedded rocket icon must decode"),
                                    ),
                                    images.add(
                                        bevy::image::Image::from_buffer(
                                            &[137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122, 244, 0, 0, 0, 251, 73, 68, 65, 84, 120, 156, 237, 151, 61, 110, 194, 64, 16, 70, 223, 44, 136, 131, 164, 70, 162, 200, 173, 56, 3, 84, 156, 129, 138, 155, 112, 135, 116, 72, 105, 66, 110, 65, 129, 146, 248, 165, 240, 88, 114, 164, 132, 38, 254, 105, 246, 171, 102, 181, 171, 125, 159, 191, 177, 100, 15, 0, 106, 168, 37, 235, 141, 122, 84, 95, 213, 198, 255, 171, 201, 187, 142, 234, 38, 25, 69, 13, 122, 240, 200, 122, 175, 222, 6, 128, 254, 165, 155, 186, 255, 193, 85, 151, 185, 56, 245, 14, 126, 58, 204, 211, 119, 106, 242, 206, 78, 167, 100, 46, 187, 20, 118, 185, 113, 31, 24, 252, 155, 145, 123, 214, 59, 128, 80, 215, 192, 11, 176, 0, 10, 208, 246, 102, 60, 9, 52, 192, 23, 240, 92, 128, 45, 176, 202, 205, 177, 225, 125, 198, 10, 216, 134, 250, 6, 60, 77, 104, 0, 218, 20, 0, 222, 67, 245, 225, 209, 145, 85, 122, 110, 230, 144, 83, 188, 116, 143, 20, 101, 70, 56, 208, 182, 160, 26, 168, 6, 170, 129, 106, 160, 26, 168, 6, 234, 231, 248, 74, 155, 194, 148, 73, 116, 188, 107, 1, 206, 180, 41, 52, 19, 26, 104, 146, 121, 158, 255, 183, 60, 34, 46, 192, 33, 13, 124, 48, 110, 43, 76, 198, 2, 56, 68, 196, 101, 254, 209, 204, 185, 135, 211, 222, 98, 150, 241, 252, 27, 170, 132, 79, 195, 98, 249, 236, 84, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130],
                                            bevy::image::ImageType::Extension("png"),
                                            bevy::image::CompressedImageFormats::NONE,
                                            true,
                                            bevy::image::ImageSampler::Default,
                                            bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                        )
                                        .expect("embedded rounded surface must decode"),
                                    ),
                                    images.add(
                                        bevy::image::Image::from_buffer(
                                            &[137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122, 244, 0, 0, 1, 237, 73, 68, 65, 84, 120, 156, 237, 151, 61, 107, 20, 81, 20, 134, 159, 51, 187, 9, 162, 136, 27, 140, 141, 65, 109, 2, 18, 176, 183, 178, 176, 88, 100, 177, 179, 181, 72, 103, 145, 202, 74, 16, 139, 228, 47, 216, 88, 8, 254, 0, 177, 181, 16, 132, 84, 54, 106, 29, 16, 59, 77, 64, 59, 21, 53, 235, 71, 118, 31, 139, 61, 55, 14, 203, 130, 176, 153, 196, 102, 95, 56, 220, 185, 115, 63, 222, 247, 158, 185, 51, 243, 222, 160, 6, 181, 21, 17, 131, 90, 125, 1, 144, 131, 227, 107, 153, 119, 156, 35, 106, 100, 85, 68, 12, 213, 51, 192, 45, 160, 7, 172, 100, 159, 96, 58, 20, 241, 219, 192, 11, 224, 65, 68, 108, 169, 17, 17, 238, 11, 168, 145, 95, 3, 30, 1, 75, 57, 176, 15, 12, 167, 36, 175, 227, 68, 150, 191, 129, 59, 192, 125, 160, 138, 136, 1, 106, 43, 69, 116, 253, 139, 103, 234, 117, 117, 81, 237, 100, 156, 154, 34, 58, 234, 130, 122, 89, 125, 88, 155, 127, 61, 57, 219, 168, 161, 158, 86, 63, 100, 227, 221, 6, 86, 60, 17, 106, 79, 253, 174, 14, 212, 43, 245, 134, 123, 73, 254, 60, 235, 45, 181, 157, 226, 154, 136, 150, 58, 159, 115, 111, 212, 178, 92, 161, 206, 169, 47, 83, 85, 55, 7, 180, 15, 97, 245, 85, 198, 162, 250, 69, 221, 85, 47, 144, 207, 232, 115, 166, 166, 147, 157, 167, 221, 245, 255, 20, 145, 229, 102, 102, 161, 91, 49, 122, 85, 234, 113, 20, 40, 223, 129, 65, 117, 68, 132, 227, 40, 25, 142, 255, 37, 96, 31, 51, 1, 51, 1, 51, 1, 51, 1, 51, 1, 51, 1, 69, 64, 249, 13, 31, 138, 15, 152, 128, 194, 99, 149, 149, 226, 128, 6, 147, 251, 55, 142, 189, 44, 231, 42, 224, 7, 176, 3, 28, 3, 150, 211, 13, 53, 254, 104, 138, 203, 82, 143, 3, 151, 24, 89, 244, 157, 42, 34, 250, 192, 38, 208, 2, 214, 242, 192, 208, 42, 246, 169, 65, 242, 249, 136, 24, 2, 55, 128, 179, 192, 22, 240, 182, 216, 242, 139, 234, 47, 245, 167, 218, 171, 13, 172, 154, 136, 218, 124, 231, 212, 247, 233, 7, 111, 150, 155, 229, 96, 114, 59, 27, 250, 105, 157, 151, 104, 8, 234, 73, 117, 85, 125, 151, 28, 79, 82, 92, 123, 252, 104, 182, 14, 108, 228, 184, 93, 224, 21, 7, 219, 152, 193, 232, 13, 91, 97, 148, 118, 128, 199, 192, 26, 240, 105, 92, 101, 17, 115, 85, 125, 170, 126, 179, 57, 12, 212, 215, 234, 234, 56, 95, 140, 137, 104, 71, 196, 94, 94, 159, 7, 150, 107, 43, 153, 6, 229, 251, 242, 17, 120, 147, 89, 30, 17, 231, 233, 248, 15, 174, 141, 117, 33, 160, 231, 172, 167, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130],
                                            bevy::image::ImageType::Extension("png"),
                                            bevy::image::CompressedImageFormats::NONE,
                                            true,
                                            bevy::image::ImageSampler::Default,
                                            bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                        )
                                        .expect("embedded focus outline must decode"),
                                    ),
                                ]);
                            }

                            if slots.is_none() {
                                handles.as_ref().cloned().into_iter().for_each(
                                    |[
                                        home_icon,
                                        registry_icon,
                                        app_icon,
                                        flask_icon,
                                        rocket_icon,
                                        round_solid,
                                        round_outline,
                                    ]: [bevy::asset::Handle<bevy::image::Image>; 7]| {
                                        *slots = Some([
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.090; 3],
                                                        ),
                                                        custom_size: Some(sidebar.size()),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        sidebar.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - sidebar.center().y,
                                                        1.0,
                                                    ),
                                                    if title_metrics.0 == 0.0 && hud.unwrap_or(false) {
                                                        bevy::camera::visibility::Visibility::Inherited
                                                    } else {
                                                        bevy::camera::visibility::Visibility::Hidden
                                                    },
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.204; 3],
                                                        ),
                                                        custom_size: Some(bevy::math::Vec2::new(
                                                            1.5,
                                                            sidebar.height(),
                                                        )),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        title_metrics.1 - 0.75
                                                            - viewport.x / 2.0,
                                                        0.0,
                                                        2.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.204; 3],
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(panel.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        panel.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - panel.center().y,
                                                        1.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.063; 3],
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(bevy::math::Vec2::new(
                                                            (panel.width() - 2.0).max(1.0),
                                                            (panel.height() - 2.0).max(1.0),
                                                        )),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        panel.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - panel.center().y,
                                                        1.1,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.133; 3],
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(home_tab.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        home_tab.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - home_tab.center().y,
                                                        4.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.133; 3],
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(registry_tab.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        registry_tab.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - registry_tab.center().y,
                                                        4.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.463, 0.463, 0.463, 0.0,
                                                        ),
                                                        image: round_outline.clone(),
                                                        custom_size: Some(home_tab.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        home_tab.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - home_tab.center().y,
                                                        4.5,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.463, 0.463, 0.463, 0.0,
                                                        ),
                                                        image: round_outline.clone(),
                                                        custom_size: Some(registry_tab.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        registry_tab.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - registry_tab.center().y,
                                                        4.5,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.173; 3],
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(app_tile.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        app_tile.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - app_tile.center().y,
                                                        5.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.902; 3],
                                                        ),
                                                        image: app_icon.clone(),
                                                        rect: Some(bevy::math::Rect::new(
                                                            6.0, 4.0, 20.0, 22.0,
                                                        )),
                                                        custom_size: Some(bevy::math::Vec2::new(
                                                            24.0, 31.0,
                                                        )),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        app_tile.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - app_tile.center().y,
                                                        6.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Text2d(std::string::String::from(
                                                        if cfg!(target_os = "linux") {
                                                            "Linux-Subsystem-for-Arxumbra"
                                                        } else if cfg!(target_os = "macos") {
                                                            "macOS-Subsystem-for-Arxumbra"
                                                        } else {
                                                            "Windows-Subsystem-for-Arxumbra"
                                                        },
                                                    )),
                                                    bevy::text::TextFont {
                                                        font_size: bevy::text::FontSize::Px(17.0),
                                                        weight: bevy::text::FontWeight::SEMIBOLD,
                                                        ..bevy::text::TextFont::default()
                                                    },
                                                    bevy::text::TextColor(
                                                        bevy::color::Color::srgb_from_array(
                                                            [0.902; 3],
                                                        ),
                                                    ),
                                                    bevy::text::TextBounds::UNBOUNDED,
                                                    bevy::text::TextLayout::no_wrap(),
                                                    bevy::sprite::Anchor::CENTER_LEFT,
                                                    bevy::transform::components::Transform::from_xyz(
                                                        app_tile.max.x + 14.0 - viewport.x / 2.0,
                                                        viewport.y / 2.0 - app_tile.center().y,
                                                        6.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.173; 3],
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(home_tile.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        home_tile.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - home_tile.center().y,
                                                        5.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.902; 3],
                                                        ),
                                                        image: home_icon.clone(),
                                                        custom_size: Some(bevy::math::Vec2::new(
                                                            24.0, 20.0,
                                                        )),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        home_tile.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - home_tile.center().y,
                                                        6.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.173; 3],
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(registry_tile.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        registry_tile.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - registry_tile.center().y,
                                                        5.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.902; 3],
                                                        ),
                                                        image: registry_icon.clone(),
                                                        custom_size: Some(bevy::math::Vec2::new(
                                                            24.0, 20.0,
                                                        )),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        registry_tile.center().x
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - registry_tile.center().y,
                                                        6.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Text2d(std::string::String::from(
                                                        "Home",
                                                    )),
                                                    bevy::text::TextFont {
                                                        font_size: bevy::text::FontSize::Px(16.0),
                                                        ..bevy::text::TextFont::default()
                                                    },
                                                    bevy::text::TextColor(
                                                        bevy::color::Color::srgb_from_array(
                                                            [0.686; 3],
                                                        ),
                                                    ),
                                                    bevy::text::TextBounds::UNBOUNDED,
                                                    bevy::text::TextLayout::no_wrap(),
                                                    bevy::sprite::Anchor::CENTER_LEFT,
                                                    bevy::transform::components::Transform::from_xyz(
                                                        home_tile.max.x + 12.0 - viewport.x / 2.0,
                                                        viewport.y / 2.0 - home_tab.center().y,
                                                        6.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Text2d(std::string::String::from(
                                                        "Extension Registry",
                                                    )),
                                                    bevy::text::TextFont {
                                                        font_size: bevy::text::FontSize::Px(16.0),
                                                        ..bevy::text::TextFont::default()
                                                    },
                                                    bevy::text::TextColor(
                                                        bevy::color::Color::srgb_from_array(
                                                            [0.686; 3],
                                                        ),
                                                    ),
                                                    bevy::text::TextBounds::UNBOUNDED,
                                                    bevy::text::TextLayout::no_wrap(),
                                                    bevy::sprite::Anchor::CENTER_LEFT,
                                                    bevy::transform::components::Transform::from_xyz(
                                                        registry_tile.max.x + 12.0
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - registry_tab.center().y,
                                                        6.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.204, 0.204, 0.204, 0.0,
                                                        ),
                                                        custom_size: Some(bevy::math::Vec2::new(
                                                            section_divider.width(),
                                                            1.5,
                                                        )),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        section_divider.center().x
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - section_divider.center().y,
                                                        2.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.133, 0.133, 0.133, 0.0,
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(
                                                            registry_header_tile.size(),
                                                        ),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        registry_header_tile.center().x
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - registry_header_tile.center().y,
                                                        3.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.902, 0.902, 0.902, 0.0,
                                                        ),
                                                        image: registry_icon.clone(),
                                                        custom_size: Some(bevy::math::Vec2::new(
                                                            28.0, 24.0,
                                                        )),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        registry_header_tile.center().x
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - registry_header_tile.center().y,
                                                        4.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Text2d(std::string::String::from(
                                                        "Extension Registry",
                                                    )),
                                                    bevy::text::TextFont {
                                                        font_size: bevy::text::FontSize::Px(20.0),
                                                        weight: bevy::text::FontWeight::SEMIBOLD,
                                                        ..bevy::text::TextFont::default()
                                                    },
                                                    bevy::text::TextColor(bevy::color::Color::srgba(
                                                        0.902, 0.902, 0.902, 0.0,
                                                    )),
                                                    bevy::text::TextBounds::UNBOUNDED,
                                                    bevy::text::TextLayout::no_wrap(),
                                                    bevy::sprite::Anchor::CENTER_LEFT,
                                                    bevy::transform::components::Transform::from_xyz(
                                                        registry_header_tile.max.x + 14.0
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - registry_header_tile.center().y,
                                                        4.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgb_from_array(
                                                            [0.204; 3],
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(card.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        card.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - card.center().y,
                                                        2.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.090, 0.090, 0.090, 0.0,
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(bevy::math::Vec2::new(
                                                            (card.width() - 2.0).max(1.0),
                                                            (card.height() - 2.0).max(1.0),
                                                        )),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        card.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - card.center().y,
                                                        2.1,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.173, 0.173, 0.173, 0.0,
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(flask_tile.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        flask_tile.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - flask_tile.center().y,
                                                        4.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.902, 0.902, 0.902, 0.0,
                                                        ),
                                                        image: flask_icon.clone(),
                                                        custom_size: Some(bevy::math::Vec2::new(
                                                            25.5, 30.0,
                                                        )),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        flask_tile.center().x - viewport.x / 2.0,
                                                        viewport.y / 2.0 - flask_tile.center().y,
                                                        5.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Text2d(std::string::String::from(
                                                        "Laboratory",
                                                    )),
                                                    bevy::text::TextFont {
                                                        font_size: bevy::text::FontSize::Px(18.0),
                                                        weight: bevy::text::FontWeight::SEMIBOLD,
                                                        ..bevy::text::TextFont::default()
                                                    },
                                                    bevy::text::TextColor(bevy::color::Color::srgba(
                                                        0.902, 0.902, 0.902, 0.0,
                                                    )),
                                                    bevy::text::TextBounds::UNBOUNDED,
                                                    bevy::text::TextLayout::no_wrap(),
                                                    bevy::sprite::Anchor::CENTER_LEFT,
                                                    bevy::transform::components::Transform::from_xyz(
                                                        flask_tile.max.x + 12.0
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0 - flask_tile.center().y,
                                                        5.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.204, 0.204, 0.204, 0.0,
                                                        ),
                                                        custom_size: Some(bevy::math::Vec2::new(
                                                            card_divider.width(),
                                                            card_divider.height(),
                                                        )),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        card_divider.center().x
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - card_divider.center().y,
                                                        3.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.133, 0.133, 0.133, 0.0,
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(launch_button.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        launch_button.center().x
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - launch_button.center().y,
                                                        3.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.463, 0.463, 0.463, 0.0,
                                                        ),
                                                        image: round_outline.clone(),
                                                        custom_size: Some(launch_button.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        launch_button.center().x
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - launch_button.center().y,
                                                        3.5,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.204, 0.204, 0.204, 0.0,
                                                        ),
                                                        image: round_solid.clone(),
                                                        custom_size: Some(rocket_tile.size()),
                                                        image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                            bevy::sprite::TextureSlicer {
                                                                border: bevy::sprite::BorderRect::all(
                                                                    8.0,
                                                                ),
                                                                center_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                sides_scale_mode: bevy::sprite::SliceScaleMode::Stretch,
                                                                max_corner_scale: 2.0,
                                                            },
                                                        ),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        rocket_tile.center().x
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0 - rocket_tile.center().y,
                                                        4.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Sprite {
                                                        color: bevy::color::Color::srgba(
                                                            0.686, 0.686, 0.686, 0.0,
                                                        ),
                                                        image: rocket_icon.clone(),
                                                        custom_size: Some(bevy::math::Vec2::splat(
                                                            26.0,
                                                        )),
                                                        ..bevy::sprite::Sprite::default()
                                                    },
                                                    bevy::transform::components::Transform::from_xyz(
                                                        rocket_tile.center().x
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - rocket_tile.center().y,
                                                        5.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                            commands
                                                .spawn((
                                                    bevy::sprite::Text2d(std::string::String::from(
                                                        "Launch",
                                                    )),
                                                    bevy::text::TextFont {
                                                        font_size: bevy::text::FontSize::Px(15.0),
                                                        ..bevy::text::TextFont::default()
                                                    },
                                                    bevy::text::TextColor(bevy::color::Color::srgba(
                                                        0.686, 0.686, 0.686, 0.0,
                                                    )),
                                                    bevy::text::TextBounds::UNBOUNDED,
                                                    bevy::text::TextLayout::no_wrap(),
                                                    bevy::sprite::Anchor::CENTER_LEFT,
                                                    bevy::transform::components::Transform::from_xyz(
                                                        rocket_tile.max.x + 10.0
                                                            - viewport.x / 2.0,
                                                        viewport.y / 2.0
                                                            - launch_button.center().y,
                                                        5.0,
                                                    ),
                                                    bevy::camera::visibility::Visibility::Hidden,
                                                ))
                                                .id(),
                                        ]);
                                    },
                                );
                            }

                            (*slots).into_iter().for_each(
                                |[
                                    sidebar_entity,
                                    sidebar_divider_entity,
                                    panel_border_entity,
                                    panel_fill_entity,
                                    home_tab_entity,
                                    registry_tab_entity,
                                    home_ring_entity,
                                    registry_ring_entity,
                                    app_tile_entity,
                                    app_icon_entity,
                                    title_entity,
                                    home_tile_entity,
                                    home_icon_entity,
                                    registry_tile_entity,
                                    registry_icon_entity,
                                    home_label_entity,
                                    registry_label_entity,
                                    section_divider_entity,
                                    section_tile_entity,
                                    section_icon_entity,
                                    section_title_entity,
                                    card_border_entity,
                                    card_fill_entity,
                                    flask_tile_entity,
                                    flask_icon_entity,
                                    card_title_entity,
                                    card_divider_entity,
                                    launch_button_entity,
                                    launch_ring_entity,
                                    rocket_tile_entity,
                                    rocket_icon_entity,
                                    launch_label_entity,
                                ]: [bevy::ecs::entity::Entity; 32]| {
                                    [
                                        (
                                            sidebar_entity,
                                            bevy::color::Color::srgb_from_array([0.090; 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            sidebar_divider_entity,
                                            bevy::color::Color::srgb_from_array([0.204; 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            panel_border_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.204; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            panel_fill_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.063; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            home_tab_entity,
                                            bevy::color::Color::srgb_from_array([
                                                0.133
                                                    + (0.173 - 0.133) * anim.0[0][0]
                                                    + ((0.110 - 0.133)
                                                        - (0.173 - 0.133) * anim.0[0][0])
                                                        * anim.0[2][0]
                                                    + (0.855
                                                        + (0.894 - 0.855) * anim.0[0][0]
                                                        + ((0.792 - 0.855)
                                                            - (0.894 - 0.855) * anim.0[0][0])
                                                            * anim.0[2][0]
                                                        - (0.133
                                                            + (0.173 - 0.133) * anim.0[0][0]
                                                            + ((0.110 - 0.133)
                                                                - (0.173 - 0.133) * anim.0[0][0])
                                                                * anim.0[2][0]))
                                                        * (1.0 - anim.1[0]); 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            registry_tab_entity,
                                            bevy::color::Color::srgb_from_array([
                                                0.133
                                                    + (0.173 - 0.133) * anim.0[0][1]
                                                    + ((0.110 - 0.133)
                                                        - (0.173 - 0.133) * anim.0[0][1])
                                                        * anim.0[2][1]
                                                    + (0.855
                                                        + (0.894 - 0.855) * anim.0[0][1]
                                                        + ((0.792 - 0.855)
                                                            - (0.894 - 0.855) * anim.0[0][1])
                                                            * anim.0[2][1]
                                                        - (0.133
                                                            + (0.173 - 0.133) * anim.0[0][1]
                                                            + ((0.110 - 0.133)
                                                                - (0.173 - 0.133) * anim.0[0][1])
                                                                * anim.0[2][1]))
                                                        * anim.1[0]; 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            home_ring_entity,
                                            bevy::color::Color::srgba(
                                                0.463,
                                                0.463,
                                                0.463,
                                                anim.0[1][0],
                                            ),
                                            hud.unwrap_or(false) && anim.0[1][0] > 0.0,
                                        ),
                                        (
                                            registry_ring_entity,
                                            bevy::color::Color::srgba(
                                                0.463,
                                                0.463,
                                                0.463,
                                                anim.0[1][1],
                                            ),
                                            hud.unwrap_or(false) && anim.0[1][1] > 0.0,
                                        ),
                                        (
                                            app_tile_entity,
                                            bevy::color::Color::srgb_from_array([0.173; 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            app_icon_entity,
                                            bevy::color::Color::srgb_from_array([0.902; 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            home_tile_entity,
                                            bevy::color::Color::srgb_from_array([
                                                0.173
                                                    + (0.204 - 0.173) * (1.0 - anim.1[0])
                                                    + (0.173 - 0.133) * anim.0[0][0]
                                                    - 0.050 * anim.0[2][0]; 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            home_icon_entity,
                                            bevy::color::Color::srgb_from_array([
                                                0.686
                                                    + (0.902 - 0.686) * anim.0[0][0]
                                                    + (0.902
                                                        - (0.686
                                                            + (0.902 - 0.686) * anim.0[0][0]))
                                                        * (1.0 - anim.1[0]); 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            registry_tile_entity,
                                            bevy::color::Color::srgb_from_array([
                                                0.173
                                                    + (0.204 - 0.173) * anim.1[0]
                                                    + (0.173 - 0.133) * anim.0[0][1]
                                                    - 0.050 * anim.0[2][1]; 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            registry_icon_entity,
                                            bevy::color::Color::srgb_from_array([
                                                0.686
                                                    + (0.902 - 0.686) * anim.0[0][1]
                                                    + (0.902
                                                        - (0.686
                                                            + (0.902 - 0.686) * anim.0[0][1]))
                                                        * anim.1[0]; 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            section_divider_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.204; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            section_tile_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.133; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            section_icon_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.902; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            card_border_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.204; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            card_fill_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.090; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            flask_tile_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.173; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            flask_icon_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.902; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            card_divider_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.204; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            launch_button_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([
                                                    0.133
                                                        + (0.173 - 0.133) * anim.0[0][2]
                                                        + ((0.110 - 0.133)
                                                            - (0.173 - 0.133) * anim.0[0][2])
                                                            * anim.0[2][2]; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            launch_ring_entity,
                                            bevy::color::Color::srgba(
                                                0.463,
                                                0.463,
                                                0.463,
                                                anim.0[1][2] * anim.1[1],
                                            ),
                                            hud.unwrap_or(false)
                                                && navigation_state.0
                                                && title_metrics.0 > 0.0
                                                && anim.0[1][2] > 0.0,
                                        ),
                                        (
                                            rocket_tile_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([
                                                    0.204
                                                        + (0.04 * anim.0[0][2])
                                                        - (0.05 * anim.0[2][2]); 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            rocket_icon_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([
                                                    0.686
                                                        + (0.902 - 0.686) * anim.0[0][2]; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                    ]
                                    .into_iter()
                                    .for_each(
                                        |(entity, colour, shown): (
                                            bevy::ecs::entity::Entity,
                                            bevy::color::Color,
                                            bool,
                                        )| {
                                            sprites
                                                .get(entity)
                                                .map(
                                                    |(sprite, visibility): (
                                                        Option<&bevy::sprite::Sprite>,
                                                        &bevy::camera::visibility::Visibility,
                                                    )| {
                                                        sprite.into_iter().for_each(
                                                            |sprite: &bevy::sprite::Sprite| {
                                                                if sprite.color != colour {
                                                                    commands
                                                                        .entity(entity)
                                                                        .insert(bevy::sprite::Sprite {
                                                                            color: colour,
                                                                            ..sprite.clone()
                                                                        });
                                                                }
                                                            },
                                                        );
                                                        if (*visibility
                                                            == bevy::camera::visibility::Visibility::Inherited)
                                                            != shown
                                                        {
                                                            commands.entity(entity).insert(if shown {
                                                                bevy::camera::visibility::Visibility::Inherited
                                                            } else {
                                                                bevy::camera::visibility::Visibility::Hidden
                                                            });
                                                        }
                                                    },
                                                )
                                                .ok();
                                        },
                                    );
                                    [
                                        (
                                            title_entity,
                                            bevy::color::Color::srgb_from_array([0.902; 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            home_label_entity,
                                            bevy::color::Color::srgb_from_array([
                                                0.686
                                                    + (0.902 - 0.686) * anim.0[0][0]
                                                    + (0.090
                                                        - (0.686
                                                            + (0.902 - 0.686) * anim.0[0][0]))
                                                        * (1.0 - anim.1[0]); 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            registry_label_entity,
                                            bevy::color::Color::srgb_from_array([
                                                0.686
                                                    + (0.902 - 0.686) * anim.0[0][1]
                                                    + (0.090
                                                        - (0.686
                                                            + (0.902 - 0.686) * anim.0[0][1]))
                                                        * anim.1[0]; 3]),
                                            hud.unwrap_or(false),
                                        ),
                                        (
                                            section_title_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.902; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            card_title_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([0.902; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                        (
                                            launch_label_entity,
                                            <bevy::color::Color as bevy::color::Alpha>::with_alpha(
                                                &bevy::color::Color::srgb_from_array([
                                                    0.686
                                                        + (0.902 - 0.686) * anim.0[0][2]; 3]),
                                                anim.1[1],
                                            ),
                                            hud.unwrap_or(false) && navigation_state.0 && title_metrics.0 > 0.0,
                                        ),
                                    ]
                                    .into_iter()
                                    .for_each(
                                        |(entity, colour, shown): (
                                            bevy::ecs::entity::Entity,
                                            bevy::color::Color,
                                            bool,
                                        )| {
                                            texts
                                                .get(entity)
                                                .map(
                                                    |(text_colour, visibility): (
                                                        Option<&bevy::text::TextColor>,
                                                        &bevy::camera::visibility::Visibility,
                                                    )| {
                                                        text_colour.into_iter().for_each(
                                                            |text_colour: &bevy::text::TextColor| {
                                                                if text_colour.0 != colour {
                                                                    commands.entity(entity).insert(
                                                                        bevy::text::TextColor(colour),
                                                                    );
                                                                }
                                                            },
                                                        );
                                                        if (*visibility
                                                            == bevy::camera::visibility::Visibility::Inherited)
                                                            != shown
                                                        {
                                                            commands.entity(entity).insert(if shown {
                                                                bevy::camera::visibility::Visibility::Inherited
                                                            } else {
                                                                bevy::camera::visibility::Visibility::Hidden
                                                            });
                                                        }
                                                    },
                                                )
                                                .ok();
                                        },
                                    );
                                },
                            );
                        },
                    );
                    if anim.0.iter().flatten().zip(
                        [
                            [
                                (interaction_state[0] == Some(0u8)) as u8 as f32,
                                (interaction_state[0] == Some(1u8)) as u8 as f32,
                                (interaction_state[0] == Some(2u8)) as u8 as f32,
                            ],
                            [
                                (interaction_state[1] == Some(0u8)) as u8 as f32,
                                (interaction_state[1] == Some(1u8)) as u8 as f32,
                                (interaction_state[1] == Some(2u8)) as u8 as f32,
                            ],
                            [
                                ((interaction_state[2] == Some(0u8)
                                    && interaction_state[0] == Some(0u8))
                                    as u8 as f32),
                                ((interaction_state[2] == Some(1u8)
                                    && interaction_state[0] == Some(1u8))
                                    as u8 as f32),
                                ((interaction_state[2] == Some(2u8)
                                    && interaction_state[0] == Some(2u8))
                                    as u8 as f32),
                            ],
                        ]
                        .iter()
                        .flatten(),
                    )
                    .any(|(displayed, target): (&f32, &f32)| {
                        (*displayed - *target).abs() > 0.001
                    })
                        || (anim.1[0] - navigation_state.0 as u8 as f32).abs() > 0.001
                        || (anim.1[1]
                            - if hud.unwrap_or(false) && navigation_state.0 {
                                1.0
                            } else {
                                0.0
                            })
                            .abs()
                            > 0.001
                        || keys.pressed(bevy::input::keyboard::KeyCode::Escape)
                        || title_metrics.0 == 0.0
                        || title_metrics.2 < 1.0
                    {
                        if settings.focused_mode
                            != bevy::winit::UpdateMode::reactive_low_power(
                                std::time::Duration::from_millis(16),
                            )
                        {
                            settings.focused_mode =
                                bevy::winit::UpdateMode::reactive_low_power(
                                    std::time::Duration::from_millis(16),
                                );
                            settings.unfocused_mode = settings.focused_mode;
                        }
                    } else if settings.focused_mode
                        != bevy::winit::UpdateMode::reactive_low_power(
                            std::time::Duration::MAX,
                        )
                    {
                        settings.focused_mode =
                            bevy::winit::UpdateMode::reactive_low_power(
                                std::time::Duration::MAX,
                            );
                        settings.unfocused_mode = settings.focused_mode;
                    }
                });
            },
        )
        .run();
}
