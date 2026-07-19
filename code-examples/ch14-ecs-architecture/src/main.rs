//! # 🏗️ Chapter 14: Bevy ECS Physics Architecture
//! Run with: `cargo run -p ch14`
use bevy::prelude::*;

#[derive(Component)] struct Position(Vec2);
#[derive(Component)] struct Velocity(Vec2);
#[derive(Component)] struct Mass(f32);
#[derive(Resource)] struct Gravity(Vec2);

fn apply_gravity(mut query: Query<(&Mass, &mut Velocity)>, gravity: Res<Gravity>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mass, mut vel) in query.iter_mut() {
        vel.0 += gravity.0 * mass.0 * dt;
    }
}

fn integrate(mut query: Query<(&Velocity, &mut Position)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (vel, mut pos) in query.iter_mut() { pos.0 += vel.0 * dt; }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Gravity(Vec2::new(0.0, -9.81)));
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Position(Vec2::ZERO), Velocity(Vec2::new(10.0, 50.0)), Mass(1.0)));
    println!("🏗️ ECS Physics Architecture initialized!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Gravity(Vec2::new(0.0, -9.81)))
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_gravity, integrate).chain())
        .run();
}
