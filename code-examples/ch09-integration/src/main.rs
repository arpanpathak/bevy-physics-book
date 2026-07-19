/// # 🔄 Integration Demo
///
/// Compares Symplectic Euler vs Verlet integration visually.
/// Run with: `cargo run -p ch09`
use bevy::prelude::*;
mod physics;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(physics::PhysicsPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}
fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        physics::components::Position::new(0.0, 200.0),
        physics::components::Velocity::new(60.0, 100.0),
        physics::components::Acceleration::default(),
        physics::components::Mass::default(),
        physics::components::ForceAccumulator::default(),
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.5, 0.0), Vec2::new(20.0, 20.0)),
            ..default()
        },
    ));
    println!("🔄 Integration demo started! Object uses Symplectic Euler.");
}
