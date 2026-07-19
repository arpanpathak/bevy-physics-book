# ⚙️ Setting Up Your Bevy Physics Playground

> **"Before you can simulate physics, you need to understand the simulation framework. Bevy's architecture isn't just 'how we set up the project'  -  it's the conceptual foundation for everything that follows."** 🔨

---

## 🎯 What We'll Build

By the end of this chapter, you won't just have a working Bevy project. You'll understand **what each piece does**, **why it's structured that way**, and **how the ECS engine processes your physics code**:

```
📁 my_physics_game/
├── Cargo.toml            # 📦 Dependency declarations (what crates we use)
├── src/
│   ├── main.rs           # 🎬 App construction & system registration
│   └── physics/
│       ├── mod.rs        # 📝 Module declarations & plugin definition
│       ├── components.rs # 📍 Data definitions (Position, Velocity, etc.)
│       ├── integration.rs# 🔄 Logic (how position changes over time)
│       └── collision.rs  # 🧱 Logic (what overlaps with what)
```

---

## 📦 Step 1: Understanding Dependencies

When you write `bevy = "0.15"` in Cargo.toml, you're importing Bevy's **core crate**, which re-exports everything you need:

```toml
[package]
name = "my_physics_game"
version = "0.1.0"
edition = "2021"

[dependencies]
# 🎯 Bevy 0.15  -  the entire game framework
# This ONE crate gives us:
#   - ECS World (entities, components, systems)
#   - Renderer (window, sprites, camera)
#   - Input (keyboard, mouse, gamepad)
#   - Audio, UI, asset system, and much more
bevy = "0.15"
```

### What's Inside Bevy?

Bevy is not a monolith  -  it's a **modular collection of plugins** under one crate:

```
bevy 0.15 ─┬── bevy_ecs        🧠 Core ECS (World, entities, components, systems)
           ├── bevy_render     🎨 GPU rendering pipeline
           ├── bevy_sprite     🖼️ 2D sprite rendering
           ├── bevy_window     🪟 Window creation & management
           ├── bevy_input      ⌨️ Keyboard/mouse/gamepad input
           ├── bevy_time       ⏱️ Frame timing, delta time
           ├── bevy_audio      🔊 Sound playback
           └── bevy_ui         🧩 User interface elements
```

When you call `.add_plugins(DefaultPlugins)` in your app, you're enabling ALL of these at once. For physics, the essential ones are:

- `bevy_ecs`  -  The **entire physics engine** runs on this
- `bevy_time`  -  Provides `Time.delta_secs()` for integration
- `bevy_render` + `bevy_sprite`  -  Visualizes physics objects
- `bevy_input`  -  Enables interactive physics controls

> 💡 **Key Insight:** The physics engine we build will run ENTIRELY within `bevy_ecs`. The other plugins are just for visualization and interaction. If you removed `DefaultPlugins`, your physics code would still compile and run  -  you just wouldn't see anything!

---

## 🧠 Step 2: Understanding the ECS World

Before writing a single line of physics, you MUST understand how Bevy's World processes data. This is the single most important concept in this entire book:

### The Mental Model

Think of Bevy's World as a **relational database** for game objects:

```
WORLD
├── "Tables" = Archetypes (unique combinations of component types)
├── "Rows" = Entities (identified by ID)
├── "Columns" = Components (typed columns per archetype)
└── "Queries" = SELECT statements (find entities matching component patterns)
```

### What happens when you call `.run()`?

```rust
fn main() {
    App::new()                    // 1. Create empty application
        .add_plugins(DefaultPlugins) // 2. Register built-in systems
        .add_systems(Startup, setup) // 3. Register startup logic
        .add_systems(Update, physics_step) // 4. Register per-frame logic
        .run();                   // 5. 🚀 ENTER THE MAIN LOOP
}
```

Let's trace through exactly what `.run()` does:

