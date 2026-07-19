//! # 💥 Chapter 8: Dynamics & Forces
//! Run with: `cargo run -p ch08`
use bevy::prelude::*;
fn main() {
    let mass = 2.0;
    let gravity = Vec2::new(0.0, -500.0);
    let force_gravity = gravity * mass;
    println!("🌍 Gravitational force on {mass}kg: ({:.0}, {:.0})", force_gravity.x, force_gravity.y);
    let vel = Vec2::new(100.0, 0.0);
    let damping = 0.1;
    let drag_force = -vel * damping;
    println!("🌬️ Drag force at vel=({:.0}, {:.0}): ({:.1}, {:.1})", vel.x, vel.y, drag_force.x, drag_force.y);
    let total_force = force_gravity + drag_force;
    let accel = total_force / mass;
    println!("⚡ F = ma → a = ({:.1}, {:.1})", accel.x, accel.y);
    println!("✅ Dynamics complete!");
}
