# 🌀 Quaternions: Rotations Without Gimbal Lock

> **"Quaternions are the dark magic of 3D rotation  -  they look like 4D gibberish but rotate things with divine elegance. No gimbal lock, no singularities, just perfect spherical interpolation."** 🧙‍♂️

---

## 🎯 The Problem: Why Euler Angles Fail

**Euler angles** (pitch, yaw, roll) are the intuitive way to think about rotation. You've seen them in every 3D application: "rotate 30° around X, then 45° around Y, then 90° around Z."

But they have a **fundamental, fatal flaw**:

### Gimbal Lock Explained

```
Imagine three nested rings, each controlling one axis of rotation:

         YAW (Y axis)          PITCH (X axis)         ROLL (Z axis)
           |                       |                       |
         +-+-+                   +-+-+                   +-+-+
         |   |                   |   |                   |   |
    -----+   +-----         -----+   +-----         -----+   +-----
         |   |                   |   |                   |   |
         +-----+                   +-----+                   +-----+

When you pitch the plane UP by 90°, the yaw and roll rings ALIGN.
Now yaw and roll do the SAME thing  -  you've lost a degree of freedom!

  Before (three independent axes):   After (two axes aligned):
    YAW -->  horizontal rotation       YAW -->  same as roll! ❌
    PITCH --> vertical rotation         PITCH --> vertical rotation
    ROLL -->  bank/tilt                 ROLL -->  same as yaw! ❌
    
  Result: jerky, unpredictable rotation. Try pitching 90° in any
  3D program and then yawing  -  the rotation "breaks."
```

### The Mathematical Root Cause

Euler angles represent rotation as THREE SEPARATE 2D rotations applied in sequence. Each rotation is relative to the PREVIOUS coordinate system, not a fixed global one. When one rotation hits 90°, the next two axes become parallel, and the representation collapses.

**Quaternions solve this by representing rotation as a SINGLE 4D object  -  no axes, no sequencing, no singularities.**

---

## 🧮 What IS a Quaternion?

A quaternion has four components: `w`, `x`, `y`, `z`.

```
Quaternion = w + x·i + y·j + z·k

Where i, j, k are "imaginary units" satisfying:
  i² = j² = k² = i·j·k = -1
```

But you don't need to understand the complex algebra. What matters is the **geometric meaning**:

```
A quaternion represents: "Rotate by θ degrees around axis A"

  θ = angle of rotation
  A = (ax, ay, az) = unit vector axis of rotation

  Quaternion components:
    w = cos(θ/2)          <- "amount" of rotation
    x = ax × sin(θ/2)     <- X component of rotation axis
    y = ay × sin(θ/2)     <- Y component of rotation axis
    z = az × sin(θ/2)     <- Z component of rotation axis

EXAMPLE: Rotate 90° around the Z axis:
  θ = 90° = π/2
  A = (0, 0, 1)  <- Z axis

  w = cos(45°) ≈ 0.707
  x = 0 × sin(45°) = 0
  y = 0 × sin(45°) = 0
  z = 1 × sin(45°) ≈ 0.707

  Result: Quat(w=0.707, x=0, y=0, z=0.707)
  This represents: "rotate 90° around the Z axis"
```

### Why θ/2? The Double Cover Property

The "half angle" is what makes quaternion composition work:

```
If Q₁ = "rotate θ₁ around A₁" and Q₂ = "rotate θ₂ around A₂"

Then Q₂ × Q₁ = "rotate by θ₁, THEN rotate by θ₂"

The angles add because of the half-angle in the definition:
  cos(θ₁/2) × cos(θ₂/2) - sin(θ₁/2) × sin(θ₂/2) × (A₁ · A₂) = cos((θ₁+θ₂)/2)

Additionally, q and -q represent the SAME rotation (double cover).
This is why SLERP can always find the SHORTEST path between two rotations.
```

```rust
use bevy::prelude::*;

/// Creating a quaternion from axis-angle is the most intuitive method.
/// You say: "I want to rotate `angle` radians around this `axis`."
pub fn create_rotation_from_axis_and_angle() -> Quat {
    let rotation_axis = Vec3::new(0.0, 1.0, 0.0); // Y axis (yaw)
    let rotation_angle = std::f32::consts::FRAC_PI_4; // 45°
    
    Quat::from_axis_angle(rotation_axis, rotation_angle)
}

/// Alternatively, use the shorthand for axis-specific rotations:
let pitch_quaternion = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4); // Nod "yes"
let yaw_quaternion = Quat::from_rotation_y(std::f32::consts::FRAC_PI_4);   // Shake "no"
let roll_quaternion = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);  // Tilt head
```