```
.run() ENTRY POINT
│
├── Phase 1: BUILD SCHEDULE
│   ├── Collect ALL registered systems
│   ├── Analyze component access patterns (reads vs writes)
│   ├── Build dependency graph from .before()/.after()/.chain()
│   └── Determine parallel execution groups
│
├── Phase 2: RUN STARTUP (once)
│   ├── Execute all Startup systems in registration order
│   │   └── setup() runs here:
│   │       ├── commands.spawn(Camera2dBundle) → Queues camera creation
│   │       ├── commands.spawn((Position, Velocity, SpriteBundle)) → Queues entity
│   │       └── All queued commands execute at sync point
│   └── ─── SYNC POINT ─── (Commands are flushed, entities become real)
│
└── Phase 3: MAIN LOOP (every frame, until window closes)
    ├── 1. Event readers are advanced (previous frame's events cleared)
    ├── 2. Time.delta_secs() is updated (time since last frame)
    ├── 3. SCHEDULER RUNS Update systems
    │   └── physics_step() runs here:
    │       ├── Query<(Entity, &Velocity, &mut Position)>
    │       ├── For each matching entity: pos += vel * dt
    │       └── Reads Time resource for dt
    ├── 4. ─── SYNC POINT ─── (any Commands from this frame)
    ├── 5. RENDER: Bevy reads Transform components, draws sprites
    └── 6. Repeat
```

### The Critical Insight: Systems Iterate, They Don't "Call"

This is the most important mental shift from traditional game programming:

```rust
// ❌ TRADITIONAL APPROACH (object-oriented)
fn game_loop(objects: &mut Vec<GameObject>) {
    for obj in objects.iter_mut() {   // Manual iteration
        obj.update(dt);               // Virtual method call
    }
}

// ✅ BEVY APPROACH (data-oriented)
fn physics_step(
    query: Query<(&Velocity, &mut Position)>,  // Declarative query
    time: Res<Time>,
) {
    // Bevy handles the iteration  -  it finds ALL matching entities
    // and feeds them to this function body
    for (vel, mut pos) in query.iter_mut() {
        pos.0 += vel.0 * time.delta_secs();
    }
}
```

The difference is subtle but profound:
- **OO:** You tell objects what to do. Objects own their data.
- **ECS:** You describe a transformation on data. Bevy finds the data for you.

This enables Bevy's **parallelism guarantee**: If two systems access different components (or the same components but only read), they can run in parallel automatically. The scheduler proves safety at compile time.

---

## 🏗️ Step 3: Project Structure  -  Why Module Separation Matters

```bash
mkdir -p src/physics
touch src/physics/mod.rs
touch src/physics/components.rs
touch src/physics/integration.rs
touch src/physics/collision.rs
```

This structure isn't arbitrary. Each file has a **clear responsibility**:

```
src/
├── main.rs              # 🎬 Main entry: App construction, system registration
│                        #    "THE ORCHESTRATOR"  -  knows what systems exist
│                        #    and in what order they run, but doesn't
│                        #    know HOW they work internally
│
└── physics/             # 🧠 Physics engine: encapsulated module
    ├── mod.rs           # 📝 Module declarations & PhysicsPlugin definition
    │                    #    "THE BOUNDARY"  -  external code only sees
    │                    #    this module's public API
    │
    ├── components.rs    # 📍 Data types: Position, Velocity, Mass, etc.
    │                    #    "THE VOCABULARY"  -  defines what physics
    │                    #    concepts exist (pure data, no logic)
    │
    ├── integration.rs   # 🔄 Systems that UPDATE components
    │                    #    "THE PHYSICS"  -  implements Euler integration,
    │                    #    force accumulation, damping
    │
    └── collision.rs     # 🧱 Systems that DETECT and RESPOND to overlaps
                         #    "THE COLLISIONS"  -  broad phase, narrow phase,
                         #    impulse resolution
```

### 💡 Why This Separation Matters for Learning

Each component type in your physics engine needs THREE things:

| # | What | Where | Example |
|---|------|-------|---------|
| 1 | **Data definition** | `components.rs` | `struct Position(Vec2)` |
| 2 | **Creation logic** | Setup/system | `Position::new(0.0, 0.0)` |
| 3 | **Update logic** | System | `pos.0 += vel.0 * dt` |

By separating data (`components.rs`) from logic (`integration.rs`, `collision.rs`), you enforce the ECS discipline: **components are dumb data, systems are smart logic**. This makes your code easier to reason about, test, and parallelize.

---

## 📝 Step 4: Components  -  The Deep Dive

Let's examine each physics component and understand **why it exists**, **what it represents mathematically**, and **how it connects to the physics pipeline**.

### Position: Where Is the Object?

