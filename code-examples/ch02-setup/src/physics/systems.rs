/// # 🔄 Physics Systems
///
/// This module contains the SYSTEMS (logic) for our physics engine.
/// Each system is a function that operates on components to implement
/// one step of the physics pipeline.
///
/// SYSTEM SCHEDULING (order of execution):
///   1. clear_force_accumulators - Reset all forces to zero
///   2. apply_gravity - Add gravitational force to each entity
///   3. integrate_positions - a → v → x (Symplectic Euler)
///   4. sync_positions_to_transforms - Physics → Rendering bridge

use bevy::prelude::*;
use crate::physics::components::*;

/// SYSTEM 1: Clears all force accumulators at the START of each frame.
///
/// WHY IS THIS NECESSARY?
/// Forces do NOT persist between frames. If you push an object in frame 1,
/// the push is over. The object keeps moving due to its VELOCITY (inertia),
/// but the FORCE that caused it is gone.
///
/// If we don't clear forces, they accumulate across frames:
///   Frame 1: gravity = (0, -500), total = (0, -500)  ✅
///   Frame 2: gravity = (0, -500), total = (0, -1000) ❌ DOUBLE!
///   Frame 3: gravity = (0, -500), total = (0, -1500) ❌ TRIPLE!
///   → Runaway acceleration! Objects fly off at insane speeds!
///
/// This system MUST run BEFORE any force-application systems.
pub fn clear_force_accumulators(
    mut force_query: Query<&mut ForceAccumulator>,
) {
    // Iterate over ALL entities that have a ForceAccumulator component.
    // For each one, reset the accumulated force to zero.
    for mut force_accumulator in force_query.iter_mut() {
        force_accumulator.clear();
    }
}

/// SYSTEM 2: Applies gravitational force to all entities with mass.
///
/// MATHEMATICAL DERIVATION:
///   Newton's Law of Universal Gravitation tells us:
///   F_gravity = m × g
///
///   Where:
///     m = mass of the entity
///     g = gravitational acceleration (a constant vector, typically (0, -9.81))
///
///   Since a = F/m = (m × g) / m = g, the mass CANCELS OUT!
///   All objects fall at the same rate regardless of mass.
///   (In vacuum. Air resistance changes this, but that's a separate force.)
///
/// GAME TUNING:
///   Earth-like:    g = (0, -9.81)   → Realistic, slow-falling
///   Platformer:    g = (0, -500)    → Snappy, responsive jumping
///   Moon:          g = (0, -1.62)   → Floaty, high jumps
///   "Juice":       g = (0, -2000)   → Dramatic, heavy falls
///
/// There is NO "correct" gravity for a game. Tune it until it feels right.
pub fn apply_gravity_to_all_entities(
    mut force_query: Query<(&Mass, &mut ForceAccumulator)>,
    physics_settings: Res<PhysicsSettings>,
) {
    // The PhysicsSettings resource contains global simulation parameters.
    // We read `gravity` from it to avoid hard-coding values.
    let gravitational_acceleration = physics_settings.gravity;

    for (mass, mut force_accumulator) in force_query.iter_mut() {
        // F = m × g
        // We multiply mass by gravity to get the force vector.
        // This force points downward (negative Y in Bevy's coordinate system).
        let gravitational_force = gravitational_acceleration * mass.value;

        // Add this force to the accumulator. Other systems may also add forces
        // (drag, thrust, collisions). They all SUM together.
        force_accumulator.add_force(gravitational_force);
    }
}

/// SYSTEM 3: Applies Symplectic Euler integration to all physics entities.
///
/// This is the HEART of the physics engine. It converts forces into motion
/// through the integration chain:
///
///   ForceAccumulator → a = F/m → v += a×dt → x += v×dt
///
/// SYMPLECTIC VS EXPLICIT EULER:
///
///   Explicit Euler (BAD - do not use):
///     v_new = v_old + a × dt
///     x_new = x_old + v_old × dt    ← Uses OLD velocity!
///
///   Symplectic Euler (GOOD - this is what we use):
///     v_new = v_old + a × dt
///     x_new = x_old + v_new × dt    ← Uses NEW velocity!
///
///   The difference is subtle but CRITICAL. By using the NEW velocity
///   to update position, we estimate the AVERAGE velocity over the
///   timestep rather than the START velocity. This conserves energy.
///
///   CONSEQUENCE:
///     Explicit Euler: orbits SPIRAL OUTWARD (energy increases!)
///     Symplectic Euler: orbits STAY STABLE (energy conserved!)
pub fn integrate_positions_using_symplectic_euler(
    mut physics_query: Query<(
        &ForceAccumulator,
        &Mass,
        &mut Acceleration,
        &mut Velocity,
        &mut Position,
    )>,
    physics_settings: Res<PhysicsSettings>,
) {
    // Use the FIXED timestep, not the real frame delta.
    // This ensures deterministic, framerate-independent simulation.
    let delta_time = physics_settings.fixed_delta_time;

    for (
        force_accumulator,
        mass,
        mut acceleration,
        mut velocity,
        mut position,
    ) in physics_query.iter_mut()
    {
        // ─── Step 1: F = ma → a = F/m (Newton's Second Law) ───
        // This converts accumulated forces into acceleration.
        if mass.value > 0.0 {
            acceleration.value = force_accumulator.total_force / mass.value;
        } else {
            // mass = 0 means STATIC (immovable). No acceleration.
            acceleration.value = Vec2::ZERO;
        }

        // ─── Step 2: Integrate acceleration into velocity ───
        // v_new = v_old + a × dt
        // The acceleration tells us how velocity changes over time.
        velocity.value += acceleration.value * delta_time;

        // ─── Step 3: Integrate velocity into position ───
        // x_new = x_old + v_new × dt
        // We use the NEWLY COMPUTED velocity (Symplectic Euler).
        // This is the key difference from Explicit Euler.
        position.value += velocity.value * delta_time;
    }
}

/// SYSTEM 4: Syncs physics Position to Bevy's Transform for rendering.
///
/// WHY DO WE NEED THIS?
/// We keep Position separate from Transform so physics can modify
/// position freely without triggering Bevy's rendering hierarchy updates.
/// But ultimately, Bevy's renderer reads Transform to draw sprites.
/// This system bridges the gap.
///
/// WHAT THIS DOES:
///   For each entity that has BOTH a physics Position AND a Bevy Transform,
///   we copy Position → Transform.translation.
///
///   Physics Position:  (100.0, 200.0)    ← 2D vector
///   Bevy Transform:    Vec3(100.0, 200.0, 0.0)  ← 3D with z=0 for 2D
pub fn sync_physics_positions_to_render_transforms(
    mut sync_query: Query<(&Position, &mut Transform)>,
) {
    for (physics_position, mut render_transform) in sync_query.iter_mut() {
        // Copy x and y from physics, set z to 0 (we're in 2D).
        render_transform.translation = Vec3::new(
            physics_position.value.x,
            physics_position.value.y,
            0.0,
        );
    }
}

/// Global physics settings stored as a Bevy Resource.
///
/// A Resource is a SINGLETON - there's exactly ONE instance in the World.
/// Unlike components (which are per-entity), resources affect everything.
/// This is where we store world-wide physics parameters.
#[derive(Resource)]
pub struct PhysicsSettings {
    /// Gravitational acceleration vector. Standard: (0, -500.0) for 2D games.
    pub gravity: Vec2,
    /// Fixed physics timestep. 1/60 = ~16.67ms for 60 Hz simulation.
    pub fixed_delta_time: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -500.0),
            fixed_delta_time: 1.0 / 60.0,
        }
    }
}
