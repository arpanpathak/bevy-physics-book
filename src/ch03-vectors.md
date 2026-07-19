# 🧮 Vector Mathematics: The Language of Space

> **Imagine you're making a game where a character needs to chase a target. You have two positions: `player = (100, 200)` and `enemy = (300, 150)`. How do you figure out which way to move? What's the distance? How fast should you go? Vectors answer ALL of these with one elegant system.** 🗺️

---

## 🎯 The Problem Before the Definition

Before I tell you what a vector IS, let me show you what problem it solves.

**You're building an enemy AI. The enemy needs to move toward the player. Here's the brute-force approach WITHOUT vectors:**

```rust
fn move_enemy_toward_player(
    enemy_x: f32, enemy_y: f32,
    player_x: f32, player_y: f32,
    speed: f32,
) -> (f32, f32) {
    // Step 1: Figure out the difference in x and y
    let difference_x = player_x - enemy_x;  // = 200
    let difference_y = player_y - enemy_y;  // = -50
    
    // Step 2: Figure out the total distance (Pythagorean theorem)
    let distance = (difference_x * difference_x + difference_y * difference_y).sqrt();
    // = sqrt(200^2 + (-50)^2) = sqrt(40000 + 2500) = sqrt(42500) ≈ 206.2
    
    // Step 3: Figure out the direction (divide each component by distance)
    let direction_x = difference_x / distance;  // = 200 / 206.2 ≈ 0.97
    let direction_y = difference_y / distance;  // = -50 / 206.2 ≈ -0.24
    
    // Step 4: Multiply direction by speed
    let velocity_x = direction_x * speed;
    let velocity_y = direction_y * speed;
    
    (velocity_x, velocity_y)
}
```

This works. But look at all the manual x/y bookkeeping. Every time you work with positions, you manually separate and recombine x and y. This is where bugs hide.

**WITH vectors, the SAME logic is THREE LINES:**

```rust
use bevy::prelude::*;

fn move_enemy_toward_player(
    enemy_position: Vec2,
    player_position: Vec2,
    speed: f32,
) -> Vec2 {
    let direction_toward_player = (player_position - enemy_position).normalize();
    direction_toward_player * speed
}
```

**That's it.** Subtraction to find the offset. Normalize to get pure direction. Multiply by speed to get velocity. Three operations, zero manual x/y management.

This is what vectors do: they let you think about SPATIAL RELATIONSHIPS as UNITS, not as pairs of numbers.

---

## 📐 What Is a Vector? (The Definition That Actually Makes Sense)

A **vector** is a quantity that has both **magnitude** (how much) and **direction** (which way).

**Think of it as an ARROW:**
- The arrow's length = magnitude
- Where the arrow points = direction
- The arrow's tip location = the vector's (x, y) components

```
        y
        |
    5   |   +---> (3, 4)         <- This arrow IS the vector (3, 4)
        |   |   \
        |   |  \  length = 5     <- magnitude = √(3² + 4²) = 5
        |   | \
        |   |\  angle ≈ 53°     <- direction = arctan(4/3) ≈ 53°
        +---+---------------> x
            0   3
    
    The vector (3, 4) says: "go 3 units right and 4 units up"
    The length 5 says: "the straight-line distance is 5 units"
    The angle 53° says: "the direction is about 53° from horizontal"
```

### Two Things a Vector Can Represent

This is the MOST important concept in this entire chapter. A vector can represent TWO different things, and you MUST know which one you're using:

#### 1. A POSITION (a point in space)

📍 Position vector: measured FROM the origin (0,0) TO a location.

```rust
let player_position = Vec2::new(400.0, 300.0);
// "The player is 400 pixels right and 300 pixels up from (0,0)."
// The origin is implied. The vector tells you WHERE.
```

#### 2. A CHANGE/DIRECTION (an offset, a movement, a velocity)

🏃 Direction vector: a CHANGE to apply to a position.

```rust
let movement_this_frame = Vec2::new(50.0, 0.0);
// "Move 50 pixels right and 0 pixels up from your current position."
// There's no origin. The vector tells you HOW TO MOVE.
```

