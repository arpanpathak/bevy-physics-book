# 🧮 Vector Mathematics: The Language of Space

> **"Vectors are to game physics what nouns are to language — the fundamental building blocks of everything you want to express."** 🗺️

---

## 📐 What Is a Vector?

A **vector** is a quantity that has both **magnitude** (length) and **direction**. 

```
Think of an arrow:   ⬆️  Which way? Up!
                     📏  How far? 5 units!

        y
        │
    5   │   ┌───► (3, 4)
        │   │   ╱
        │   │  ╱  length = 5
        │   │ ╱
        │   │╱
        ────┼──────────────────► x
        │   │
        │   │
```

In Bevy, 2D vectors are `Vec2` and 3D vectors are `Vec3`:

```rust
use bevy::prelude::*;

// 📍 A 2D vector representing a point or direction
let position: Vec2 = Vec2::new(3.0, 4.0);

// 📍 A 3D vector
let velocity: Vec3 = Vec3::new(1.0, 0.5, 0.0);

// 🏠 Vectors are Copy types — they're cheap to pass around
```

---

## 🎯 Vectors Represent Two Things

In game physics, vectors play **dual roles**:

| Role | Example | Description |
|------|---------|-------------|
| **📍 Point (Position)** | `Vec2::new(100, 200)` | A location in space relative to origin |
| **🏃 Direction/Movement** | `Vec2::new(5, -3)` | A change or direction, no fixed location |

```rust
/// 📍 Position is a POINT vector — it anchors us in space
let player_pos = Vec2::new(400.0, 300.0);

/// 🏃 Velocity is a DIRECTION vector — it tells us where we're going
let player_vel = Vec2::new(50.0, -20.0);

/// After 1 second, our new position is:
let new_pos = player_pos + player_vel;  // = (450, 280)
```

> 💡 **Key Insight:** The math is the same for both! Adding a direction to a point moves the point. This is the foundation of all physics integration.

---

## ➕ Vector Operations

### 1️⃣ Addition: `a + b`

Vectors add **component-wise**: x + x, y + y.

```
    a = (3, 1)     b = (1, 2)
    
         a + b = (3+1, 1+2) = (4, 3)
    
    y
    │
    3 ──────► (a+b)  ← The combined result
    │      ╱
    2 ──► b      ╱
    │   ╱  ╱
    1─►a   ╱
    │  ╱
    ────┼───► x
        1 2 3 4
```

```rust
// 🎮 In code: moving a character
let character_pos = Vec2::new(100.0, 200.0);
let movement = Vec2::new(50.0, 0.0); // Move right 50 units

let new_position = character_pos + movement;
// new_position = (150, 200) — the character moved right!

// 📝 Bevy's Vec2 already overloads the + operator
```

### 2️⃣ Subtraction: `a - b`

Tells us the vector **from b to a**.

```rust
// 🎯 Finding the direction from enemy to player
let enemy_pos = Vec2::new(100.0, 100.0);
let player_pos = Vec2::new(200.0, 150.0);

// Vector FROM enemy TO player
let enemy_to_player = player_pos - enemy_pos;
// enemy_to_player = (100, 50)

// This is SUPER useful for:
// - AI chasing the player
// - Projectiles homing in on targets
// - Calculating distances between objects
```

### 3️⃣ Scalar Multiplication: `a × s`

Scales the vector by a factor: each component gets multiplied.

```rust
let direction = Vec2::new(2.0, 1.0);

// 🏃 Double the speed
let double = direction * 2.0;
// double = (4, 2)

// 🐢 Half the speed
let half = direction * 0.5;
// half = (1, 0.5)

// 🔄 Reverse direction
let reverse = direction * (-1.0);
// reverse = (-2, -1)
```

```
    Visual: Multiplying by scalars
    y
    │
    2 ────► direction * 2 = (4, 2)
    │    ╱
    1 ──► direction = (2, 1)
    │  ╱
    ────┼───► x
        1 2 3 4
```

---

## 📏 Vector Magnitude (Length)

The **magnitude** (also called norm or length) is the distance from the vector's tail to its tip.

```
    For v = (x, y):
    ‖v‖ = √(x² + y²)

    Example: v = (3, 4)
    ‖v‖ = √(3² + 4²) = √(9 + 16) = √25 = 5
```

