//! # 🏃 Chapter 7: Kinematics
//! Run with: `cargo run -p ch07`
use bevy::prelude::*;
fn main() {
    let pos = Vec2::ZERO;
    let vel = Vec2::new(100.0, 200.0);
    let acc = Vec2::new(0.0, -500.0);
    let dt = 1.0 / 60.0;
    let mut p = pos; let mut v = vel;
    // Semi-implicit Euler
    v += acc * dt;
    p += v * dt;
    println!("🔄 After 1 frame: pos=({:.1}, {:.1}) vel=({:.1}, {:.1})", p.x, p.y, v.x, v.y);
    // Predict future position
    let t = 1.0;
    let future = pos + vel * t + 0.5 * acc * t * t;
    println!("🔮 Predicted at t=1s: ({:.1}, {:.1})", future.x, future.y);
    println!("✅ Kinematics complete!");
}
