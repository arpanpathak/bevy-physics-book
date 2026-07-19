//! # 🤝 Chapter 11: Collision Response
//! Run with: `cargo run -p ch11`
use bevy::prelude::*;
fn main() {
    let normal = Vec2::new(0.0, 1.0);
    let mut vel_a = Vec2::new(0.0, -10.0); let mass_a = 1.0;
    let mut vel_b = Vec2::ZERO; let mass_b = 2.0;
    let restitution = 0.5;
    let rel_vel = vel_a - vel_b;
    let rel_vel_n = rel_vel.dot(normal);
    if rel_vel_n < 0.0 {
        let inv_a = 1.0 / mass_a; let inv_b = 1.0 / mass_b;
        let j = -(1.0 + restitution) * rel_vel_n / (inv_a + inv_b);
        let impulse = normal * j;
        vel_a += impulse * inv_a;
        vel_b -= impulse * inv_b;
        println!("💥 After collision: A=({:.1}, {:.1}) B=({:.1}, {:.1})", vel_a.x, vel_a.y, vel_b.x, vel_b.y);
        let ke_before = 0.5 * mass_a * (10.0*10.0);
        let ke_after = 0.5 * (mass_a * vel_a.length_squared() + mass_b * vel_b.length_squared());
        println!("📊 KE: before={ke_before:.1}, after={ke_after:.1} (lost={:.1})", ke_before - ke_after);
    }
    println!("✅ Collision response complete!");
}
