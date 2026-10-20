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
            bevy::log::LogPlugin {
                level: bevy::log::Level::ERROR,
                filter: std::string::String::from("wgpu=error,naga=error,sctk_adwaita=off"),
                ..Default::default()
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
                    title: std::string::String::from(if cfg!(target_os = "linux") {
                        "Linux-Subsystem-for-Arxumbra"
                    } else if cfg!(target_os = "macos") {
                        "macOS-Subsystem-for-Arxumbra"
                    } else {
                        "Windows-Subsystem-for-Arxumbra"
                    }),
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
            0.035, 0.035, 0.035,
        )))
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive(std::time::Duration::from_millis(
                100,
            )),
        })
        .add_systems(
            bevy::app::Startup,
            |mut commands: bevy::ecs::system::Commands| {
                commands.spawn((bevy::camera::Camera2d, bevy::render::view::Msaa::Off));
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
             time: bevy::ecs::system::Res<bevy::time::Time>,
             windows: bevy::ecs::system::Query<
                &bevy::window::Window,
                bevy::ecs::query::With<bevy::window::PrimaryWindow>,
            >,
             mut images: bevy::ecs::system::ResMut<bevy::asset::Assets<bevy::image::Image>>,
             mut commands: bevy::ecs::system::Commands,
             mut sprites: bevy::ecs::system::Query<
                (
                    &mut bevy::sprite::Sprite,
                    &mut bevy::camera::visibility::Visibility,
                ),
                bevy::ecs::query::Without<bevy::text::TextColor>,
            >,
             mut texts: bevy::ecs::system::Query<
                (
                    &mut bevy::text::TextColor,
                    &mut bevy::camera::visibility::Visibility,
                ),
                bevy::ecs::query::Without<bevy::sprite::Sprite>,
            >,
             mut hud: bevy::ecs::system::Local<bool>,
             mut boot: bevy::ecs::system::Local<bool>,
             mut selection: bevy::ecs::system::Local<(bool, f32, f32)>,
             mut nav: bevy::ecs::system::Local<[Option<bool>; 3]>,
             mut anim: bevy::ecs::system::Local<([[f32; 2]; 3], [f32; 2])>,
             mut geometry: bevy::ecs::system::Local<(
                [f32; 3],
                Option<[bevy::math::Rect; 4]>,
            )>,
             mut slots: bevy::ecs::system::Local<
                Option<[bevy::ecs::entity::Entity; 14]>,
            >,
             mut handles: bevy::ecs::system::Local<
                Option<[bevy::asset::Handle<bevy::image::Image>; 3]>,
            >| {
                windows
                    .single()
                    .into_iter()
                    .for_each(|window| {
                        if !*boot {
                            *hud = true;
                            *boot = true;
                        }

                        if geometry.0[0] != window.width() || geometry.0[1] != window.height() {
                            geometry.0 = [
                                window.width(),
                                window.height(),
                                (window.width() * 0.22)
                                    .clamp(200.0, 280.0)
                                    .min(window.width() * 0.5),
                            ];
                            geometry.1 = Some([
                                bevy::math::Rect::new(0.0, 0.0, geometry.0[2], geometry.0[1]),
                                bevy::math::Rect::new(
                                    geometry.0[2] + 20.0,
                                    24.0,
                                    (geometry.0[0] - 24.0).max(geometry.0[2] + 60.0),
                                    (geometry.0[1] - 24.0).max(48.0),
                                ),
                                bevy::math::Rect::new(
                                    20.0,
                                    78.0,
                                    (geometry.0[2] - 20.0).max(21.0),
                                    130.0f32.min(geometry.0[1] - 8.0).max(79.0),
                                ),
                                bevy::math::Rect::new(
                                    20.0,
                                    138.0,
                                    (geometry.0[2] - 20.0).max(21.0),
                                    190.0f32.min(geometry.0[1] - 8.0).max(139.0),
                                ),
                            ]);
                            (*slots).into_iter().flatten().for_each(|entity| {
                                commands.entity(entity).despawn();
                            });
                            *slots = None;
                        }

                        geometry.1.into_iter().for_each(|[sidebar, panel, home_tab, registry_tab]| {
                            *hud = if keys.just_pressed(bevy::input::keyboard::KeyCode::F1) {
                                !*hud
                            } else {
                                *hud
                            };

                            nav[0] = match window.cursor_position() {
                                Some(cursor)
                                    if *hud && window.focused && home_tab.contains(cursor) =>
                                {
                                    Some(false)
                                }
                                Some(cursor)
                                    if *hud && window.focused && registry_tab.contains(cursor) =>
                                {
                                    Some(true)
                                }
                                _ => None,
                            };

                            nav[1] = if !*hud || !window.focused {
                                None
                            } else if keys.just_pressed(bevy::input::keyboard::KeyCode::Tab) {
                                if keys.pressed(bevy::input::keyboard::KeyCode::ShiftLeft)
                                    || keys.pressed(bevy::input::keyboard::KeyCode::ShiftRight)
                                {
                                    if nav[1] == Some(true) {
                                        Some(false)
                                    } else {
                                        Some(true)
                                    }
                                } else if nav[1] == Some(false) {
                                    Some(true)
                                } else {
                                    Some(false)
                                }
                            } else {
                                nav[1]
                            };

                            selection.0 = match nav[2] {
                                Some(armed)
                                    if mouse.just_released(
                                        bevy::input::mouse::MouseButton::Left,
                                    ) && nav[0] == Some(armed)
                                        && time.elapsed_secs() - selection.2 <= 1.5f32 =>
                                {
                                    armed
                                }
                                _ => match nav[1] {
                                    Some(focused)
                                        if (keys.just_pressed(
                                            bevy::input::keyboard::KeyCode::Enter,
                                        ) || keys.just_pressed(
                                            bevy::input::keyboard::KeyCode::Space,
                                        )) && *hud =>
                                    {
                                        focused
                                    }
                                    _ => selection.0,
                                },
                            };

                            nav[2] = if !*hud
                                || !window.focused
                                || time.delta_secs() > 0.35f32
                            {
                                None
                            } else if mouse.just_pressed(bevy::input::mouse::MouseButton::Left) {
                                selection.2 = time.elapsed_secs();
                                nav[0]
                            } else if mouse.pressed(bevy::input::mouse::MouseButton::Left) {
                                nav[2]
                            } else {
                                None
                            };

                            selection.1 = if keys.pressed(bevy::input::keyboard::KeyCode::Escape) {
                                (selection.1 + time.delta_secs()).min(7.0)
                            } else {
                                0.0
                            };

                            if selection.1 >= 7.0 {
                                commands.write_message(bevy::app::AppExit::Success);
                            }

                            *anim = (
                                [
                                    [
                                        anim.0[0][0]
                                        + ((nav[0] == Some(false)) as u8 as f32 - anim.0[0][0])
                                            .clamp(
                                                -time.delta_secs() / 0.15,
                                                time.delta_secs() / 0.15,
                                            ),
                                    anim.0[0][1]
                                        + ((nav[0] == Some(true)) as u8 as f32 - anim.0[0][1])
                                            .clamp(
                                                -time.delta_secs() / 0.15,
                                                time.delta_secs() / 0.15,
                                            ),
                                    ],
                                    [
                                        anim.0[1][0]
                                        + ((nav[1] == Some(false)) as u8 as f32 - anim.0[1][0])
                                            .clamp(
                                                -time.delta_secs() / 0.11,
                                                time.delta_secs() / 0.11,
                                            ),
                                    anim.0[1][1]
                                        + ((nav[1] == Some(true)) as u8 as f32 - anim.0[1][1])
                                            .clamp(
                                                -time.delta_secs() / 0.11,
                                                time.delta_secs() / 0.11,
                                            ),
                                    ],
                                    [
                                        anim.0[2][0]
                                        + ((nav[2] == Some(false)) as u8 as f32 - anim.0[2][0])
                                            .clamp(
                                                -time.delta_secs() / 0.06,
                                                time.delta_secs() / 0.06,
                                            ),
                                    anim.0[2][1]
                                        + ((nav[2] == Some(true)) as u8 as f32 - anim.0[2][1])
                                            .clamp(
                                                -time.delta_secs() / 0.06,
                                                time.delta_secs() / 0.06,
                                            ),
                                    ],
                                ],
                                [
                                    anim.1[0]
                                        + ((selection.0 as u8 as f32) - anim.1[0]).clamp(
                                            -time.delta_secs() / 0.15,
                                            time.delta_secs() / 0.15,
                                        ),
                                    if *hud && selection.0 {
                                        anim.1[1]
                                            + (1.0 - anim.1[1]).clamp(
                                                -time.delta_secs() / 0.14,
                                                time.delta_secs() / 0.14,
                                            )
                                    } else {
                                        0.0
                                    },
                                ],
                            );

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
                                ]);
                            }

                            if slots.is_none() {
                                handles.as_ref().into_iter().for_each(|[home_icon, registry_icon, app_icon]| {
                                    *slots = Some([
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    color: bevy::color::Color::srgb(
                                                        0.071, 0.071, 0.071,
                                                    ),
                                                    custom_size: Some(sidebar.size()),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    sidebar.center().x - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - sidebar.center().y,
                                                    2.0,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    color: bevy::color::Color::srgb(
                                                        0.149, 0.149, 0.149,
                                                    ),
                                                    custom_size: Some(bevy::math::Vec2::new(
                                                        1.0,
                                                        sidebar.height(),
                                                    )),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    sidebar.max.x - 0.5 - geometry.0[0] / 2.0,
                                                    0.0,
                                                    3.0,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    color: bevy::color::Color::srgba(
                                                        0.169, 0.169, 0.169, 0.0,
                                                    ),
                                                    custom_size: Some(panel.size()),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    panel.center().x - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - panel.center().y,
                                                    1.0,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    color: bevy::color::Color::srgba(
                                                        0.078, 0.078, 0.078, 0.0,
                                                    ),
                                                    custom_size: Some(bevy::math::Vec2::new(
                                                        (panel.width() - 2.0).max(1.0),
                                                        (panel.height() - 2.0).max(1.0),
                                                    )),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    panel.center().x - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - panel.center().y,
                                                    1.5,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    color: bevy::color::Color::srgb(
                                                        0.125, 0.125, 0.125,
                                                    ),
                                                    custom_size: Some(home_tab.size()),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    home_tab.center().x - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - home_tab.center().y,
                                                    5.0,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    color: bevy::color::Color::srgb(
                                                        0.125, 0.125, 0.125,
                                                    ),
                                                    custom_size: Some(registry_tab.size()),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    registry_tab.center().x - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - registry_tab.center().y,
                                                    5.0,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    color: bevy::color::Color::srgba(
                                                        0.459, 0.459, 0.459, 0.0,
                                                    ),
                                                    custom_size: Some(bevy::math::Vec2::new(
                                                        home_tab.width() + 4.0,
                                                        home_tab.height() + 4.0,
                                                    )),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    home_tab.center().x - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - home_tab.center().y,
                                                    4.0,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    color: bevy::color::Color::srgba(
                                                        0.459, 0.459, 0.459, 0.0,
                                                    ),
                                                    custom_size: Some(bevy::math::Vec2::new(
                                                        registry_tab.width() + 4.0,
                                                        registry_tab.height() + 4.0,
                                                    )),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    registry_tab.center().x - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - registry_tab.center().y,
                                                    4.0,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    image: home_icon.clone(),
                                                    color: bevy::color::Color::srgb(
                                                        0.067, 0.067, 0.067,
                                                    ),
                                                    custom_size: Some(bevy::math::Vec2::new(
                                                        20.0,
                                                        20.0 * 22.0 / 26.0,
                                                    )),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    48.0 - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - home_tab.center().y,
                                                    7.0,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    image: registry_icon.clone(),
                                                    color: bevy::color::Color::srgb(
                                                        0.871, 0.871, 0.871,
                                                    ),
                                                    custom_size: Some(bevy::math::Vec2::new(
                                                        20.0,
                                                        20.0 * 22.0 / 26.0,
                                                    )),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    48.0 - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - registry_tab.center().y,
                                                    7.0,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                        commands
                                            .spawn((
                                                bevy::sprite::Sprite {
                                                    image: app_icon.clone(),
                                                    color: bevy::color::Color::srgb(
                                                        0.871, 0.871, 0.871,
                                                    ),
                                                    custom_size: Some(bevy::math::Vec2::splat(
                                                        18.0,
                                                    )),
                                                    ..Default::default()
                                                },
                                                bevy::transform::components::Transform::from_xyz(
                                                    30.0 - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - 32.0,
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
                                                    font_size: bevy::text::FontSize::Px(
                                                        (sidebar.width() / 18.666_667)
                                                            .clamp(11.5, 15.0),
                                                    ),
                                                    ..Default::default()
                                                },
                                                bevy::text::TextColor(bevy::color::Color::srgb(
                                                    0.871, 0.871, 0.871,
                                                )),
                                                bevy::text::TextBounds::new_horizontal(
                                                    (sidebar.width() - 56.0).max(48.0),
                                                ),
                                                bevy::sprite::Anchor::TOP_LEFT,
                                                bevy::transform::components::Transform::from_xyz(
                                                    48.0 - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - 22.0,
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
                                                    font_size: bevy::text::FontSize::Px(15.0),
                                                    ..Default::default()
                                                },
                                                bevy::text::TextColor(bevy::color::Color::srgb(
                                                    0.067, 0.067, 0.067,
                                                )),
                                                bevy::text::TextBounds::new_horizontal(
                                                    (sidebar.width() - 88.0).max(48.0),
                                                ),
                                                bevy::sprite::Anchor::CENTER_LEFT,
                                                bevy::transform::components::Transform::from_xyz(
                                                    68.0 - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - home_tab.center().y,
                                                    8.0,
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
                                                    font_size: bevy::text::FontSize::Px(15.0),
                                                    ..Default::default()
                                                },
                                                bevy::text::TextColor(bevy::color::Color::srgb(
                                                    0.871, 0.871, 0.871,
                                                )),
                                                bevy::text::TextBounds::new_horizontal(
                                                    (sidebar.width() - 88.0).max(48.0),
                                                ),
                                                bevy::sprite::Anchor::CENTER_LEFT,
                                                bevy::transform::components::Transform::from_xyz(
                                                    68.0 - geometry.0[0] / 2.0,
                                                    geometry.0[1] / 2.0 - registry_tab.center().y,
                                                    8.0,
                                                ),
                                                bevy::camera::visibility::Visibility::Hidden,
                                            ))
                                            .id(),
                                    ]);
                                });
                            }

                            (*slots).into_iter().for_each(
                                |[
                                    sidebar_entity,
                                    divider_entity,
                                    panel_border_entity,
                                    panel_fill_entity,
                                    home_bg_entity,
                                    registry_bg_entity,
                                    home_ring_entity,
                                    registry_ring_entity,
                                    home_icon_entity,
                                    registry_icon_entity,
                                    app_icon_entity,
                                    title_entity,
                                    home_label_entity,
                                    registry_label_entity,
                                ]| {
                                    [
                                        (
                                            sidebar_entity,
                                            bevy::color::Color::srgb(0.071, 0.071, 0.071),
                                            *hud,
                                        ),
                                        (
                                            divider_entity,
                                            bevy::color::Color::srgb(0.149, 0.149, 0.149),
                                            *hud,
                                        ),
                                        (
                                            panel_border_entity,
                                            bevy::color::Color::srgba(
                                                0.169,
                                                0.169,
                                                0.169,
                                                anim.1[1],
                                            ),
                                            *hud && selection.0,
                                        ),
                                        (
                                            panel_fill_entity,
                                            bevy::color::Color::srgba(
                                                0.078,
                                                0.078,
                                                0.078,
                                                anim.1[1],
                                            ),
                                            *hud && selection.0,
                                        ),
                                        (
                                            home_bg_entity,
                                            bevy::color::Color::srgb_from_array(
                                                [(0.125 + 0.8 * (1.0 - anim.1[0]))
                                                    + ((0.188 + 0.812 * (1.0 - anim.1[0]))
                                                        - (0.125 + 0.8 * (1.0 - anim.1[0])))
                                                        * anim.0[0][0]
                                                    + ((0.235 + 0.615 * (1.0 - anim.1[0]))
                                                        - ((0.125 + 0.8 * (1.0 - anim.1[0]))
                                                            + ((0.188 + 0.812 * (1.0 - anim.1[0]))
                                                                - (0.125 + 0.8 * (1.0 - anim.1[0])))
                                                                * anim.0[0][0]))
                                                        * anim.0[2][0];
                                                    3],
                                            ),
                                            *hud,
                                        ),
                                        (
                                            registry_bg_entity,
                                            bevy::color::Color::srgb_from_array(
                                                [(0.125 + 0.8 * anim.1[0])
                                                    + ((0.188 + 0.812 * anim.1[0])
                                                        - (0.125 + 0.8 * anim.1[0]))
                                                        * anim.0[0][1]
                                                    + ((0.235 + 0.615 * anim.1[0])
                                                        - ((0.125 + 0.8 * anim.1[0])
                                                            + ((0.188 + 0.812 * anim.1[0])
                                                                - (0.125 + 0.8 * anim.1[0]))
                                                                * anim.0[0][1]))
                                                        * anim.0[2][1];
                                                    3],
                                            ),
                                            *hud,
                                        ),
                                        (
                                            home_ring_entity,
                                            bevy::color::Color::srgba(
                                                0.459,
                                                0.459,
                                                0.459,
                                                anim.0[1][0],
                                            ),
                                            *hud && anim.0[1][0] > 0.0,
                                        ),
                                        (
                                            registry_ring_entity,
                                            bevy::color::Color::srgba(
                                                0.459,
                                                0.459,
                                                0.459,
                                                anim.0[1][1],
                                            ),
                                            *hud && anim.0[1][1] > 0.0,
                                        ),
                                        (
                                            home_icon_entity,
                                            bevy::color::Color::srgb_from_array(
                                                [0.871 + (0.067 - 0.871) * (1.0 - anim.1[0]); 3],
                                            ),
                                            *hud,
                                        ),
                                        (
                                            registry_icon_entity,
                                            bevy::color::Color::srgb_from_array(
                                                [0.871 + (0.067 - 0.871) * anim.1[0]; 3],
                                            ),
                                            *hud,
                                        ),
                                        (
                                            app_icon_entity,
                                            bevy::color::Color::srgb(0.871, 0.871, 0.871),
                                            *hud,
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
                                                .get_mut(entity)
                                                .map(|(sprite, visibility)| {
                                                    if sprite.color != colour {
                                                        sprite.into_inner().color = colour;
                                                    }
                                                    if (*visibility
                                                        == bevy::camera::visibility::Visibility::Inherited)
                                                        != shown
                                                    {
                                                        *visibility.into_inner() = if shown {
                                                            bevy::camera::visibility::Visibility::Inherited
                                                        } else {
                                                            bevy::camera::visibility::Visibility::Hidden
                                                        };
                                                    }
                                                })
                                                .ok();
                                        },
                                    );

                                    [
                                        (
                                            title_entity,
                                            bevy::color::Color::srgb(0.871, 0.871, 0.871),
                                            *hud,
                                        ),
                                        (
                                            home_label_entity,
                                            bevy::color::Color::srgb_from_array(
                                                [0.871 + (0.067 - 0.871) * (1.0 - anim.1[0]); 3],
                                            ),
                                            *hud,
                                        ),
                                        (
                                            registry_label_entity,
                                            bevy::color::Color::srgb_from_array(
                                                [0.871 + (0.067 - 0.871) * anim.1[0]; 3],
                                            ),
                                            *hud,
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
                                                .get_mut(entity)
                                                .map(|(text_colour, visibility)| {
                                                    if text_colour.0 != colour {
                                                        text_colour.into_inner().0 = colour;
                                                    }
                                                    if (*visibility
                                                        == bevy::camera::visibility::Visibility::Inherited)
                                                        != shown
                                                    {
                                                        *visibility.into_inner() = if shown {
                                                            bevy::camera::visibility::Visibility::Inherited
                                                        } else {
                                                            bevy::camera::visibility::Visibility::Hidden
                                                        };
                                                    }
                                                })
                                                .ok();
                                        },
                                    );
                                },
                            );
                        });
                    });
            },
        )
        .run();
}
