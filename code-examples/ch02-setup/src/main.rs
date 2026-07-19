//! # ⚙️ Chapter 2: Setting Up Your Bevy Physics Playground
//!
//! Run with: `cargo run -p ch02`
//!
//! This demonstrates the basic Bevy app setup with physics components.

use bevy::prelude::*;

// 📍 Physics components
#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

/// 🔄 Simple physics system: integrate velocity into position
fn physics_step(mut query: Query<(&Velocity, &mut Position)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (vel, mut pos) in query.iter_mut() {
        pos.0 += vel.0 * dt;
    }
}

/// 🎨 Sync physics Position → Bevy Transform for rendering
fn render_sync(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(pos.0.x, pos.0.y, 0.0);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Position(Vec2::new(0.0, 0.0)),
        Velocity(Vec2::new(50.0, 100.0)),
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.5, 0.2), Vec2::new(20.0, 20.0)),
            ..default()
        },
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (physics_step, render_sync).chain())
        .run();
}
