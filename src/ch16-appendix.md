# 📚 Appendix: Rust Patterns & References

> **"Every great physicist knows their tools. Here are the tools of the Rust game physics trade."** 🛠️

---

## 📋 Quick Reference Cards

### 🔢 Vector Operations Reference

```rust
use bevy::prelude::*;

// ─── CREATION ───
Vec2::new(x, y)                    // Explicit
Vec2::ZERO                         // (0, 0)
Vec2::ONE                          // (1, 1)
Vec2::X                            // (1, 0)
Vec2::Y                            // (0, 1)
Vec2::splat(v)                     // (v, v)  -  all components same

// ─── BASIC OPERATIONS ───
a + b                              // Component-wise addition
a - b                              // Component-wise subtraction
a * s                              // Scalar multiplication
a / s                              // Scalar division
-a                                 // Negation

// ─── PROPERTIES ───
a.length()                         // Magnitude: √(x² + y²)
a.length_squared()                 // Squared magnitude: x² + y²
a.normalize()                      // Unit vector (length = 1)
a.normalize_or_zero()              // Safe normalize (returns ZERO for zero vectors)

// ─── PRODUCTS ───
a.dot(b)                           // Dot product: a.x·b.x + a.y·b.y
a.perp_dot(b)                      // 2D cross: a.x·b.y - a.y·b.x
a.perp()                           // Perpendicular: (-a.y, a.x)

// ─── DISTANCE ───
a.distance(b)                      // Euclidean distance
a.distance_squared(b)              // Squared distance (faster!)
a.angle_between(b)                 // Angle between vectors (radians)

// ─── INTERPOLATION ───
a.lerp(b, t)                       // Linear interpolation: a + (b-a)·t
```

### 🔄 Matrix Operations Reference

```rust
use bevy::prelude::*;

// ─── 2D TRANSFORM MATRICES (Mat3) ───
Mat3::IDENTITY                     // No transformation
Mat3::from_translation(Vec2::new(tx, ty))  // Translation
Mat3::from_angle(θ)                // Rotation by θ radians
Mat3::from_scale(Vec2::new(sx, sy)) // Scaling

// ─── APPLICATION ───
m.transform_point2(p)              // Transform a point (includes translation)
m.transform_vector2(v)             // Transform a direction (no translation)

// ─── DECOMPOSITION ───
m.to_scale_angle_translation()     // Extract (scale, angle, translation)

// ─── COMPOSITION ───
t * r * s                          // Apply S, then R, then T
```

### 🌀 Quaternion Operations Reference

```rust
use bevy::prelude::*;

// ─── CREATION ───
Quat::IDENTITY                     // No rotation
Quat::from_rotation_x(θ)           // Rotation around X axis
Quat::from_rotation_y(θ)           // Rotation around Y axis
Quat::from_rotation_z(θ)           // Rotation around Z axis
Quat::from_axis_angle(axis, θ)     // Rotation around arbitrary axis
Quat::from_euler(EulerRot::XYZ, pitch, yaw, roll)  // From Euler angles
Quat::from_rotation_arc(from, to)  // Rotation that turns `from` into `to`

// ─── APPLICATION ───
q * Vec3::new(x, y, z)             // Rotate a vector

// ─── COMBINATION ───
b * a                              // Apply a, then b

// ─── INTERPOLATION ───
a.slerp(b, t)                      // Spherical linear interpolation

// ─── CONVERSION ───
q.to_euler(EulerRot::XYZ)          // To Euler angles (pitch, yaw, roll)
```

### 📐 Physics Constants

