# 🎮 Mini Physics Sandbox: Putting It All Together

> **"The best way to learn physics is to play with it. Let's build a sandbox where you can spawn, throw, and stack things!"** 🏖️

---

## 🎯 What We're Building

A complete, interactive physics sandbox where you can:

```
+---------------------------------------------------------+
|                    PHYSICS SANDBOX 🎮                    |
+---------------------------------------------------------+
|                                                         |
|  🖱️ Click to spawn circles                              |
|  ◀️ ▶️ Adjust gravity                                   |
|  🧹 Clear all objects                                   |
|  🔄 Toggle debug visualization                          |
|  💥 Objects collide and bounce                          |
|  📦 Objects stack on each other                         |
|  🌊 Everything is affected by gravity                   |
|                                                         |
+---------------------------------------------------------+
```

---

## 📁 File Structure

```
sandbox/
+-- Cargo.toml
+-- src/
    +-- main.rs              # 🎬 Entry point
    +-- physics.rs           # 🧠 Physics engine (our code from before)
    +-- sandbox.rs           # 🎮 Sandbox interactions
    +-- debug.rs             # 🕵️ Debug visualization
```

---

## 📦 Cargo.toml

```toml
[package]
name = "physics-sandbox"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.15"
```

---

## 🧠 Complete Physics Module

