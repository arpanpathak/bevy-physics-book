//! # 🧮 Chapter 3: Vector Mathematics
//!
//! Run with: `cargo run -p ch03`
//!
//! Demonstrates vector operations: dot product, normalization,
//! and movement using vectors.

use bevy::prelude::*;

fn main() {
    // ─── Vector creation ───
    let position = Vec2::new(100.0, 200.0);
    let velocity = Vec2::new(50.0, -20.0);
    let new_position = position + velocity;
    println!("📍 Position + Velocity = ({:.1}, {:.1})", new_position.x, new_position.y);

    // ─── Distance ───
    let player = Vec2::new(0.0, 0.0);
    let enemy = Vec2::new(3.0, 4.0);
    let dist = player.distance(enemy);
    let dist_sq = player.distance_squared(enemy);
    println!("📏 Distance = {dist} (sq = {dist_sq})");

    // ─── Dot product ───
    let facing = Vec2::new(1.0, 0.0).normalize();
    let to_target = Vec2::new(1.0, 1.0).normalize();
    let dot = facing.dot(to_target);
    println!("🎯 Dot product (cos angle) = {dot:.3}");
    println!("   → Target is {} of player", if dot > 0.0 { "IN FRONT" } else { "BEHIND" });

    // ─── Normalization → Direction ───
    let raw_dir = Vec2::new(3.0, 4.0);
    let unit = raw_dir.normalize();
    println!("🧭 Unit vector = ({:.3}, {:.3}), length = {:.1}", unit.x, unit.y, unit.length());

    // ─── Lerp ───
    let start = Vec2::ZERO;
    let end = Vec2::new(100.0, 0.0);
    let halfway = start.lerp(end, 0.5);
    println!("📊 Lerp(0.5) = ({:.1}, {:.1})", halfway.x, halfway.y);

    // ─── Clamp length ───
    let fast = Vec2::new(200.0, 0.0);
    let clamped = if fast.length_squared() > 100.0 * 100.0 {
        fast.normalize() * 100.0
    } else {
        fast
    };
    println!("⚡ Clamped speed: ({:.1}, {:.1}), len = {:.1}", clamped.x, clamped.y, clamped.length());

    println!("\n✅ All vector operations complete!");
}
