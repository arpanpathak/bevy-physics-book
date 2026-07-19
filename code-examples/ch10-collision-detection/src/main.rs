/// # 🧱 Collision Detection Demo
///
/// Two objects collide and the collision is detected.
/// Run with: `cargo run -p ch10`
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
    // Left object moving right
    commands.spawn((
        physics::components::Position::new(-200.0, 0.0),
        physics::components::Velocity::new(200.0, 0.0),
        physics::components::Acceleration::default(),
        physics::components::Mass::default(),
        physics::components::ForceAccumulator::default(),
        physics::components::Collider::Circle { radius: 20.0 },
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::splat(40.0)),
            ..default()
        },
    ));
    // Right object moving left
    commands.spawn((
        physics::components::Position::new(200.0, 0.0),
        physics::components::Velocity::new(-200.0, 0.0),
        physics::components::Acceleration::default(),
        physics::components::Mass::default(),
        physics::components::ForceAccumulator::default(),
        physics::components::Collider::Circle { radius: 20.0 },
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(0.2, 0.2, 1.0), Vec2::splat(40.0)),
            ..default()
        },
    ));
    println!("🧱 Collision demo started! Two objects heading toward each other.");
}