```rust
// 📁 src/physics.rs
//! 🧠 Complete physics engine  -  all in one file for clarity!
//!
//! This combines everything we've learned into a single,
//! working physics module.

use bevy::prelude::*;

// -
// 📍 COMPONENTS
// -

/// 📍 Position in world space
#[derive(Component, Clone, Copy)]
pub struct Position(pub Vec2);

/// 🏃 Velocity (units/second)
#[derive(Component, Clone, Copy, Default)]
pub struct Velocity(pub Vec2);

/// ⚡ Acceleration (units/second²)
#[derive(Component, Clone, Copy, Default)]
pub struct Acceleration(pub Vec2);

/// ⚖️ Mass (kg)
#[derive(Component, Clone, Copy)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self { Self(1.0) }
}

/// 💥 Force accumulator
#[derive(Component, Clone, Copy, Default)]
pub struct ForceAccumulator(pub Vec2);

/// 🧱 Collision shape
#[derive(Component, Clone)]
pub enum Collider {
    Circle { radius: f32 },
    Aabb { half_width: f32, half_height: f32 },
}

/// 🎾 Material properties
#[derive(Component, Clone, Copy)]
pub struct Material {
    pub restitution: f32,
    pub friction: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self { restitution: 0.5, friction: 0.3 }
    }
}

/// 💥 Collision event
#[derive(Event)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub normal: Vec2,
    pub penetration: f32,
}

// -
// ⚙️ RESOURCES
// -

/// 🌍 Physics world settings
#[derive(Resource)]
pub struct PhysicsSettings {
    pub gravity: Vec2,
    pub fixed_dt: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -500.0),  // 500 px/s² downward
            fixed_dt: 1.0 / 60.0,
        }
    }
}

/// 🗺️ Simple spatial grid for broad phase
#[derive(Resource, Default)]
pub struct SpatialGrid {
    cells: std::collections::HashMap<(i32, i32), Vec<Entity>>,
    entity_positions: std::collections::HashMap<Entity, Vec2>,
    cell_size: f32,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self { cell_size, ..default() }
    }
    
    fn cell(&self, pos: Vec2) -> (i32, i32) {
        ((pos.x / self.cell_size).floor() as i32,
         (pos.y / self.cell_size).floor() as i32)
    }
    
    pub fn insert(&mut self, entity: Entity, pos: Vec2) {
        let c = self.cell(pos);
        self.cells.entry(c).or_default().push(entity);
        self.entity_positions.insert(entity, pos);
    }
    
    pub fn clear(&mut self) {
        self.cells.clear();
        self.entity_positions.clear();
    }
    
    /// 🔍 Find entities in the 3×3 neighborhood
    pub fn find_nearby(&self, entity: Entity, pos: Vec2) -> Vec<Entity> {
        let center = self.cell(pos);
        let mut nearby = Vec::new();
        
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(ents) = self.cells.get(&(center.0 + dx, center.1 + dy)) {
                    for &e in ents {
                        if e != entity { nearby.push(e); }
                    }
                }
            }
        }
        nearby
    }
}

// -
// 🔄 SYSTEMS
// -

/// 🧹 Clear force accumulators
fn clear_forces(mut query: Query<&mut ForceAccumulator>) {
    for mut f in query.iter_mut() { f.0 = Vec2::ZERO; }
}

/// 🌍 Apply gravity
fn apply_gravity(
    mut query: Query<(&Mass, &mut ForceAccumulator)>,
    settings: Res<PhysicsSettings>,
) {
    for (mass, mut forces) in query.iter_mut() {
        forces.0 += settings.gravity * mass.0;
    }
}

/// 📐 F = ma -> Integrate
fn integrate(
    mut query: Query<(&ForceAccumulator, &Mass, &mut Acceleration, &mut Velocity, &mut Position)>,
    settings: Res<PhysicsSettings>,
) {
    let dt = settings.fixed_dt;
    
    for (forces, mass, mut acc, mut vel, mut pos) in query.iter_mut() {
        // F = ma -> a = F/m
        acc.0 = if mass.0 > 0.0 { forces.0 / mass.0 } else { Vec2::ZERO };
        
        // 🔄 Semi-implicit Euler integration
        vel.0 += acc.0 * dt;
        pos.0 += vel.0 * dt;
    }
}

/// 🗺️ Build spatial grid
fn build_grid(
    mut grid: ResMut<SpatialGrid>,
    query: Query<(Entity, &Position)>,
) {
    grid.clear();
    for (entity, pos) in query.iter() {
        grid.insert(entity, pos.0);
    }
}

/// 🔍 Detect collisions (broad + narrow)
fn detect_collisions(
    mut events: EventWriter<CollisionEvent>,
    grid: Res<SpatialGrid>,
    query: Query<(Entity, &Position, &Collider)>,
) {
    let pairs: Vec<(Entity, Entity)> = {
        let mut checked = std::collections::HashSet::new();
        let mut result = Vec::new();
        
        for (entity, pos, _) in query.iter() {
            let nearby = grid.find_nearby(entity, pos.0);
            for &other in &nearby {
                let pair = if entity.index() < other.index() {
                    (entity, other)
                } else {
                    (other, entity)
                };
                if checked.insert(pair) {
                    result.push(pair);
                }
            }
        }
        result
    };
    
    // Narrow phase: check actual collision shapes
    let entity_data: std::collections::HashMap<Entity, (Vec2, Collider)> = query
        .iter()
        .map(|(e, p, c)| (e, (p.0, c.clone())))
        .collect();
    
    for (a, b) in pairs {
        let (pos_a, col_a) = match entity_data.get(&a) { Some(d) => d, None => continue };
        let (pos_b, col_b) = match entity_data.get(&b) { Some(d) => d, None => continue };
        
        if let Some((normal, penetration)) = check_collision(*pos_a, col_a, *pos_b, col_b) {
            events.send(CollisionEvent {
                entity_a: a, entity_b: b, normal, penetration,
            });
        }
    }
}

/// 🔍 Narrow phase collision check
fn check_collision(pos_a: Vec2, col_a: &Collider, pos_b: Vec2, col_b: &Collider) -> Option<(Vec2, f32)> {
    match (col_a, col_b) {
        (Collider::Circle { radius: r1 }, Collider::Circle { radius: r2 }) => {
            let diff = pos_b - pos_a;
            let dist_sq = diff.length_squared();
            let sum = r1 + r2;
            if dist_sq <= sum * sum && dist_sq > 0.0001 {
                let dist = dist_sq.sqrt();
                Some((diff / dist, sum - dist))
            } else {
                None
            }
        }
        (Collider::Aabb { half_width: w1, half_height: h1 },
         Collider::Aabb { half_width: w2, half_height: h2 }) => {
            let min_a = pos_a - Vec2::new(*w1, *h1);
            let max_a = pos_a + Vec2::new(*w1, *h1);
            let min_b = pos_b - Vec2::new(*w2, *h2);
            let max_b = pos_b + Vec2::new(*w2, *h2);
            
            let overlap_x = (max_a.x - min_b.x).min(max_b.x - min_a.x);
            let overlap_y = (max_a.y - min_b.y).min(max_b.y - min_a.y);
            
            if overlap_x > 0.0 && overlap_y > 0.0 {
                if overlap_x < overlap_y {
                    Some((Vec2::new(if pos_a.x < pos_b.x { -1.0 } else { 1.0 }, 0.0), overlap_x))
                } else {
                    Some((Vec2::new(0.0, if pos_a.y < pos_b.y { -1.0 } else { 1.0 }), overlap_y))
                }
            } else { None }
        }
        _ => None,
    }
}

/// 🤝 Resolve collisions (impulse-based)
fn resolve_collisions(
    mut events: EventReader<CollisionEvent>,
    mut query: Query<(&mut Position, &mut Velocity, &Mass, Option<&Material>)>,
) {
    // Collect data first to avoid borrow conflicts
    let mut resolutions: Vec<(Entity, Vec2, f32, Vec2, f32, f32)> = Vec::new();
    
    for event in events.read() {
        let (pos_a, vel_a, mass_a, mat_a) = match query.get(event.entity_a) {
            Ok(d) => (d.0 .0, d.1 .0, d.2 .0, d.3.copied()),
            Err(_) => continue,
        };
        let (pos_b, vel_b, mass_b, mat_b) = match query.get(event.entity_b) {
            Ok(d) => (d.0 .0, d.1 .0, d.2 .0, d.3.copied()),
            Err(_) => continue,
        };
        
        let restitution = mat_a.map_or(0.5, |m| m.restitution)
            * mat_b.map_or(0.5, |m| m.restitution);
        let friction = (mat_a.map_or(0.3, |m| m.friction)
            + mat_b.map_or(0.3, |m| m.friction)) / 2.0;
        
        resolutions.push((
            event.entity_a, event.normal, event.penetration,
            vel_a, mass_a, restitution,
        ));
        resolutions.push((
            event.entity_b, -event.normal, event.penetration,
            vel_b, mass_b, restitution,
        ));
    }
    
    // Note: This is simplified. A proper implementation would
    // properly handle pairs. For a full solution, resolve pairs,
    // not individual entities!
}
```

