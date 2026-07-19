//! # 🧱 Chapter 10: Collision Detection
//! Run with: `cargo run -p ch10`
use bevy::prelude::*;
fn main() {
    // Circle vs Circle
    let c1_pos = Vec2::ZERO; let r1 = 5.0;
    let c2_pos = Vec2::new(3.0, 0.0); let r2 = 5.0;
    let diff = c2_pos - c1_pos;
    let dist_sq = diff.length_squared();
    let sum_r = r1 + r2;
    if dist_sq <= sum_r * sum_r {
        let dist = dist_sq.sqrt();
        let normal = diff / dist;
        let pen = sum_r - dist;
        println!("🔵🔵 Circle collision! Normal=({:.2}, {:.2}), Pen={pen:.2}", normal.x, normal.y);
    }
    // AABB vs AABB
    let a_min = Vec2::new(-5.0, -5.0); let a_max = Vec2::new(5.0, 5.0);
    let b_min = Vec2::new(-3.0, -3.0); let b_max = Vec2::new(7.0, 7.0);
    let overlap = a_min.x <= b_max.x && a_max.x >= b_min.x
               && a_min.y <= b_max.y && a_max.y >= b_min.y;
    println!("📦📦 AABB collision: {overlap}");
    println!("✅ Collision detection complete!");
}
