# 🏗️ Bevy ECS Physics Architecture

> **"The ECS pattern isn't just an architectural choice  -  it's a fundamental shift in how we THINK about game state. Components are data. Systems are transformations on that data. Entities are nothing but an ID. This separation is the key to building physics that scales."** 🧠

---

## 🎯 The Central Problem ECS Solves

Before we dive into ECS, let's understand the problem it solves. Consider the traditional **object-oriented** approach to a physics object:

```rust
// ❌ THE OO WAY (what we're trying to avoid)
struct PhysicsObject {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    mass: f32,
    collider: ColliderShape,
    is_static: bool,
    restitution: f32,
    friction: f32,
    sprite: Sprite,
    health: f32,
    is_player: bool,
    team: u8,
}

fn update_objects(objects: &mut Vec<PhysicsObject>) {
    for obj in objects.iter_mut() {
        // Every object carries ALL data, even unused fields ❌
        // Cache misses everywhere ❌
        // Hard to parallelize ❌
        // Adding a new field means changing the struct ❌
    }
}
```

This approach has **three fatal problems** for game physics:

### 🐢 Problem 1: Cache Inefficiency (The Killer)

Modern CPUs are **memory-bound**, not compute-bound. When you iterate `Vec<PhysicsObject>`, you load the ENTIRE struct into cache  -  even fields you don't need:

```
Memory Layout (OO):
+---------------------------------------------------------+
| pos | vel | acc | mass | collider | is_static | sprite | ... 
| pos | vel | acc | mass | collider | is_static | sprite | ... 
| pos | vel | acc | mass | collider | is_static | sprite | ... 
+----- Loading ALL of this just to update velocity!
     Most of it is wasted cache space! 💀
```

With **ECS**, data is stored in **contiguous arrays per component**:

```
Memory Layout (ECS):
Position:  | p1 | p2 | p3 | p4 | p5 | p6 | p7 | ... +---- Only positions!
Velocity:  | v1 | v2 | v3 | v4 | v5 | v6 | v7 | ... +---- Only velocities!
Mass:      | m1 | m2 | m3 | m4 | m5 | m6 | m7 | ... +---- Only masses!

When physics runs, it ONLY touches Velocity + Mass + Position.
NO wasted cache bandwidth on colliders, sprites, health, etc. ⚡
```

### 🔒 Problem 2: Parallelism Nightmare

In OO, `update_objects` has a `&mut Vec<PhysicsObject>`. You CANNOT safely run two threads modifying different objects because Rust's borrow checker can't prove they're different objects:

```rust
fn update_objects(objects: &mut Vec<PhysicsObject>) {
    // Can't split this into threads  -  Rust sees one mutable borrow
    for obj in objects.iter_mut() { /* ... */ }
}
```

With ECS, **Bevy's scheduler** proves at compile time that systems don't conflict:

```rust
// ✅ These can run in PARALLEL because they access DIFFERENT components
fn apply_gravity(mut query: Query<(&Mass, &mut Velocity)>) { /* ... */ }
fn render_sync(query: Query<(&Position, &mut Transform)>) { /* ... */ }

// ❌ These CANNOT  -  both write to Velocity
fn apply_gravity(mut query: Query<(&mut Velocity)>) { /* ... */ }
fn apply_drag(mut query: Query<(&mut Velocity)>) { /* ... */ }
```

### 🧩 Problem 3: Rigid Data Layout

In OO, adding a new behavior means adding a field to every object. In ECS, you just add a new component to specific entities:

```rust
// Want to add sleeping? Just add a component:
#[derive(Component)]
struct Sleeping;

// Now query ONLY awake objects:
fn update_awake_objects(query: Query<(&mut Velocity), Without<Sleeping>>) {
    // Only touches entities that DON'T have Sleeping
}

// Want to add health? Add it as a component to specific entities:
#[derive(Component)]
struct Health(f32);

// Player: commands.spawn((Position, Velocity, Health(100), Player))
// Wall:   commands.spawn((Position, StaticBody))
// Health NEVER touches the wall's iteration
```

---

## 🧱 The Three Pillars of ECS

Let's examine each pillar in depth:

### 1️⃣ Entities  -  Just IDs

An **Entity** in Bevy is nothing but a u64 (32-bit index + 32-bit generation):

