/// # 🏃 Kinematics Demo
///
/// Demonstrates basic kinematics: position, velocity, and acceleration.
/// An object is spawned at a height and falls under gravity.
///
/// MODULE STRUCTURE:
///   src/
///     main.rs              - App setup and entity spawning
///     physics/
///       mod.rs             - PhysicsPlugin definition
///       components.rs      - Position, Velocity, Mass, etc.
///       systems.rs         - Integration, forces, sync
///
/// Run with: `cargo run -p ch07`

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
        physics::components::Position::new(0.0, 300.0),
        physics::components::Velocity::new(50.0, 0.0),  // Moving right
        physics::components::Acceleration::default(),
        physics::components::Mass::default(),
        physics::components::ForceAccumulator::default(),
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(0.2, 0.8, 1.0), Vec2::new(30.0, 30.0)),
            ..default()
        },
    ));
    println!("🏃 Kinematics demo started!");
    println!("📐 Object spawned at (0, 300) with velocity (50, 0)");
    println!("🌍 Gravity is pulling it down.");
}
