# 🏃 Kinematics: The Geometry of Motion

> **"Kinematics asks 'where' and 'how fast' — without caring 'why.' The 'why' is Dynamics (next chapter)."** 🎯

---

## 📐 What Is Kinematics?

**Kinematics** describes motion using three quantities:

```
┌─────────────────────────────────────────────────────┐
│                  KINEMATIC TRIAD                      │
│                                                       │
│    📍 Position  (x)    — Where are you?              │
│    🏃 Velocity  (v)    — How fast & which way?       │
│    ⚡ Acceleration (a) — How is velocity changing?    │
│                                                       │
│    These three are connected by DERIVATIVES:          │
│                                                       │
│         d(position)                                   │
│    v =  ────────────  ← velocity is position's change │
│            dt                                         │
│                                                       │
│         d(velocity)                                   │
│    a =  ────────────  ← acceleration is velocity's    │
│            dt                  change                 │
│                                                       │
└─────────────────────────────────────────────────────┘
```

### 🔄 The Kinematic Chain

```
    Acceleration ──∫──► Velocity ──∫──► Position
    
    "Integrate once"    "Integrate twice"
    
    Also works in reverse:
    
    Position ──d/dt──► Velocity ──d/dt──► Acceleration
    
    "Differentiate"      "Differentiate"
```

---

## 📊 The Equations of Motion (SUVAT)

For **constant acceleration** (like gravity), we have five famous equations:

```rust
/// 📐 SUVAT equations — the bread and butter of kinematics
///
/// s = displacement (change in position)
/// u = initial velocity
/// v = final velocity
/// a = acceleration (constant)
/// t = time
struct Suvat;

impl Suvat {
    /// v = u + at
    /// Final velocity after constant acceleration
    fn v_from_u_at(u: f32, a: f32, t: f32) -> f32 {
        u + a * t
    }
    
    /// s = ut + ½at²
    /// Displacement after constant acceleration
    fn s_from_u_a_t(u: f32, a: f32, t: f32) -> f32 {
        u * t + 0.5 * a * t * t
    }
    
    /// v² = u² + 2as
    /// Final velocity from displacement (time-independent!)
    fn v_from_u_a_s(u: f32, a: f32, s: f32) -> f32 {
        (u * u + 2.0 * a * s).sqrt()
    }
}
```

```
📊 Visual: Constant Acceleration (e.g., Gravity)

    Position (parabolic)        Velocity (linear)       Acceleration (constant)
         │                         │                         │
        ╱╲                        ╱                         ───
       ╱  ╲                      ╱                          │
      ╱    ╲                    ╱                           │ a
     ╱      ╲                  ╱                            │
    ╱        ╲               ╱                              ───
   ╱          ╲             ╱
  ────────────────► t     ────────────────► t           ───────────► t
  
  s = ut + ½at²          v = u + at               a = constant
```

---

## 🎮 Kinematics in Game Physics

The **Euler Integration** we used earlier IS kinematics in action:

```rust
/// 🔄 The core physics step: kinematics update
fn kinematic_update(
    mut query: Query<(&mut Position, &mut Velocity, &Acceleration)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    
    for (mut pos, mut vel, acc) in query.iter_mut() {
        // ⚡ Step 1: Update velocity from acceleration
        // v_new = v_old + a × dt
        vel.0 += acc.0 * dt;
        
        // 📍 Step 2: Update position from velocity
        // x_new = x_old + v_new × dt
        // NOTE: We use v_new (semi-implicit Euler) instead of v_old
        // This is called "Symplectic Euler" — much more stable!
        pos.0 += vel.0 * dt;
    }
}

/// 💡 Semi-Implicit vs Explicit Euler:
///
/// Explicit (bad):   v_new = v_old + a·dt    (uses old v)
///                   x_new = x_old + v_old·dt 
///
/// Semi-Implicit (good, what we use):
///                   v_new = v_old + a·dt    (uses new v!)
///                   x_new = x_old + v_new·dt
///
/// The semi-implicit version conserves energy better!
/// Objects in orbit stay in orbit instead of flying away. 🛰️
```

---

## 🎯 Kinematic Trajectory Prediction

One of the most powerful uses of kinematics is **predicting the future**:

```rust
/// 🎯 Predict where a projectile will be in `t` seconds
///
/// This is CRITICAL for:
/// - AI aiming (lead targets!)
/// - Planning jumps
/// - Timing attacks
/// - Bullet collision optimization
fn predict_position(
    current_pos: Vec2,
    current_vel: Vec2,
    acceleration: Vec2,
    t: f32,
) -> Vec2 {
    // s = ut + ½at²  (for each axis independently)
    current_pos + current_vel * t + 0.5 * acceleration * t * t
}

/// 🎯 AI: Where will the player be when my bullet reaches them?
fn ai_aim(
    ai_pos: Vec2,
    bullet_speed: f32,
    player_pos: Vec2,
    player_vel: Vec2,
    gravity: Vec2,
) -> Vec2 {
    // STEP 1: Estimate time for bullet to reach player
    let distance = ai_pos.distance(player_pos);
    let travel_time = distance / bullet_speed;  // First guess
    
    // STEP 2: Predict where player will be
    let predicted_pos = predict_position(
        player_pos,
        player_vel,
        gravity,  // Player is also affected by gravity
        travel_time,
    );
    
    // STEP 3: Refine with better distance estimate
    let refined_distance = ai_pos.distance(predicted_pos);
    let refined_time = refined_distance / bullet_speed;
    
    predict_position(
        player_pos,
        player_vel,
        gravity,
        refined_time,
    )
    // 📝 One iteration is usually enough for games!
    // For NASA accuracy: iterate 3-5 times
}

/// 💡 Without prediction, AI always shoots where the player WAS.
/// With prediction, AI shoots where the player WILL BE.
/// This is the difference between "dumb" and "scary" enemies! 😱
```

