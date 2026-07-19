# 🏗️ Bevy ECS Physics Architecture

> **"ECS isn't just an architecture — it's a philosophy. Data and logic are separate. Components are data. Systems are logic. Together, they're beautiful."** 🎨

---

## 🧠 The ECS Model

Bevy uses the **Entity-Component-System** pattern:

```
┌─────────────────────────────────────────────────────────┐
│                     ECS ARCHITECTURE                     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ENTITIES (IDs)      COMPONENTS (data)   SYSTEMS (logic)│
│                                                         │
│  ┌──────────┐       ┌──────────────┐    ┌───────────┐  │
│  │ Entity 1 │───────│ Position     │    │ Physics   │  │
│  └──────────┘       │ Velocity     │───►│ Systems   │  │
│                     │ Mass         │    │           │  │
│                     │ Collider     │    │ Move      │  │
│  ┌──────────┐       └──────────────┘    │ Collide   │  │
│  │ Entity 2 │───────┐                   │ Respond   │  │
│  └──────────┘       │  ┌──────────────┐ └───────────┘  │
│                     │  │ Position     │                 │
│                     └──│ Velocity     │    ┌───────────┐│
│                        │ RenderDepth  │───►│ Render    ││
│                        └──────────────┘    │ Systems   ││
│                                            └───────────┘│
└─────────────────────────────────────────────────────────┘
```

---

## 🎯 Physics Plugin Architecture

```rust
// 📁 src/physics/mod.rs
//! 🧠 Physics Engine Plugin for Bevy

use bevy::prelude::*;

pub mod components;
pub mod integration;
pub mod collision;
pub mod constraints;

/// 🎯 Plugin that registers all physics systems
///
/// Plugins are Bevy's way of packaging functionality.
/// Users add ONE line: `.add_plugins(PhysicsPlugin)`
/// and get everything they need.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // ⚙️ Resources (singleton data)
            .init_resource::<integration::PhysicsSettings>()
            .init_resource::<collision::SpatialGrid>()
            .init_resource::<constraints::ConstraintSettings>()
            
            // 📢 Events (one-frame communication)
            .add_event::<collision::CollisionEvent>()
            
            // 🔄 Startup systems (run once)
            .add_systems(Startup, setup_physics_debug)
            
            // 🔄 Main physics loop (in correct order!)
            .add_systems(Update, (
                // Phase 1: Input & Forces
                clear_forces,
                apply_gravity,
                apply_drag,
                
                // Phase 2: Integrate motion
                integration::euler_integration,
                
                // Phase 3: Detect collisions
                collision::rebuild_spatial_grid,
                collision::broad_phase,
                collision::narrow_phase,
                
                // Phase 4: Resolve collisions
                collision::resolve_collisions,
                
                // Phase 5: Apply constraints
                constraints::solve_constraints,
                
                // Phase 6: Sync to render
                sync_physics_to_render,
                
            ).chain())
            
            // 💡 `.chain()` ensures systems run in order!
            // Without it, Bevy runs systems in parallel (which
            // would cause race conditions in physics!)
            
            // 🎨 Debug visualization (optional)
            .add_systems(Update, debug_draw_colliders);
    }
}

/// 🔄 Sync physics positions to Bevy's Transform for rendering
fn sync_physics_to_render(
    mut query: Query<(&components::Position, &mut Transform)>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(pos.0.x, pos.0.y, 0.0);
    }
}

/// 🧹 Clear all force accumulators
fn clear_forces(mut query: Query<&mut components::ForceAccumulator>) {
    for mut forces in query.iter_mut() {
        forces.clear();
    }
}

/// 🌍 Apply gravity to all objects with mass
fn apply_gravity(
    mut query: Query<(&components::Mass, &mut components::ForceAccumulator)>,
    settings: Res<integration::PhysicsSettings>,
) {
    for (mass, mut forces) in query.iter_mut() {
        forces.apply(settings.gravity * mass.0);
    }
}

/// 🌬️ Apply damping/drag
fn apply_drag(
    mut query: Query<(
        &components::Velocity,
        &mut components::ForceAccumulator,
        Option<&components::LinearDamping>,
    )>,
) {
    for (vel, mut forces, damping) in query.iter_mut() {
        let damping = damping.map_or(0.01, |d| d.0);
        forces.apply(-vel.0 * damping);
    }
}
```

