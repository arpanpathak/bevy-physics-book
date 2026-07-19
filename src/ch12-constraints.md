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

/// Resolves a **distance constraint** between two particles.
///
/// A distance constraint ensures that two particles remain at a fixed
/// distance from each other  -  like a rigid rod connecting them.
///
/// # How It Works
///
/// 1. Compute the current distance between the two particles.
/// 2. Compare it to the target (rest) distance.
/// 3. If there's a difference, push the particles toward or away from
///    each other along the line connecting them.
/// 4. The amount each particle moves is PROPORTIONAL to its inverse mass
///    (heavier particles move less).
///
/// # The Correction Formula
///
/// ```text
/// displacement = direction × (current_distance - target_distance) × stiffness
/// particle_1 -= displacement × (inverse_mass_1 / total_inverse_mass)
/// particle_2 += displacement × (inverse_mass_2 / total_inverse_mass)
/// ```
///
/// This is a **position-based** constraint solver  -  we directly modify
/// positions rather than applying forces. This is what makes Verlet
/// integration so powerful: constraints become trivial position adjustments.
///
/// # Arguments
/// * `position_of_first_particle` - The position of particle 1. Modified in-place.
/// * `position_of_second_particle` - The position of particle 2. Modified in-place.
/// * `inverse_mass_of_first_particle` - 1/mass of particle 1 (0 = infinite mass).
/// * `inverse_mass_of_second_particle` - 1/mass of particle 2 (0 = infinite mass).
/// * `target_distance` - The desired distance between the two particles.
/// * `stiffness` - How rigid the constraint is (0.0 to 1.0).
///   1.0 = perfectly rigid, 0.5 = soft/springy.
pub fn distance_constraint(
    position_of_first_particle: &mut Vec2,
    position_of_second_particle: &mut Vec2,
    inverse_mass_of_first_particle: f32,
    inverse_mass_of_second_particle: f32,
    target_distance: f32,
    stiffness: f32,
) {
    // Step 1: Compute the vector from particle 2 to particle 1.
    // This tells us the direction we need to push/pull along.
    let vector_between_particles =
        *position_of_first_particle - *position_of_second_particle;

    // Step 2: Compute the current distance (the length of the vector).
    let current_distance = vector_between_particles.length();

    // ⛔ Guard against division by zero.
    // If both particles are at the exact same position, the direction
    // vector is undefined. We just bail out  -  the constraint can't
    // be resolved until the particles separate.
    if current_distance < 0.0001 {
        return;
    }

    // Step 3: Compute the direction (unit vector from particle 2 to particle 1).
    let direction_from_second_to_first =
        vector_between_particles / current_distance;

    // Step 4: How far off are we from the target distance?
    // Positive = particles are too far apart (need to pull together).
    // Negative = particles are too close (need to push apart).
    let distance_error = current_distance - target_distance;

    // Step 5: Compute the correction displacement.
    // The stiffness factor allows soft constraints where we only
    // partially correct the error each frame.
    let correction_displacement =
        direction_from_second_to_first * distance_error * stiffness;

    // Step 6: Distribute the correction based on inverse masses.
    // Heavier particles (lower inverse mass) move less.
    // This conserves the center of mass of the system.
    let total_inverse_mass =
        inverse_mass_of_first_particle + inverse_mass_of_second_particle;

    if total_inverse_mass > 0.0 {
        // Particle 1 moves proportional to its OWN inverse mass
        // (actually, proportional to the OTHER particle's mass share)
        *position_of_first_particle -=
            correction_displacement
            * (inverse_mass_of_first_particle / total_inverse_mass);

        // Particle 2 moves in the OPPOSITE direction
        *position_of_second_particle +=
            correction_displacement
            * (inverse_mass_of_second_particle / total_inverse_mass);
    }
    // If total_inverse_mass is 0, both particles are immovable (mass = ∞).
    // Nothing we can do  -  they're pinned to the world.
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

/// 📍 A single particle in a Verlet chain simulation.
///
/// Verlet integration stores the PREVIOUS position instead of velocity.
/// This allows the integration to be done purely through position
/// manipulation, which makes constraints trivially simple to implement.
#[derive(Component)]
struct ChainParticle {
    /// The position of this particle in the PREVIOUS frame.
    /// Used by the Verlet integrator to compute motion:
    /// new_position = 2 × current - previous + acceleration × dt²
    previous_position: Vec2,
}

/// 🔗 A distance constraint between two particles in the chain.
///
/// This is a rigid rod connecting two particles. The constraint solver
/// will push/pull the particles so they maintain exactly `rest_length`
/// distance from each other.
#[derive(Component)]
struct ChainLink {
    /// The particle at one end of the rod.
    first_particle_entity: Entity,
    /// The particle at the other end of the rod.
    second_particle_entity: Entity,
    /// The desired distance between the two particles (the rod length).
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
            first_particle_entity: entities[i],
            second_particle_entity: entities[i + 1],
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
    for (mut position, mut particle) in particle_query.iter_mut() {
        let current_position = position.0;
        
        // Verlet formula: new_position = 2 × current - previous + acceleration × dt²
        // The acceleration is just gravity in this case.
        position.0 = position.0 * 2.0 - particle.previous_position + gravity * dt * dt;
        
        // Store current position as previous for the next frame.
        particle.previous_position = current_position;
    }
    
    // ─── 2️⃣ Apply constraints (multiple iterations!) ───
    for _ in 0..5 {  // Multiple iterations = more stable
        for link in link_query.iter() {
            let (mut position_of_first_particle, mut position_of_second_particle) = 
                get_pair_mut!(particle_query, link.first_particle_entity, link.second_particle_entity);
            
            distance_constraint(
                &mut position_of_first_particle.0,
                &mut position_of_second_particle.0,
                1.0,  // inverse_mass = 1 for all particles (equal weight)
                1.0,
                link.rest_length,
                1.0,  // Full stiffness (perfectly rigid constraint)
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
/// 🔄 Hinge / Pivot joint  -  objects rotate around a common point
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

> **Key Takeaway:** Constraints are the grammar that turns independent particles into structured objects. Distance constraints + Verlet integration = cloth, ropes, chains, and ragdolls with minimal code. Run 5-10 constraint iterations per frame for stability. The magic isn't in any single constraint  -  it's in how they interact through iteration! 🏗️

---

**[← Previous: Collision Response](ch11-collision-response.md)** | **[Next: Spatial Partitioning →](ch13-spatial-partitioning.md)**
