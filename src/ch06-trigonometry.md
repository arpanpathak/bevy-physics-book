# 📐 Trigonometry for Game Physics

> **"Trigonometry is the bridge between ANGLES and POSITIONS. It's how you turn 'face 45° to the right' into 'move (0.707, 0.707) units per second.' Without trig, rotation doesn't exist."** 🌀

---

## 🎯 The Central Problem Trigonometry Solves

You're building a game. A player presses the joystick at a 45° angle. You need to move the character in that direction at speed 100 px/s.

**Without trig**, what velocity do you set?

```rust
// ❌ GUESSING: Which way does 45° go?
let velocity = Vec2::new(100.0, 100.0);
// Problem: length = √(100² + 100²) = 141.4  -  41% too fast!
// And is (100, 100) actually 45°? Yes... but the speed is wrong.
```

**With trig, it's exact:**

```rust
// ✅ EXACT: sin and cos give the correct unit vector at ANY angle
let angle_in_radians = 45.0_f32.to_radians(); // = π/4 ≈ 0.785
let direction = Vec2::new(
    angle_in_radians.cos(),  // = 0.707
    angle_in_radians.sin(),  // = 0.707
);
let velocity = direction * 100.0; // = (70.7, 70.7)  -  length = 100 ✅
```

**This is the fundamental job of trigonometry in games: convert between angles and vectors.**

---

## 📐 The Unit Circle: Where Sin and Cos Come From

A **unit circle** is a circle with radius 1 centered at the origin. The angle θ is measured counterclockwise from the positive X axis.

```
            sin(θ) axis (Y)
                ^
                |
     Quadrant II|Quadrant I
       (-,+)    |    (+,+)
                |
  <--------------+--------------> cos(θ) axis (X)
  (-1, 0)       |(1, 0)          cos(θ) = x-coordinate
                |                sin(θ) = y-coordinate
  Quadrant III  |Quadrant IV
       (-,-)    |    (+,-)
                |
                v
    (0, -1)
```

**The key insight:** For any angle θ, the point `(cos(θ), sin(θ))` is on the unit circle. This is a UNIT VECTOR at angle θ.

📐 Verify the unit circle property:
 For ANY angle θ, the vector (cos(θ), sin(θ)) has length exactly 1.

```rust
pub fn verify_unit_circle_property() {
    let angles = [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, std::f32::consts::PI];
    
    for angle in angles {
        let unit_vector = Vec2::new(angle.cos(), angle.sin());
        let length = unit_vector.length();
        
        // length should ALWAYS be 1.0 (within floating-point precision)
        assert!((length - 1.0).abs() < 0.0001, 
            "Angle {angle}: length = {length}, expected 1.0");
    }
    // This passes for ALL angles. This is the Pythagorean identity:
    // cos²(θ) + sin²(θ) = 1  ->  ‖(cos(θ), sin(θ))‖ = 1
}
```

### The Right Triangle Connection

If the unit circle seems abstract, the RIGHT TRIANGLE definition is more intuitive:

```
For a right triangle with angle θ at the origin:

           |
           |
      hyp  |  opp          sin(θ) = opposite / hypotenuse
     \    |                cos(θ) = adjacent / hypotenuse
    \ θ   |                tan(θ) = opposite / adjacent
   --------+
   adj

On the unit circle (hypotenuse = 1):
  cos(θ) = adjacent  <- the x-coordinate
  sin(θ) = opposite  <- the y-coordinate
```

---

## 🔄 The Three Core Functions

📐 SIN(θ): Returns the y-coordinate on the unit circle.

 Range: [-1, 1]
 Period: 2π (360°)  -  sin repeats every full rotation.
 Key values:
   sin(0) = 0      sin(π/2) = 1     sin(π) = 0      sin(3π/2) = -1

 Game uses:
   - Vertical oscillation (bouncing, waves)
   - Cross product magnitude
   - Sound wave generation
 📐 COS(θ): Returns the x-coordinate on the unit circle.

 Range: [-1, 1]
 Period: 2π  -  cos repeats every full rotation.
 Key values:
   cos(0) = 1      cos(π/2) = 0     cos(π) = -1     cos(3π/2) = 0

 Game uses:
   - Horizontal oscillation
   - Dot product
   - Rotation matrix construction
 📐 TAN(θ): Returns the slope = sin(θ) / cos(θ).

 Range: (-∞, ∞)  -  tan has ASYMPTOTES where cos(θ) = 0.
 Period: π (180°)  -  tan repeats every half rotation.

 Game uses:
   - Computing slopes
   - Line-of-sight angle checks
   - (Rarely used directly  -  atan2 is more useful)

