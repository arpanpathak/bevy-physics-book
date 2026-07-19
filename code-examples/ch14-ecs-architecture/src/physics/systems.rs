/// # 🔄 Physics Systems
///
/// This module contains the SYSTEMS (logic) for our physics engine.
/// Each system is a function that operates on components to implement
/// one step of the physics pipeline.
///
/// SYSTEM SCHEDULING (order of execution):
///   1. clear_force_accumulators - Reset all forces to zero
///   2. apply_gravity_to_all_entities - Add gravitational force
///   3. integrate_positions_using_symplectic_euler - a -> v -> x
///   4. sync_physics_positions_to_render_transforms - Physics -> Render

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
///   Frame 1: gravity = (0, -500), total = (0, -500)  OK
///   Frame 2: gravity = (0, -500), total = (0, -1000) DOUBLE!
///   Frame 3: gravity = (0, -500), total = (0, -1500) TRIPLE!
///   -> Runaway acceleration! Objects fly off at insane speeds!
///
/// This system MUST run BEFORE any force-application systems.
pub fn clear_force_accumulators(
    mut force_query: Query<&mut ForceAccumulator>,
) {
    for mut force_accumulator in force_query.iter_mut() {
        force_accumulator.clear();
    }
}

/// SYSTEM 2: Applies gravitational force to all entities with mass.
///
/// F = m x g. Since a = F/m = (m x g)/m = g, all objects fall at the
/// same rate regardless of mass (in vacuum).
pub fn apply_gravity_to_all_entities(
    mut force_query: Query<(&Mass, &mut ForceAccumulator)>,
    physics_settings: Res<PhysicsSettings>,
) {
    let gravitational_acceleration = physics_settings.gravity;

    for (mass, mut force_accumulator) in force_query.iter_mut() {
        let gravitational_force = gravitational_acceleration * mass.value;
        force_accumulator.add_force(gravitational_force);
    }
}

/// SYSTEM 3: Applies Symplectic Euler integration to all physics entities.
///
/// SYMPLECTIC VS EXPLICIT EULER:
///   Explicit Euler (BAD):     x_new = x_old + v_old x dt  (OLD velocity!)
///   Symplectic Euler (GOOD):  x_new = x_old + v_new x dt  (NEW velocity!)
///
/// The difference is subtle but CRITICAL. Symplectic Euler conserves energy.
/// Explicit Euler causes orbits to spiral outward!
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
    let delta_time = physics_settings.fixed_delta_time;

    for (force_accumulator, mass, mut acceleration, mut velocity, mut position) in
        physics_query.iter_mut()
    {
        // Step 1: F = ma -> a = F/m (Newton's Second Law)
        if mass.value > 0.0 {
            acceleration.value = force_accumulator.total_force / mass.value;
        } else {
            acceleration.value = Vec2::ZERO;
        }

        // Step 2: Integrate acceleration into velocity (v_new = v_old + a x dt)
        velocity.value += acceleration.value * delta_time;

        // Step 3: Integrate velocity into position (x_new = x_old + v_new x dt)
        // Uses NEW velocity (Symplectic Euler)!
        position.value += velocity.value * delta_time;
    }
}

/// SYSTEM 4: Syncs physics Position to Bevy's Transform for rendering.
///
/// We keep Position separate from Transform so physics can modify
/// position freely without triggering Bevy's rendering hierarchy updates.
pub fn sync_physics_positions_to_render_transforms(
    mut sync_query: Query<(&Position, &mut Transform)>,
) {
    for (physics_position, mut render_transform) in sync_query.iter_mut() {
        render_transform.translation = Vec3::new(
            physics_position.value.x,
            physics_position.value.y,
            0.0,
        );
    }
}

/// Global physics settings stored as a Bevy Resource.
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