---

## 🎯 Creating Quaternions in Bevy: Four Methods

```rust
use bevy::prelude::*;

/// --- Method 1: From an axis and angle (MOST DIRECT) ---
///
/// Specify exactly what axis to spin around and how much.
/// The axis MUST be a unit vector (length = 1).
pub fn method_axis_angle() -> Quat {
    // Rotate 45° around the world Y axis (yaw left)
    Quat::from_axis_angle(Vec3::Y, std::f32::consts::FRAC_PI_4)
}

/// --- Method 2: From a specific axis (CONVENIENT) ---
pub fn method_axis_shorthand() -> Quat {
    // These are equivalent to from_axis_angle with the corresponding axis:
    let pitch = Quat::from_rotation_x(0.5);  // Nod up by 0.5 radians
    let yaw = Quat::from_rotation_y(0.3);    // Turn right by 0.3 radians
    let roll = Quat::from_rotation_z(1.0);   // Roll by 1 radian (~57°)
    
    // Compose them: roll × yaw × pitch
    // (read right-to-left: pitch first, then yaw, then roll)
    roll * yaw * pitch
}

/// --- Method 3: From Euler angles (CONVENIENT BUT DANGEROUS) ---
///
/// WARNING: Euler angles can cause gimbal lock!
/// Only use this for simple cases or when reading user input.
/// Convert to quaternion immediately and store as quaternion.
pub fn method_euler() -> Quat {
    Quat::from_euler(
        EulerRot::XYZ,                     // Convention: pitch, yaw, roll
        std::f32::consts::FRAC_PI_6,       // Pitch (X): 30°
        std::f32::consts::FRAC_PI_4,       // Yaw (Y): 45°
        0.0,                               // Roll (Z): 0°
    )
}

/// --- Method 4: Look rotation (FACE TOWARD A TARGET) ---
///
/// Creates a rotation that makes `forward` direction point toward
/// `target`. The `up` vector prevents rolling (defines which way is up).
pub fn method_look_rotation() -> Quat {
    let current_forward = Vec3::Z;           // Object's default forward
    let desired_forward = Vec3::new(1.0, 0.0, 1.0).normalize();
    
    // Creates the rotation that turns current_forward into desired_forward
    Quat::from_rotation_arc(current_forward, desired_forward)
}
```

---

## 🔄 Applying Quaternions to Vectors

```rust
/// Rotating a vector by a quaternion is a SINGLE multiplication.
pub fn apply_quaternion_to_vector() {
    let rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2); // 90° around Z
    let original_vector = Vec3::new(1.0, 0.0, 0.0); // Points along X axis
    let rotated_vector = rotation * original_vector;
    
    // rotated_vector ≈ (0, 1, 0)  -  the vector now points along Y!
    // The point (1,0,0) rotated 90° around Z -> (0,1,0)
    
    /// 💡 QUATERNION ON THE LEFT, VECTOR ON THE RIGHT:
    ///   rotated = quaternion × vector
    /// This is NOT commutative  -  reversed multiplication gives nonsense.
}

/// ⭐ GETTING AXIS DIRECTIONS AFTER ROTATION:
///
/// This is how you find "which way is the object facing?"
pub fn get_rotated_axes(object_rotation: Quat) -> (Vec3, Vec3, Vec3) {
    let forward = object_rotation * Vec3::NEG_Z; // Forward in Bevy = -Z
    let right = object_rotation * Vec3::X;        // Right = +X
    let up = object_rotation * Vec3::Y;           // Up = +Y
    
    (forward, right, up)
}

// These directions are now in WORLD space. If the object is rotated
// 45° to the right:
//   forward ≈ (sin45°, 0, -cos45°) ≈ (0.707, 0, -0.707)
//   right   ≈ (cos45°, 0, sin45°)  ≈ (0.707, 0, 0.707)
```

---

## 🔗 Composing Rotations: Quaternion Multiplication

Just like matrices, quaternions COMPOSE through multiplication:

```rust
/// Composing two rotations: apply `first_rotation`, then `second_rotation`.
///
/// CRITICAL: Quaternion multiplication is NOT commutative!
/// second_rotation × first_rotation ≠ first_rotation × second_rotation
///
/// Read right-to-left: the RIGHTMOST quaternion is applied FIRST.
pub fn compose_rotations() {
    let pitch_up = Quat::from_rotation_x(std::f32::consts::FRAC_PI_6);   // 30° pitch
    let yaw_left = Quat::from_rotation_y(std::f32::consts::FRAC_PI_4);   // 45° yaw
    
    // "Pitch up FIRST, THEN yaw left"
    // (right-to-left: pitch is rightmost -> applied first)
    let pitch_then_yaw = yaw_left * pitch_up;
    
    // "Yaw left FIRST, THEN pitch up"
    let yaw_then_pitch = pitch_up * yaw_left;
    
    // pitch_then_yaw ≠ yaw_then_pitch!
    // Imagine an airplane: pitching up THEN yawing left is DIFFERENT
    // from yawing left THEN pitching up. Both are valid orientations,
    // but they lead to different final positions.
}

/// --- Practical: flying an aircraft ---
///
/// In local rotation (multiply on the RIGHT), the rotation axes
/// rotate WITH the object. This is what you want for aircraft:
/// pitching up ALWAYS nods the nose, regardless of orientation.
pub fn local_vs_world_rotation() {
    let mut current_orientation = Quat::IDENTITY;
    
    // LOCAL rotation (right-multiply): axes move with the object
    let pitch_input = Quat::from_rotation_x(0.1);
    current_orientation = current_orientation * pitch_input; // Nose nods up
    // This works the same even if the plane is upside down!
    
    // WORLD rotation (left-multiply): axes are fixed in the world
    let yaw_input = Quat::from_rotation_y(0.1);
    current_orientation = yaw_input * current_orientation; // Turn around world Y
    // This turns around the ABSOLUTE up direction, not the plane's.
}
```

---

## 🧭 SLERP: The Killer Feature

**SLERP** (Spherical Linear Interpolation) is THE reason to use quaternions:

```rust
/// SLERP interpolates between two rotations along the SHORTEST PATH.
///
/// WITHOUT SLERP (interpolating Euler angles):
///   The rotation "wobbles"  -  angular velocity changes mid-path,
///   causing a jerky, unnatural feel.
///
/// WITH SLERP (interpolating quaternions):
///   CONSTANT angular velocity  -  the object rotates at exactly
///   the same speed throughout. Smooth as glass. 🪟
pub fn slerp_between_orientations(
    start_rotation: Quat,
    end_rotation: Quat,
    interpolation_progress: f32, // 0.0 = start, 1.0 = end
) -> Quat {
    start_rotation.slerp(end_rotation, interpolation_progress)
}

/// --- Frame-rate-independent SLERP for smooth camera movement ---
pub fn smooth_slerp(
    current_rotation: Quat,
    target_rotation: Quat,
    smoothness: f32,    // Higher = faster (try 4.0-8.0)
    delta_seconds: f32, // Time.delta_secs()
) -> Quat {
    // The formula `1 - e^(-smoothness × dt)` ensures the interpolation
    // speed is consistent regardless of framerate. At 60 FPS, the
    // interpolation factor is small but frequent. At 30 FPS, it's
    // larger but less frequent. The RESULT is the same.
    let interpolation_factor = 1.0 - (-smoothness * delta_seconds).exp();
    
    current_rotation.slerp(target_rotation, interpolation_factor)
}

/// --- Complete: Smooth camera system ---
#[derive(Component)]
pub struct SmoothCamera {
    /// The rotation we want to reach.
    pub target_rotation: Quat,
    /// How quickly we catch up (higher = snappier, 4.0-8.0).
    pub smoothness: f32,
}

pub fn smooth_camera_system(
    time: Res<Time>,
    mut camera_query: Query<(&SmoothCamera, &mut Transform)>,
) {
    let delta_seconds = time.delta_secs();
    
    for (camera_state, mut transform) in camera_query.iter_mut() {
        // Frame-rate-independent SLERP
        let interpolation_factor =
            1.0 - (-camera_state.smoothness * delta_seconds).exp();
        
        transform.rotation = transform
            .rotation
            .slerp(camera_state.target_rotation, interpolation_factor);
    }
}
```

---

## 🚀 Complete Example: 3D Spaceship with Quaternion Rotation