```rust
/// 📍 Position represents a point in 2D Euclidean space.
///
/// MATHEMATICAL MEANING:
/// Position is a VECTOR from the origin (0,0) to the object's location.
/// In physics terms: position = r(t)  -  the object's location at time t.
///
/// WHY A SEPARATE COMPONENT?
/// Bevy's built-in `Transform` bundles position + rotation + scale together.
/// For rendering, this is convenient. For physics, it's a PROBLEM:
///
/// Problem 1: Transform.hierarchy
///   Transform is part of Bevy's render graph. Modifying it triggers
///   hierarchy recomputation (children move with parents). Physics
///   doesn't need this  -  we just want raw x,y coordinates.
///
/// Problem 2: Transform contains rotation + scale
///   Physics doesn't need these for integration. Carrying them in
///   every physics iteration wastes cache bandwidth.
///
/// Problem 3: Separation of concerns
///   Physics should own its own data. We write Position, and then
///   SYNC to Transform for rendering. This makes the physics pipeline
///   independent of the rendering pipeline.
#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub Vec2);

impl Position {
    /// Create a position from (x, y) coordinates
    /// x = horizontal axis (positive = right)
    /// y = vertical axis (positive = up in Bevy 2D)
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}
```

### Velocity: How Is Position Changing?

```rust
/// 🏃 Velocity represents the RATE OF CHANGE of position.
///
/// MATHEMATICAL MEANING:
/// Velocity is the DERIVATIVE of position with respect to time:
///   v(t) = dr/dt = lim(Δt→0) [r(t+Δt) - r(t)] / Δt
///
/// In discrete terms (what we actually compute):
///   v = Δr / Δt  →  Δr = v × Δt
///
/// UNITS: pixels per second (or meters per second in a physics simulation)
///
/// WHAT VELOCITY "FEELS LIKE" IN A GAME:
///   - v = (50, 0)   → Moving right at 50 px/s
///   - v = (0, -200) → Falling at 200 px/s (terminal velocity from gravity)
///   - v = (0, 0)    → Stationary (but forces may change this next frame!)
///
/// CRITICAL INSIGHT:
/// Velocity INTERPOLATES position between frames. If your game runs
/// at 60 FPS and velocity is (60, 0), the object moves 1 pixel per frame.
/// At 30 FPS, it moves 2 pixels per frame. The MOTION is the same  - 
/// just the stepping granularity changes. This is WHY we multiply by dt.
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity(pub Vec2);
```

### Acceleration: What Forces Act on This Object?

```rust
/// ⚡ Acceleration represents the RATE OF CHANGE of velocity.
///
/// MATHEMATICAL MEANING:
/// Acceleration is the SECOND DERIVATIVE of position:
///   a(t) = dv/dt = d²r/dt²
///
/// This is where Newton's Second Law lives:
///   F = ma  →  a = F/m
///
/// In our engine, we:
///   1. ACCUMULATE forces on an entity (gravity + drag + thrust + ...)
///   2. DIVIDE by mass to get acceleration
///   3. INTEGRATE acceleration to update velocity
///   4. INTEGRATE velocity to update position
///
/// THE ACCELERATION PIPELINE:
///
///   Frame Start
///       │
///       ▼
///   acc = (0, 0)          ← Clear acceleration from last frame
///       │
///       ▼
///   acc += gravity         ← Add gravitational acceleration
///   acc += drag / mass     ← Add drag force (converted to accel)
///   acc += thrust / mass   ← Add player input force
///       │
///       ▼
///   vel += acc × dt        ← Integrate: acceleration → velocity
///   pos += vel × dt        ← Integrate: velocity → position
///       │
///       ▼
///   acc = (0, 0)          ← Clear for next frame
///
/// IMPORTANT: Acceleration is RECALCULATED every frame.
/// We don't "store" acceleration between frames  -  it's ephemeral,
/// derived from the forces currently acting on the object.
/// This is why we clear it after integration.
///
/// UNITS: pixels per second² (or m/s²)
///   - Gravity on Earth: ~9.81 m/s² ≈ 500 px/s² in many games
///   - Mario jump: ~2000-4000 px/s² (snappy!)
///   - Space: 0 px/s² (no gravity)
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Acceleration(pub Vec2);
```

### Mass: How Hard Is It to Move This Object?

