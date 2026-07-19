//! # 🌀 Chapter 5: Quaternions
//! Run with: `cargo run -p ch05`
use bevy::prelude::*;
fn main() {
    let q = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
    let v = Vec3::new(1.0, 0.0, 0.0);
    let rotated = q * v;
    println!("🔄 (1,0,0) rotated 90° around Z → ({:.1}, {:.1}, {:.1})", rotated.x, rotated.y, rotated.z);
    let start = Quat::IDENTITY;
    let end = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
    let mid = start.slerp(end, 0.5);
    let mid_v = mid * Vec3::X;
    println!("📊 SLERP halfway: ({:.3}, {:.3})", mid_v.x, mid_v.y);
    let forward = q * Vec3::NEG_Z;
    println!("🚀 Forward vector: ({:.3}, {:.3}, {:.3})", forward.x, forward.y, forward.z);
    println!("✅ Quaternion operations complete!");
}
