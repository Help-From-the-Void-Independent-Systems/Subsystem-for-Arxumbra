#[derive(bevy::ecs::component::Component, Default)]
struct Background;

#[derive(bevy::ecs::component::Component, Default)]
struct Panel;

#[derive(bevy::ecs::component::Component)]
struct Pending(bevy::tasks::Task<bevy::image::Image>);

fn decode<M: bevy::ecs::component::Component + Default>(
    buffer: &'static [u8],
    mut commands: bevy::ecs::system::Commands<'_, '_>,
) {
    commands.spawn((
        M::default(),
        Pending(
            bevy::tasks::AsyncComputeTaskPool::get().spawn(async move {
                bevy::image::Image::from_buffer(
                    buffer,
                    bevy::image::ImageType::Format(bevy::image::ImageFormat::Ktx2),
                    bevy::image::CompressedImageFormats::BC,
                    true,
                    bevy::image::ImageSampler::Default,
                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                )
                .unwrap_or_else(|_| bevy::image::Image::default())
            }),
        ),
    ));
}

fn quad<M: bevy::ecs::component::Component + Default>(
    image: bevy::image::Image,
    size: bevy::math::Vec2,
    position: bevy::math::Vec3,
    mut commands: bevy::ecs::system::Commands<'_, '_>,
    assets: &mut bevy::asset::Assets<bevy::image::Image>,
    meshes: &mut bevy::asset::Assets<bevy::mesh::Mesh>,
    materials: &mut bevy::asset::Assets<bevy::sprite_render::ColorMaterial>,
) {
    commands.spawn((
        M::default(),
        bevy::mesh::Mesh2d(meshes.add(bevy::shape::Rectangle::from_size(size))),
        bevy::sprite_render::MeshMaterial2d(
            materials.add(bevy::sprite_render::ColorMaterial {
                color: bevy::color::Color::WHITE,
                alpha_mode: bevy::sprite_render::AlphaMode2d::Opaque,
                uv_transform: bevy::math::Affine2::from_mat2_translation(
                    bevy::math::Mat2::from_diagonal(bevy::math::Vec2::new(
                        (image.width() as f32 / size.x)
                            .min(image.height() as f32 / size.y)
                            .recip(),
                        (image.height() as f32 / size.y)
                            .min(image.width() as f32 / size.x)
                            .recip(),
                    )),
                    (bevy::math::Vec2::ONE
                        - bevy::math::Vec2::new(
                            (image.width() as f32 / size.x)
                                .min(image.height() as f32 / size.y)
                                .recip(),
                            (image.height() as f32 / size.y)
                                .min(image.width() as f32 / size.x)
                                .recip(),
                        ))
                        * 0.5,
                ),
                texture: Some(assets.add(image)),
            }),
        ),
        bevy::transform::components::Transform::from_translation(position),
    ));
}

fn startup(
    mut commands: bevy::ecs::system::Commands,
    mut windows: bevy::ecs::system::Query<
        bevy::ecs::entity::Entity,
        bevy::ecs::query::With<bevy::window::PrimaryWindow>,
    >,
) {
    for entity in &mut windows {
        commands.entity(entity).insert(bevy::window::CursorOptions {
            visible: false,
            ..Default::default()
        });
    }

    commands.spawn(bevy::camera::Camera2d);

    decode::<Background>(include_bytes!("../assets/000.ktx2"), commands.reborrow());

    decode::<Panel>(include_bytes!("../assets/001.ktx2"), commands.reborrow());
}

fn present(
    mut windows: bevy::ecs::system::Query<
        &mut bevy::window::Window,
        bevy::ecs::query::With<bevy::window::PrimaryWindow>,
    >,
    monitors: bevy::ecs::system::Query<&bevy::window::Monitor>,
) {
    for monitor in &monitors {
        for mut window in &mut windows {
            window.mode = bevy::window::WindowMode::BorderlessFullscreen(
                bevy::window::MonitorSelection::Primary,
            );
            window.visible = true;
            window.decorations = false;
            window.transparent = false;
            window.resizable = false;
            window.skip_taskbar = true;
            window.has_shadow = false;
            window.present_mode = bevy::window::PresentMode::AutoVsync;
            window.position = bevy::window::WindowPosition::At(monitor.physical_position);
            window
                .resolution
                .set_physical_resolution(monitor.physical_width, monitor.physical_height);
            window.resolution.set_scale_factor_override(Some(1.0));
        }
    }
}

fn apply(
    mut commands: bevy::ecs::system::Commands,
    mut assets: bevy::ecs::system::ResMut<bevy::asset::Assets<bevy::image::Image>>,
    mut meshes: bevy::ecs::system::ResMut<bevy::asset::Assets<bevy::mesh::Mesh>>,
    mut materials: bevy::ecs::system::ResMut<bevy::asset::Assets<bevy::sprite_render::ColorMaterial>>,
    monitors: bevy::ecs::system::Query<&bevy::window::Monitor>,
    mut pending: bevy::ecs::system::Query<(
        bevy::ecs::entity::Entity,
        &mut Pending,
        Option<&Background>,
        Option<&Panel>,
    )>,
) {
    for monitor in &monitors {
        for (entity, mut task, background, panel) in &mut pending {
            task.0
                .is_finished()
                .then(|| bevy::tasks::block_on(&mut task.0))
                .map(|image| {
                    if background.is_some() {
                        quad::<Background>(
                            image,
                            bevy::math::Vec2::new(
                                monitor.physical_width as f32,
                                monitor.physical_height as f32,
                            ),
                            bevy::math::Vec3::new(0.0, 0.0, 0.0),
                            commands.reborrow(),
                            &mut assets,
                            &mut meshes,
                            &mut materials,
                        );
                    } else if panel.is_some() {
                        quad::<Panel>(
                            image,
                            bevy::math::Vec2::new(
                                monitor.physical_width as f32 / 3.0,
                                monitor.physical_height as f32,
                            ),
                            bevy::math::Vec3::new(-(monitor.physical_width as f32) / 3.0, 0.0, 1.0),
                            commands.reborrow(),
                            &mut assets,
                            &mut meshes,
                            &mut materials,
                        );
                    }

                    commands.entity(entity).despawn();
                });
        }
    }
}

fn main() {
    bevy::app::App::new()
        .add_plugins(bevy::app::PluginGroup::set(
            bevy::DefaultPlugins,
            bevy::render::RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(Box::new(
                    bevy::render::settings::WgpuSettings {
                        limits: bevy::render::settings::WgpuLimits {
                            max_texture_dimension_2d: 16384,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
        ))
        .add_systems(bevy::app::Startup, startup)
        .add_systems(bevy::app::Update, (present, apply))
        .run();
}
