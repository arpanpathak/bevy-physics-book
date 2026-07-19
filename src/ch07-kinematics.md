# 🏃 Kinematics: The Geometry of Motion

> **"Kinematics is the grammar of motion. It tells you WHERE something is, HOW FAST it's moving, and HOW that speed is changing — without asking WHY. The 'why' comes next chapter (Dynamics). For now, we just describe."** 🎯

---

## 🎯 The Problem Kinematics Solves

You have a game object. It's at position (100, 200). It's moving. Where will it be in 1 second? In 5 seconds? How fast will it be going? These are **kinematic** questions.

Kinematics gives you the tools to answer them with a simple, elegant framework:

```
┌──────────────────────────────────────────────────────────┐
│                THE KINEMATIC TRIAD                        │
│                                                          │
│    📍 POSITION    (x)    — "Where are you?"              │
│    🏃 VELOCITY    (v)    — "How fast? Which way?"        │
│    ⚡ ACCELERATION (a)   — "How is velocity changing?"    │
│                                                          │
│    These three are LINKED by calculus:                   │
│                                                          │
│         d(position)            d(velocity)               │
│    v = ────────────       a = ────────────               │
│            dt                      dt                    │
│                                                          │
│    "Velocity is the rate of change of position"          │
│    "Acceleration is the rate of change of velocity"      │
└──────────────────────────────────────────────────────────┘
```

---

## 🔄 The Chain: How They Connect

The three kinematic quantities form an **integration chain**:

```
    Acceleration ──∫──► Velocity ──∫──► Position
    
    "Integrate once"       "Integrate twice"
    
    Going backward (differentiation):
    
    Position ──d/dt──► Velocity ──d/dt──► Acceleration
    
    "The slope of position is velocity"
    "The slope of velocity is acceleration"
```

### What This Actually MEANS in a Game

Every frame, your physics engine does exactly this:

```rust
// THIS IS THE CORE OF ALL GAME PHYSICS:
fn kinematic_step(pos: &mut Vec2, vel: &mut Vec2, acc: &Vec2, dt: f32) {
    *vel += *acc * dt;  // Acceleration changes velocity
    *pos += *vel * dt;  // Velocity changes position
}
```

Let's understand what these two lines mean, **really** mean:

#### Line 1: `vel += acc * dt` (Acceleration → Velocity)

```
acc * dt = "acceleration applied for dt seconds"

If acceleration = -500 px/s² (gravity) and dt = 1/60 s:
  acc × dt = -500 × 0.01667 = -8.33 px/s

This means: "In this 1/60th of a second, gravity changed
your velocity by 8.33 px/s downward."

ANALOGY: If your car accelerates at 10 mph/s, and you
hold the gas for 0.5 seconds, your speed increases by
10 × 0.5 = 5 mph. That's EXACTLY what this line does.
```

#### Line 2: `pos += vel * dt` (Velocity → Position)

```
vel * dt = "velocity sustained for dt seconds"

If velocity = 50 px/s and dt = 1/60 s:
  vel × dt = 50 × 0.01667 = 0.833 px

This means: "In this 1/60th of a second, you moved 0.833 pixels."

ANALOGY: If you drive at 60 mph for 0.5 hours, you travel
60 × 0.5 = 30 miles. SAME operation, different units.
```

### The Full Picture: 60 Frames of Free Fall

Let's trace an object falling from rest under gravity:

```
Initial: pos.y = 300, vel.y = 0, gravity = -500 px/s²

Frame    vel.y          pos.y          What happens
──────────────────────────────────────────────────────
0        0.00           300.00         Start: at rest
1        -8.33          299.86         Starts falling
2        -16.67         299.58         Speeding up
3        -25.00         299.17         Faster still
5        -41.67         297.92         ...
10       -83.33         290.28         
30       -250.00        193.06         Halfway down
60       -500.00        46.94          Near ground
↓        ↓              ↓
    Each frame:        Each frame:
    vel += -500 × dt   pos += vel × dt
    vel drops by 8.33  pos drops by vel × dt
                       (which increases each frame!)
```

Notice: velocity **accumulates** (it keeps getting more negative), while position **accelerates** downward (it drops further each frame). That's the hallmark of constant acceleration — the velocity graph is a straight line, and the position graph is a parabola.