```rust
/// ⚖️ Mass represents INERTIA  -  resistance to acceleration.
///
/// MATHEMATICAL MEANING:
/// Mass is the proportionality constant between force and acceleration:
///   F = ma  →  a = F/m
///
/// PHYSICAL INTUITION:
///   - Push a feather (low mass): it accelerates easily
///   - Push a boulder (high mass): it barely moves
///
/// GAME DESIGN IMPLICATIONS:
///   mass = 1.0    ← Default "normal" object
///   mass = 10.0   ← Heavy object (slow to accelerate, hard to push)
///   mass = 0.1    ← Light object (very responsive, easy to launch)
///   mass = 0.0    ← STATIC/immovable (infinite mass  -  walls, floors)
///
/// THE SPECIAL CASE OF mass = 0.0:
/// When mass is zero, F = ma would give a = F/0 = ∞ (division by zero).
/// So we handle it specially: entities with zero mass don't move.
/// They're STATIC COLLIDERS  -  they participate in collision detection
/// but are never moved by the physics engine.
///
/// This is how we create walls, floors, and other immovable objects.
#[derive(Component, Debug, Clone, Copy)]
pub struct Mass(pub f32);
```

### The Complete Data Flow

Understanding how these four components interact is the foundation of ALL game physics:

```
┌─────────────────────────────────────────────────────────────────┐
│                    ONE PHYSICS TIMESTEP                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌──────────────┐                                                 │
│  │ Acceleration │ ◄──── Force Accumulation (gravity, drag, etc.) │
│  │   (a = F/m)  │       a_new = F_gravity/m + F_drag/m + ...    │
│  └──────┬───────┘                                                 │
│         │                                                         │
│         │ INTEGRATE: v_new = v_old + a × dt                      │
│         ▼                                                         │
│  ┌──────────────┐                                                 │
│  │   Velocity   │ ◄──── Now holds updated velocity                │
│  │   (v_new)    │                                                 │
│  └──────┬───────┘                                                 │
│         │                                                         │
│         │ INTEGRATE: x_new = x_old + v_new × dt                  │
│         ▼                                                         │
│  ┌──────────────┐                                                 │
│  │   Position   │ ◄──── Now holds updated position                │
│  │   (x_new)    │                                                 │
│  └──────────────┘                                                 │
│                                                                   │
│  Then: Acceleration is CLEARED for next frame                     │
│  (forces are re-calculated each frame)                            │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🧮 Step 5: Integration  -  The Engine That Moves Things

The integration module is where kinematics happens. Let's understand every line:

### PhysicsSettings Resource

```rust
/// 🎯 Resource that controls physics simulation parameters
///
/// A `Resource` in Bevy is a SINGLETON  -  there's only ONE instance
/// of this data in the entire World. Unlike components (which are
/// per-entity), resources are global.
///
/// Why use a Resource instead of a Component?
/// - Gravity affects ALL objects equally (it's not per-entity)
/// - Timestep is a WORLD setting, not an object property
/// - There's only ONE physics world
#[derive(Resource)]
pub struct PhysicsSettings {
    /// 🌍 Global gravity vector (e.g., (0, -9.81) for Earth-like)
    /// Units: pixels/second² (or m/s² in a physics-scale simulation)
    ///
    /// For 2D games, gravity typically only has a Y component.
    /// Negative Y = pulls objects downward.
    /// Zero X = no horizontal gravity (wind would change this).
    pub gravity: Vec2,
    
    /// ⏱️ Fixed timestep in seconds
    /// 1/60 = ~16.67ms (standard game physics rate)
    ///
    /// WHY FIXED? NOT variable like the frame rate?
    /// If dt varied with frame rate:
    ///   - 30 FPS: dt = 33ms, objects jump twice as far per step
    ///   - 120 FPS: dt = 8ms, objects move half as far per step
    /// This means physics FRAMERATE DEPENDENT  -  different computers
    /// would get different simulation results!
    ///
    /// Fixed timestep solves this:
    ///   - Physics always advances by 1/60 second per step
    ///   - If a frame takes 33ms (30 FPS), physics runs TWICE
    ///   - If a frame takes 8ms (120 FPS), physics runs ONCE
    ///   - The RESULT is identical regardless of framerate!
    pub fixed_dt: f32,
    
