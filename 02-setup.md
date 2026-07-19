# ⚙️ Setting Up Your Bevy Physics Playground

> **"A craftsman is only as good as their workshop. Let's build a glorious one!"** 🔨

---

## 📋 What We'll Build

By the end of this chapter, you'll have:

```
📁 my_physics_game/
├── Cargo.toml        # 📦 Dependencies
├── src/
│   ├── main.rs       # 🎬 Entry point
│   ├── physics/      # 🧠 Our physics engine core
│   │   ├── mod.rs
│   │   ├── components.rs   # 📍 Position, Velocity, etc.
│   │   ├── integration.rs  # 🔄 Euler, Verlet solvers
│   │   └── collision.rs    # 🧱 Collision detection
│   └── render.rs     # 🎨 Visualization helpers
```

---

## 🦀 Step 1: Create the Project

```bash
# Create a new Rust binary project
cargo new my_physics_game
cd my_physics_game
```

---

## 📦 Step 2: Configure Dependencies

```toml
# Cargo.toml
[package]
name = "my_physics_game"
version = "0.1.0"
edition = "2021"
description = "🎮 A physics playground built with Bevy & Rust"

[dependencies]
# 🎯 Bevy — the game engine we'll use for rendering and ECS
bevy = "0.15"

# 📐 Euclid — robust 2D/3D geometry types (we'll use Vec2, etc.)
# We use bevy's math types primarily, but euclid for comparison
# Optional: debug visualization
bevy-inspector-egui = "0.28"  # 🕵️ Debug GUI (optional)

[profile.dev]
# ⚡ Faster compilation for development
opt-level = 1

[profile.release]
# 🚀 Optimize for performance
lto = true
codegen-units = 1
```

### 💡 Why These Dependencies?

| Crate | Purpose | Why Not Alternatives? |
|-------|---------|----------------------|
| `bevy` | Rendering, ECS, input, window management | The best Rust game engine 🏆 |
| `bevy-inspector-egui` | Debug visualization (optional) | Lets you inspect entities at runtime |

---

## 🏗️ Step 3: Project Structure

Let's create our physics module structure:

```bash
# Create the physics module directory
mkdir -p src/physics

# Create module files
touch src/physics/mod.rs
touch src/physics/components.rs
touch src/physics/integration.rs
touch src/physics/collision.rs
```

---

## 📝 Step 4: The Physics Components

```rust
// 📁 src/physics/components.rs
//! # 📍 Physics Components
//!
//! These components define the physical state of entities.
//! We keep them separate from Bevy's built-in Transform
//! to maintain clean separation between simulation and rendering.

use bevy::prelude::*;

// ═══════════════════════════════════════════════════════════════
// 🎯 CORE PHYSICS COMPONENTS
// ═══════════════════════════════════════════════════════════════

/// 📍 **Position** — Where an object is in world space.
///
/// Stored as a 2D vector. In game physics, position is always
/// the first thing we integrate. It's our "ground truth" location.
///
/// ## Why a separate component?
/// Bevy's `Transform` includes position, rotation, AND scale
/// combined. For physics simulation, we want raw, unfiltered
/// position data that we can manipulate mathematically without
/// triggering transform hierarchy updates every frame.
#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub Vec2);

impl Position {
    /// 🆕 Create a new position from x,y coordinates
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

impl Default for Position {
    fn default() -> Self {
        Self(Vec2::ZERO) // Spawn at origin by default
    }
}

/// 🏃 **Velocity** — Rate of change of position (units/second).
///
/// Think of velocity as "where the object wants to go."
/// A positive x means "moving right," negative y means "moving down"
/// (in Bevy's default 2D coordinate system).
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec2::ZERO) // Start stationary
    }
}

/// ⚡ **Acceleration** — Rate of change of velocity (units/s²).
///
/// This is where **forces** live! Gravity, thrust, wind — they all
/// contribute to acceleration. We accumulate forces each frame,
/// then integrate acceleration to update velocity.
///
/// ## Physics Connection
/// Newton's Second Law: F = ma → a = F/m
/// So acceleration IS the force, just divided by mass.
#[derive(Component, Debug, Clone, Copy)]
pub struct Acceleration(pub Vec2);

impl Default for Acceleration {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

/// ⚖️ **Mass** — How heavy an object is.
///
/// Mass affects how forces change velocity:
/// - High mass → Hard to accelerate → "Heavy" feel
/// - Low mass → Easy to accelerate → "Light" feel
///
/// ## Special Values
/// - `mass = 0.0` → Static/immovable object (infinite mass)
/// - `mass < 0.0` → Nonsense physics (avoid!)
#[derive(Component, Debug, Clone, Copy)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.0) // Default mass of 1 unit
    }
}

// ═══════════════════════════════════════════════════════════════
// 💥 COLLISION COMPONENTS
// ═══════════════════════════════════════════════════════════════

/// 🔵 **Collider** — The shape used for collision detection.
///
/// For now, we support two shapes:
/// - `Circle`: Simple, fast, rotation-independent
/// - `Aabb`: Axis-Aligned Bounding Box, still simple but orientation matters
///
/// We'll add more shapes (polygons, capsules) in later chapters!
#[derive(Component, Debug, Clone)]
pub enum Collider {
    /// 🔴 Circle with radius
    Circle { radius: f32 },
    /// 📦 Axis-Aligned Bounding Box with half-extents
    Aabb { half_width: f32, half_height: f32 },
}

/// 🏷️ **Collision Tag** — Marks an entity as participating in collision.
///
/// This makes it easy to query: "give me all things that can collide"
/// without checking for collider components on every entity.
#[derive(Component, Debug, Clone)]
pub struct CollisionTag;

// ═══════════════════════════════════════════════════════════════
// 🧪 TEST HELPERS
// ═══════════════════════════════════════════════════════════════

/// 🎯 Creates a bundle of physics components for quick spawning
#[derive(Bundle)]
pub struct PhysicsBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub mass: Mass,
}

impl PhysicsBundle {
    /// Create a new physics bundle with sensible defaults
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Position::new(x, y),
            velocity: Velocity::default(),
            acceleration: Acceleration::default(),
            mass: Mass::default(),
        }
    }

    /// Set initial velocity (chainable builder pattern)
    pub fn with_velocity(mut self, x: f32, y: f32) -> Self {
        self.velocity = Velocity::new(x, y);
        self
    }

    /// Set mass (chainable)
    pub fn with_mass(mut self, mass: f32) -> Self {
        self.mass = Mass(mass);
        self
    }
}
```