```rust
use bevy::prelude::*;

let v = Vec2::new(3.0, 4.0);

// 📏 Get the length
let length = v.length();   // = 5.0

// 📏 Squared length (faster, no square root!)
let length_sq = v.length_squared();  // = 25.0

/// 💡 USE CASES:
/// - length()     : When you need the actual distance
/// - length_squared() : When comparing distances (avoids sqrt!)
```

### ⚡ Length Squared: The Performance Trick

Computing `length()` requires a **square root** — one of the most expensive math operations. `length_squared()` skips it:

```rust
// ❌ Slow approach (square root for every pair)
fn find_closest_slow(player: Vec2, enemies: &[Vec2]) -> Vec2 {
    let mut closest = enemies[0];
    let mut min_dist = f32::MAX;
    
    for enemy in enemies {
        let dist = (player - *enemy).length();  // 😱 sqrt() every time!
        if dist < min_dist {
            min_dist = dist;
            closest = *enemy;
        }
    }
    closest
}

// ✅ Fast approach (no square roots!)
fn find_closest_fast(player: Vec2, enemies: &[Vec2]) -> Vec2 {
    let mut closest = enemies[0];
    let mut min_dist_sq = f32::MAX;
    
    for enemy in enemies {
        let dist_sq = player.distance_squared(*enemy);  // ⚡ No sqrt!
        if dist_sq < min_dist_sq {
            min_dist_sq = dist_sq;
            closest = *enemy;
        }
    }
    closest
}

// 📊 For 1000 enemies: ~200x faster with squared comparison
```

---

## 🧭 Vector Normalization

**Normalization** scales a vector to have **length = 1** while preserving its direction. The result is called a **unit vector**.

```rust
let v = Vec2::new(3.0, 4.0);
let unit = v.normalize();  // = (0.6, 0.8)
// Check: ‖unit‖ = √(0.6² + 0.8²) = √(0.36 + 0.64) = √1 = 1.0 ✓
```

```
    Before (length = 5):          After (length = 1):
        (3, 4)                        (0.6, 0.8)
        
    y                               y
    │                               │
    4 ────► (3,4)                   1─►(0.6, 0.8)
    │   ╱                          │ ╱
    │  ╱ 5                         │╱ 1
    │ ╱                            │
    ────┼───► x                    ────┼───► x
        3                               1
```

```rust
/// 🎯 The most common use: create a direction from positions
fn chase_player(enemy_pos: Vec2, player_pos: Vec2, speed: f32) -> Vec2 {
    // 1️⃣ Vector from enemy to player
    let direction = player_pos - enemy_pos;
    
    // 2️⃣ Normalize to get a purely directional unit vector
    let direction_normalized = direction.normalize();
    
    // 3️⃣ Multiply by speed to get velocity
    let velocity = direction_normalized * speed;
    
    velocity
}

/// 💡 If direction is (0, 0), normalize() returns (0, 0)
/// Bevy handles this gracefully — no NaN!
```

### ⚠️ The Zero Vector Problem

```rust
let zero = Vec2::ZERO;

// normalize() on zero vector returns zero (Bevy safety)
let safe = zero.normalize();  // → Vec2::ZERO

// But if you want explicit handling:
let vec = some_function();
if vec == Vec2::ZERO {
    // Don't normalize — just skip or use a default direction
} else {
    let dir = vec.normalize();
}
```

---

## 🎯 The Dot Product: The Most Useful Operation

The **dot product** takes two vectors and returns a **scalar**:

```
    a · b = a.x × b.x + a.y × b.y
    
    Also: a · b = ‖a‖ × ‖b‖ × cos(θ)
    
    Where θ is the angle BETWEEN the two vectors
```

```rust
let a = Vec2::new(1.0, 0.0);   // Points right
let b = Vec2::new(0.0, 1.0);   // Points up

let dot = a.dot(b);  // = 0 (they're perpendicular!)
```

### What the Dot Product Tells You

| Dot Product | Angle | Meaning |
|-------------|-------|---------|
| `> 0` | < 90° | Pointing in **similar** directions |
| `= 0` | = 90° | **Perpendicular** (orthogonal) |
| `< 0` | > 90° | Pointing in **opposite** directions |

```
    a · b > 0            a · b = 0            a · b < 0
    ╱                   ╱                      ╲
   a                   a    ↑                  a   ↑
  ╱                    ╱    │                   ╲  │
  ──► b               ──►  b                   ──► b
  "Same way"         "Sideways"              "Opposite"
```