```rust
pub fn sine_example(angle_radians: f32) -> f32 {
    angle_radians.sin()
}

pub fn cosine_example(angle_radians: f32) -> f32 {
    angle_radians.cos()
}

pub fn tangent_example(angle_radians: f32) -> f32 {
    angle_radians.tan()
    // ⚠️ WARNING: tan(π/2) = ∞ (division by zero!)
    // Use atan2() instead for angle calculations.
}
```

---

## 🔄 atan2: The Function You'll Use Most

**`atan2(y, x)` is the inverse of sin/cos**  -  given a vector, it returns the angle:

🎯 atan2(y, x) returns the angle of the vector (x, y).

 WHY atan2 IS BETTER THAN atan(y/x):

 atan(y/x) has TWO problems:
   1. Division by zero when x = 0
   2. Can't distinguish between Quadrants I and III
      (because y/x = (-y)/(-x), so (1,1) and (-1,-1) give the same angle!)

 atan2(y, x) solves BOTH:
   1. Handles x = 0 (returns ±π/2)
   2. Uses the SIGNS of BOTH arguments to determine the correct quadrant

 Range: [-π, π] = [-180°, 180°]
   (0, 1)  ->  π/2   (up)
   (1, 0)  ->  0     (right)
   (0, -1) -> -π/2   (down)
   (-1, 0) ->  π     (left)
 --- Complete: Aim a weapon toward the mouse cursor ---
 --- Complete: Move in the direction of an angle ---
 These two functions form a ROUND-TRIP:

 vector -> atan2 -> angle -> cos/sin -> same vector (up to length)
   (3, 4)  ->  0.927  ->  (0.6, 0.8)  ->  direction of (3, 4)!

 angle -> cos/sin -> vector -> atan2 -> same angle
   0.927  ->  (0.6, 0.8)  ->  0.927  ->  ✅

```rust
pub fn angle_of_vector(vector: Vec2) -> f32 {
    vector.y.atan2(vector.x)
}

pub fn aim_toward_target(
    shooter_position: Vec2,
    target_position: Vec2,
) -> f32 {
    // Step 1: Vector from shooter to target
    let direction_to_target = target_position - shooter_position;
    
    // Step 2: Convert to angle using atan2
    // This works for ANY relative position  -  left, right, up, down, diagonal
    let angle_to_target = direction_to_target.y.atan2(direction_to_target.x);
    
    angle_to_target
}

pub fn velocity_from_angle(angle_radians: f32, speed: f32) -> Vec2 {
    Vec2::new(
        angle_radians.cos() * speed,  // X component
        angle_radians.sin() * speed,  // Y component
    )
}

pub fn demonstrate_round_trip() {
    let original_vector = Vec2::new(3.0, 4.0);
    
    // Vector -> angle
    let angle = original_vector.y.atan2(original_vector.x); // ≈ 0.927 rad
    
    // Angle -> unit vector (same direction)
    let reconstructed_direction = Vec2::new(angle.cos(), angle.sin());
    
    // They should point in the same direction
    let dot_product = original_vector.normalize().dot(reconstructed_direction);
    assert!((dot_product - 1.0).abs() < 0.0001); // ≈ 1.0 -> same direction! ✅
}
```

---

## 🏗️ Practical Example 1: Wave Motion (Sine & Cosine in Action)

🌊 Floating platform that bobs up and down using a sine wave.

 The general form of a sine wave is:
   y(t) = amplitude × sin(2π × frequency × t + phase)

 Where:
   amplitude = how far it moves from center
   frequency = how many complete cycles per second
   phase = horizontal shift (for variety between objects)
   t = time in seconds
 The center Y position around which the platform bobs.
 How far up and down the platform moves (peak-to-peak = 2× this).
 How many full bobs per second. 1.0 = one cycle/second.
 Phase offset in radians. Use different values for different
 platforms so they don't all bob in sync.
 🌊 BUTTER SMOOTH sine wave animation:

 If you want a more "natural" feel, use a cosine wave for the
 horizontal component and sine for vertical  -  this creates
 circular/elliptical motion:

```rust
#[derive(Component)]
pub struct FloatingPlatform {
    pub base_y: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub phase_offset: f32,
}

pub fn update_floating_platform_system(
    time: Res<Time>,
    mut platform_query: Query<(&FloatingPlatform, &mut Transform)>,
) {
    let current_time = time.elapsed_secs();
    
    for (platform, mut transform) in platform_query.iter_mut() {
        // The sine wave formula:
        // offset = A × sin(2πft + φ)
        //
        // sin oscillates between -1 and 1
        // multiplying by amplitude gives range [-amplitude, +amplitude]
        // TAU = 2π = one full cycle
        let vertical_offset = platform.amplitude
            * (platform.frequency * current_time * std::f32::consts::TAU
               + platform.phase_offset)
                  .sin();
        
        transform.translation.y = platform.base_y + vertical_offset;
    }
}

pub fn circular_motion_example(
    center: Vec2,
    radius: f32,
    speed: f32,
    time: f32,
) -> Vec2 {
    let angle = speed * time;
    Vec2::new(
        center.x + angle.cos() * radius,  // Horizontal: cosine
        center.y + angle.sin() * radius,  // Vertical: sine
    )
    // This traces a PERFECT CIRCLE at constant angular velocity.
    // (cos(t), sin(t)) = UNIT CIRCLE = circular motion!
}
```

---

## 🏀 Practical Example 2: Projectile Motion

Fires a projectile toward a target using trigonometric decomposition.

 The key insight: we use atan2 to find the angle, then cos/sin to
 decompose the velocity into x and y components. Gravity then
 curves the trajectory into a parabola.
 The FULL projectile position equation at time t:

   x(t) = x₀ + v₀·cos(θ)·t
   y(t) = y₀ + v₀·sin(θ)·t - ½·g·t²

 This is the combination of:
   - Linear motion in x (no horizontal force, no drag)
   - Constant acceleration in y (gravity)

 The result is a PARABOLA. At 45°, you get MAXIMUM range.

```rust
pub fn fire_projectile_toward_target(
    commands: &mut Commands,
    origin_position: Vec2,
    target_position: Vec2,
    muzzle_velocity: f32,
) {
    // Step 1: Find the angle to the target using atan2.
    // atan2 handles ALL quadrants  -  works for targets left, right,
    // above, or below. Regular atan would fail.
    let direction_to_target = target_position - origin_position;
    let launch_angle = direction_to_target.y.atan2(direction_to_target.x);
    
    // Step 2: Decompose the velocity vector into x and y components.
    // v_x = v × cos(θ)   -  horizontal component (constant without drag)
    // v_y = v × sin(θ)   -  vertical component (affected by gravity)
    let velocity = Vec2::new(
        launch_angle.cos() * muzzle_velocity,
        launch_angle.sin() * muzzle_velocity,
    );
    
    // Step 3: Spawn the projectile with the computed velocity.
    // Gravity will automatically pull it down, creating a parabolic arc.
    commands.spawn((
        Position(origin_position),
        Velocity(velocity),
        Mass(1.0),
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(8.0)),
            ..default()
        },
    ));
}

pub fn projectile_position_at_time(
    origin: Vec2,
    initial_velocity: Vec2,
    gravity: f32,
    time: f32,
) -> Vec2 {
    Vec2::new(
        origin.x + initial_velocity.x * time,
        origin.y + initial_velocity.y * time - 0.5 * gravity * time * time,
    )
}
```

```
Projectile trajectory at different launch angles:

  angle = 75°:     \‾‾\     High arc, short range
                   \    \
                  
  angle = 45°:    \\       MAXIMUM RANGE 🏆
                 \  \
                 
  angle = 15°:   \\        Low arc, long range
                \  \
               
  The 45° angle gives the BEST balance of vertical and horizontal
  velocity. sin(45°) = cos(45°) = 0.707  -  equal components!
```

---

## 👁️ Practical Example 3: Field of View Detection

Checks whether `target_position` is within the field of view of
 an observer at `observer_position` facing `observer_facing_direction`.

 This uses the dot product, which IS cos(θ) when both vectors are
 unit vectors. We avoid computing the actual angle  -  comparing
 cosines is equivalent and MUCH faster.