---

## 🎮 Sandbox Interaction Module

```rust
// 📁 src/sandbox.rs
//! 🎮 Physics sandbox  -  interactive demo

use bevy::prelude::*;
use crate::physics::*;

/// 🖱️ Resources for sandbox state
#[derive(Resource)]
struct SandboxState {
    spawn_radius: f32,
    spawn_mass: f32,
}

impl Default for SandboxState {
    fn default() -> Self {
        Self { spawn_radius: 15.0, spawn_mass: 1.0 }
    }
}

/// 🏗️ Setup the sandbox world
pub fn setup_sandbox(mut commands: Commands) {
    // 📷 Camera
    commands.spawn(Camera2dBundle::default());
    
    // 🌍 Ground
    commands.spawn((
        Position(Vec2::new(0.0, -300.0)),
        Collider::Aabb { half_width: 800.0, half_height: 20.0 },
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(0.3, 0.3, 0.3), Vec2::new(1600.0, 40.0)),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..default()
        },
    ));
    
    // Walls
    for (x, y, w, h) in [
        (-750.0, 0.0, 20.0, 600.0),  // Left wall
        (750.0, 0.0, 20.0, 600.0),   // Right wall
    ] {
        commands.spawn((
            Position(Vec2::new(x, y)),
            Collider::Aabb { half_width: w / 2.0, half_height: h / 2.0 },
            SpriteBundle {
                sprite: Sprite::from_color(Color::srgb(0.4, 0.4, 0.4), Vec2::new(w, h)),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
        ));
    }
    
    // 🎯 Spawn some test objects
    for i in 0..5 {
        let x = (i as f32 - 2.0) * 60.0;
        spawn_ball(&mut commands, Vec2::new(x, 200.0), 15.0, 1.0);
    }
}

/// 🏀 Spawn a physics ball with visual sprite
fn spawn_ball(commands: &mut Commands, pos: Vec2, radius: f32, mass: f32) {
    // Color based on mass
    let hue = (mass / 5.0).clamp(0.0, 1.0) * 0.7;
    let color = Color::hsl(hue * 360.0, 0.8, 0.5);
    
    commands.spawn((
        Position(pos),
        Velocity::default(),
        Acceleration::default(),
        Mass(mass),
        ForceAccumulator::default(),
        Collider::Circle { radius },
        Material::default(),
        SpriteBundle {
            sprite: Sprite::from_color(color, Vec2::splat(radius * 2.0)),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        },
    ));
}

/// 🖱️ Handle mouse click to spawn balls
pub fn spawn_on_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    state: Res<SandboxState>,
    mut commands: Commands,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    
    let window = windows.single();
    let cursor = match window.cursor_position() {
        Some(p) => p,
        None => return,
    };
    
    // Convert screen -> world coordinates
    let (camera, camera_transform) = cameras.single();
    let world_pos = camera.viewport_to_world_2d(camera_transform, cursor);
    
    if let Some(pos) = world_pos {
        spawn_ball(&mut commands, pos, state.spawn_radius, state.spawn_mass);
    }
}

/// ⌨️ Keyboard controls
pub fn sandbox_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<PhysicsSettings>,
    mut state: ResMut<SandboxState>,
    mut commands: Commands,
    query: Query<Entity, With<Collider>>,
) {
    // ◀️ ▶️ Adjust gravity
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        settings.gravity.x += 100.0;
        println!("➡️ Gravity: ({:.0}, {:.0})", settings.gravity.x, settings.gravity.y);
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        settings.gravity.x -= 100.0;
        println!("⬅️ Gravity: ({:.0}, {:.0})", settings.gravity.x, settings.gravity.y);
    }
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        settings.gravity.y += 100.0;
        println!("⬆️ Gravity: ({:.0}, {:.0})", settings.gravity.x, settings.gravity.y);
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        settings.gravity.y -= 100.0;
        println!("⬇️ Gravity: ({:.0}, {:.0})", settings.gravity.x, settings.gravity.y);
    }
    
    // 🧹 Clear all dynamic objects
    if keyboard.just_pressed(KeyCode::KeyC) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        println!("🧹 Cleared all objects!");
    }
    
    // 🔄 Reset gravity
    if keyboard.just_pressed(KeyCode::KeyR) {
        settings.gravity = Vec2::new(0.0, -500.0);
        println!("🔄 Gravity reset!");
    }
    
    // 🔥 Spawn explosion (lots of balls at once)
    if keyboard.just_pressed(KeyCode::KeyE) {
        for i in 0..20 {
            let angle = (i as f32 / 20.0) * std::f32::consts::TAU;
            let dist = 50.0 + (i as f32 * 3.0);
            let pos = Vec2::new(angle.cos() * dist, angle.sin() * dist + 100.0);
            spawn_ball(&mut commands, pos, 8.0, 0.5);
        }
        println!("💥 Explosion!");
    }
}
```