### 🎮 Game Uses of Dot Product

```rust
/// 🎯 1. Is the player in front of me?
fn is_in_front_of(character: Vec2, facing: Vec2, target: Vec2) -> bool {
    // Vector from character to target
    let to_target = (target - character).normalize();
    
    // Dot product: positive → in front, negative → behind
    let dot = facing.dot(to_target);
    
    dot > 0.0  // In front!
}

/// 🔦 2. Light/visibility cone check
fn is_in_cone(facing: Vec2, to_target: Vec2, cone_half_angle: f32) -> bool {
    let facing_norm = facing.normalize();
    let to_target_norm = to_target.normalize();
    
    // cos(angle) from dot product
    let cos_angle = facing_norm.dot(to_target_norm);
    let cos_cone = cone_half_angle.cos();
    
    cos_angle > cos_cone  // Inside the cone!
}

/// 💫 3. Project one vector onto another
fn project_onto(a: Vec2, b: Vec2) -> Vec2 {
    let b_norm = b.normalize();
    b_norm * a.dot(b_norm)  // "Shadow" of a onto b
}
```

---

## ✖️ The Cross Product (2D "Perp" Operation)

In 2D, the cross product of two vectors returns a **scalar** representing the signed area of the parallelogram:

```
    a × b = a.x × b.y - a.y × b.x
    
    Positive → b is to the LEFT of a
    Negative → b is to the RIGHT of a
    Zero     → a and b are parallel
```

```rust
let a = Vec2::new(1.0, 0.0);   // Right
let b = Vec2::new(0.0, 1.0);   // Up

let cross = a.perp_dot(b);     // = 1.0 (b is left of a)
// In Bevy, use .perp_dot() for 2D cross product
```

### 🎮 Game Uses

```rust
/// 🔄 1. Determine turning direction
fn turn_toward(facing: Vec2, target_dir: Vec2) -> f32 {
    let cross = facing.perp_dot(target_dir);
    
    if cross > 0.0 {
        1.0   // Turn left (counter-clockwise)
    } else if cross < 0.0 {
        -1.0  // Turn right (clockwise)
    } else {
        0.0   // Already facing the target
    }
}

/// 📐 2. Get the perpendicular (90° rotated) vector
let right = Vec2::new(1.0, 0.0);
let up = right.perp();           // (0, 1) — rotate 90° CCW
let down = right.perp().perp();  // (-1, 0) — rotate 180° total
```

---

## 🏗️ Building a Vector Math Module

Let's implement a custom `VecMath` trait to extend Bevy's vectors:

```rust
// 📁 src/physics/vec_math.rs
//! 🧮 Extended vector math utilities
//!
//! Bevy's Vec2/Vec3 are excellent, but sometimes we want
//! convenience methods specific to game physics.

use bevy::prelude::*;

/// 🎯 Extension trait adding physics-specific vector operations
pub trait VecPhysics {
    /// 📏 Compute the angle of this vector (in radians)
    fn angle(&self) -> f32;
    
    /// 🔄 Rotate the vector by an angle (in radians)
    fn rotate(&self, angle: f32) -> Self;
    
    /// 🧭 Linear interpolation towards another vector
    fn lerp(&self, other: Self, t: f32) -> Self;
    
    /// 🎯 Clamp the magnitude to a maximum value
    fn clamp_length(&self, max: f32) -> Self;
}

impl VecPhysics for Vec2 {
    fn angle(&self) -> f32 {
        // atan2 handles all quadrants correctly
        // Returns angle in radians: -π to +π
        self.y.atan2(self.x)
    }
    
    fn rotate(&self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        // 📐 2D rotation matrix:
        // [cos  -sin] [x]
        // [sin   cos] [y]
        Vec2::new(
            self.x * cos - self.y * sin,
            self.x * sin + self.y * cos,
        )
    }
    
    fn lerp(&self, other: Self, t: f32) -> Self {
        // 📊 Linear interpolation: a + (b - a) * t
        // t = 0 → self, t = 1 → other
        *self + (other - *self) * t.clamp(0.0, 1.0)
    }
    
    fn clamp_length(&self, max: f32) -> Self {
        if self.length_squared() > max * max {
            self.normalize() * max
        } else {
            *self
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// 🧪 TESTS
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_angle() {
        let v = Vec2::new(1.0, 0.0);
        assert!((v.angle() - 0.0).abs() < 1e-6);
        
        let v = Vec2::new(0.0, 1.0);
        assert!((v.angle() - std::f32::consts::FRAC_PI_2).abs() < 1e-6);
    }
    
    #[test]
    fn test_clamp_length() {
        let v = Vec2::new(10.0, 0.0);
        let clamped = v.clamp_length(5.0);
        assert!((clamped.length() - 5.0).abs() < 1e-6);
        
        let v = Vec2::new(3.0, 0.0);
        let clamped = v.clamp_length(5.0);
        assert!((clamped.length() - 3.0).abs() < 1e-6); // Unchanged
    }
    
    #[test]
    fn test_rotate() {
        let v = Vec2::new(1.0, 0.0);
        let rotated = v.rotate(std::f32::consts::FRAC_PI_2);
        assert!((rotated.x).abs() < 1e-6);   // Should be ~0
        assert!((rotated.y - 1.0).abs() < 1e-6); // Should be 1
    }
}
```