---

## 📐 The SUVAT Equations

For **constant acceleration** (which covers gravity, frictionless motion, and many game scenarios), we have exact formulas:

```rust
/// 📐 SUVAT: the five equations of constant-acceleration motion
///
/// s = displacement (Δposition)
/// u = initial velocity  
/// v = final velocity
/// a = constant acceleration
/// t = time

fn suvat_examples() {
    // Example: Jumping
    // A player jumps upward at 10 m/s² with gravity -9.81 m/s²
    
    let u = 10.0;   // Initial jump velocity (upward)
    let a = -9.81;  // Gravity (downward)
    
    // After 0.5 seconds, how fast are they going?
    let t = 0.5;
    let v = u + a * t;  // v = 10 + (-9.81 × 0.5) = 5.095 m/s
    
    // How high did they get at t=0.5s?
    let s = u * t + 0.5 * a * t * t;  // s = 10×0.5 + 0.5×(-9.81)×0.25 = 3.77m
    
    // How high will they go total? (At peak, v = 0)
    // v² = u² + 2as → s = (v² - u²) / 2a
    let max_height = (0.0 - u * u) / (2.0 * a);  // = 5.1m
    
    // How long until they hit peak?
    let time_to_peak = -u / a;  // = 1.02 seconds
    
    // Total time in air? (peak × 2 = 2.04 seconds)
    let total_time = 2.0 * time_to_peak;
}
```

### Visualizing the Jump

```
    height (m)                 velocity (m/s)
      5 │    ╱╲                   10 │    ╱
      4 │   ╱  ╲                  5 │   ╱
      3 │  ╱    ╲                 0 │  ╱──────► t
      2 │ ╱      ╲        ← peak   -5 │ ╲
      1 │╱        ╲               -10 │  ╲
        └───────────► t                └─────────► t
        0   1   2                     0   1   2
        
    Parabolic position!         Linear velocity!
    s = ut + ½at²               v = u + at
    
    The position peaks when velocity hits zero.
    Velocity is the SLOPE of position — when the
    parabola flattens at the top, velocity is zero.
```

---

## 🔮 Trajectory Prediction: The Killer Feature

Kinematics lets you **predict the future** with perfect accuracy (assuming constant acceleration):

```rust
/// 🎯 Predict where a projectile will be in `t` seconds
///
/// This is the SINGLE MOST POWERFUL use of kinematics in games.
fn predict_position(
    current_pos: Vec2,
    current_vel: Vec2,
    acceleration: Vec2,  // Usually gravity
    t: f32,              // How far into the future
) -> Vec2 {
    // s = ut + ½at² — the full kinematic equation, in 2D!
    // X and Y are independent:
    //   x(t) = x₀ + vx·t + ½·ax·t²
    //   y(t) = y₀ + vy·t + ½·ay·t²
    current_pos + current_vel * t + 0.5 * acceleration * t * t
}

/// 🎯 AI: Lead the target (shoot where they WILL be)
fn aim_at_future(
    ai_pos: Vec2,
    bullet_speed: f32,
    target_pos: Vec2,
    target_vel: Vec2,
    gravity: Vec2,
) -> Vec2 {
    // Step 1: Estimate flight time (rough)
    let distance = ai_pos.distance(target_pos);
    let flight_time = distance / bullet_speed;
    
    // Step 2: Predict where target will be
    let predicted = predict_position(target_pos, target_vel, gravity, flight_time);
    
    // Step 3: Refine (one iteration is usually enough for games)
    let refined_dist = ai_pos.distance(predicted);
    let refined_time = refined_dist / bullet_speed;
    
    predict_position(target_pos, target_vel, gravity, refined_time)
}
```

```
WITHOUT prediction:               WITH prediction:
                                  
    Player ●───►                    Player ●───►       ● (future)
       ↑                                ↑            ↗
    AI shoots here ❌                 AI aims here ✅
    (always misses!)                  (leads target!)
```

---

## 📈 Higher-Order Kinematics: Jerk, Snap, etc.

The kinematic chain DOESN'T stop at acceleration:

```
     Quantity  |  Name     |  What it describes
    ───────────┼───────────┼────────────────────────
    x(t)       │  Position  │  Where it is
    v(t)       │  Velocity  │  How position changes
    a(t)       │  Accel.   │  How velocity changes
    j(t)       │  Jerk     │  How acceleration changes
    s(t)       │  Snap     │  How jerk changes
```

Most games stop at acceleration, but **jerk-limited camera smoothing** is a game-changer:

```rust
/// 📷 Jerk-limited camera — silky smooth, no snapping
#[derive(Component)]
struct SmoothCamera {
    cam_vel: Vec3,      // Current velocity of camera
    cam_accel: Vec3,    // Current acceleration of camera  
    smooth_time: f32,   // How responsive (lower = snappier)
    max_jerk: f32,      // Max jerk (limits how fast accel can change)
}

fn smooth_camera(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    mut cam: Query<(&mut Transform, &mut SmoothCamera)>,
) {
    let dt = time.delta_secs();
    let target = player.single().translation;
    let (mut tf, mut cam) = cam.single_mut();
    
    // Spring force toward target
    let omega = 2.0 / cam.smooth_time;
    let diff = tf.translation - target;
    let spring_accel = -omega * omega * diff - 2.0 * omega * cam.cam_vel;
    
    // ⛔ Limit JERK (change in acceleration)
    let desired_jerk = spring_accel - cam.cam_accel;
    let clamped_jerk = desired_jerk.clamp_length_max(cam.max_jerk * dt);
    cam.cam_accel += clamped_jerk;
    
    // Integrate: accel → vel → pos
    cam.cam_vel += cam.cam_accel * dt;
    tf.translation += cam.cam_vel * dt;
}
```

Without jerk limiting, the camera snaps instantly when the player changes direction. With jerk limiting, it glides — giving a **cinematic feel**.

---

## 🎯 The Complete Picture: Projectile with Air Resistance

```rust
/// Realistic projectile simulation using kinematics:
fn simulate_projectile(
    pos: Vec2,      // Starting position
    vel: Vec2,      // Initial velocity (direction × speed)
    gravity: Vec2,  // Gravitational acceleration
    drag: f32,      // Drag coefficient
    dt: f32,        // Timestep
    steps: u32,     // Number of steps to simulate
) -> Vec<Vec2> {
    let mut positions = Vec::with_capacity(steps as usize);
    let mut p = pos;
    let mut v = vel;
    
    for _ in 0..steps {
        positions.push(p);
        
        // Acceleration = gravity + drag (opposing velocity)
        let accel = gravity - v * drag;
        
        // Integrate (symplectic Euler)
        v += accel * dt;
        p += v * dt;
    }
    
    positions
}

/// You can use this to:
/// - Preview a grenade arc before throwing
/// - Compute if a jump is reachable
/// - Calculate bullet drop over distance
/// - Visualize paths for trajectory-based puzzles
```

---

## 🎯 Chapter Summary

```
KINEMATICS IS THE LANGUAGE OF MOTION:

    ┌────────────────────────────────────────────────┐
    │  a(t) ──∫──► v(t) ──∫──► x(t)                │
    │         integrate     integrate                │
    │                                                │
    │  EVERY FRAME:                                 │
    │    vel += acc × dt    (acceleration → velocity)│
    │    pos += vel × dt    (velocity → position)    │
    │                                                │
    │  THIS IS ALL OF GAME PHYSICS                   │
    │  Everything else is just figuring out what     │
    │  acceleration should be.                       │
    └────────────────────────────────────────────────┘
    
    KEY EQUATIONS (constant acceleration):
    v = u + at                     ← Final velocity
    s = ut + ½at²                  ← Displacement
    v² = u² + 2as                  ← No-time-needed version
    pos + vel × t + ½ × acc × t²  ← Future position
    
    THE INSIGHT: Position, velocity, and acceleration
    are NOT separate things. They're the SAME thing
    at different levels of differentiation. Every
    frame, you're doing calculus — one addition at a time.
```

> **Master kinematics and you've mastered 90% of what a game physics engine does. All the complexity is in figuring out acceleration (forces, collisions, constraints). The motion itself is just `vel += acc × dt; pos += vel × dt`. Period.** 🏃

---

**[← Previous: Trigonometry](ch06-trigonometry.md)** | **[Next: Dynamics →](ch08-dynamics.md)**
