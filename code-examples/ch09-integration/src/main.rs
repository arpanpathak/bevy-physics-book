//! # 🔄 Chapter 9: Integration Methods
//! Run with: `cargo run -p ch09`
use bevy::prelude::*;
fn main() {
    let dt = 1.0 / 60.0;
    let mut pos = Vec2::ZERO;
    let mut vel = Vec2::new(50.0, 100.0);
    let acc = Vec2::new(0.0, -500.0);
    // Symplectic Euler (recommended)
    for _ in 0..60 {
        vel += acc * dt;
        pos += vel * dt;
    }
    println!("🔄 Symplectic Euler after 60 frames: ({:.1}, {:.1})", pos.x, pos.y);
    // Verlet
    let mut x = 0.0; let mut prev_x = 0.0;
    let a = -500.0; let v0 = 100.0;
    prev_x = x - v0 * dt;  // Initial prev
    for _ in 0..60 {
        let temp = x;
        x = 2.0 * x - prev_x + a * dt * dt;
        prev_x = temp;
    }
    println!("🔮 Verlet after 60 frames: y={x:.1}");
    println!("✅ Integration methods complete!");
}