```rust
// 🔢 Entity is literally just:
struct Entity {
    index: u32,      // Which slot in the storages
    generation: u32, // Prevents use-after-delete (generation counter)
}

// No data lives on the Entity itself!
// It's just a KEY to look up components
```

This means:
- Entity creation is **O(1)**  -  just increment a counter
- Entity deletion is **O(1)**  -  mark slot as free, bump generation
- Entity ID can be stored safely even if the entity is despawned (the generation prevents use-after-free)
- An entity is just a "bundle of components"  -  its identity comes from what components are attached

### 2️⃣ Components  -  Pure Data

A **Component** is any Rust type that implements the `Component` trait:

```rust
#[derive(Component)]
struct Position(Vec2);  // ✅ Just data, no methods

// Components should be:
// ✅ Small (one concern per component)
// ✅ Pure data (no logic methods)
// ✅ Copy when possible (enables better optimization)
// ✅ Densely packed (CPU cache friendly)

// COMPONENTS ARE NOT OBJECTS. They don't have methods.
// All logic lives in SYSTEMS.
```

Bevy stores components in **Archetypes**:

### 🔬 Deep Dive: How Bevy Stores Components

This is the most important implementation detail to understand:

```
World
+-- Archetype 1: [Position, Velocity, Mass]
|   +-- Entities: [Entity(1), Entity(4), Entity(7)]
|   +-- Position:  [p1, p4, p7]     <- Contiguous array! 
|   +-- Velocity:  [v1, v4, v7]     <- Same order!
|   +-- Mass:      [m1, m4, m7]
|
+-- Archetype 2: [Position, Velocity, Mass, Collider]
|   +-- Entities: [Entity(2), Entity(5)]
|   +-- Position:  [p2, p5]
|   +-- Velocity:  [v2, v5]
|   +-- Mass:      [m2, m5]
|   +-- Collider:  [c2, c5]
|
+-- Archetype 3: [Position, Sprite]  <- No Velocity! (static objects)
    +-- Entities: [Entity(3), Entity(6)]
    +-- Position:  [p3, p6]
    +-- Sprite:    [s3, s6]
```

**Archetype** = a unique combination of component types. Every entity belongs to exactly one archetype. When you add or remove a component, the entity **moves** to a different archetype (this is called an "archetype move").

Why this matters for YOU as a physics programmer:

```rust
// ✅ FAST: Query iterates over CONTIGUOUS memory
fn move_objects(query: Query<(&mut Position, &Velocity)>) {
    // Position and Velocity arrays are traversed in lockstep
    // CPU prefetcher loves this! 🏎️
    for (mut pos, vel) in query.iter_mut() {
        pos.0 += vel.0;
    }
}

// ❌ If you had Position and Velocity on DIFFERENT archetypes...
// Bevy would still iterate them efficiently per archetype,
// but there would be a boundary cost between archetype chunks.
// This is usually negligible (< 5% overhead).
```

### 3️⃣ Systems  -  Pure Logic

A **System** is a function that operates on components:

```rust
// A Bevy system is literally just:
// fn(Params) where Params: SystemParam

// Examples of system parameters:
fn my_system(
    // 🔍 Query: read/write components matching a pattern
    query: Query<(&mut Position, &Velocity)>,
    
    // 📦 Resource: singleton data (not per-entity)
    settings: Res<PhysicsSettings>,
    
    // 📢 Event reader: read events from this frame
    mut events: EventReader<CollisionEvent>,
    
    // 📝 Commands: spawn/despawn/modify entities (deferred)
    mut commands: Commands,
    
    // ⏱️ Time: frame timing info
    time: Res<Time>,
) {
    // System body runs every frame (or when scheduled)
}
```

**Critical insight:** Systems are **stateless**  -  they read state from components/resources and write state to components/resources. This is what enables Bevy's scheduler to reorder and parallelize them safely.

---

## ⚡ System Scheduling: The Heart of Bevy

Bevy doesn't run systems in a fixed order. It has a **scheduler** that:

1. Builds a **dependency graph** from your `.before()`, `.after()`, and `.chain()` annotations
2. Groups **compatible** systems (no conflicting accesses) into parallel sets
3. Runs each set in parallel, then moves to the next