    /// 🔄 Number of sub-steps per physics step
    /// More sub-steps = more stable but slower
    /// 1 = normal, 4 = very stable, 8 = overkill
    pub substeps: u32,
}
```

### The Integration System

```rust
/// 🔄 **Euler Integration System**
///
/// This is the HEART of our physics engine. Every frame, this system
/// transforms forces into motion through the chain:
///
///   Forces ──► Acceleration ──► Velocity ──► Position
///
/// The mathematical operation is called "integration" because we're
/// COMPUTING THE AREA under the acceleration curve.
///
/// ANALOGY: If acceleration is the gas pedal, velocity is the car's
/// speed, and position is how far the car has traveled. You press
/// the gas (acceleration) → car speeds up (velocity) → car moves
/// (position changes).
pub fn euler_integration(
    // 🎯 Query: find all entities with ALL four of these components
    mut query: Query<(
        &mut Position,      // 📍 We WRITE to position
        &mut Velocity,      // 🏃 We WRITE to velocity
        &mut Acceleration,  // ⚡ We READ and WRITE to acceleration
        &Mass,              // ⚖️ We READ mass (don't change it)
    )>,
    // 🔧 Read global settings
    settings: Res<PhysicsSettings>,
) {
    let dt = settings.fixed_dt;
    let substeps = settings.substeps;
    let sub_dt = dt / substeps as f32;

    for (mut pos, mut vel, mut acc, mass) in query.iter_mut() {
        // ⏭️ Sub-stepping: run physics multiple times per frame
        // Each sub-step is smaller = more stable integration
        for _ in 0..substeps {
            // STEP 1: Apply gravity to acceleration
            // Gravity is a CONSTANT force (always pulling down)
            // a_gravity = g (doesn't depend on mass!)
            // F_gravity = m × g, so a = F/m = g
            // This is why all objects fall at the same rate
            // (in vacuum  -  we'll add air resistance later)
            acc.0 += settings.gravity;

            // STEP 2: INTEGRATE acceleration → velocity
            // v_new = v_old + a × Δt
            //
            // This is a RECTANGLE APPROXIMATION of the area
            // under the acceleration curve:
            //
            //   a(t)
            //   │
            //   │   ┌──────────────────── ← a is constant over Δt
            //   │   │                    │
            //   │   │  area = a × Δt     │
            //   │   │                    │
            //   └───┴────────────────────► t
            //       Δt
            //
            // The area = CHANGE IN VELOCITY. This is the
            // FUNDAMENTAL THEOREM OF CALCULUS in action!
            if mass.0 > 0.0 {
                vel.0 += acc.0 * sub_dt;
            }

            // STEP 3: INTEGRATE velocity → position
            // x_new = x_old + v_new × Δt
            //
            // ⚠️ NOTE: We use v_new (just updated), not v_old!
            // This is "SEMI-IMPLICIT" or "SYMPLECTIC" Euler.
            //
            // Why it matters:
            //   Explicit Euler (bad):   x += v_old × dt
            //   Symplectic Euler (good): x += v_new × dt
            //
            // The difference: by using the NEW velocity, we estimate
            // the AVERAGE velocity over the timestep, not the START.
            // This small change conserves energy  -  objects in orbit
            // stay in orbit instead of spiraling outward!
            pos.0 += vel.0 * sub_dt;

            // STEP 4: Clear acceleration for next frame
            // Forces don't persist  -  each frame calculates them fresh
            // If we don't clear, old forces would accumulate and
            // objects would accelerate forever (RUNAWAY PHYSICS!)
            acc.0 = Vec2::ZERO;
        }
    }
}
```

---

## 🎬 Step 6: Main Entry Point  -  The Full Picture

Now let's understand the main.rs in its entirety:

```rust
use bevy::prelude::*;

mod physics;