---

## 🧮 Step 5: Integration Module

```rust
// 📁 src/physics/integration.rs
//! # 🔄 Physics Integration
//!
//! Integration is how we step the simulation forward in time.
//! Given forces and accelerations, we update velocities and positions.
//!
//! ## The Integration Pipeline
//!
//! ```
//! ┌──────────┐    ┌──────────┐    ┌──────────┐
//! │  Forces  │ -> │ Velocity │ -> │ Position │
//! │ (causes) │    │ (change) │    │ (result) │
//! └──────────┘    └──────────┘    └──────────┘
//!      a = F/m     v += a·dt       x += v·dt
//! ```
//!
//! This is called **Euler Integration** — simple, fast, and
//! good-enough for most games!

use bevy::prelude::*;
use crate::physics::components::*;

/// 🎯 Resource that controls physics simulation parameters
///
/// We use a `Resource` (not a component) because these settings
/// apply to the entire world, not individual entities.
#[derive(Resource)]
pub struct PhysicsSettings {
    /// 🌍 Global gravity vector (e.g., (0, -9.81) for Earth-like)
    pub gravity: Vec2,
    /// ⏱️ Fixed timestep in seconds (1/60 = ~16.67ms)
    pub fixed_dt: f32,
    /// 🔄 Number of sub-steps per frame (higher = more stable)
    pub substeps: u32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            // 🌍 Gravity pointing downward (in Bevy's coordinate system)
            // 9.81 m/s² feels natural — not too fast, not too slow
            gravity: Vec2::new(0.0, -9.81),
            // 🎯 60 FPS physics timestep
            fixed_dt: 1.0 / 60.0,
            // 🏃 Single pass per frame (increase for stability)
            substeps: 1,
        }
    }
}

/// 🔄 **Euler Integration System**
///
/// This is the heart of our physics simulation. Every frame, we:
/// 1. Apply gravity to acceleration
/// 2. Integrate acceleration into velocity
/// 3. Integrate velocity into position
///
/// ## The Math
///
/// ```
/// Given:  a(t) = F/m          (acceleration from force)
///         v(t + dt) = v(t) + a(t) × dt   (integrate once)
///         x(t + dt) = x(t) + v(t) × dt   (integrate again)
/// ```
///
/// ## Why It Works
/// Integration is just **accumulation of small changes**.
/// If you're driving at 60 mph for 0.5 hours, you travel 30 miles:
///     position += velocity × time
/// It's exactly the same in physics!
pub fn euler_integration(
    mut query: Query<(
        &mut Position,
        &mut Velocity,
        &mut Acceleration,
        &Mass,
    )>,
    settings: Res<PhysicsSettings>,
    time: Res<Time>,
) {
    // ⏱️ Use the fixed timestep, not the frame delta
    // This ensures deterministic, framerate-independent physics
    let dt = settings.fixed_dt;
    let substeps = settings.substeps;
    let sub_dt = dt / substeps as f32;

    for (mut pos, mut vel, mut acc, mass) in query.iter_mut() {
        // ⏭️ Run multiple sub-steps for better stability
        for _ in 0..substeps {
            // STEP 1: 🌍 Apply gravity to acceleration
            // Gravity is a CONSTANT force, so it's always applied
            acc.0 += settings.gravity;

            // STEP 2: 🏃 Integrate acceleration → velocity
            //   v_new = v_old + a × dt
            // If mass > 0, acceleration tells us how velocity changes
            if mass.0 > 0.0 {
                vel.0 += acc.0 * sub_dt;
            }

            // STEP 3: 📍 Integrate velocity → position
            //   x_new = x_old + v × dt
            // Velocity tells us how position changes
            pos.0 += vel.0 * sub_dt;

            // 🧹 Reset acceleration for next frame
            // Forces are re-calculated each frame, so we clear them now
            acc.0 = Vec2::ZERO;
        }
    }
}

