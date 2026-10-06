fn hello_world() {
    std::println!("Hello, world!");
    std::process::exit(0);
}

fn main() {
    bevy::app::App::new()
        .add_systems(bevy::app::Startup, crate::hello_world)
        .run();
}