```rust
/// 🌍 Standard gravity values (pixels/s²  -  game scale!)
pub const GRAVITY_EARTH_LIKE: Vec2 = Vec2::new(0.0, -500.0);
pub const GRAVITY_MOON_LIKE: Vec2 = Vec2::new(0.0, -81.0);
pub const GRAVITY_MARIO_LIKE: Vec2 = Vec2::new(0.0, -2000.0);  // Snappy!
pub const GRAVITY_FLIP: Vec2 = Vec2::new(0.0, 500.0);          // Upside-down!
pub const GRAVITY_ZERO: Vec2 = Vec2::ZERO;                      // Space!

/// 📐 Common angles (radians)
pub const DEG_45: f32 = 0.7853981633974483;   // π/4
pub const DEG_90: f32 = 1.5707963267948966;   // π/2
pub const DEG_180: f32 = 3.141592653589793;   // π
pub const DEG_360: f32 = 6.283185307179586;   // 2π

/// ⏱️ Common physics timesteps
pub const DT_30FPS: f32 = 1.0 / 30.0;   // 33.33ms  -  low quality
pub const DT_60FPS: f32 = 1.0 / 60.0;   // 16.67ms  -  standard
pub const DT_120FPS: f32 = 1.0 / 120.0; // 8.33ms  -  high quality
pub const DT_240FPS: f32 = 1.0 / 240.0; // 4.17ms  -  overkill
```

---

## 🧮 Common Physics Formulas

### Motion

| Formula | Description |
|---------|-------------|
| `v = u + at` | Velocity from acceleration |
| `s = ut + ½at²` | Displacement from acceleration |
| `v² = u² + 2as` | Velocity-displacement relation |
| `F = ma` | Newton's Second Law |
| `p = mv` | Momentum |
| `KE = ½mv²` | Kinetic energy |
| `W = F·d` | Work done by a force |

### Collisions

| Formula | Description |
|---------|-------------|
| `j = -(1+e)·v_rel·n / (1/m₁ + 1/m₂)` | Impulse magnitude |
| `v₁' = v₁ + j·n/m₁` | Post-collision velocity |
| `v₂' = v₂ - j·n/m₂` | Post-collision velocity |
| `F_friction ≤ μ·F_normal` | Coulomb friction law |

### Rotations

| Formula | Description |
|---------|-------------|
| `ω = dθ/dt` | Angular velocity |
| `α = dω/dt` | Angular acceleration |
| `τ = I·α` | Torque = moment of inertia × angular accel |
| `L = I·ω` | Angular momentum |
| `v_tangential = ω × r` | Tangential velocity |

---

## 🦀 Essential Rust Patterns

### Builder Pattern for Components

```rust
/// 🏗️ Builder pattern for physics objects
#[derive(Component)]
struct PhysicsObject {
    pos: Vec2,
    vel: Vec2,
    mass: f32,
    restitution: f32,
}

impl PhysicsObject {
    fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            mass: 1.0,
            restitution: 0.5,
        }
    }
    
    fn with_velocity(mut self, x: f32, y: f32) -> Self {
        self.vel = Vec2::new(x, y);
        self
    }
    
    fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }
    
    fn bouncy(mut self) -> Self {
        self.restitution = 0.9;
        self
    }
}

// Usage: PhysicsObject::new(0.0, 100.0).with_velocity(50.0, 0.0).bouncy()
```

### Type State for Safety

```rust
/// 🛡️ Using types to prevent invalid physics states
struct Resting;     // Object is at rest
struct Moving;      // Object is in motion

/// 👻 Physics state machine encoded in types!
#[derive(Component)]
struct RigidBody<State = Moving> {
    pos: Vec2,
    vel: Vec2,
    _state: std::marker::PhantomData<State>,
}

impl RigidBody<Moving> {
    fn integrate(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }
    
    fn sleep(self) -> RigidBody<Resting> {
        RigidBody {
            pos: self.pos,
            vel: Vec2::ZERO,
            _state: std::marker::PhantomData,
        }
    }
}

impl RigidBody<Resting> {
    fn wake(self) -> RigidBody<Moving> {
        RigidBody {
            pos: self.pos,
            vel: Vec2::ZERO,
            _state: std::marker::PhantomData,
        }
    }
}
```

### Newtype Pattern for Units

