//! # 🔗 Chapter 12: Constraints & Joints
//! Run with: `cargo run -p ch12`
use bevy::prelude::*;
fn main() {
    let anchor = Vec2::ZERO;
    let mut pos = Vec2::new(100.0, 0.0);
    let target_dist = 50.0;
    let delta = pos - anchor;
    let dist = delta.length();
    if dist > 0.001 {
        let dir = delta / dist;
        let error = dist - target_dist;
        pos -= dir * error * 0.5;
        println!("🔗 Distance constraint: dist moved from {dist:.1} to {:.1}", pos.length());
    }
    // Chain simulation
    let mut particles: Vec<Vec2> = (0..5).map(|i| Vec2::new(i as f32 * 30.0, 100.0)).collect();
    let gravity = Vec2::new(0.0, -200.0);
    let dt = 1.0 / 60.0;
    for _ in 0..120 {
        for i in 0..particles.len() {
            particles[i] += gravity * dt * dt;
        }
        for _ in 0..5 {
            for i in 0..particles.len() - 1 {
                let d = particles[i+1] - particles[i];
                let len = d.length();
                if len > 0.001 {
                    let correction = d / len * (len - 30.0) * 0.5;
                    particles[i] += correction;
                    particles[i+1] -= correction;
                }
            }
        }
        particles[0] = Vec2::ZERO; // Pin first particle
    }
    println!("⛓️ Chain end position after 2s: ({:.1}, {:.1})", particles[4].x, particles[4].y);
    println!("✅ Constraints complete!");
}
