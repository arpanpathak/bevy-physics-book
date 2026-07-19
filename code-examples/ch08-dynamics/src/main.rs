/// # 💥 Dynamics Demo
///
/// Demonstrates forces in action: gravity pulls objects down,
/// drag slows them, and different masses respond differently.
///
/// Run with: `cargo run -p ch08`

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
    // Heavy object (falls same rate but harder to push)
    commands.spawn((
        physics::components::Position::new(-100.0, 200.0),
        physics::components::Velocity::default(),
        physics::components::Acceleration::default(),
        physics::components::Mass { value: 5.0 },
        physics::components::ForceAccumulator::default(),
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::new(40.0, 40.0)),
            ..default()
        },
    ));
    // Light object (same gravity response, different mass)
    commands.spawn((
        physics::components::Position::new(100.0, 200.0),
        physics::components::Velocity::default(),
        physics::components::Acceleration::default(),
        physics::components::Mass { value: 0.5 },
        physics::components::ForceAccumulator::default(),
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(0.2, 1.0, 0.2), Vec2::new(20.0, 20.0)),
            ..default()
        },
    ));
    println!("💥 Dynamics demo started! Both objects fall at same rate.");
    println!("⚖️ Red = heavy (mass 5), Green = light (mass 0.5)");
}
