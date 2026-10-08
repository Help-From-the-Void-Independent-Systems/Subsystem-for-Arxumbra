fn decode(buffer: &[u8]) -> bevy::prelude::Image {
    bevy::prelude::Image::from_buffer(
        buffer,
        bevy::image::ImageType::Format(bevy::image::ImageFormat::Ktx2),
        bevy::image::CompressedImageFormats::BC,
        true,
        bevy::image::ImageSampler::Default,
        bevy::asset::RenderAssetUsages::RENDER_WORLD,
    )
    .unwrap()
}

fn span(size: bevy::prelude::Vec2, image: &bevy::prelude::Image) -> bevy::prelude::Vec2 {
    bevy::prelude::Vec2::new(
        (size.x * image.height() as f32 / (size.y * image.width() as f32)).min(1.0),
        (size.y * image.width() as f32 / (size.x * image.height() as f32)).min(1.0),
    )
}

fn size(monitors: &bevy::prelude::Query<&bevy::window::Monitor>) -> bevy::prelude::Vec2 {
    bevy::prelude::Vec2::new(
        monitors.iter().next().unwrap().physical_width as f32,
        monitors.iter().next().unwrap().physical_height as f32,
    )
}

fn quad(
    commands: &mut bevy::prelude::Commands,
    images: &mut bevy::prelude::Assets<bevy::prelude::Image>,
    meshes: &mut bevy::prelude::Assets<bevy::prelude::Mesh>,
    materials: &mut bevy::prelude::Assets<bevy::sprite_render::ColorMaterial>,
    image: bevy::prelude::Image,
    size: bevy::prelude::Vec2,
    position: bevy::prelude::Vec3,
) {
    commands.spawn((
        bevy::prelude::Mesh2d(meshes.add(bevy::prelude::Rectangle::from_size(size))),
        bevy::prelude::MeshMaterial2d(materials.add(bevy::sprite_render::ColorMaterial {
            uv_transform: bevy::math::Affine2::from_mat2_translation(
                bevy::math::Mat2::from_diagonal(span(size, &image)),
                (bevy::prelude::Vec2::ONE - span(size, &image)) * 0.5,
            ),
            texture: Some(images.add(image)),
            ..bevy::prelude::default()
        })),
        bevy::prelude::Transform::from_translation(position),
    ));
}

fn startup(
    mut commands: bevy::prelude::Commands,
    mut images: bevy::prelude::ResMut<bevy::prelude::Assets<bevy::prelude::Image>>,
    mut meshes: bevy::prelude::ResMut<bevy::prelude::Assets<bevy::prelude::Mesh>>,
    mut materials: bevy::prelude::ResMut<bevy::prelude::Assets<bevy::sprite_render::ColorMaterial>>,
    monitors: bevy::prelude::Query<&bevy::window::Monitor>,
) {
    commands.spawn(bevy::prelude::Camera2d);
    quad(
        &mut commands,
        &mut images,
        &mut meshes,
        &mut materials,
        decode(include_bytes!("../assets/000.ktx2")),
        size(&monitors),
        bevy::prelude::Vec3::ZERO,
    );
    quad(
        &mut commands,
        &mut images,
        &mut meshes,
        &mut materials,
        decode(include_bytes!("../assets/001.ktx2")),
        bevy::prelude::Vec2::new(size(&monitors).x / 3.0, size(&monitors).y),
        bevy::prelude::Vec3::new(-size(&monitors).x / 3.0, 0.0, 1.0),
    );
}

fn main() {
    bevy::prelude::App::new()
        .add_plugins(
            bevy::app::PluginGroup::set(
                bevy::prelude::DefaultPlugins,
                bevy::prelude::WindowPlugin {
                    primary_window: Some(bevy::prelude::Window {
                        mode: bevy::window::WindowMode::BorderlessFullscreen(
                            bevy::prelude::MonitorSelection::Primary,
                        ),
                        decorations: false,
                        resizable: false,
                        transparent: false,
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..bevy::prelude::default()
                    }),
                    primary_cursor_options: Some(bevy::window::CursorOptions {
                        visible: false,
                        ..bevy::prelude::default()
                    }),
                    ..bevy::prelude::default()
                },
            )
            .set(bevy::render::RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(
                    Box::new(bevy::render::settings::WgpuSettings {
                        limits: bevy::render::settings::WgpuLimits {
                            max_texture_dimension_2d: 16384,
                            ..bevy::prelude::default()
                        },
                        ..bevy::prelude::default()
                    }),
                ),
                ..bevy::prelude::default()
            }),
        )
        .add_systems(bevy::prelude::Startup, startup)
        .run();
}