```rust
#[derive(Component)]
pub struct SpaceShip3D {
    /// Maximum rotation speed around each local axis (radians/second).
    /// Typical values: pitch = 2.0, yaw = 3.0, roll = 4.0
    pub max_angular_velocity: Vec3,
}

pub fn spaceship_rotation_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ship_query: Query<(&SpaceShip3D, &mut Transform)>,
) {
    let delta_seconds = time.delta_secs();
    
    for (ship_data, mut transform) in ship_query.iter_mut() {
        // --- Step 1: Read input -> angular velocity for this frame ---
        let mut angular_velocity = Vec3::ZERO;
        
        // Pitch (rotate around LOCAL X axis  -  nod "yes")
        if keyboard_input.pressed(KeyCode::KeyW) {
            angular_velocity.x -= ship_data.max_angular_velocity.x;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            angular_velocity.x += ship_data.max_angular_velocity.x;
        }
        
        // Yaw (rotate around LOCAL Y axis  -  shake "no")
        if keyboard_input.pressed(KeyCode::KeyA) {
            angular_velocity.y -= ship_data.max_angular_velocity.y;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            angular_velocity.y += ship_data.max_angular_velocity.y;
        }
        
        // Roll (rotate around LOCAL Z axis  -  tilt head)
        if keyboard_input.pressed(KeyCode::KeyQ) {
            angular_velocity.z -= ship_data.max_angular_velocity.z;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            angular_velocity.z += ship_data.max_angular_velocity.z;
        }
        
        // --- Step 2: Convert angular velocity to delta quaternion ---
        // Each axis of rotation becomes a small quaternion.
        let delta_angle = angular_velocity * delta_seconds;
        
        let pitch_delta_quat = Quat::from_rotation_x(delta_angle.x);
        let yaw_delta_quat = Quat::from_rotation_y(delta_angle.y);
        let roll_delta_quat = Quat::from_rotation_z(delta_angle.z);
        
        // Combine: roll × yaw × pitch (standard aircraft convention)
        let total_delta_quat = roll_delta_quat * yaw_delta_quat * pitch_delta_quat;
        
        // --- Step 3: Apply delta to current rotation ---
        // Right-multiply = LOCAL rotation (axes move with ship)
        // Left-multiply = WORLD rotation (axes fixed)
        transform.rotation = transform.rotation * total_delta_quat;
        
        // ⭐ NO GIMBAL LOCK! The ship can face ANY direction.
        // Try flying straight up (pitch 90°) and then yawing.
        // With Euler angles: the yaw breaks (gimbal lock).
        // With quaternions: it works perfectly. 🎯
        
        // --- Step 4: Get forward direction for thrust ---
        let forward_direction = transform.rotation * Vec3::NEG_Z;
        // Use forward_direction × thruster_force to move the ship
    }
}
```

---

## 📊 Comparison: Euler vs Quaternion vs Matrix

| Feature | Euler Angles | Quaternions | Matrices |
|---------|-------------|-------------|----------|
| **Gimbal Lock** | ❌ Yes | ✅ No | ✅ No |
| **SLERP** | ❌ Wobbly | ✅ Smooth | ❌ No |
| **Memory** | 3 floats | 4 floats | 9 floats |
| **Composition** | ❌ Complex | ✅ Multiply | ✅ Multiply |
| **Batch transform** | ❌ N/A | ❌ N/A | ✅ Fast |
| **Human-readable** | ✅ Yes | ❌ No | ❌ No |
| **Bevy type** | `Vec3` | `Quat` | `Mat3`/`Mat4` |

---

## 🎯 Chapter Summary

```
USE QUATERNIONS FOR:
  ✓ Storing entity rotation (4 floats, no gimbal lock)
  ✓ Smooth camera animation (SLERP is magical)
  ✓ Combining rotations (fast composition)
  ✓ Avoiding gimbal lock (impossible to create)

USE MATRICES FOR:
  ✓ Transforming many points at once (batched rendering)
  ✓ Combining rotation WITH translation and scale

USE EULER ANGLES FOR:
  ✓ Displaying angles to the user
  ✓ Reading rotation from input
  -> Convert to quaternion immediately after!

Quaternion = (cos(θ/2), axis × sin(θ/2))
     ^            ^
     amount       axis of rotation
```

> **Quaternions are THE professional choice for 3D rotation. No gimbal lock, perfect SLERP, compact storage. The 4D math looks intimidating but in practice you just call `.slerp()` and `from_axis_angle()`  -  Bevy handles the dark magic. Your players will never experience gimbal lock again.** 🌀

> 💡 **Full source code for this chapter:** [code-examples/ch05-quaternions/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch05-quaternions)
> 
> The runnable project includes Cargo.toml, main.rs, and complete module files.

---

**[<- Previous: Matrices & Transformations](ch04-matrices.md)** | **[Next: Trigonometry ->](ch06-trigonometry.md)**
