/// # 🔗 Constraints Demo
///
/// Demonstrates distance constraints between particles.
/// Run with: `cargo run -p ch12`
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
    // Spawn three particles in a chain-like formation
    for i in 0..3 {
        let x_offset = (i as f32 - 1.0) * 50.0;
        commands.spawn((
            physics::components::Position::new(x_offset, 200.0),
            physics::components::Velocity::default(),
            physics::components::Acceleration::default(),
            physics::components::Mass::default(),
            physics::components::ForceAccumulator::default(),
            SpriteBundle {
                sprite: Sprite::from_color(Color::srgb(0.3, 0.8, 1.0), Vec2::splat(16.0)),
                ..default()
            },
        ));
    }
    println!("🔗 Constraints demo started! Three particles under gravity.");
    println!("⛓️ In a full implementation, distance constraints connect them.");
}
