# 🔗 Constraints & Joints

> **"Constraints are the rules of the physics world. 'You must be this distance from your neighbor.' 'You must pivot here.' They turn chaos into structure."** 🏗️

---

## 🤔 What Are Constraints?

**Constraints** restrict the motion of objects. They're the foundation of:

- 🚪 **Doors** that rotate around a hinge
- ⛓️ **Chains** of connected links
- 👕 **Cloth** that drapes and folds
- 🧍 **Ragdolls** with skeleton joints
- 🚗 **Vehicles** with wheels and suspension

```
Without constraints:    With constraints:
    ○   ○   ○              ○───○───○
    (independent)         (connected!)
    
    Positions can be      Fixed distances,
    anywhere.             pivot points.
            ↓                    ↓
    Chaos!                Structure!
```

---

## 📐 Distance Constraint

The simplest and most useful constraint:

```rust
/// 🔗 Distance constraint: keeps two points at a fixed distance
fn distance_constraint(
    pos_a: &mut Vec2,
    pos_b: &mut Vec2,
    inv_mass_a: f32,
    inv_mass_b: f32,
    target_distance: f32,
    stiffness: f32,  // 0.0 = no correction, 1.0 = full correction
) {
    // 📐 Vector between the two points
    let delta = *pos_a - *pos_b;
    let current_distance = delta.length();
    
    // ⛔ Avoid division by zero
    if current_distance < 0.0001 {
        return;
    }
    
    // 🧭 Direction (normalized)
    let direction = delta / current_distance;
    
    // 📏 How far off are we?
    let error = current_distance - target_distance;
    
    // 🧮 Correction amount (with stiffness for soft constraints)
    let correction = direction * error * stiffness;
    
    // 🔄 Apply correction proportional to inverse masses
    // Heavier object (lower inv mass) moves less
    let total_inv_mass = inv_mass_a + inv_mass_b;
    
    if total_inv_mass > 0.0 {
        *pos_a -= correction * (inv_mass_a / total_inv_mass);
        *pos_b += correction * (inv_mass_b / total_inv_mass);
    }
}
```

```
Distance Constraint:

    Before:                     After:
    
    ○───────○                    ○───────○
    │       │                    │       │
    │   too │                    │  just │
    │   far │                    │ right │
    │       │                    │       │
    ○       ○                    ○───────○
    
    dist ≠ target               dist = target ✅
    → APPLY CONSTRAINT!
```

---

## ⛓️ Chain of Particles (Verlet Style)

Using Verlet integration, constraints become **trivially simple**:

```rust
/// ⛓️ A chain made of Verlet particles with distance constraints

/// 📍 A single particle in our chain
#[derive(Component)]
struct ChainParticle {
    prev_pos: Vec2,
}

/// 🔗 A constraint between two particles
#[derive(Component)]
struct ChainLink {
    entity_a: Entity,
    entity_b: Entity,
    rest_length: f32,
}

/// 🏗️ Build a chain from start to end
fn spawn_chain(
    commands: &mut Commands,
    start: Vec2,
    end: Vec2,
    link_count: usize,
    link_length: f32,
) -> Vec<Entity> {
    let mut entities = Vec::new();
    let direction = (end - start) / link_count as f32;
    
    // Create particles along the chain
    for i in 0..=link_count {
        let pos = start + direction * i as f32;
        let entity = commands.spawn((
            Position(pos),
            ChainParticle { prev_pos: pos },
            Mass(1.0),
            SpriteBundle {
                sprite: Sprite::from_color(Color::WHITE, Vec2::splat(4.0)),
                ..default()
            },
        )).id();
        entities.push(entity);
    }
    
    // Create distance constraints between adjacent particles
    for i in 0..link_count {
        commands.spawn(ChainLink {
            entity_a: entities[i],
            entity_b: entities[i + 1],
            rest_length: link_length,
        });
    }
    
    entities
}

/// 🔄 Update chain physics (Verlet integration)
fn update_chain(
    mut particle_query: Query<(&mut Position, &mut ChainParticle)>,
    link_query: Query<&ChainLink>,
    settings: Res<PhysicsSettings>,
) {
    let dt = settings.fixed_dt;
    let gravity = settings.gravity;
    
    // ─── 1️⃣ Verlet integration step ───
    for (mut pos, mut particle) in particle_query.iter_mut() {
        let current = pos.0;
        
        // Verlet: new_pos = 2×current - prev + acceleration × dt²
        pos.0 = pos.0 * 2.0 - particle.prev_pos + gravity * dt * dt;
        
        // Store current as previous for next frame
        particle.prev_pos = current;
    }
    
    // ─── 2️⃣ Apply constraints (multiple iterations!) ───
    for _ in 0..5 {  // Multiple iterations = more stable
        for link in link_query.iter() {
            let (mut pos_a, mut pos_b) = 
                get_pair_mut!(particle_query, link.entity_a, link.entity_b);
            
            distance_constraint(
                &mut pos_a.0,
                &mut pos_b.0,
                1.0,  // inv_mass = 1 for all particles
                1.0,
                link.rest_length,
                1.0,  // Full stiffness
            );
        }
    }
}

/// 💡 Verlet chain secrets:
/// 1. Run Verlet integration first (gravity, velocity)
/// 2. Run constraint solver (iteratively!)
/// 3. Repeat
/// The constraints "pull" particles to valid positions.
/// Multiple iterations (5-10) give near-perfect results.
```

