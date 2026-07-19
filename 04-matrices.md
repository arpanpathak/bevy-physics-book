# 🔢 Matrices & Transformations

> **"Matrices are the verbs of game physics — they describe how things MOVE, SPIN, and STRETCH."** 🔄

---

## 🤔 Why Matrices?

A **matrix** is a rectangular array of numbers. In game physics, matrices of size 3×3 (2D) or 4×4 (3D) are used to **transform** vectors:

```
┌─────────────────────────────────────────────────────┐
│  What matrices do:                                  │
│                                                     │
│  🏃 TRANSLATION:  Move an object from A to B        │
│  🔄 ROTATION:     Spin an object around a point     │
│  📏 SCALING:      Make an object bigger or smaller  │
│  ✂️ SHEARING:     Skew an object (less common)      │
│                                                     │
│  Best of all: MULTIPLY matrices to COMBINE them!    │
└─────────────────────────────────────────────────────┘
```

---

## 📐 2D Transformation Matrices

A 2D transformation in Bevy uses a 3×3 matrix (affine transformation). The extra dimension enables translation:

```
┌────────────────────────────────────┐
│ 2D Affine Matrix (3×3):           │
│                                    │
│  [ a  b  tx ]   ┌── Scale/Rotate  │
│  [ c  d  ty ]   │                 │
│  [ 0  0  1  ]   └── Translation   │
│                                    │
│  "1" at bottom-right is the       │
│  homogeneous coordinate magic!    │
└────────────────────────────────────┘
```

### 🏃 Translation Matrix

```rust
use bevy::prelude::*;

/// 📍 Create a translation matrix that moves things by (dx, dy)
fn translate_2d(dx: f32, dy: f32) -> Mat3 {
    // Bevy's Mat3::from_translation takes a Vec2
    Mat3::from_translation(Vec2::new(dx, dy))
}

// What it looks like:
// [ 1  0  dx ]   [ x ]   [ x + dx ]
// [ 0  1  dy ] × [ y ] = [ y + dy ]
// [ 0  0  1  ]   [ 1 ]   [ 1      ]
```

### 🔄 Rotation Matrix

```rust
/// 🌀 Create a rotation matrix that rotates by `angle` radians
fn rotate_2d(angle: f32) -> Mat3 {
    Mat3::from_angle(angle)
}

// What it looks like:
// [ cos(θ)  -sin(θ)  0 ]   [ x ]   [ x·cos(θ) - y·sin(θ) ]
// [ sin(θ)   cos(θ)  0 ] × [ y ] = [ x·sin(θ) + y·cos(θ) ]
// [ 0        0       1 ]   [ 1 ]   [ 1                   ]
```

### 📏 Scaling Matrix

```rust
/// 📏 Create a scale matrix that stretches by (sx, sy)
fn scale_2d(sx: f32, sy: f32) -> Mat3 {
    Mat3::from_scale(Vec2::new(sx, sy))
}

// What it looks like:
// [ sx  0   0 ]   [ x ]   [ x·sx ]
// [ 0   sy  0 ] × [ y ] = [ y·sy ]
// [ 0   0   1 ]   [ 1 ]   [ 1    ]
```

---

## 🎯 Combining Transformations

This is where matrices **shine**! Multiply them to combine effects:

```rust
/// 🔗 Create a combined transform: Scale → Rotate → Translate
fn transform_2d(pos: Vec2, rotation: f32, scale: Vec2) -> Mat3 {
    // ORDER MATTERS! We read operations right-to-left:
    // 1. First scale
    // 2. Then rotate
    // 3. Finally translate
    let t = Mat3::from_translation(pos);
    let r = Mat3::from_angle(rotation);
    let s = Mat3::from_scale(scale);
    
    // 🔗 Compose: T × R × S
    // Applied as: T × (R × (S × point))
    t * r * s
}

/// 💡 Why T × R × S and not S × R × T?
///
/// S × R × T would:
/// 1. Translate first (move away from origin)
/// 2. Rotate around origin (swings the moved point in an arc)
/// 3. Scale around origin (also affects the offset!)
///
/// T × R × S:
/// 1. Scale around origin (safe, no rotation distortion)
/// 2. Rotate around origin (clean rotation)
/// 3. Translate to final position (just moves the result)
///
/// This is called "Scale → Rotate → Translate" and is the
/// standard convention in game engines!
```

### 📊 Visual: Transform Order

```
Scale → Rotate → Translate (✅ Correct)
=========================================

   ┌─────┐        ┌─────┐        .─.
   │     │        │     │       (   )─.
   │     │  scale │  ──►│ rotate│  ──►│  translate  │●──┘
   └─────┘        └─────┘       '─'    └─────┘
   Original       2x bigger     45°     Move to (300,200)

Rotate → Scale → Translate (❌ Wrong — shears the shape!)
==========================================================

   ┌─────┐         .─.           ╱╲
   │     │         ╱   ╲       ╱  ╲
   │     │  rotate╱     ╲scale╱    ╲
   └─────┘       ╲     ╱    ╱      ╲
                  ╲   ╱    ╱   ←shear!
                   '─'    ╱
```