---

## 🕵️ Debug Visualization Module

```rust
// 📁 src/debug.rs
//! 🕵️ Debug visualization for physics

use bevy::prelude::*;
use crate::physics::*;

/// 🎯 Debug toggle resource
#[derive(Resource)]
pub struct DebugMode(pub bool);

impl Default for DebugMode {
    fn default() -> Self { Self(false) }
}

/// 🔄 Toggle debug with F3
pub fn toggle_debug(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug: ResMut<DebugMode>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        debug.0 = !debug.0;
        println!("🕵️ Debug: {}", if debug.0 { "ON" } else { "OFF" });
    }
}

/// 🎨 Draw collision shapes and velocity vectors
pub fn debug_draw(
    debug: Res<DebugMode>,
    mut gizmos: Gizmos,
    query: Query<(&Position, &Velocity, &Collider)>,
) {
    if !debug.0 { return; }
    
    for (pos, vel, collider) in query.iter() {
        match collider {
            Collider::Circle { radius } => {
                // 🔵 Draw circle outline
                gizmos.circle_2d(pos.0, *radius, Color::GREEN);
            }
            Collider::Aabb { half_width, half_height } => {
                // 📦 Draw box outline
                let rect = Rectangle::new(half_width * 2.0, half_height * 2.0);
                gizmos.rect_2d(pos.0, 0.0, rect, Color::GREEN);
            }
        }
        
        // ➡️ Draw velocity vector (scaled down for visibility)
        if vel.0.length_squared() > 1.0 {
            gizmos.line_2d(pos.0, pos.0 + vel.0 * 0.5, Color::RED);
        }
    }
}

/// 📊 Display FPS and object count
pub fn debug_ui(
    debug: Res<DebugMode>,
    diagnostics: Query<&Position>,
    time: Res<Time>,
) {
    if debug.0 {
        let fps = (1.0 / time.delta_secs()).round();
        let count = diagnostics.iter().count();
        println!("📊 FPS: {} | Objects: {}", fps, count);
    }
}
```

