# 🌀 Quaternions: Rotations Without Gimbal Lock

> **"Quaternions are the dark magic of game physics — they look like 4D gibberish but rotate things with divine elegance."** 🧙‍♂️

---

## 🤔 The Problem with Euler Angles

**Euler angles** (pitch, yaw, roll) are intuitive but have a fatal flaw:

```
                 Gimbal Lock! ❌
    ┌─────────────────────────────────────┐
    │                                     │
    │  When two axes align, you lose      │
    │  a degree of freedom!               │
    │                                     │
    │  Try pitching 90° up:               │
    │  Now yaw and roll are the SAME!     │
    │                                     │
    │  Result: jerky, unpredictable       │
    │  camera rotation. 🤢                │
    │                                     │
    └─────────────────────────────────────┘
```

**Quaternions solve this** by representing rotation as a **4D unit sphere** — no singularities, no gimbal lock, smooth interpolation.

---

## 🧮 What IS a Quaternion?

A quaternion has 4 components: `w, x, y, z`

```rust
use bevy::prelude::*;

/// 🌀 A quaternion in Bevy
// q = w + xi + yj + zk
let q = Quat::from_rotation_z(FRAC_PI_4);  // 45° around Z

// 📐 Breaking it down:
// w = cos(θ/2)    → "real" part (amount of rotation)
// x = axis_x * sin(θ/2)  → rotation axis component
// y = axis_y * sin(θ/2)  → rotation axis component
// z = axis_z * sin(θ/2)  → rotation axis component
//
// For 2D rotation (around Z):
// Let axis = (0, 0, 1), θ = 90°
// w = cos(45°) ≈ 0.707
// z = sin(45°) ≈ 0.707
// x = 0, y = 0
```

```
Visualizing a Quaternion:

    Think of it as: "Rotate by θ around axis A"
    
        ↑ axis
        │
        │    ╱
        │   ╱  ← rotation
        │  ╱
        │ ╱ θ
        └──────────→
        
    Quaternion = (cos(θ/2), axis·sin(θ/2))
    
    Why θ/2? Because quaternions use double-cover
    representation (q and -q represent the SAME rotation).
    This is what makes SLERP possible!
```

---

## 🎯 Creating Quaternions in Bevy

```rust
use bevy::prelude::*;

// ─── From an axis and angle ───
/// Rotate 90° around the X axis
let q1 = Quat::from_axis_angle(Vec3::X, FRAC_PI_2);

/// Rotate 45° around the Y axis
let q2 = Quat::from_axis_angle(Vec3::Y, FRAC_PI_4);

// ─── From Euler angles ───
/// (yaw, pitch, roll) = (Z, X, Y) in Bevy's convention
let q3 = Quat::from_euler(EulerRot::XYZ, 0.0, 0.5, 0.0);

// ─── From rotation around a specific axis ───
let q_z = Quat::from_rotation_z(1.0);  // Roll
let q_x = Quat::from_rotation_x(0.5);  // Pitch
let q_y = Quat::from_rotation_y(0.3);  // Yaw

// ─── Look rotation (face toward something!) ───
let forward = Vec3::new(1.0, 0.0, 0.0);
let up = Vec3::Y;
let q_look = Quat::from_rotation_arc(forward, up);
```

---

## 🔄 Applying Quaternions to Vectors

```rust
/// 🎯 Rotate a 3D vector using a quaternion
let v = Vec3::new(1.0, 0.0, 0.0);
let rotation = Quat::from_rotation_z(FRAC_PI_2); // Rotate 90° around Z

let rotated = rotation * v;
// rotated ≈ (0, 1, 0) — point went from X axis to Y axis!

/// 💡 Quaternion * Vector ALWAYS works this way.
/// The quaternion goes on the LEFT, vector on the RIGHT.
```

---

## 🔗 Multiplying Quaternions (Composition)

Just like matrices, you **multiply** quaternions to combine rotations:

```rust
// ─── Combine rotations ───
let yaw = Quat::from_rotation_y(0.5);     // Turn left
let pitch = Quat::from_rotation_x(0.3);   // Look up

// 🔗 Apply yaw, THEN pitch (read right-to-left!)
// Result = yaw * pitch
let combined = yaw * pitch;

// 🔄 "Apply rotation a, then rotation b"
let total_rotation = b * a;  // First a, then b

// 💡 Very different from a * b!
// Quaternion multiplication is NOT commutative
```

---

## 🧭 SLERP: Smooth Interpolation

**SLERP** (Spherical Linear Interpolation) is the killer feature of quaternions. It gives smooth, constant-speed rotation between two orientations:

```rust
/// 🎬 SLERP between two rotations over time
fn slerp_demo() {
    let start = Quat::IDENTITY;  // No rotation
    let end = Quat::from_rotation_z(FRAC_PI_2);  // 90° rotation
    
    // t goes from 0 to 1 over time
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let interpolated = start.slerp(end, t);
        // At t=0: start rotation
        // At t=0.5: 45° rotation (halfway)
        // At t=1: end rotation
    }
}
```

```
SLERP Visualization:

  start ───── . ───── . ───── . ──── end
   (0%)   (25%)  (50%)  (75%)   (100%)
   
   ════════════════════════════════════
   Constant angular velocity!
   No speed changes, no weird wobbles!
   
   Compare with Lerp on Euler angles:
   
   start -/--/-/-/------\--\-\--- end
          ╰── jerky, uneven ──╯
```

### 🎮 Practical SLERP: Camera Rotation

