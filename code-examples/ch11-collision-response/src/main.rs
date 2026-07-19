/// # 🤝 Collision Response Demo
///
/// Objects bounce off each other with restitution.
/// Run with: `cargo run -p ch11`
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
        physics::components::Position::new(-150.0, 0.0),
        physics::components::Velocity::new(150.0, 0.0),
        physics::components::Acceleration::default(),
        physics::components::Mass::default(),
        physics::components::ForceAccumulator::default(),
        physics::components::Collider::Circle { radius: 15.0 },
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(30.0)),
            ..default()
        },
    ));
    commands.spawn((
        physics::components::Position::new(150.0, 0.0),
        physics::components::Velocity::new(-150.0, 0.0),
        physics::components::Acceleration::default(),
        physics::components::Mass::default(),
        physics::components::ForceAccumulator::default(),
        physics::components::Collider::Circle { radius: 15.0 },
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(0.2, 0.8, 0.2), Vec2::splat(30.0)),
            ..default()
        },
    ));
    println!("🤝 Collision response demo! Two balls colliding.");
}