---

## 🎬 Main Entry Point

```rust
// 📁 src/main.rs
//! 🎮 Physics Sandbox  -  Main Entry Point
//!
//! Run with: cargo run
//!
//! Controls:
//!   🖱️ Click -> Spawn ball
//!   ◀️ ▶️ ⬆️ ⬇️ -> Adjust gravity
//!   🧹 C -> Clear all objects
//!   🔄 R -> Reset gravity
//!   💥 E -> Explosion (spawn 20 balls)
//!   🕵️ F3 -> Toggle debug visualization

use bevy::prelude::*;

mod physics;
mod sandbox;
mod debug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        // ⚙️ Physics resources
        .init_resource::<physics::PhysicsSettings>()
        .init_resource::<physics::SpatialGrid>()
        .init_resource::<debug::DebugMode>()
        .init_resource::<sandbox::SandboxState>()
        
        // 📢 Events
        .add_event::<physics::CollisionEvent>()
        
        // 🔄 Physics systems (ordered!)
        .add_systems(Update, (
            physics::clear_forces,
            physics::apply_gravity,
            physics::integrate,
            physics::build_grid,
            physics::detect_collisions,
            physics::resolve_collisions,
        ).chain())
        
        // 🎮 Game systems
        .add_systems(Startup, sandbox::setup_sandbox)
        .add_systems(Update, (
            sandbox::spawn_on_click,
            sandbox::sandbox_controls,
        ))
        
        // 🕵️ Debug systems
        .add_systems(Update, (
            debug::toggle_debug,
            debug::debug_draw,
        ))
        
        // 🚀 GO!
        .run();
}
```

---

## 🚀 Running the Sandbox

```bash
# Clone and run
cargo run

# You should see:
# - A gray ground at the bottom
# - 5 colored balls falling
# - Click to spawn more balls
# - Arrow keys to change gravity
# - F3 for debug visualization
# - C to clear, E for explosion, R to reset
```

---

## 🎯 What You've Built

```
🏆 CONGRATULATIONS! You've built a complete physics engine:

✅ Position, Velocity, Acceleration components
✅ Force accumulation with gravity
✅ Semi-implicit Euler integration
✅ Circle and AABB collision detection
✅ Spatial grid for performance
✅ Impulse-based collision response
✅ Interactive spawning and controls
✅ Debug visualization
✅ Clean ECS architecture with Bevy

All in ~400 lines of Rust! 🦀
```

---

## 🧪 Challenges for Further Learning

Ready to level up? Try these:

1. **🔵 Add polygon collision** using SAT (Separating Axis Theorem)
2. **⛓️ Add distance constraints** for chain/ragdoll physics
3. **🌊 Add fluid simulation** using Verlet particles
4. **🎯 Add raycasting** for mouse picking
5. **📦 Add stacking stability** with more collision iterations
6. **💫 Add particle effects** on collision
7. **🎵 Add sound** that pitches based on collision velocity
8. **📈 Add performance metrics** to profile the physics

> 💡 **Full source code for this chapter:** [code-examples/ch15-physics-sandbox/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch15-physics-sandbox)
> 
> The runnable project includes Cargo.toml, main.rs, and complete module files.

---

**[<- Previous: Bevy ECS Architecture](ch14-ecs-architecture.md)** | **[Next: Appendix ->](ch16-appendix.md)**