```rust
/// 📷 Smooth camera rotation toward a target
#[derive(Component)]
struct SmoothCamera {
    /// Current target rotation
    target_rotation: Quat,
    /// How fast to interpolate (0 = no lerp, 1 = instant)
    lerp_speed: f32,
}

fn smooth_camera_rotation(
    time: Res<Time>,
    mut query: Query<(&mut SmoothCamera, &mut Transform)>,
) {
    let dt = time.delta_secs();
    
    for (mut cam, mut transform) in query.iter_mut() {
        // 🧮 SLERP toward target
        // The factor (1 - e^(-speed * dt)) gives frame-rate
        // independent interpolation
        let t = 1.0 - (-cam.lerp_speed * dt).exp();
        transform.rotation = transform.rotation.slerp(cam.target_rotation, t);
    }
}
```

---

## 🔄 Converting Between Euler and Quaternion

```rust
/// 💡 Euler ↔ Quaternion conversion

// Euler → Quaternion
let euler = Vec3::new(0.5, 0.3, 0.0);  // (pitch, yaw, roll)
let quat = Quat::from_euler(EulerRot::XYZ, euler.x, euler.y, euler.z);

// Quaternion → Euler
let (pitch, yaw, roll) = quat.to_euler(EulerRot::XYZ);
println!("Pitch: {:.2}°, Yaw: {:.2}°, Roll: {:.2}°", 
    pitch.to_degrees(), yaw.to_degrees(), roll.to_degrees());
```

---

## 🎯 Practical: 3D Spaceship with Quaternion Rotation

```rust
/// 🚀 3D spaceship that can face any direction without gimbal lock

#[derive(Component)]
struct Ship3D {
    /// Angular velocity (rotation speed around each axis)
    angular_velocity: Vec3,
    /// Max angular speed per axis
    max_angular_speed: f32,
}

fn spaceship_rotation_3d(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Ship3D, &mut Transform)>,
) {
    let dt = time.delta_secs();
    
    for (mut ship, mut transform) in query.iter_mut() {
        // ─── 1️⃣ Read input for angular velocity ───
        ship.angular_velocity = Vec3::ZERO;
        
        if keyboard.pressed(KeyCode::KeyW) {  // Pitch down
            ship.angular_velocity.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {  // Pitch up
            ship.angular_velocity.x += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {  // Yaw left
            ship.angular_velocity.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {  // Yaw right
            ship.angular_velocity.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyQ) {  // Roll left
            ship.angular_velocity.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyE) {  // Roll right
            ship.angular_velocity.z += 1.0;
        }
        
        ship.angular_velocity *= ship.max_angular_speed;
        
        // ─── 2️⃣ Convert angular velocity to quaternion delta ───
        // Each axis rotation becomes a small quaternion
        let delta_pitch = Quat::from_rotation_x(ship.angular_velocity.x * dt);
        let delta_yaw = Quat::from_rotation_y(ship.angular_velocity.y * dt);
        let delta_roll = Quat::from_rotation_z(ship.angular_velocity.z * dt);
        
        // ⚠️ Order matters! We apply pitch, then yaw, then roll
        // This is the standard aircraft rotation order
        let delta = delta_roll * delta_yaw * delta_pitch;
        
        // ─── 3️⃣ Apply delta to current rotation ───
        // Multiply on the right = rotate in LOCAL space
        // Multiply on the left = rotate in WORLD space
        transform.rotation = transform.rotation * delta;
        
        // ─── 4️⃣ Get forward vector for thrust ───
        let forward = transform.rotation * Vec3::NEG_Z;
        // 🚀 Now you can apply thrust in the forward direction!
    }
}

/// 💡 NO GIMBAL LOCK! Try pitching 90° and then yawing.
/// With Euler angles: the axes align and you lose control.
/// With quaternions: it just works! Smooth as butter. 🧈
```

---

## 📊 Quaternion vs Euler Comparison

| Feature | Euler Angles | Quaternions |
|---------|-------------|-------------|
| Intuitiveness | ✅ Intuitive | ❌ Abstract |
| Gimbal Lock | ❌ Can fail | ✅ No singularities |
| Memory | 3 floats | 4 floats |
| Interpolation | ❌ Jerky | ✅ Smooth SLERP |
| Composition | ❌ Complex | ✅ Simple multiply |
| Bevy Type | `Vec3` | `Quat` |

---

## 🎯 Chapter Summary

```rust
/// 📝 Everything you need to remember about quaternions:

// 1️⃣ Creation
let q = Quat::from_rotation_z(angle);
let q = Quat::from_axis_angle(axis, angle);
let q = Quat::from_euler(EulerRot::XYZ, pitch, yaw, roll);

// 2️⃣ Application to vectors
let rotated = q * Vec3::new(1.0, 0.0, 0.0);

// 3️⃣ Composition (combine rotations)
let combined = q2 * q1;  // First q1, then q2

// 4️⃣ Smooth interpolation
let result = start.slerp(end, t);

// 5️⃣ Getting axes
let forward = rotation * Vec3::NEG_Z;
let right = rotation * Vec3::X;
let up = rotation * Vec3::Y;
```

> **Key Takeaway:** Quaternions are the professional's choice for 3D rotation. They eliminate gimbal lock, enable smooth interpolation, and compose elegantly. Embrace the 4D — your rotations will thank you! 🌀

---

**[← Previous: Matrices & Transformations](04-matrices.md)** | **[Next: Trigonometry →](06-trigonometry.md)**
