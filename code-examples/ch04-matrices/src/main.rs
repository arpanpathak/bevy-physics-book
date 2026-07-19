//! # 🔢 Chapter 4: Matrices & Transformations
//!
//! Run with: `cargo run -p ch04`
//!
//! Demonstrates transform composition: T × R × S

use bevy::prelude::*;

fn main() {
    // ─── Individual transforms ───
    let t = Mat3::from_translation(Vec2::new(100.0, 50.0));
    let r = Mat3::from_angle(std::f32::consts::FRAC_PI_4); // 45°
    let s = Mat3::from_scale(Vec2::new(2.0, 2.0));

    // ─── Compose: T × R × S ───
    let transform = t * r * s;
    let point = Vec2::new(10.0, 0.0);
    let transformed = transform.transform_point2(point);
    println!("📍 Point ({:.0}, {:.0}) → ({:.1}, {:.1})", point.x, point.y, transformed.x, transformed.y);

    // ─── Decompose ───
    let (scale, angle, translation) = transform.to_scale_angle_translation();
    println!("📏 Scale: ({:.1}, {:.1})", scale.x, scale.y);
    println!("🔄 Rotation: {:.1}°", angle.to_degrees());
    println!("📍 Translation: ({:.1}, {:.1})", translation.x, translation.y);

    // ─── Inverse ───
    let inverse = transform.inverse();
    let back = inverse.transform_point2(transformed);
    println!("🔄 Inverse brought back to ({:.1}, {:.1})", back.x, back.y);

    println!("\n✅ All matrix operations complete!");
}