---

## 📦 Component Bundles

```rust
/// 🎯 Convenient bundles that spawn complete physics objects

/// 🌟 A dynamic physics body (moves, collides, responds)
#[derive(Bundle)]
pub struct DynamicBody {
    pub position: components::Position,
    pub velocity: components::Velocity,
    pub acceleration: components::Acceleration,
    pub mass: components::Mass,
    pub force_accumulator: components::ForceAccumulator,
    pub collider: components::Collider,
    pub collision_tag: components::CollisionTag,
    pub material: components::Material,
}

impl DynamicBody {
    /// 🆕 Create a new dynamic body
    pub fn circle(x: f32, y: f32, radius: f32, mass: f32) -> Self {
        Self {
            position: components::Position::new(x, y),
            velocity: components::Velocity::default(),
            acceleration: components::Acceleration::default(),
            mass: components::Mass(mass),
            force_accumulator: components::ForceAccumulator::default(),
            collider: components::Collider::Circle { radius },
            collision_tag: components::CollisionTag,
            material: components::Material::default(),
        }
    }
    
    /// 📦 Create a box-shaped dynamic body
    pub fn box_shape(x: f32, y: f32, w: f32, h: f32, mass: f32) -> Self {
        Self {
            position: components::Position::new(x, y),
            ..Self::circle(x, y, w.max(h) / 2.0, mass)
        }
    }
}

/// 🧱 A static physics body (doesn't move, but collides)
#[derive(Bundle)]
pub struct StaticBody {
    pub position: components::Position,
    pub collider: components::Collider,
    pub collision_tag: components::CollisionTag,
}

impl StaticBody {
    pub fn circle(x: f32, y: f32, radius: f32) -> Self {
        Self {
            position: components::Position::new(x, y),
            collider: components::Collider::Circle { radius },
            collision_tag: components::CollisionTag,
        }
    }
    
    pub fn wall(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            position: components::Position::new(x, y),
            collider: components::Collider::Aabb {
                half_width: w / 2.0,
                half_height: h / 2.0,
            },
            collision_tag: components::CollisionTag,
        }
    }
}
```

---

## 🔄 System Ordering & Schedules

```rust
/// ⏱️ Physics happens on a FIXED timestep, independent of frame rate
///
/// Schedule:
///
/// ┌────────────────────────────────────────────────────┐
/// │                    ONE FRAME                       │
/// ├────────────────────────────────────────────────────┤
/// │                                                    │
/// │  ┌──────────┐    ┌──────────┐    ┌──────────────┐ │
/// │  │ FIXED    │    │ VARIABLE │    │ RENDER       │ │
/// │  │ PHYSICS  │    │ UPDATE   │    │              │ │
/// │  │          │    │          │    │              │ │
/// │  │ 1. Forces│    │ Input    │    │ Draw sprites │ │
/// │  │ 2. Integ │───►│ AI       │───►│ Draw UI      │ │
/// │  │ 3. Coll  │    │ Anim     │    │ Present      │ │
/// │  │ 4. Resp  │    │ Audio    │    │              │ │
/// │  └──────────┘    └──────────┘    └──────────────┘ │
/// │       │                                               │
/// │  May run 0, 1, or multiple times per frame!           │
/// └────────────────────────────────────────────────────┘

/// 🎯 Define custom schedules for clean ordering
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum PhysicsSet {
    ForceAccumulation,
    Integration,
    CollisionDetection,
    CollisionResponse,
    ConstraintSolving,
    RenderSync,
}

fn configure_physics_schedule(app: &mut App) {
    app.configure_sets(Update,
        // ✅ Forces must come first
        PhysicsSet::ForceAccumulation,
        // 🔄 Then integrate
        PhysicsSet::Integration.after(PhysicsSet::ForceAccumulation),
        // 🧱 Then detect collisions
        PhysicsSet::CollisionDetection.after(PhysicsSet::Integration),
        // 🤝 Then resolve them
        PhysicsSet::CollisionResponse.after(PhysicsSet::CollisionDetection),
        // 🔗 Then constraints
        PhysicsSet::ConstraintSolving.after(PhysicsSet::CollisionResponse),
        // 🎨 Finally sync to render
        PhysicsSet::RenderSync.after(PhysicsSet::ConstraintSolving),
    );
}
```

