# 📐 Trigonometry for Game Physics

> **"Trigonometry is the geometry of the circle — it turns angles into action."** 🌀

---

## 🎯 Why Trig Is Everywhere in Games

```
┌──────────────────────────────────────────────────────────┐
│                   TRIGONOMETRY IN GAMES                   │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  🌊 Wave motion:  y = sin(time × frequency)             │
│  🏀 Projectiles:  x = cos(θ) × speed, y = sin(θ) × speed│
│  🌀 Rotation:     x' = x·cos(θ) - y·sin(θ)              │
│  🧭 FOV checks:   cos(θ) = (a·b) / (|a|·|b|)           │
│  🎵 Audio pan:    pan = sin(angle_to_listener)          │
│  🌓 Day/night:    light = max(0, sin(sun_angle))        │
│  💫 Particle trails: spiral = (cos(t), sin(t)) × t      │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## 📐 The Unit Circle

The **unit circle** is the foundation of all trig:

```
                    sin(θ)
                      ↑
         ┌───────────┬───────────┐
         │           │           │
    -1,0│   QII     │    QI     │1,0
    ←───┼───────────┼───────────┼───→ cos(θ)
         │           │           │
         │   QIII    │    QIV    │
         │           │           │
         └───────────┴───────────┘
                    ↓
                   -1,0
```

```rust
/// 📐 The unit circle in code
let angle = std::f32::consts::FRAC_PI_4;  // 45°

// cos(θ) = x-coordinate on unit circle
let x = angle.cos();  // ≈ 0.707

// sin(θ) = y-coordinate on unit circle  
let y = angle.sin();  // ≈ 0.707

// tan(θ) = sin(θ) / cos(θ) = slope
let slope = angle.tan();  // ≈ 1.0

/// 💡 Key insight: (cos(θ), sin(θ)) is a UNIT VECTOR
/// pointing in direction θ!
let direction = Vec2::new(angle.cos(), angle.sin());
// ‖direction‖ = 1.0 always!
```

---

## 🎯 The Trig Functions

```rust
/// 📐 SIN: y-coordinate on unit circle
/// Usage: vertical motion, wave effects, up/down
let sin_45 = std::f32::consts::FRAC_PI_4.sin();

/// 📐 COS: x-coordinate on unit circle  
/// Usage: horizontal motion, cycle timing
let cos_45 = std::f32::consts::FRAC_PI_4.cos();

/// 📐 TAN: slope = sin/cos
/// Usage: aiming, angle calculations
let tan_45 = std::f32::consts::FRAC_PI_4.tan();
```

### 🔄 Common Angles Reference

| Degrees | Radians | cos(θ) | sin(θ) | Use Case |
|---------|---------|--------|--------|----------|
| 0° | 0 | 1 | 0 | Facing right |
| 45° | π/4 | 0.707 | 0.707 | Diagonal movement |
| 90° | π/2 | 0 | 1 | Facing up |
| 180° | π | -1 | 0 | Facing left |
| 270° | 3π/2 | 0 | -1 | Facing down |
| 360° | 2π | 1 | 0 | Full rotation |

---

## 🏗️ Wave Motion (Sine & Cosine)

```rust
/// 🌊 Floating platform that bobs up and down
#[derive(Component)]
struct FloatingPlatform {
    /// Base Y position
    base_y: f32,
    /// How far up/down it goes
    amplitude: f32,
    /// How fast it bobs
    frequency: f32,
    /// Phase offset for variety
    phase: f32,
}

fn update_floating_platforms(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &FloatingPlatform)>,
) {
    let t = time.elapsed_secs();
    
    for (mut transform, platform) in query.iter_mut() {
        // 🌊 y = base_y + amplitude × sin(2π × frequency × t + phase)
        // sin gives smooth oscillation between -1 and 1
        let offset = platform.amplitude * 
            (platform.frequency * t * std::f32::consts::TAU + platform.phase).sin();
        
        transform.translation.y = platform.base_y + offset;
    }
}

/// 💡 Use sin() for VERTICAL oscillation, cos() for HORIZONTAL
/// Or add them for circular motion!
```

```
Pure Sine Wave:                 Sine + Cosine (Circle):
                                
    y                            y
    │   ╱╲                       │   ╭───╮
    │  ╱  ╲                      │  ╱     ╲
    │ ╱    ╲                     │ ╱       ╲
  ──┼──────────────► t           │╱         ╲► t
    │ ╲    ╱                     │╲         ╱
    │  ╲  ╱                      │ ╲       ╱
    │   ╲╱                       │  ╲     ╱
    │                            │   ╰───╯
```

---

## 🏀 Projectile Motion

```rust
/// 🏀 Fire a projectile at an angle
#[derive(Bundle)]
struct Projectile {
    pos: Position,
    vel: Velocity,
    mass: Mass,
    sprite: SpriteBundle,
}

/// 🎯 Fire at a target with proper trig
fn fire_projectile(
    commands: &mut Commands,
    origin: Vec2,
    target: Vec2,
    speed: f32,
) {
    // STEP 1: 🧭 Calculate direction using atan2
    let dx = target.x - origin.x;
    let dy = target.y - origin.y;
    let angle = dy.atan2(dx);  // atan2 handles ALL quadrants
    
    // STEP 2: 📐 Decompose velocity using trig
    // speed_x = cos(angle) × speed
    // speed_y = sin(angle) × speed
    let velocity = Vec2::new(
        angle.cos() * speed,
        angle.sin() * speed,
    );
    
    // STEP 3: 🎯 Apply gravity for arc
    // Gravity will pull it down naturally!
    commands.spawn((
        Position(origin),
        Velocity(velocity),
        Mass(1.0),
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(8.0)),
            ..default()
        },
    ));
}