```rust
pub fn is_target_in_field_of_view(
    observer_position: Vec2,
    observer_facing_direction: Vec2, // MUST be a unit vector!
    target_position: Vec2,
    field_of_view_half_angle: f32,   // In radians (e.g., 45° = π/4)
    maximum_detection_distance: f32,
) -> bool {
    // --- Step 1: Compute the vector from observer to target ---
    let vector_to_target = target_position - observer_position;
    
    // --- Step 2: Quick distance rejection (avoids trig entirely) ---
    // If the target is too far, don't bother with angle check.
    let distance_squared = vector_to_target.length_squared();
    let max_distance_squared = maximum_detection_distance * maximum_detection_distance;
    if distance_squared > max_distance_squared {
        return false; // Too far away
    }
    
    // --- Step 3: Compute cos(θ) using the DOT PRODUCT ---
    // a · b = |a| × |b| × cos(θ)
    // Since observer_facing IS a unit vector (|a| = 1):
    // observer_facing · normalized(to_target) = cos(θ)
    let direction_to_target_normalized = vector_to_target.normalize_or_zero();
    let cosine_of_angle = observer_facing_direction.dot(direction_to_target_normalized);
    
    // --- Step 4: Compare against cos(fov_angle) ---
    // cos(θ) is a DECREASING function from 0 to π:
    //   θ = 0° -> cos(θ) = 1 (directly ahead)
    //   θ = 45° -> cos(θ) = 0.707
    //   θ = 90° -> cos(θ) = 0 (to the side)
    //   θ = 180° -> cos(θ) = -1 (behind)
    //
    // So if cos(θ) > cos(fov), then θ < fov -> target IS inside FOV
    let cosine_of_field_of_view = field_of_view_half_angle.cos();
    cosine_of_angle > cosine_of_field_of_view
}
```

---

## 📊 Quick Reference: Trig for Games

| Operation | Formula | Bevy Code | When to Use |
|-----------|---------|-----------|-------------|
| Angle -> X component | x = cos(θ) × speed | `angle.cos() * speed` | Moving along an angle |
| Angle -> Y component | y = sin(θ) × speed | `angle.sin() * speed` | Moving along an angle |
| Vector -> Angle | θ = atan2(y, x) | `vector.y.atan2(vector.x)` | Aiming toward a point |
| Smooth oscillation | f(t) = A×sin(2πft) | `amplitude * (freq * t * TAU).sin()` | Bouncing, waves |
| Circular motion | (r·cos(t), r·sin(t)) | `Vec2::new(t.cos(), t.sin()) * r` | Orbiting, spinning |
| Field of view | cos(θ) = dot product | `facing.dot(to_target)` | Vision cones |
| Arc-projection | s = (v²×sin(2θ))/g | Projectile range | Weapon balancing |

---

## 🎯 Chapter Summary

```
TRIGONOMETRY IS THE BRIDGE BETWEEN ANGLES AND VECTORS:

  Angle -> Vector:     Vec2::new(angle.cos(), angle.sin()) × speed
  Vector -> Angle:     vector.y.atan2(vector.x)
  Oscillation:        amplitude × (frequency × t).sin()
  FOV Check:          facing.dot(to_target) > cos(half_fov)
  
  KEY INSIGHT: sin and cos CONVERT rotation into translation.
  To move at an angle, you don't "rotate the velocity"  -  you
  COMPUTE the velocity from the angle using trig.
  
  atan2 IS THE MOST IMPORTANT TRIG FUNCTION for games.
  It converts "where is the target?" into "which way do I aim?"
  Always use atan2(y, x), never atan(y/x).
```

> **Trig is the machinery hidden behind almost every game feature: aiming, movement, camera control, projectile physics, wave effects, field-of-view, and more. `cos`, `sin`, and `atan2`  -  master these three functions and you can build anything involving angles and positions.** 📐

> 💡 **Full source code for this chapter:** [code-examples/ch06-trigonometry/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch06-trigonometry)
> 
> The runnable project includes Cargo.toml, main.rs, and complete module files.

---

**[<- Previous: Quaternions](ch05-quaternions.md)** | **[Next: Kinematics ->](ch07-kinematics.md)**