---

## 🎯 Putting Vectors to Use: A Movement System

```rust
/// 🏃 A complete 2D movement system using vectors

#[derive(Component)]
struct Player {
    speed: f32,      // Max movement speed
    acceleration: f32, // How fast we reach max speed
}

/// 🎮 Player movement — handles WASD input using vector math
fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Velocity, &mut Position)>,
) {
    let dt = time.delta_secs();
    
    for (player, mut velocity, mut position) in query.iter_mut() {
        // STEP 1: 🎮 Build input direction vector
        let mut input_dir = Vec2::ZERO;
        
        if keyboard.pressed(KeyCode::KeyW) { input_dir.y += 1.0; }
        if keyboard.pressed(KeyCode::KeyS) { input_dir.y -= 1.0; }
        if keyboard.pressed(KeyCode::KeyA) { input_dir.x -= 1.0; }
        if keyboard.pressed(KeyCode::KeyD) { input_dir.x += 1.0; }
        
        // STEP 2: 🧭 Normalize so diagonal isn't faster
        // Without normalization: pressing W+D gives velocity = (1, 1)
        //   length = √(1+1) ≈ 1.414 — 41% faster than just W!
        if input_dir != Vec2::ZERO {
            input_dir = input_dir.normalize();
        }
        
        // STEP 3: 🏃 Apply acceleration toward target velocity
        let target_vel = input_dir * player.speed;
        velocity.0 = velocity.0.lerp(target_vel, player.acceleration * dt);
        
        // STEP 4: 📍 Update position using the velocity
        position.0 += velocity.0 * dt;
    }
}

/// 💡 Vector math gives us diagonal movement for free!
/// The lerp gives smooth acceleration/deceleration.
/// Normalization ensures consistent speed in all directions.
```

---

## 📊 Quick Reference: Vector Operations

| Operation | Math Notation | Bevy Code | Result |
|-----------|--------------|-----------|--------|
| Addition | a + b | `a + b` | Component-wise sum |
| Subtraction | a - b | `a - b` | Difference vector |
| Scale | a × s | `a * s` | Scaled vector |
| Magnitude | ∥a∥ | `a.length()` | Length as f32 |
| Normalize | â | `a.normalize()` | Unit vector |
| Dot Product | a · b | `a.dot(b)` | f32 (direction similarity) |
| Perp Dot | a × b | `a.perp_dot(b)` | f32 (left/right test) |
| Distance | ∥a - b∥ | `a.distance(b)` | f32 (between points) |
| Lerp | a + t(b-a) | `a.lerp(b, t)` | Interpolated vector |

---

## 🎯 Chapter Summary

```
🧮 Vectors are everywhere in game physics:
    📍 Position     → Where am I?
    🏃 Velocity     → How fast and where am I going?
    ⚡ Acceleration → What forces are acting on me?
    🧭 Direction    → Which way should I face/move?
    📏 Distance     → How far apart are things?
    🎯 Dot Product  → Are they facing each other?
```

> **Key Takeaway:** Master vectors, and you've mastered 80% of game physics math. Everything else builds on top of this foundation. 🏗️

---

**[← Previous: Setup](02-setup.md)** | **[Next: Matrices & Transformations →](04-matrices.md)**