fn main() {
    App::new()
        // ─── DEFAULT PLUGINS ───
        // This ONE call enables: window, renderer, input, audio, time, UI
        // Without it, you'd have a headless physics simulation
        // (which is valid for server-side physics!)
        .add_plugins(DefaultPlugins)
        
        // ─── CUSTOM PHYSICS PLUGIN ───
        // Our physics module exposes a Plugin that registers
        // all physics systems, resources, and events.
        // Adding it here = installing the physics engine.
        .add_plugins(physics::PhysicsPlugin)
        
        // ─── STARTUP SYSTEMS ───
        // Systems that run ONCE when the app starts.
        // Use these for: spawning initial entities, setting up
        // resources that need computation, camera setup.
        .add_systems(Startup, setup)
        
        // ─── UPDATE SYSTEMS ───
        // Systems that run EVERY FRAME.
        // Our physics systems are registered inside PhysicsPlugin,
        // but game-specific logic goes here.
        // Note: game systems should run AFTER physics to avoid
        // reading stale state.
        .add_systems(Update, (
            player_input,
            camera_follow,
        ).after(physics::PhysicsPlugin))
        
        // 🚀 ENTER THE MAIN LOOP
        .run();
}
```

### What Bevy Does After `.run()`

1. **Builds the system graph** from all `.add_systems()` calls
2. **Determines parallel execution** groups (systems accessing different components can run simultaneously)
3. **Runs all Startup systems** (your `setup()` function creates the initial world state)
4. **Enters the render loop**: For each frame, run Update systems → sync Commands → render

---

## 🔬 The Complete Data Flow: Trace Through One Frame

Let's trace a complete frame from start to finish, showing exactly what data exists and how it transforms:

```
INITIAL STATE (Frame N, start):
────────────────────────────────
World:
  Entity(1):
    Position: (0.0, 300.0)
    Velocity: (0.0, 0.0)
    Acceleration: (0.0, 0.0)    ← Cleared at end of last frame
    Mass: 1.0

  Resource:
    PhysicsSettings.gravity: (0.0, -500.0)  ← 500 px/s² downward
    PhysicsSettings.fixed_dt: 0.01667        ← 1/60 second

FRAME N BEGINS:
────────────────

STEP 1: clear_forces()
  Entity(1).Acceleration = (0.0, 0.0)  ← Already zero, just confirming

STEP 2: apply_gravity()
  Entity(1).Acceleration += (0.0, -500.0) × 1.0 (mass is implicit here)
  Entity(1).Acceleration = (0.0, -500.0)
  ↓
  Now acceleration says: "I'm accelerating downward at 500 px/s²"

STEP 3: integrate()
  Sub-step 1 (sub_dt = 0.01667):
    a = (0.0, -500.0)                      ← From F = ma
    v += (0.0, -500.0) × 0.01667
    v = (0.0, -8.33)                       ← Fell 8.33 px/s in this step
    x += (0.0, -8.33) × 0.01667
    x = (0.0, 299.86)                      ← Moved down 0.14 pixels
    a = (0.0, 0.0)                         ← Cleared for next sub-step

  (If substeps = 1, we're done. If substeps = 4,
   repeat 3 more times with smaller steps)

STEP 4: sync_to_render()
  Entity(1).Transform.translation = (0.0, 299.86, 0.0)  ← Bevy's renderer sees this

FRAME N ENDS:
────────────────
  BOTTOM LINE: Entity moved from y=300 to y=299.86 (falling!)
  Next frame: velocity continues accumulating, object accelerates
  After ~60 frames (1 second): object has fallen ~250 pixels
```

---

## 🎯 What We Learned (The Deep Version)

| Concept | What It Means | Why It Matters |
|---------|---------------|----------------|
| **App** | A builder for the entire game | Everything is registered here  -  plugins, systems, resources |
| **DefaultPlugins** | Built-in window/render/input/audio | Provides the "runtime" for our physics to exist in |
| **Plugin** | A package of systems + resources | Encapsulates our physics engine as a reusable unit |
| **System** | A function that transforms ECS data | Stateless, parallelizable, scheduled by Bevy |
| **Query** | A pattern-matching access to components | Like a database query  -  find entities with specific components |
| **Component** | A piece of typed data on an entity | Position, Velocity, etc.  -  PURE DATA, NO LOGIC |
| **Resource** | Global singleton data | PhysicsSettings  -  affects all entities |
| **Commands** | Deferred spawn/despawn/modify | Batched for efficiency, executed at sync points |
| **Sync Point** | Where deferred ops are flushed | Stops parallel execution temporarily  -  MINIMIZE THESE |
| **Integration** | Transforming acceleration → velocity → position | The mathematical heart of physics simulation |
| **Fixed Timestep** | Physics runs at constant rate regardless of FPS | Deterministic, framerate-independent simulation |

---

> **Key Takeaway:** Bevy's architecture isn't just ceremony  -  it's a carefully designed system that enables parallel, cache-efficient, deterministic physics simulation. Every struct, every trait, every registration call serves a purpose in the data flow. Understanding this flow is the difference between "copying code" and "knowing how to build." 🏗️

---

**[← Previous: Foreword & Index](ch01-foreword.md)** | **[Next: Vector Mathematics →](ch03-vectors.md)**