```
Chain Simulation:

    Frame 1:            Frame 10:           Frame 60:
    
    ●───●───●            ●                   ●
    │   │   │            │╲                  │╲
    │   │   │            │ ╲                 │ ╲
    │   │   │            ●  ╲                ●  ╲
    ●───●───●            │   ╲               │   ●
        ↑                │    ●              │   │
    Start: rigid         │    │              │   │
                         ●    │              ●   │
                              │              │   │
                              ●              │   ●
                                             │   │
                          Mid: falling       │   │
                          with drag          ●   │
                                                  │
                                                  ●
                                             End: draped
                                             over peg!
```

---

## 🔄 Rotational Joint

```rust
/// 🔄 Hinge / Pivot joint — objects rotate around a common point
#[derive(Component)]
struct HingeJoint {
    /// World-space position of the hinge
    pivot: Vec2,
    /// Entity connected (None = connected to world)
    connected: Option<Entity>,
    /// Min/max angle (None = free rotation)
    min_angle: Option<f32>,
    max_angle: Option<f32>,
}

fn solve_hinge_joint(
    mut joint_query: Query<(&HingeJoint, &mut PhysicsTransform)>,
) {
    for (joint, mut transform) in joint_query.iter_mut() {
        // 📐 Vector from pivot to object center
        let offset = transform.translation - joint.pivot;
        let dist = offset.length();
        
        if dist < 0.001 {
            continue;
        }
        
        // 🔄 The distance from pivot is fixed (rigid arm)
        // Only rotation is free
        let direction = offset / dist;
        let angle = direction.y.atan2(direction.x);
        
        // ⛔ Apply angle limits (if any)
        let clamped_angle = match (joint.min_angle, joint.max_angle) {
            (Some(min), Some(max)) => angle.clamp(min, max),
            _ => angle,
        };
        
        // 🔄 Position the object at the correct angle
        transform.translation = joint.pivot + Vec2::new(
            clamped_angle.cos() * dist,
            clamped_angle.sin() * dist,
        );
        transform.rotation = clamped_angle;
    }
}

/// 💡 Hinge joints are everywhere:
/// - 🚪 Door hinges
/// - 🦾 Robot arms
/// - 🌲 Tree branches
/// - 🚗 Car suspension
/// - 🧍 Ragdoll elbows & knees
```

---

## 🧍 Simple Ragdoll