```rust
fn configure_scheduling(app: &mut App) {
    app.add_systems(Update, (
        system_a,  // Reads Position, writes Velocity
        system_b,  // Reads Velocity, writes Transform
        system_c,  // Reads Transform, writes Sprite
        system_d,  // Reads Position, reads Velocity  -  compatible with both!
    ));
}
```

Without ordering, Bevy analyzes component access:

```
System      Reads        Writes       Can run with...
-------------------------------------------------------
system_a    Position     Velocity     -
system_b    Velocity     Transform    system_d ✅ (no overlap)
system_c    Transform    Sprite       system_d ✅
system_d    Position,    (none)       system_b ✅, system_c ✅
             Velocity
```

Bevy's scheduler produces:

```
Frame -> [system_a] -> [system_b, system_d] -> [system_c] -> render
           (alone)       (parallel!)              (alone)
```

### System Ordering Rules for Physics

Physics has **strict ordering requirements** that MUST be enforced:

```rust
// ❌ WRONG: No ordering  -  Bevy may run integrate BEFORE apply_gravity
// If integrate runs first, gravity has no effect this frame!
app.add_systems(Update, (apply_gravity, integrate, detect_collisions));

// ✅ CORRECT: Forces -> Integrate -> Detect -> Resolve
app.add_systems(Update, (
    clear_forces,
    apply_gravity,
    integrate,
    detect_collisions,
    resolve_collisions,
    sync_to_render,
).chain());  // 👈 Runs in EXACT order, one after another
```

`.chain()` is shorthand for:
```rust
clear_forces
    .chain(apply_gravity)
    .chain(integrate)
    .chain(detect_collisions)
    .chain(resolve_collisions)
    .chain(sync_to_render)
```

Which is equivalent to:
```rust
clear_forces.after(apply_gravity)
apply_gravity.after(integrate)
// ... etc
```

### System Sets for Clean Organization

When you have dozens of systems, individual `.after()` calls get messy. Use **System Sets** to group them:

```rust
// 🎯 Define logical groups
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum PhysicsSet {
    ForceAccumulation,  // Phase 1
    Integration,        // Phase 2  
    CollisionDetection, // Phase 3
    CollisionResponse,  // Phase 4
}

// 📝 Register systems into sets
app.add_systems(Update, (
    clear_forces.in_set(PhysicsSet::ForceAccumulation),
    apply_gravity.in_set(PhysicsSet::ForceAccumulation),
    apply_drag.in_set(PhysicsSet::ForceAccumulation),
    
    euler_integration.in_set(PhysicsSet::Integration),
    verlet_integration.in_set(PhysicsSet::Integration),
    
    broad_phase.in_set(PhysicsSet::CollisionDetection),
    narrow_phase.in_set(PhysicsSet::CollisionDetection),
    
    resolve_collisions.in_set(PhysicsSet::CollisionResponse),
    apply_friction.in_set(PhysicsSet::CollisionResponse),
));

// 📐 Configure set ordering (MUCH cleaner!)
app.configure_sets(Update,
    PhysicsSet::ForceAccumulation,
    PhysicsSet::Integration.after(PhysicsSet::ForceAccumulation),
    PhysicsSet::CollisionDetection.after(PhysicsSet::Integration),
    PhysicsSet::CollisionResponse.after(PhysicsSet::CollisionDetection),
);
```

### Run Conditions

Some systems should only run conditionally:

```rust
fn pause_physics(mut physics_time: ResMut<PhysicsTime>) {
    physics_time.paused = !physics_time.paused;
}

// ⏯️ Only run physics when not paused
fn physics_system(
    query: Query<...>,
    physics_time: Res<PhysicsTime>,
) {
    if physics_time.paused { return; }  // ❌ Runs system but exits immediately
    // ... physics logic
}

// ✅ BETTER: Don't run the system at ALL when paused
app.add_systems(Update, physics_system.run_if(not(resource_equals(PhysicsTime::paused))));

// ✅ EVEN BETTER: Run only when there are entities to process
app.add_systems(Update, collision_detection.run_if(any_with_component::<Collider>));
```

---

## 📊 Queries: The Art of Component Access

Queries are how systems access component data. Understanding their full power is essential for writing efficient physics:

### Basic Query Patterns

