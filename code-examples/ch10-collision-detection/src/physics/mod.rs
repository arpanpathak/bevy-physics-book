/// # Physics Module
///
/// This module encapsulates the entire physics engine as a self-contained,
/// reusable unit. External code only needs to:
///   1. Add components to entities (Position, Velocity, Mass, etc.)
///   2. Call `app.add_plugins(PhysicsPlugin)` to enable physics.
///
/// MODULE STRUCTURE:
///   physics/
///     mod.rs       - PhysicsPlugin definition and re-exports
///     components.rs  - Position, Velocity, Mass, ForceAccumulator, etc.
///     systems.rs     - Physics systems: integration, forces, sync

pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

/// The main Physics Plugin. Register this with your Bevy app to enable
/// the physics engine.
///
/// USAGE:
/// ```rust,no_run
/// use bevy::prelude::*;
/// use physics::PhysicsPlugin;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(PhysicsPlugin)
///         .run();
/// }
/// ```
///
/// SYSTEM EXECUTION ORDER (enforced by `.chain()`):
///   1. clear_force_accumulators
///   2. apply_gravity_to_all_entities
///   3. integrate_positions_using_symplectic_euler
///   4. sync_physics_positions_to_render_transforms
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PhysicsSettings>()
            .add_systems(
                Update,
                (
                    clear_force_accumulators,
                    apply_gravity_to_all_entities,
                    integrate_positions_using_symplectic_euler,
                    sync_physics_positions_to_render_transforms,
                )
                    .chain(),
            );
    }
}