**THE KEY INSIGHT: The MATH is identical for both.** You can add a direction to a position to get a new position. You can subtract two positions to get a direction. The SAME vector type handles both concepts:

```rust
let position = Vec2::new(100.0, 200.0);  // 📍 WHERE I AM
let velocity = Vec2::new(50.0, -20.0);   // 🏃 HOW I'M MOVING
let new_position = position + velocity;   // 📍 WHERE I'LL BE
// = (150, 180)  -  moved right and up slightly
```

**This ability to mix positions and directions with the SAME math is why vectors are the foundation of ALL game physics.**

---

## ➕ Addition: How Everything Moves

Vector addition is the SINGLE most important operation in game physics. It's how everything moves. Let me explain why.

**Think about what "moving" actually means.** You're at position A. You want to be at position B. The difference between B and A is a vector - it tells you how far and which way to go. Adding that difference to your current position gives you the new position.

```
position + movement = new_position
   (A)      (B-A)        (B)
```

This is so fundamental that we often don't think about it. But every frame of every game, this exact operation happens for every moving object.

**The geometric meaning:** Place two vectors tip-to-tail. The result goes from the first tail to the second tip.

```
  Visual: Adding (3, 1) + (1, 2) = (4, 3)

    y
    |
    3 ----> (4, 3)    <- The result: start at (0,0), go to (4,3)
    |    /
    2 --> b   /        <- b = (1, 2) placed at a's tip
    | \   /
    1->a  /            <- a = (3, 1) from the origin
    |\/
    ----+---> x
        1 2 3 4
```

**Why this works for physics:** Position and velocity are BOTH Vec2. Adding them is physically meaningful - "where I am plus how I'm moving tells me where I'll be."

Moving a character by adding velocity to position:

```rust
let character_position = Vec2::new(100.0, 200.0);
let velocity = Vec2::new(50.0, 0.0);  // Moving right 50 px/s
let delta_time = 1.0 / 60.0;          // One frame

// THIS IS THE ENTIRE PHYSICS ENGINE, IN ONE LINE:
let new_position = character_position + velocity * delta_time;
// = (100, 200) + (0.833, 0) = (100.833, 200)
// The character moved 0.833 pixels to the right this frame.

// Why velocity * delta_time? Because velocity is measured in
// "pixels per SECOND." Multiplying by delta_time (seconds per frame)
// converts it to "pixels THIS FRAME." This is called the "delta."
```

Every frame of every physics simulation does exactly this. All the complexity of forces, collisions, and constraints eventually feeds into this single addition.

---

## ➖ Subtraction: Finding What's Between Two Points

Subtraction tells you "what vector takes me from point A to point B."

```rust
let player_position = Vec2::new(200.0, 150.0);
let enemy_position = Vec2::new(100.0, 100.0);

// 🧭 FROM enemy TO player:
let enemy_to_player = player_position - enemy_position;
// = (200 - 100, 150 - 100) = (100, 50)
// "To reach the player, the enemy must go 100 RIGHT and 50 UP."

// 🧭 FROM player TO enemy (just reverse the subtraction):
let player_to_enemy = enemy_position - player_position;
// = (-100, -50)
// "To reach the enemy, the player must go 100 LEFT and 50 DOWN."
```

**This is used EVERYWHERE in games:**
- AI pathfinding (subtract positions to find the direction to move)
- Projectile aiming (subtract shooter pos from target pos to find aim direction)
- Distance calculation (subtract, then measure the length of the result)
- Field-of-view checks (subtract, then dot with facing direction)
- Collision detection (subtract centers to find overlap direction)

---

## ✖️ Scalar Multiplication: Speed Control

Multiplying a vector by a number changes its length but NOT its direction:

🧭 PURE DIRECTION (length = 1):
 🏃 DIFFERENT SPEEDS, SAME DIRECTION:
 THE CORE PATTERN: Direction × Speed = Velocity

 This is the single most common vector pattern in game physics.
 1. Find the direction (unit vector from you to target)
 2. Multiply by the desired speed
 3. The result is the velocity!