---

## 📊 Query Patterns

```rust
/// 🎯 Common ECS query patterns for physics

// ─── 1. Read-only queries (fast, parallel) ───
fn read_positions(query: Query<&Position>) {
    for pos in query.iter() {
        // Just reading — can run in parallel! ⚡
    }
}

// ─── 2. Mutable queries (exclusive) ───
fn update_positions(mut query: Query<&mut Position>) {
    for mut pos in query.iter_mut() {
        // Writing — exclusive access 🚦
    }
}

// ─── 3. Filtered queries ───
fn dynamic_bodies(query: Query<(&Position, &Velocity), Without<Sleeping>>) {
    // Only awake objects! 😴
}

// ─── 4. Multi-component queries ───
fn physics_objects(query: Query<(
    &Position,
    &Velocity,
    &Mass,
    &Collider,
    Option<&Material>,  // Optional! ⭐
)>) {
    // All physics data in one query 🎯
}

// ─── 5. Change detection ───
fn on_position_changed(
    query: Query<&Transform, Changed<Transform>>,
) {
    // Only entities whose transform changed this frame 🎯
}

// ─── 6. Entity iteration with IDs ───
fn with_entity_ids(query: Query<(Entity, &Position, &Velocity)>) {
    for (entity, pos, vel) in query.iter() {
        // We can use `entity` to spawn/despawn/command! 📝
    }
}
```

---

## 🎮 Example: Complete Plugin Configuration

```rust
/// 🏗️ Full physics plugin with all systems configured

use bevy::prelude::*;
use crate::physics::*;

pub struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // ─── Core Physics ───
            .add_plugins(physics::PhysicsPlugin)
            
            // ─── Game Systems ───
            .add_systems(Startup, (
                spawn_player,
                spawn_walls,
                spawn_enemies,
            ))
            
            .add_systems(Update, (
                player_input,
                enemy_ai,
                camera_follow_player,
            ).after(
                physics::PhysicsPlugin  // Run AFTER physics!
            ))
            
            // ─── Debug ───
            .add_systems(Update, toggle_debug_mode.on_key_pressed(KeyCode::F3));
    }
}

/// 💡 Why game systems run AFTER physics:
/// 1. Physics runs (moves objects)
/// 2. Game logic reads the RESULTS (where is the player NOW?)
/// 3. Game logic writes commands (move AI toward player)
/// 4. Next frame: physics runs with updated commands
///
/// This prevents "same-frame" race conditions!
```

---

## 🎯 Chapter Summary

```rust
/// 📝 ECS Architecture principles:

// 🎯 SYSTEMS should:
// ✅ Read what they need (queries)
// ✅ Write what they own (mut queries)
// ✅ Run in the right order (.chain() / .before() / .after())
// ✅ Be stateless (all state in components/resources)

// 🎯 COMPONENTS should:
// ✅ Be pure data (no methods that modify other components)
// ✅ Be small (one concern per component)
// ✅ Be optional (use Option<T> in queries)

// 🎯 PLUGINS should:
// ✅ Register all systems, resources, events
// ✅ Configure system ordering
// ✅ Be self-contained (add_plugins and done)

// 🎯 The pipeline order:
// Forces → Integrate → Detect → Resolve → Constrain → Render
```

> **Key Takeaway:** ECS architecture is about **data-oriented design**. Components are pure data (Position, Velocity). Systems are pure logic (move, collide). Plugins package everything together. The magic is in BEVY'S SCHEDULING — `.chain()` ensures correct ordering, queries ensure efficient data access, and the whole thing runs in parallel automatically. 🏗️

---

**[← Previous: Spatial Partitioning](13-spatial-partitioning.md)** | **[Next: Mini Physics Sandbox →](15-physics-sandbox.md)**
