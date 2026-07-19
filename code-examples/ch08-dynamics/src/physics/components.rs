/// # 📍 Physics Components
///
/// This module defines the COMPONENTS (data) for our physics engine.
/// In Bevy's ECS, components are pure data attached to entities.
/// Each component represents ONE physical quantity.
///
/// IMPORTANT RULE: Components have NO methods that modify other components.
/// All logic lives in SYSTEMS (see physics::systems).
/// Components are just data containers.

use bevy::prelude::*;

/// Represents the spatial location of an entity in 2D world space.
///
/// In mathematical terms, this is a VECTOR from the world origin (0,0)
/// to the entity's position. Adding a Velocity to this over time
/// produces smooth movement.
#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    /// The x,y coordinates in world space.
    /// x increases to the right, y increases upward (in Bevy's 2D system).
    pub value: Vec2,
}

impl Position {
    /// Creates a new Position at the given (x, y) coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Self { value: Vec2::new(x, y) }
    }
}

impl Default for Position {
    fn default() -> Self { Self { value: Vec2::ZERO } }
}

/// Represents the RATE OF CHANGE of position over time.
///
/// In calculus terms: v = dr/dt (velocity is the derivative of position).
/// In discrete terms: position_change = velocity × delta_time.
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity {
    /// The velocity vector in world space. Units: pixels/second.
    pub value: Vec2,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { value: Vec2::new(x, y) }
    }
}

impl Default for Velocity {
    fn default() -> Self { Self { value: Vec2::ZERO } }
}

/// Represents the RATE OF CHANGE of velocity over time.
///
/// In physics terms: a = F/m (Newton's Second Law).
/// Acceleration is where FORCES live. It is RECALCULATED every frame.
#[derive(Component, Debug, Clone, Copy)]
pub struct Acceleration {
    /// The acceleration vector. Reset to ZERO at the start of each frame.
    pub value: Vec2,
}

impl Default for Acceleration {
    fn default() -> Self { Self { value: Vec2::ZERO } }
}

/// Represents the INERTIA of an entity - how much it resists acceleration.
///
/// mass = 0.0 → STATIC / immovable object (walls, floor).
/// mass > 0.0 → Dynamic object (responds to forces).
#[derive(Component, Debug, Clone, Copy)]
pub struct Mass {
    /// The mass value in arbitrary mass units.
    pub value: f32,
}

impl Default for Mass {
    fn default() -> Self { Self { value: 1.0 } }
}

/// Accumulates ALL forces acting on an entity for the current frame.
///
/// FORCE ACCUMULATION PIPELINE:
///   1. Frame starts: ForceAccumulator is CLEARED to (0, 0).
///   2. Each force system (gravity, drag, input, collisions) ADDS to it.
///   3. At integration time: a = F / m (Newton's Second Law).
///   4. ForceAccumulator is cleared at the START of the NEXT frame.
///
/// Why clear every frame? Forces are INSTANTANEOUS events.
/// If you push an object, the push lasts while you're touching it.
/// The object's VELOCITY persists (inertia), but the FORCE does not.
#[derive(Component, Debug, Clone, Copy)]
pub struct ForceAccumulator {
    /// The sum of ALL forces this frame. Reset to ZERO at frame start.
    pub total_force: Vec2,
}

impl ForceAccumulator {
    /// Adds a force vector to the accumulator.
    /// Multiple forces SUM together (superposition principle).
    pub fn add_force(&mut self, force: Vec2) {
        self.total_force += force;
    }

    /// Resets the accumulator to zero. Called at the START of each frame.
    /// If you forget to clear, forces accumulate and objects rocket off!
    pub fn clear(&mut self) {
        self.total_force = Vec2::ZERO;
    }
}

impl Default for ForceAccumulator {
    fn default() -> Self {
        Self { total_force: Vec2::ZERO }
    }
}

/// The collision shape for an entity.
/// Determines what kind of collision detection is used.
#[derive(Component, Debug, Clone)]
pub enum Collider {
    /// A circle centered on the entity's position.
    Circle { radius: f32 },
    /// An axis-aligned bounding box centered on the entity's position.
    Aabb { half_width: f32, half_height: f32 },
}

/// Tags an entity as participating in collision detection and response.
#[derive(Component, Debug, Clone)]
pub struct CollisionTag;