```rust
// --- Read multiple components ---
fn read(query: Query<(&Position, &Velocity, &Mass)>) {
    // All three must exist on the entity
    for (pos, vel, mass) in query.iter() { /* ... */ }
}

// --- Write one, read others ---
fn write(query: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in query.iter_mut() { /* ... */ }
}

// --- Optional components ---
fn with_optional(
    query: Query<(
        &Position,
        &Velocity,
        Option<&Mass>,  // 👈 Entity may or may not have Mass
    )>,
) {
    for (pos, vel, mass) in query.iter() {
        let mass = mass.map_or(1.0, |m| m.0); // Default if missing
    }
}
```

### Query Filters

Filters let you include or exclude entities based on component presence:

```rust
// --- With: Only include entities that ALSO have this component ---
fn only_players(query: Query<&Transform, With<Player>>) {
    // Only entities that have BOTH Transform AND Player
}

// --- Without: Exclude entities that have this component ---
fn only_non_sleeping(query: Query<&mut Velocity, Without<Sleeping>>) {
    // Skip sleeping entities  -  they don't need physics updates
}

// --- Or/And/Nor: Combine filters ---
fn complex_filter(
    query: Query<
        &Position,
        (
            Or<(With<Player>, With<Enemy>)>,  // Player OR Enemy
            Without<Sleeping>,                // AND not sleeping
        ),
    >,
) {
    // Players and enemies that are awake
}
```

### Change Detection

Bevy tracks whether components have been modified:

```rust
// --- Only entities whose Position changed this frame ---
fn react_to_movement(query: Query<&Transform, Changed<Transform>>) {
    for tf in query.iter() {
        // Trigger something when transform changes
    }
}

// --- Only entities where Velocity was JUST added ---
fn on_spawn(query: Query<&Velocity, Added<Velocity>>) {
    for vel in query.iter() {
        // This runs exactly ONCE when Velocity is first added
        // Perfect for initialization
    }
}

// --- Removed detection ---
fn on_collider_removed(
    mut removals: RemovedComponents<Collider>,
    // ...
) {
    for entity in removals.read() {
        // Handle entities that lost their collider
    }
}
```

### Performance Considerations

```rust
// ⚡ FAST: Dense component access (contiguous memory)
fn fast(query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in query.iter_mut() {
        pos.0 += vel.0;
    }
}

// 🐢 SLOWER: Sparse access  -  but still fast enough for most uses
fn single(query: Query<(&mut Position, &Velocity)>) {
    if let Some((mut pos, vel)) = query.iter_mut().next() {
        pos.0 += vel.0;
    }
}

// 🐌 SLOWEST: Random access by entity  -  USE SPARSE
fn random_access(query: Query<&mut Position>) {
    for entity_id in some_list_of_ids {
        if let Ok(mut pos) = query.get_mut(entity_id) {
            pos.0 += 10.0;
        }
    }
}
// 💡 Prefer .iter_mut() over .get_mut() whenever possible
```

---

## 📦 Resources vs Events vs Components

Understanding when to use each is critical:

| Feature | When to Use | Example | Lifetime |
|---------|------------|---------|----------|
| **Component** | Per-entity data | Position, Velocity, Mass | Entity lifetime |
| **Resource** | Singleton global data | PhysicsSettings, Gravity | App lifetime |
| **Event** | One-frame communication | CollisionEvent, SpawnCommand | One frame |

### Resources are NOT global variables!

Resources are **typed singletons** managed by Bevy's World. They participate in the scheduling system:

```rust
// ✅ CORRECT: Resources are injected like any other param
fn physics_system(
    settings: Res<PhysicsSettings>,          // Read-only access
    mut time: ResMut<PhysicsTime>,           // Mutable access
    // Cannot have Res<PhysicsTime> AND ResMut<PhysicsTime> in same system!
) {
    // ...
}

// A resource can only have ONE system writing at any time
// This prevents race conditions AT COMPILE TIME
```

### Events are FRAME-LOCAL

Events are stored per-frame and automatically cleared:

```rust
// 📢 Event emission
fn collision_detection(
    mut events: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider, &Position)>,
) {
    let pairs = find_collisions(&query);
    for (a, b, normal) in pairs {
        events.send(CollisionEvent { entity_a: a, entity_b: b, normal });
    }
}

// 📢 Event consumption (MUST consume in same frame!)
fn collision_response(
    mut events: EventReader<CollisionEvent>,
    // Events are automatically drained at the start of each frame
) {
    for event in events.read() {
        // Handle collision
    }
    // If you DON'T read them, they're LOST
}
```