/// 🌬️ **Drag/Damping System**
///
/// Real objects don't move forever — friction and air resistance
/// slow them down. We apply a simple damping factor each frame.
///
/// The formula: v *= (1 - damping)^dt
/// This is an exponential decay model — the faster you go, the
/// more drag you experience (simplified Stokes' drag).
#[derive(Component, Debug, Clone, Copy)]
pub struct LinearDamping(pub f32);

impl Default for LinearDamping {
    fn default() -> Self {
        Self(0.01) // 1% velocity loss per second (subtle)
    }
}

pub fn apply_damping(
    mut query: Query<(&mut Velocity, &LinearDamping)>,
    settings: Res<PhysicsSettings>,
) {
    let dt = settings.fixed_dt;

    for (mut vel, damping) in query.iter_mut() {
        // 📉 Exponential decay: v *= (1 - damping)^(dt)
        let factor = (1.0 - damping.0).powf(dt);
        vel.0 *= factor;
    }
}
```

---

## 🎬 Step 6: Main Entry Point

```rust
// 📁 src/main.rs
//! # 🎮 My Physics Game
//!
//! This is our main entry point. We set up Bevy, register our
//! physics systems, and create some initial objects to watch them fall.

use bevy::prelude::*;

// 📦 Import our physics module
mod physics;
mod render;

/// 🎯 The `main` function — where everything begins!
///
/// Bevy applications are constructed using the `App` builder pattern.
/// We add plugins, resources, systems, and entities in a declarative way.
fn main() {
    App::new()
        // 🎬 Bevy's built-in plugins (windowing, rendering, input, audio...)
        .add_plugins(DefaultPlugins)

        // ⚙️ Register our custom physics plugin
        .add_plugins(physics::PhysicsPlugin)

        // 🎨 Register visualization helpers
        .add_plugins(render::RenderPlugin)

        // 🏗️ Systems that run once at startup
        .add_systems(Startup, setup)

        // 🌀 Systems that run every frame
        // Note: physics runs BEFORE rendering to ensure
        // positions are up-to-date when we draw
        .run();
}

/// 🏗️ **Setup** — Create the initial world state
///
/// This function runs once when the app starts. We create:
/// 1. A camera to see things
/// 2. Some test objects to watch them fall
fn setup(mut commands: Commands) {
    // 📷 Camera setup
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    });

    // 🎯 Spawn a falling square
    commands.spawn((
        // Physics components
        physics::components::PhysicsBundle::new(0.0, 300.0),
        physics::components::Collider::Circle { radius: 20.0 },
        physics::components::CollisionTag,
        // Visual representation
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.3, 0.3), Vec2::new(40.0, 40.0)),
            ..default()
        },
    ));

    println!("🚀 Physics simulation initialized!");
    println!("📐 Watching objects fall under gravity...");
}
```

---

## 🧪 Step 7: Test It!

```bash
# Run the project
cargo run
```

You should see a red square falling down the screen under gravity! 🎉

---

## 🎯 What We Learned

| Concept | Implementation |
|---------|---------------|
| 📦 Module structure | Separated physics into its own module |
| 📍 Components | Position, Velocity, Acceleration, Mass |
| 🔄 Integration | Euler integration in a Bevy system |
| ⚙️ Resources | PhysicsSettings with gravity, timestep |
| 🏗️ Bundles | PhysicsBundle for quick entity setup |

---

## 🔬 Verify Your Understanding

Before moving on, make sure you can answer:

1. ❓ Why do we separate `Position` from Bevy's `Transform`?
2. ❓ What does `acc.0 = Vec2::ZERO` accomplish?
3. ❓ Why use a fixed timestep instead of the real frame delta?

> **Answers in the next chapter!** 🎯

---

**[← Previous: Foreword & Index](01-foreword-and-index.md)** | **[Next: Vector Mathematics →](03-vectors.md)**