```
Without Prediction (Aiming at current position):

    Player ●───►  (moving right)
       ↑
    AI shoots here ❌  (misses!)

With Prediction (Leading the target):

    Player ●───►       ● (predicted)
       ↑              ↗
    AI shoots here ✅  (hits!)
```

---

## 🏗️ Kinematic Platform Behavior

```rust
/// 🎮 A moving platform that follows a path
#[derive(Component)]
struct MovingPlatform {
    /// Path waypoints
    points: Vec<Vec2>,
    /// Current target index
    target_idx: usize,
    /// Speed of movement (units/sec)
    speed: f32,
}

fn move_platforms(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MovingPlatform)>,
) {
    let dt = time.delta_secs();
    
    for (mut transform, mut platform) in query.iter_mut() {
        let current = Vec2::new(transform.translation.x, transform.translation.y);
        let target = platform.points[platform.target_idx];
        
        // 📐 Vector from current to target
        let to_target = target - current;
        let distance = to_target.length();
        
        if distance < 1.0 {
            // 🎯 Arrived! Move to next waypoint
            platform.target_idx = (platform.target_idx + 1) % platform.points.len();
            continue;
        }
        
        // 🏃 Move toward target at constant speed
        // direction × speed × dt = displacement this frame
        let direction = to_target / distance;  // Normalize
        let displacement = direction * platform.speed * dt;
        
        // ⛔ Don't overshoot
        let displacement = if displacement.length() > distance {
            to_target  // Just snap to target
        } else {
            displacement
        };
        
        transform.translation.x += displacement.x;
        transform.translation.y += displacement.y;
    }
}
```

---

## 🧮 The Relationship Between Kinematic Quantities

```
╔════════════════════════════════════════════════════════╗
║              KINEMATIC RELATIONSHIPS                   ║
╠════════════════════════════════════════════════════════╣
║                                                        ║
║  Position:       p(t)                                ║
║  Velocity:       v(t) = dp/dt                        ║
║  Acceleration:   a(t) = dv/dt = d²p/dt²              ║
║  Jerk:           j(t) = da/dt = d³p/dt³              ║
║  Snap:           s(t) = dj/dt = d⁴p/dt⁴              ║
║                                                        ║
║  Game physics: We integrate position & velocity        ║
║  Camera smoothing: We track jerk for smoothness!       ║
║                                                        ║
╚════════════════════════════════════════════════════════╝
```

```rust
/// 📷 Smooth camera with jerk-limited motion
/// This prevents "snapping" and gives cinematic feel
#[derive(Component)]
struct SmoothCameraFollow {
    /// Current velocity of the camera
    cam_vel: Vec3,
    /// Smoothing time constant
    smooth_time: f32,
    /// Max acceleration (limits jerk)
    max_accel: f32,
}

fn smooth_camera(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &mut SmoothCameraFollow), With<Camera>>,
) {
    let dt = time.delta_secs();
    let player = player_query.single();
    let (mut cam, mut follow) = camera_query.single_mut();
    
    // 🎯 Target position (player)
    let target = player.translation;
    
    // 🧮 Critically-damped spring smoothing
    let omega = 2.0 / follow.smooth_time;
    let x = cam.translation - target;
    let spring_force = -omega * omega * x;  // Hooke's law
    let damping = -2.0 * omega * follow.cam_vel;  // Critical damping
    
    // 📐 Acceleration from spring physics
    let accel = spring_force + damping;
    
    // ⛔ Clamp acceleration
    let accel = accel.clamp_length_max(follow.max_accel);
    
    // 🔄 Integrate (kinematics!)
    follow.cam_vel += accel * dt;
    cam.translation += follow.cam_vel * dt;
}
```

---

## 🎯 Chapter Summary

```rust
/// 📝 Kinematics cheat sheet

// The kinematic chain
// a → v → x (integrate forward)
// x → v → a (differentiate backward)

// 📐 Core equations (constant acceleration)
let v_final = v_initial + acceleration * dt;
let displacement = v_initial * t + 0.5 * acceleration * t * t;

// 🎯 Prediction (for AI, planning, physics)
let future_pos = pos + vel * t + 0.5 * accel * t * t;

// 🔄 Semi-implicit Euler (always use this!)
vel += accel * dt;
pos += vel * dt;  // Uses NEW velocity!

/// Key insight: Position, Velocity, and Acceleration are
/// all VECTORS. In 2D, each has x and y components that
/// evolve independently. The equations apply to each axis!
```

> **Key Takeaway:** Kinematics is the grammar of motion — position, velocity, and acceleration form a beautiful chain of derivatives and integrals. Master this chain, and you can describe ANY motion in the game world. 🏆

---

**[← Previous: Trigonometry](ch06-trigonometry.md)** | **[Next: Dynamics →](ch08-dynamics.md)**