### Commands are DEFERRED

`Commands` don't execute immediately. They buffer operations and apply them at a **sync point**:

```rust
fn spawn_objects(mut commands: Commands) {
    // These don't execute NOW  -  they go into a buffer
    let entity = commands.spawn((
        Position::new(0.0, 0.0),
        Velocity::new(10.0, 0.0),
    )).id();
    
    // The entity doesn't exist yet in this frame!!
    // You can't query it until next frame
    
    // But you CAN store the ID for later:
    commands.insert_resource(PlayerEntity(entity));
}

// 💡 Why deferred?
// Immediate commands would force a sync point  -  stopping ALL
// parallel execution to process the changes. Batching them
// means the scheduler stays efficient.
```

---

## 🏗️ Plugins: Packaging Your Physics Engine

A **Plugin** packages systems, resources, events, and initialization into a reusable unit:

```rust
/// 🎯 The complete physics plugin
pub struct PhysicsPlugin {
    /// Configuration
    pub gravity: Vec2,
    pub fixed_dt: f32,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // 📝 Register resources
            .insert_resource(PhysicsSettings {
                gravity: self.gravity,
                fixed_dt: self.fixed_dt,
            })
            .init_resource::<SpatialGrid>()
            
            // 📢 Register events
            .add_event::<CollisionEvent>()
            
            // 🏗️ Startup systems
            .add_systems(Startup, init_physics_debug)
            
            // 🔄 Update systems (ordered!)
            .add_systems(Update, (
                clear_forces,
                apply_gravity,
                apply_drag,
                integrate_positions,
                broad_phase,
                narrow_phase,
                resolve_collisions,
                sync_to_render,
            ).chain())
            
            // 🎨 Optional debug
            .add_systems(Update, debug_draw.run_if(resource_exists::<DebugMode>));
    }
}

// Usage: Just one line!
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin {
            gravity: Vec2::new(0.0, -9.81),
            fixed_dt: 1.0 / 60.0,
        })
        .run();
}
```

### Plugin Composition

Complex games compose multiple plugins:

```rust
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // --- Core systems ---
            .add_plugins(PhysicsPlugin::default())
            .add_plugins(RenderPlugin::default())
            
            // --- Game-specific ---
            .add_plugins(PlayerPlugin)
            .add_plugins(AiPlugin)
            .add_plugins(WeaponPlugin)
            
            // --- Game systems run AFTER physics ---
            .add_systems(Update, (
                player_input,
                enemy_ai,
                camera_follow,
            ).after(PhysicsPlugin));
    }
}
```

---

## 🔄 The Full App Lifecycle

Understanding when things happen is crucial:

```
App::new()
  |
  +-- .add_plugins(...)          <- Register plugins (NO execution yet)
  +-- .add_systems(Startup, ...) <- Register startup systems
  +-- .add_systems(Update, ...)  <- Register per-frame systems
  |
  +-- .run()                     <- ENTER THE MAIN LOOP
        |
        +-- 🏗️  Startup Phase --> Runs ALL Startup systems (once)
        |     +-- setup_camera()
        |     +-- spawn_initial_objects()
        |     +-- init_resources()
        |
        +-- 🔄  Main Loop (every frame)
              |
              +-- 📥  Event Readers advance (drain previous frame)
              +-- 🔄  Scheduler: Determine parallel execution plan
              +-- ⚡  Run Update systems in scheduled order
              |     +-- [parallel group] physics systems
              |     +-- SYNC POINT (apply Commands)
              |     +-- [parallel group] game systems
              |     +-- SYNC POINT (apply Commands)
              |
              +-- 🎨  Render (separate schedule: Render)
              |     +-- Bevy's renderer reads Transform components
              |
              +-- 🔁  Next frame...
```

### Sync Points

**Sync points** are where deferred operations (Commands) are executed. They're AUTOMATICALLY inserted when the scheduler detects conflicting system pairs:

```rust
fn spawn_bomb(mut commands: Commands) { commands.spawn(Bomb); }  // Writes
fn read_bombs(query: Query<&Bomb>) { /* ... */ }                  // Reads

// Scheduler sees: spawn writes Bomb, read reads Bomb
// -> Forces a SYNC POINT between them
// -> ALL parallel execution stops, commands are drained
// -> Then read_bombs can see the new Bomb

// 💡 Sync points are expensive! Minimize them by:
// 1. Batching spawn/despawn
// 2. Using events instead of immediate commands
// 3. Grouping write operations together
```

---

## 🎯 Putting It All Together: A Complete Physics Frame

Here's what happens in one physics frame, traced end to end:

```
📅 FRAME N
|
+-- 1. EVENT ADVANCE
|   +-- CollisionEvent reader is cleared
|   +-- Previous frame's events are available
|
+-- 2. PHYSICS SYSTEMS RUN (in order)
|   |
|   +-- clear_forces()
|   |   +-- Query: &mut ForceAccumulator
|   |   +-- All forces reset to zero
|   |
|   +-- apply_gravity()
|   |   +-- Query: &Mass, &mut ForceAccumulator
|   |   +-- Res: PhysicsSettings.gravity
|   |   +-- Each entity: forces += gravity × mass
|   |
|   +-- apply_drag()
|   |   +-- Query: &Velocity, &mut ForceAccumulator
|   |   +-- Each entity: forces -= velocity × damping
|   |
|   +-- integrate()
|   |   +-- Query: &ForceAccumulator, &Mass, &mut Acceleration, &mut Velocity, &mut Position
|   |   +-- a = F/m, v += a·dt, x += v·dt
|   |
|   +-- broad_phase()
|   |   +-- Build spatial grid from &Position
|   |   +-- Emit candidate pairs
|   |
|   +-- narrow_phase()
|   |   +-- Query: &Position, &Collider
|   |   +-- Check actual overlap for each pair
|   |   +-- Emit CollisionEvent
|   |
|   +-- resolve_collisions()
|   |   +-- Read CollisionEvent
|   |   +-- Query: &mut Position, &mut Velocity, &Mass
|   |   +-- Apply impulses, separate overlapping objects
|   |
|   +-- sync_to_render()
|       +-- Query: &Position, &mut Transform
|       +-- transform.translation = position
|
+-- 3. SYNC POINT (Commands execute)
|   +-- Despawned entities removed
|   +-- New entities added
|   +-- Components added/removed processed
|   +-- Archetype moves happen
|
+-- 4. GAME SYSTEMS RUN
|   +-- player_input()      <- Reads keyboard, writes Velocity
|   +-- enemy_ai()          <- Reads Position, writes Velocity
|   +-- camera_follow()     <- Reads Position, writes Transform
|
+-- 5. SECOND SYNC POINT
|
+-- 6. RENDER
    +-- Bevy draws using Transform & Sprite components
```

---

## 🎯 Chapter Summary

```
ECS Architecture is NOT optional  -  it's the FOUNDATION of scalable physics:

🎯 ENTITIES = IDs (just a number)
   - Entity(1) ≠ Entity(2) even if same components
   - Generation counter prevents use-after-free

📦 COMPONENTS = Data (plain structs)
   - Position, Velocity, Mass, Collider
   - Stored in CONTIGUOUS arrays per archetype
   - CPU cache loves this 🏎️

⚡ SYSTEMS = Logic (pure functions)
   - Queries read/write components
   - Scheduler proves safety at compile time
   - Parallel execution when accesses don't conflict

🎨 SCHEDULING = Dependency management
   - .chain() for strict ordering
   - .in_set() for logical grouping
   - .run_if() for conditional execution
   - Sync Points for deferred operations

🏆 KEY INSIGHT: Bevy's ECS isn't about "organizing code."
It's about MAXIMIZING THROUGHPUT through cache-friendly
data layouts and compile-time parallelism guarantees.
```

> **The ECS architecture lets you write physics code that's simultaneously correct (compile-time access checks), fast (cache-friendly iteration), and parallel (non-conflicting systems run simultaneously). No other pattern gives you all three.** 🏗️

> 💡 **Full source code for this chapter:** [code-examples/ch14-ecs-architecture/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch14-ecs-architecture)
> 
> The runnable project includes Cargo.toml, main.rs, and complete module files.

---

**[<- Previous: Spatial Partitioning](ch13-spatial-partitioning.md)** | **[Next: Mini Physics Sandbox ->](ch15-physics-sandbox.md)**