/// 🧮 The full projectile equation:
/// x(t) = x₀ + v₀·cos(θ)·t
/// y(t) = y₀ + v₀·sin(θ)·t - ½·g·t²
///
/// This creates a beautiful PARABOLA!
fn projectile_position(origin: Vec2, velocity: Vec2, gravity: f32, t: f32) -> Vec2 {
    Vec2::new(
        origin.x + velocity.x * t,
        origin.y + velocity.y * t - 0.5 * gravity * t * t,
    )
}
```

```
Projectile Trajectory:

    y
    ↑
    │  ╱◉  ← Launch at angle θ with speed v₀
    │ ╱
    │╱  ╲
    │     ╲  ← Parabolic arc (gravity pulling down)
    │       ╲
    │         ╲
    │           ◉ ← Impact!
    └──────────────────────────→ x
              range

    The perfect angle? θ = 45° gives MAXIMUM range! 🏆
```

---

## 👁️ Field of View Detection

```rust
/// 🎯 Check if a target is within a field-of-view cone
fn is_in_fov(
    observer_pos: Vec2,
    observer_facing: Vec2,  // Unit vector direction
    target_pos: Vec2,
    fov_angle: f32,         // Half-angle in radians
    max_distance: f32,
) -> bool {
    // STEP 1: Vector from observer to target
    let to_target = target_pos - observer_pos;
    
    // STEP 2: Check distance (quick rejection)
    let dist_sq = to_target.length_squared();
    if dist_sq > max_distance * max_distance {
        return false;  // Too far
    }
    
    // STEP 3: 🔥 THE DOT PRODUCT = cos(angle)
    // a·b = |a|·|b|·cos(θ)
    // Since observer_facing is unit and we normalize to_target:
    let to_target_norm = to_target.normalize_or_zero();
    let cos_angle = observer_facing.dot(to_target_norm);
    
    // STEP 4: Compare against cos(fov_angle)
    // cos is DECREASING from 0 to π, so:
    // If cos_angle > cos(fov) → angle < fov → INSIDE cone
    cos_angle > fov_angle.cos()
}

/// 💡 Why cos(angle) instead of computing the angle directly?
/// cos_angle is a simple dot product — no acos() needed!
/// acos() is expensive and has numerical edge cases.
```

```
    Field of View:
    
              ╱  │  ╲
             ╱   │   ╲
            ╱    │ θ  ╲  ← fov_angle
           ╱     │     ╲
          ╱      │      ╲
         ╱    ───┼───    ╲
        ╱    observer     ╲
       ╱                   ╲
      ╱   ✅ Inside FOV     ╲
     ╱                       ╲
    ╱   ❌ Outside FOV        ╲
    
    θ < fov_angle → IN
    θ > fov_angle → OUT
```

---

## 🎮 Putting It All Together: 2D Aiming System

```rust
/// 🎯 Complete 2D aiming toward mouse cursor

#[derive(Component)]
struct Gun {
    /// Barrel length (where bullets spawn)
    barrel_length: f32,
    /// Fire rate (shots per second)
    fire_rate: f32,
    timer: f32,
}

fn aim_toward_mouse(
    windows: Query<&Window>,
    mut player_query: Query<(&mut Transform, &Gun), With<Player>>,
) {
    // Get mouse position in world coordinates
    let window = windows.single();
    let cursor_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };
    
    // Convert screen coords to world coords (simplified)
    let mouse_world = Vec2::new(
        cursor_pos.x - window.width() / 2.0,
        -(cursor_pos.y - window.height() / 2.0),
    );
    
    for (mut transform, gun) in player_query.iter_mut() {
        // 📐 atan2 gives us the angle to the mouse!
        let angle = mouse_world.y.atan2(mouse_world.x);
        transform.rotation = Quat::from_rotation_z(angle);
        
        // 📍 Barrel tip position (where bullets spawn)
        let barrel_tip = Vec2::new(
            transform.translation.x + angle.cos() * gun.barrel_length,
            transform.translation.y + angle.sin() * gun.barrel_length,
        );
        
        // Store barrel tip for bullet spawning...
    }
}
```

---

## 📊 Quick Reference: Trig Identities for Games

| Identity | Formula | Use Case |
|----------|---------|----------|
| sin² + cos² | sin²(θ) + cos²(θ) = 1 | Verify unit circle |
| Double angle | sin(2θ) = 2·sin(θ)·cos(θ) | Fast oscillation |
| Law of cos | c² = a² + b² - 2ab·cos(C) | Any triangle solving |
| atan2 | atan2(y, x) | Angle from vector |
| sin → cos | cos(θ) = sin(θ + π/2) | Phase shifting |
| Dot → cos | a·b = |a||b|cos(θ) | Angle between vectors |

---

## 🎯 Chapter Summary

```rust
/// 📝 Trig cheat sheet for game physics:

// ANGLE → VECTOR (polar to cartesian)
let direction = Vec2::new(angle.cos(), angle.sin());

// VECTOR → ANGLE
let angle = vector.y.atan2(vector.x);

// SMOOTH OSCILLATION
let value = amplitude * (frequency * t).sin();

// FIELD OF VIEW CHECK
let in_fov = facing.dot(to_target_norm) > fov_angle.cos();

// PROJECTILE LAUNCH
let vx = speed * angle.cos();
let vy = speed * angle.sin();
```

> **Key Takeaway:** Trig is how angles become action. `sin` and `cos` translate rotations into movement. `atan2` translates positions into angles. Master these three functions, and you can build anything from aiming systems to orbital mechanics! ⭐

---

**[← Previous: Quaternions](05-quaternions.md)** | **[Next: Kinematics →](07-kinematics.md)**
