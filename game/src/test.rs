use bevy::{color::palettes::css::RED, prelude::*, render::view::Hdr};

// Very basic example. Spawns a red light in the center of the screen, and a few round occluders surrounding it.
// You can press the arrow keys to move the light.

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins));
    app.add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Transform::default(),
    ));

}