```rust
/// 🧍 A minimal ragdoll with constraints
///
/// Structure:
///     head
///      │
///     torso
///    ╱    ╲
///  arm    arm
///    ╲    ╱
///     hips
///    ╱    ╲
///  leg    leg

#[derive(Component)]
struct RagdollPart {
    part_type: RagdollPartType,
}

enum RagdollPartType {
    Head, Torso, UpperArm, LowerArm, UpperLeg, LowerLeg,
}

#[derive(Component)]
struct RagdollJoint {
    from: Entity,
    to: Entity,
    rest_length: f32,
}

/// 🏗️ Spawn a simple ragdoll
fn spawn_ragdoll(commands: &mut Commands, position: Vec2) {
    let head = spawn_part(commands, position + Vec2::new(0.0, 40.0), 8.0, RagdollPartType::Head);
    let torso = spawn_part(commands, position, 12.0, RagdollPartType::Torso);
    let upper_arm_l = spawn_part(commands, position + Vec2::new(-20.0, 5.0), 6.0, RagdollPartType::UpperArm);
    let lower_arm_l = spawn_part(commands, position + Vec2::new(-35.0, 0.0), 5.0, RagdollPartType::LowerArm);
    let upper_arm_r = spawn_part(commands, position + Vec2::new(20.0, 5.0), 6.0, RagdollPartType::UpperArm);
    let lower_arm_r = spawn_part(commands, position + Vec2::new(35.0, 0.0), 5.0, RagdollPartType::LowerArm);
    let upper_leg_l = spawn_part(commands, position + Vec2::new(-8.0, -25.0), 7.0, RagdollPartType::UpperLeg);
    let lower_leg_l = spawn_part(commands, position + Vec2::new(-8.0, -45.0), 6.0, RagdollPartType::LowerLeg);
    let upper_leg_r = spawn_part(commands, position + Vec2::new(8.0, -25.0), 7.0, RagdollPartType::UpperLeg);
    let lower_leg_r = spawn_part(commands, position + Vec2::new(8.0, -45.0), 6.0, RagdollPartType::LowerLeg);
    
    // Apply constraints between connected parts
    let joints = vec![
        (head, torso, 40.0),
        (torso, upper_arm_l, 20.0),
        (upper_arm_l, lower_arm_l, 15.0),
        (torso, upper_arm_r, 20.0),
        (upper_arm_r, lower_arm_r, 15.0),
        (torso, upper_leg_l, 25.0),
        (upper_leg_l, lower_leg_l, 20.0),
        (torso, upper_leg_r, 25.0),
        (upper_leg_r, lower_leg_r, 20.0),
    ];
    
    for (from, to, rest_length) in joints {
        commands.spawn(RagdollJoint { from, to, rest_length });
    }
}

fn spawn_part(commands: &mut Commands, pos: Vec2, size: f32, part_type: RagdollPartType) -> Entity {
    commands.spawn((
        Position(pos),
        Mass(1.0),
        RagdollPart { part_type },
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(0.8, 0.6, 0.4), Vec2::splat(size)),
            ..default()
        },
    )).id()
}

/// 🔄 Solve ragdoll constraints
fn solve_ragdoll(
    joint_query: Query<&RagdollJoint>,
    mut part_query: Query<&mut Position>,
) {
    for _ in 0..10 {  // Iterate for stability
        for joint in joint_query.iter() {
            let (mut pos_a, mut pos_b) = 
                get_pair_mut!(part_query, joint.from, joint.to);
            
            distance_constraint(
                &mut pos_a.0,
                &mut pos_b.0,
                1.0, 1.0,
                joint.rest_length,
                0.8,  // Slightly soft
            );
        }
    }
}
```

---

## 🎯 Constraint Solver Architecture

```rust
/// 🏗️ Full constraint solver
///
/// The trick: run constraints MULTIPLE TIMES per frame
/// More iterations = more accurate = more expensive
#[derive(Resource)]
struct ConstraintSettings {
    /// Number of constraint iterations per frame
    iterations: u32,
}

impl Default for ConstraintSettings {
    fn default() -> Self {
        Self { iterations: 5 }
    }
}

/// 🔄 Main constraint solving loop
fn constraint_solver(
    settings: Res<ConstraintSettings>,
    // ... all constraint queries ...
) {
    for _ in 0..settings.iterations {
        // Solve ALL constraints every iteration
        // This allows constraints to interact properly
        
        // 1. Distance constraints
        // solve_distances(...)
        
        // 2. Hinge joints
        // solve_hinges(...)
        
        // 3. Slide joints
        // solve_slides(...)
        
        // 4. Collision constraints (non-penetration)
        // solve_collisions(...)
    }
}

/// 💡 Constraint iteration insights:
///
/// 1 iteration  → Jelly-like, very soft
/// 3 iterations → Noticeably stiffer
/// 5 iterations → Good for most games
/// 10 iterations → Very stiff, near-rigid
/// 20+ iterations → Overkill (diminishing returns)
///
/// Each iteration compounds the corrections, converging
/// toward a valid configuration (like Gauss-Seidel).
```

---

## 🎯 Chapter Summary

```
Constraints are RULES that objects must obey:

    📏 Distance:   "Stay exactly 5 units apart"
    🔄 Hinge:      "Rotate around this point"
    🛤️ Slide:      "Move along this line only"
    🧊 Non-penetration: "Don't overlap!"
    
    ⭐ Key insight: Run ALL constraints multiple times
    per frame. Each pass gets closer to the "perfect"
    configuration. 5 passes is the sweet spot.
    
    Verlet particles + distance constraints =
    The simplest physics engine that can do cloth,
    ropes, chains, and ragdolls!
```

> **Key Takeaway:** Constraints are the grammar that turns independent particles into structured objects. Distance constraints + Verlet integration = cloth, ropes, chains, and ragdolls with minimal code. Run 5-10 constraint iterations per frame for stability. The magic isn't in any single constraint — it's in how they interact through iteration! 🏗️

---

**[← Previous: Collision Response](ch11-collision-response.md)** | **[Next: Spatial Partitioning →](ch13-spatial-partitioning.md)**