```rust
let unit_direction = Vec2::new(1.0, 0.0);  // Points right

let slow_velocity = unit_direction * 50.0;   // (50, 0)  -  slow movement
let fast_velocity = unit_direction * 200.0;  // (200, 0)  -  fast movement
let backward = unit_direction * (-1.0);      // (-1, 0)  -  reversed direction

fn compute_velocity(from: Vec2, to: Vec2, desired_speed: f32) -> Vec2 {
    let raw_direction = to - from;           // Step 1: vector to target
    let unit_direction = raw_direction.normalize();  // Step 2: pure direction
    unit_direction * desired_speed           // Step 3: apply speed
}
```

---

## 📏 Magnitude: How Far? (The Pythagorean Theorem in Disguise)

The **magnitude** (length) of a vector is the straight-line distance from its tail to its tip:

```
  For vector v = (x, y):
  
  ‖v‖ = √(x² + y²)
  
  This is the Pythagorean theorem! x and y are the legs
  of a right triangle, and the vector IS the hypotenuse:
  
         |
       y |   +---> v = (x, y)
         |   |  \
         |   | \  ‖v‖ = √(x² + y²) 
         |   |\
         +---+-------->
             x
```

```rust
let vector = Vec2::new(3.0, 4.0);
let length = vector.length();           // = 5.0  (computes sqrt)
let length_squared = vector.length_squared(); // = 25.0 (no sqrt!)

// 💡 When to use which:
// length()          -> When you need the ACTUAL distance value
// length_squared()  -> When you're COMPARING distances (much faster!)
//
// Why? sqrt() is expensive. But if a < b, then sqrt(a) < sqrt(b).
// So comparing squared values gives IDENTICAL results without sqrt.
```

### The Performance Trick: Comparing Without sqrt()

```rust
// ❌ SLOW: sqrt() for EVERY comparison (1000 enemies = 1000 sqrts)
fn find_closest_enemy_slow(
    player: Vec2,
    enemies: &[Vec2],
) -> Vec2 {
    let mut closest = enemies[0];
    let mut minimum_distance = f32::MAX;
    for enemy in enemies {
        let distance = player.distance(*enemy);  // sqrt() inside!
        if distance < minimum_distance {
            minimum_distance = distance;
            closest = *enemy;
        }
    }
    closest
}

// ✅ FAST: NO sqrt() at all (1000 enemies = 0 sqrts)
fn find_closest_enemy_fast(
    player: Vec2,
    enemies: &[Vec2],
) -> Vec2 {
    let mut closest = enemies[0];
    let mut minimum_distance_squared = f32::MAX;
    for enemy in enemies {
        let distance_squared = player.distance_squared(*enemy);  // No sqrt!
        if distance_squared < minimum_distance_squared {
            minimum_distance_squared = distance_squared;
            closest = *enemy;
        }
    }
    closest
}

// 📊 For 1000 enemies: ~200x faster with squared comparison
```

---

## 🧭 Normalization: Getting Pure Direction

**Normalization** scales a vector to have exactly length 1.0 while keeping its direction. The result is a **unit vector**.

```rust
let vector = Vec2::new(3.0, 4.0);  // Length = 5
let unit = vector.normalize();     // = (0.6, 0.8), Length = 1.0

// What happened inside normalize():
//   1. Compute length: √(3² + 4²) = 5
//   2. Divide each component: (3/5, 4/5) = (0.6, 0.8)
//   3. Verify: √(0.6² + 0.8²) = √1 = 1.0 ✅
```

**Why normalize? To prevent the "diagonal speed boost" bug:**