---

## 🔧 Matrices in Bevy: Mat2, Mat3, Mat4

```rust
/// 💡 BEVY'S MATRIX TYPES
/// 
/// Mat2: 2×2 — Rotations & scales only (no translation)
/// Mat3: 3×3 — Full 2D transforms (with translation)
/// Mat4: 4×4 — Full 3D transforms
///
/// For 2D game physics, Mat3 is your workhorse!

use bevy::prelude::*;

// ─── Mat3 Construction ───

// From translation
let t = Mat3::from_translation(Vec2::new(100.0, 50.0));

// From rotation (angle in radians)
let r = Mat3::from_angle(FRAC_PI_4);  // 45 degrees

// From scale
let s = Mat3::from_scale(Vec2::new(2.0, 2.0));

// From individual columns (column-major in memory)
let custom = Mat3::from_cols(
    Vec3::new(1.0, 0.0, 0.0),  // First column = X axis
    Vec3::new(0.0, 2.0, 0.0),  // Second column = Y axis
    Vec3::new(0.0, 0.0, 1.0),  // Third column = Translation
);

// ─── Applying Matrices to Vectors ───

let point = Vec2::new(10.0, 20.0);

// Transform a 2D point using a Mat3
// (automatically handles the homogeneous coordinate)
let transformed = t.transform_point2(point);

// Transform a 2D vector (no translation applied!)
let direction = r.transform_vector2(Vec2::X);

// ─── Inspector ───
// Decompose a matrix back into its components
fn debug_matrix(m: Mat3) {
    let (scale, rotation, translation) = m.to_scale_angle_translation();
    
    println!("📍 Translation: ({:.2}, {:.2})", translation.x, translation.y);
    println!("🔄 Rotation: {:.2}°", rotation.to_degrees());
    println!("📏 Scale: ({:.2}, {:.2})", scale.x, scale.y);
}
```

---

## 🏗️ Building a Transform Component

Let's build a physics-friendly transform:

```rust
// 📁 src/physics/transform.rs
//! 🔧 Physics Transform
//!
//! Our own transform type, separate from Bevy's Transform.
//! Physics operates on raw position/rotation/scale,
//! syncing to Bevy's Transform only for rendering.

/// 🎯 Physics Transform — holds the core spatial state
#[derive(Component, Debug, Clone, Copy)]
pub struct PhysicsTransform {
    /// 📍 World-space position
    pub translation: Vec2,
    /// 🔄 Rotation in radians
    pub rotation: f32,
    /// 📏 Scale factor
    pub scale: Vec2,
}

impl PhysicsTransform {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            translation: Vec2::new(x, y),
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
    
    /// 🔄 Get the rotation matrix component
    pub fn rotation_matrix(&self) -> Mat2 {
        Mat2::from_angle(self.rotation)
    }
    
    /// 🔗 Get the full transformation matrix
    pub fn matrix(&self) -> Mat3 {
        Mat3::from_translation(self.translation)
            * Mat3::from_angle(self.rotation)
            * Mat3::from_scale(self.scale)
    }
    
    /// 🎯 Transform a point from local → world space
    pub fn transform_point(&self, local_point: Vec2) -> Vec2 {
        self.matrix().transform_point2(local_point)
    }
    
    /// 🔄 Transform a direction from local → world space
    /// (no translation applied)
    pub fn transform_direction(&self, local_dir: Vec2) -> Vec2 {
        // Only rotation and scale affect directions
        (Mat2::from_angle(self.rotation) * Mat2::from_cols(
            Vec2::new(self.scale.x, 0.0),
            Vec2::new(0.0, self.scale.y),
        )) * local_dir
    }
    
    /// 📍 Get the "right" direction in world space
    pub fn right(&self) -> Vec2 {
        Vec2::new(self.rotation.cos(), self.rotation.sin())
    }
    
    /// 📍 Get the "up" direction in world space
    pub fn up(&self) -> Vec2 {
        Vec2::new(-self.rotation.sin(), self.rotation.cos())
    }
}

impl Default for PhysicsTransform {
    fn default() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

/// 🔄 System: Sync PhysicsTransform → Bevy Transform for rendering
pub fn sync_physics_to_render(
    mut query: Query<(&PhysicsTransform, &mut Transform)>,
) {
    for (phys, mut render) in query.iter_mut() {
        render.translation = Vec3::new(
            phys.translation.x,
            phys.translation.y,
            0.0,
        );
        render.rotation = Quat::from_rotation_z(phys.rotation);
        render.scale = Vec3::new(phys.scale.x, phys.scale.y, 1.0);
    }
}
```

