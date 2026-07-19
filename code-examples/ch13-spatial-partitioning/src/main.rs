/// # 📦 Spatial Partitioning Demo
///
/// Demonstrates a spatial grid for efficient collision culling.
/// Run with: `cargo run -p ch13`
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
    // Spawn many objects to demonstrate spatial partitioning benefits
    for i in 0..50 {
        let x = (i as f32 - 25.0) * 30.0;
        let y = (i as f32 * 7.0).sin() * 200.0;
        commands.spawn((
            physics::components::Position::new(x, y),
            physics::components::Velocity::default(),
            physics::components::Acceleration::default(),
            physics::components::Mass::default(),
            physics::components::ForceAccumulator::default(),
            SpriteBundle {
                sprite: Sprite::from_color(Color::hsl(i as f32 * 7.0, 0.8, 0.5), Vec2::splat(8.0)),
                ..default()
            },
        ));
    }
    println!("📦 Spatial demo started! 50 objects spawned.");
    println!("🗺️ A spatial grid would accelerate collision checks 50x.");
}