```rust
// Pressing W (up) only:
let input_w = Vec2::new(0.0, 1.0);
input_w.length();  // = 1.0 -> speed = 1.0 × desired_speed ✅

// Pressing W + D (up + right)  -  WITHOUT normalize:
let input_wd = Vec2::new(1.0, 1.0);
input_wd.length();  // = 1.414 -> speed = 1.414 × desired_speed ❌
// The player moves 41% FASTER diagonally!

// Pressing W + D  -  WITH normalize:
let input_wd_normalized = Vec2::new(1.0, 1.0).normalize();
input_wd_normalized.length();  // = 1.0 -> speed = 1.0 × desired_speed ✅
// Consistent speed in ALL directions!
```

### The Zero Vector Problem

```rust
let zero = Vec2::ZERO;
let result = zero.normalize();   // Returns ZERO (safe, no crash)
let result2 = zero.normalize_or_zero();  // Same, but explicit

// ALWAYS check for zero before normalizing if you need custom behavior:
fn safe_normalize(v: Vec2, default_direction: Vec2) -> Vec2 {
    if v == Vec2::ZERO {
        default_direction  // Use a fallback instead of zero
    } else {
        v.normalize()
    }
}
```

---

## 🎯 The Dot Product: Angle Without Trigonometry

The dot product is the single most useful vector operation in game physics. If you understand only one operation deeply, make it this one.

**What does the dot product DO?**

It takes two vectors and returns a single number (a scalar). That number tells you how much one vector "projects onto" another - or in plain English, **how much they point in the same direction.**

**But WHY does multiplying components and adding them work?**

Imagine you have a vector a = (3, 1). It points 3 units right and 1 unit up. Now imagine another vector b = (2, 0) - it points purely right.

```
a . b = (3)(2) + (1)(0) = 6
```

What does 6 mean? It means **a has 6 units of "rightwardness"** when measured against b. The x-component of a (which is 3) multiplied by b's x (which is 2) gives us the contribution from the x-axis. The y-components contribute nothing because b has no y. The dot product extracts how much of a is parallel to b.

**The complete geometric meaning:**

```
a . b = ‖a‖ × ‖b‖ × cos(θ)
```

Where θ is the angle BETWEEN the two vectors. This formula packs three pieces of information:

1. **‖a‖** - the length of a (how much vector a there is)
2. **‖b‖** - the length of b (how much vector b there is)
3. **cos(θ)** - how aligned they are (1.0 = same direction, 0.0 = perpendicular, -1.0 = opposite)

**The magic happens with UNIT vectors.**