---

## 🎬 Real Example: Rotating Spaceship

```rust
/// 🚀 A spaceship that rotates toward its movement direction

#[derive(Component)]
struct Spaceship {
    /// Max turn rate in radians/second
    turn_speed: f32,
}

/// 🎮 Rotate the ship to face its velocity direction
fn rotate_ship_toward_velocity(
    mut query: Query<(&Velocity, &mut PhysicsTransform, &Spaceship)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    
    for (vel, mut transform, ship) in query.iter_mut() {
        // ⛔ Don't rotate if stationary
        if vel.0.length_squared() < 0.001 {
            continue;
        }
        
        // 🧭 Target angle from velocity direction
        // atan2(y, x) gives us the angle of the velocity vector
        let target_angle = vel.0.y.atan2(vel.0.x);
        
        // 🔄 Current angle
        let current_angle = transform.rotation;
        
        // 📐 Find the shortest rotation direction
        let mut angle_diff = target_angle - current_angle;
        
        // Normalize to [-π, π] so we always take the short path
        // Without this, the ship might spin all the way around!
        while angle_diff > std::f32::consts::PI {
            angle_diff -= std::f32::consts::TAU;
        }
        while angle_diff < -std::f32::consts::PI {
            angle_diff += std::f32::consts::TAU;
        }
        
        // 🎯 Clamp rotation to max turn speed
        let max_turn = ship.turn_speed * dt;
        let rotation_step = angle_diff.clamp(-max_turn, max_turn);
        
        transform.rotation += rotation_step;
    }
}
```

---

## 🧮 Matrices for Collision Detection

Matrices aren't just for rendering — they're crucial for physics:

```rust
/// 🧱 Transform a collision shape by a physics transform
fn transform_aabb(aabb: &Aabb, transform: &PhysicsTransform) -> Aabb {
    // Get the eight corners of the AABB (4 in 2D)
    let corners = [
        Vec2::new(aabb.min.x, aabb.min.y),
        Vec2::new(aabb.max.x, aabb.min.y),
        Vec2::new(aabb.min.x, aabb.max.y),
        Vec2::new(aabb.max.x, aabb.max.y),
    ];
    
    // Transform each corner
    let transformed: Vec<Vec2> = corners
        .iter()
        .map(|c| transform.transform_point(*c))
        .collect();
    
    // Compute new bounding box
    let min_x = transformed.iter().map(|c| c.x).fold(f32::MAX, f32::min);
    let min_y = transformed.iter().map(|c| c.y).fold(f32::MAX, f32::min);
    let max_x = transformed.iter().map(|c| c.x).fold(f32::MIN, f32::max);
    let max_y = transformed.iter().map(|c| c.y).fold(f32::MIN, f32::max);
    
    Aabb {
        min: Vec2::new(min_x, min_y),
        max: Vec2::new(max_x, max_y),
    }
}

/// 📦 Axis-Aligned Bounding Box
struct Aabb {
    min: Vec2,
    max: Vec2,
}
```

---

## 📊 Quick Reference: Matrices

| Operation | 2D Bevy Code | Description |
|-----------|-------------|-------------|
| Translation | `Mat3::from_translation(v)` | Move by vector v |
| Rotation | `Mat3::from_angle(θ)` | Rotate by θ radians |
| Scale | `Mat3::from_scale(v)` | Scale by v.x, v.y |
| Identity | `Mat3::IDENTITY` | No transformation |
| Multiply | `a * b` | Combine transformations |
| Transform point | `m.transform_point2(p)` | Apply to position |
| Transform vector | `m.transform_vector2(v)` | Apply direction only |
| Decompose | `m.to_scale_angle_translation()` | Extract components |
| Inverse | `m.inverse()` | Reverse the transformation |

---

## 🎯 Chapter Summary

```
Matrices are the WORKHORSE of game transformations:

    ✅ Translation         ─ move things around
    ✅ Rotation            ─ spin things around
    ✅ Scaling             ─ make things bigger/smaller
    ✅ Composition         ─ combine all of the above
    ✅ Coordinate spaces    ─ local ↔ world ↔ camera
    ✅ Collision detection  ─ transform shapes
    ✅ Rendering pipeline   ─ every sprite on screen
    
    🔑 T × R × S — the sacred order!
```

> **Key Takeaway:** If vectors are nouns, matrices are verbs. They ACT on vectors to produce new vectors. Master both, and you describe any spatial relationship in your game. 🏆

---

**[← Previous: Vector Mathematics](03-vectors.md)** | **[Next: Quaternions →](05-quaternions.md)**
