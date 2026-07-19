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

    // ─── Individual transforms for verification ───
    // Manually verify each step of the composition:
    let scaled_point = s.transform_point2(point);
    let rotated_point = r.transform_point2(scaled_point);
    let final_point = t.transform_point2(rotated_point);
    println!("🔍 Step-by-step: scale→({:.1},{:.1}) rotate→({:.1},{:.1}) translate→({:.1},{:.1})",
        scaled_point.x, scaled_point.y,
        rotated_point.x, rotated_point.y,
        final_point.x, final_point.y);

    // ─── Inverse ───
    let inverse = transform.inverse();
    let back = inverse.transform_point2(transformed);
    println!("🔄 Inverse brought back to ({:.1}, {:.1})", back.x, back.y);

    println!("\n✅ All matrix operations complete!");
}