If BOTH vectors have length 1 (they're "unit vectors"), the formula collapses:

```
a . b = cos(θ)
```

Just the cosine of the angle. Nothing else. This means:
- Two vectors pointing the SAME direction: dot = cos(0) = +1.0
- Two vectors at RIGHT ANGLES: dot = cos(90) = 0.0
- Two vectors pointing OPPOSITE: dot = cos(180) = -1.0
- Two vectors at 45 degrees: dot = cos(45) ≈ 0.707

You now know the angle between any two directions using only four multiplications and three additions. No trig functions, no atan, no acos.

**Wait, why do we multiply components instead of computing cos directly?**

Because cos(θ) is EXPENSIVE to compute (it uses a Taylor series internally). The dot product uses only multiplication and addition - operations that CPUs can do in a single clock cycle. It's 10-100x faster than calling cos().

**What does the result mean in practice?**

```rust
let forward = Vec2::new(1.0, 0.0);  // Unit vector pointing right

// POSITIVE: Same general direction (angle < 90)
// The dot product equals cos(angle), and cos is positive for angles < 90
forward.dot(Vec2::new(1.0, 0.5).normalize());  // = 0.894 -> angle ≈ 26 degrees

// ZERO: Perpendicular (angle = 90)
// cos(90) = 0, so the dot product is exactly 0
forward.dot(Vec2::new(0.0, 1.0));  // = 0.0 -> angle is exactly 90 degrees

// NEGATIVE: Opposite direction (angle > 90)
// cos is negative for angles > 90
forward.dot(Vec2::new(-1.0, 0.0));  // = -1.0 -> angle is exactly 180 degrees
```

**The key insight:** The sign of the dot product tells you FRONT vs BACK. The magnitude (when using unit vectors) tells you exactly HOW MUCH in front or back.

### Game Uses

### Game Uses

**1. Is the target in front of or behind me?**

```rust
fn is_in_front_of(facing_direction: Vec2, target_position: Vec2) -> bool {
    facing_direction.dot(target_position.normalize()) > 0.0
}
```

**2. Is the target within my field of view?**

```rust
fn is_in_field_of_view(
    facing_direction: Vec2,
    direction_to_target: Vec2,
    half_fov_degrees: f32,
) -> bool {
    let cosine_of_half_fov = (half_fov_degrees.to_radians()).cos();
    // cos(theta) decreases as theta increases. So if our dot product
    // is GREATER than cos(half_fov), the angle is SMALLER than
    // half_fov -> we can see them!
    facing_direction.dot(direction_to_target.normalize()) > cosine_of_half_fov
}
```

**3. How much of this force is pushing in a specific direction?**

```rust
let force_vector = Vec2::new(10.0, 5.0);
let upward_normal = Vec2::new(0.0, 1.0);
let upward_force = force_vector.dot(upward_normal);  // = 5.0
// "5 units of the 10-unit force are pushing upward"
```

---

## 🔄 The 2D Cross Product: Left or Right?

The 2D cross product (also called "perp dot") tells you which SIDE one vector is on relative to another.

**The formula:**

```
a × b = a.x × b.y - a.y × b.x
```

**What the result means:**

| Result | Meaning |
|--------|---------|
| **Positive** | b is to the LEFT of a |
| **Negative** | b is to the RIGHT of a |
| **Zero** | a and b are parallel (same or opposite direction) |

**In code:**

```rust
let rightward = Vec2::new(1.0, 0.0);

// Up is to the LEFT of rightward
rightward.perp_dot(Vec2::new(0.0, 1.0));   // = 1.0

// Down is to the RIGHT of rightward
rightward.perp_dot(Vec2::new(0.0, -1.0));  // = -1.0

// Parallel: same direction
rightward.perp_dot(Vec2::new(1.0, 0.0));   // = 0.0
```

**Game use: Which way should I turn?**

The cross product tells AI which direction to rotate to face a target:

```rust
fn turn_direction(facing: Vec2, target_direction: Vec2) -> f32 {
    let cross = facing.perp_dot(target_direction);
    if cross > 0.0 { 1.0 }       // Turn LEFT
    else if cross < 0.0 { -1.0 } // Turn RIGHT
    else { 0.0 }                 // Already facing target
}
```

---

## 🎯 Chapter Summary

```
VECTORS ARE THE LANGUAGE OF SPACE:

  Addition:     pos += vel × dt         <- Motion (the ONE equation)
  Subtraction:  target - origin          <- Finding what's between points
  Scalar ×:     direction × speed        <- Speed control
  Magnitude:    ‖v‖ = √(x² + y²)         <- Distance
  Normalize:    v / ‖v‖                  <- Pure direction (length = 1)
  Dot:          a · b = cos(θ)           <- Front/behind/sideways
  Perp Dot:     a × b = left/right test  <- Which way to turn

  THE KEY INSIGHT:
  Position and direction are the SAME type (Vec2).
  You can add a direction to a position to get a new position.
  You can subtract two positions to get the direction between them.
  You can multiply a direction by speed to get velocity.
  
  ALL OF GAME PHYSICS flows from these few operations.
```

> **If vectors don't click for you, nothing else in this book will. Take the time to play with them. Write a small program that spawns two objects and makes one chase the other. Use `distance()`, `normalize()`, and `dot()`. Watch them work. Once vectors make intuitive sense, everything else  -  matrices, quaternions, kinematics, collisions  -  is just building on the same foundation.** 🧮

---

> 💡 **Full source code for this chapter:** [code-examples/ch03-vectors/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch03-vectors)

The runnable project includes Cargo.toml, main.rs, and complete module files.

---

**[<- Previous: Setup](ch02-setup.md)** | **[Next: Matrices ->](ch04-matrices.md)**
