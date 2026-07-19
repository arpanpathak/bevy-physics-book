//! # 📐 Chapter 6: Trigonometry for Game Physics
//! Run with: `cargo run -p ch06`
use bevy::prelude::*;
fn main() {
    let angle = std::f32::consts::FRAC_PI_4;
    let direction = Vec2::new(angle.cos(), angle.sin());
    println!("🧭 Direction at 45°: ({:.3}, {:.3}), len={:.1}", direction.x, direction.y, direction.length());
    let speed = 100.0;
    let velocity = direction * speed;
    println!("🏃 Velocity: ({:.1}, {:.1}) px/s", velocity.x, velocity.y);
    let mouse_pos = Vec2::new(300.0, 200.0);
    let player_pos = Vec2::new(100.0, 100.0);
    let aim_angle = (mouse_pos - player_pos).y.atan2((mouse_pos - player_pos).x);
    println!("🎯 Aim angle toward mouse: {:.1}°", aim_angle.to_degrees());
    let t = 2.0;
    let gravity = Vec2::new(0.0, -500.0);
    let proj_pos = player_pos + velocity * t + 0.5 * gravity * t * t;
    println!("🏀 Projectile at t=2s: ({:.1}, {:.1})", proj_pos.x, proj_pos.y);
    println!("✅ Trigonometry complete!");
}