```rust
/// 📏 Preventing unit confusion with newtypes
#[derive(Component)]
struct Meters(pub f32);

#[derive(Component)]
struct MetersPerSecond(pub f32);

#[derive(Component)]
struct MetersPerSecondSquared(pub f32);

// ✅ Compiler catches unit errors!
fn cannot_mix_units(pos: Meters, vel: MetersPerSecond) {
    // let bad = pos + vel;  // ❌ Compile error!
    let dt = MetersPerSecondSquared(1.0);
    // let also_bad = vel + dt;  // ❌ Compile error!
}
```

### Command Pattern for Spawning

```rust
/// 📝 Command queue for spawning physics objects
#[derive(Event)]
enum PhysicsCommand {
    SpawnBall { pos: Vec2, radius: f32, mass: f32 },
    SpawnBox { pos: Vec2, size: Vec2, mass: f32 },
    RemoveAll,
    SetGravity(Vec2),
    Explosion { center: Vec2, force: f32 },
}

fn physics_command_handler(
    mut commands: EventReader<PhysicsCommand>,
    mut spawn_commands: Commands,
    query: Query<Entity, With<Collider>>,
    mut settings: ResMut<PhysicsSettings>,
) {
    for cmd in commands.read() {
        match cmd {
            PhysicsCommand::SpawnBall { pos, radius, mass } => {
                // Spawn a ball...
            }
            PhysicsCommand::RemoveAll => {
                for entity in query.iter() {
                    spawn_commands.entity(entity).despawn();
                }
            }
            PhysicsCommand::SetGravity(g) => {
                settings.gravity = *g;
            }
            _ => {}
        }
    }
}
```

---

## 📖 Further Reading

### Books 📚

| Title | Author | Focus |
|-------|--------|-------|
| *Game Physics Engine Development* | Ian Millington | The bible of game physics |
| *Real-Time Collision Detection* | Christer Ericson | All about collisions |
| *Physics for Game Developers* | David Bourg | Practical game physics |
| *Mathematics for 3D Game Programming* | Eric Lengyel | The math behind everything |

### Online Resources 🌐

- **[Bevy Book](https://bevyengine.org/learn/)**  -  Official Bevy documentation
- **[Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)**  -  Official examples
- **[Game Physics Pearls](https://www.gamephysicspearls.com/)**  -  Gems of wisdom
- **[The Nature of Code](https://natureofcode.com/)**  -  Physics simulations in code

### Rust Crates to Explore 🦀

| Crate | Description |
|-------|-------------|
| [`bevy_rapier`](https://crates.io/crates/bevy_rapier) | Full physics engine (production-ready!) |
| [`bevy_xpbd`](https://crates.io/crates/bevy_xpbd) | Bevy-native physics engine |
| [`parry`](https://crates.io/crates/parry) | Collision detection library (Rapier's core) |
| [`nalgebra`](https://crates.io/crates/nalgebra) | Linear algebra (used by Rapier) |
| [`vek`](https://crates.io/crates/vek) | Alternative math types |

---

## 🎉 Final Words

> **"You've journeyed from vectors to Verlet, from gravity to gimbal lock, from Euler to ECS. You now hold the keys to the physics kingdom. Go build something that bounces, floats, crashes, and soars!"** 🚀

```rust
/// The final physics system  -  YOU!
fn you_as_a_physics_engine() -> &'static str {
    "You're now equipped to build game physics in Rust with Bevy! 🎮"
}

fn main() {
    println!("{}", you_as_a_physics_engine());
    println!("🎯 Master the math. 🏗️ Write clean code. 🎮 Make amazing games!");
}
```

> 💡 **Full source code for this chapter:** [code-examples/ch16-appendix/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch16-appendix)
> 
> The runnable project includes Cargo.toml, main.rs, and complete module files.

---

**[← Previous: Mini Physics Sandbox](ch15-physics-sandbox.md)** | **[Back to Index →](ch01-foreword.md)**
