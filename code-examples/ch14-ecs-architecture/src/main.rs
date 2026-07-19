/// # 🏗️ ECS Architecture Demo
///
/// Demonstrates the Bevy ECS plugin architecture for physics.
/// The physics engine is fully modular and reusable.
///
/// MODULE STRUCTURE:
///   src/
///     main.rs              - App setup and entity spawning
///     physics/
///       mod.rs             - PhysicsPlugin definition
///       components.rs      - Position, Velocity, Mass, etc.
///       systems.rs         - Integration, forces, sync
///
/// This pattern allows any Bevy project to add physics by writing:
///   .add_plugins(physics::PhysicsPlugin)
///
/// Run with: `cargo run -p ch14`
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
        physics::components::Velocity::new(100.0, 50.0),
        physics::components::Acceleration::default(),
        physics::components::Mass::default(),
        physics::components::ForceAccumulator::default(),
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(0.5, 0.3, 1.0), Vec2::new(30.0, 30.0)),
            ..default()
        },
    ));
    println!("🏗️ ECS Architecture demo! Physics runs via PhysicsPlugin.");
    println!("📦 All physics components and systems are in separate files.");
}
