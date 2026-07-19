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
/// the physics engine. It handles:
///   - Registering all physics components (Position, Velocity, etc.)
///   - Setting up PhysicsSettings as a global resource
///   - Ordering physics systems in the correct execution sequence
///
/// USAGE:
/// ```rust,no_run
/// use bevy::prelude::*;
/// use physics::PhysicsPlugin;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(PhysicsPlugin)  // Enable physics!
///         .run();
/// }
/// ```
///
/// SYSTEM EXECUTION ORDER (enforced by `.chain()`):
///   1. clear_force_accumulators  → Reset forces to zero
///   2. apply_gravity_to_all_entities → Add gravitational forces
///   3. integrate_positions_using_symplectic_euler → a → v → x
///   4. sync_physics_positions_to_render_transforms → Physics → Render
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register PhysicsSettings as a global resource.
            // This makes it available to all systems via Res<PhysicsSettings>.
            .init_resource::<PhysicsSettings>()

            // Register ALL physics systems in CHAIN order.
            // `.chain()` enforces strict sequential execution:
            //   System 1 completes → System 2 starts → etc.
            // Without `.chain()`, Bevy might run systems in parallel,
            // which would cause race conditions in physics!
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
