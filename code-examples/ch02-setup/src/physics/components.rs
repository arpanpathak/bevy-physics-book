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

// ═══════════════════════════════════════════════════════════
// 📍 CORE KINEMATIC COMPONENTS
// ═══════════════════════════════════════════════════════════

/// Represents the spatial location of an entity in 2D world space.
///
/// In mathematical terms, this is a VECTOR from the world origin (0,0)
/// to the entity's position. Adding a Velocity to this over time
/// produces smooth movement.
///
/// UNITS: pixels (or meters, depending on your game's scale).
///
/// WHY NOT USE BEVY'S TRANSFORM?
/// Bevy's Transform bundles position + rotation + scale and is tied
/// to the rendering hierarchy. For physics, we want RAW position data
/// that we can modify every frame without triggering hierarchy updates.
/// We sync Position → Transform only for rendering (see sync systems).
#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    /// The x,y coordinates in world space.
    /// x increases to the right, y increases upward (in Bevy's 2D system).
    pub value: Vec2,
}

impl Position {
    /// Creates a new Position at the given (x, y) coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            value: Vec2::new(x, y),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            value: Vec2::ZERO,
        }
    }
}

/// Represents the RATE OF CHANGE of position over time.
///
/// In calculus terms: v = dr/dt (velocity is the derivative of position).
/// In discrete terms: position_change = velocity × delta_time.
///
/// A positive x value means "moving right."
/// A negative y value means "moving downward" (in Bevy's coordinate system).
///
/// UNITS: pixels per second (px/s).
///
/// 💡 THINK OF VELOCITY AS "WHERE THE OBJECT WANTS TO GO NEXT."
/// If velocity is (50, 0), the object wants to move right at 50 px/s.
/// Physics integration makes this happen.
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity {
    /// The velocity vector in world space.
    pub value: Vec2,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            value: Vec2::new(x, y),
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            value: Vec2::ZERO,
        }
    }
}

/// Represents the RATE OF CHANGE of velocity over time.
///
/// In calculus terms: a = dv/dt = d²r/dt² (second derivative of position).
/// In physics: a = F/m (Newton's Second Law).
///
/// Acceleration is where FORCES live. Gravity, drag, thrust, and
/// collision impulses all contribute to acceleration, which then
/// modifies velocity through integration.
///
/// UNITS: pixels per second squared (px/s²).
///
/// IMPORTANT: Acceleration is RECALCULATED every frame.
/// It does NOT persist between frames. Each frame starts fresh,
/// forces are applied, and acceleration is computed from F = ma.
#[derive(Component, Debug, Clone, Copy)]
pub struct Acceleration {
    /// The acceleration vector. Reset to ZERO at the start of each frame.
    pub value: Vec2,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self {
            value: Vec2::ZERO,
        }
    }
}

// ═══════════════════════════════════════════════════════════
// ⚖️ PHYSICAL PROPERTIES
// ═══════════════════════════════════════════════════════════

/// Represents the INERTIA of an entity - how much it resists acceleration.
///
/// Newton's Second Law: F = m × a, so a = F / m.
/// Higher mass = lower acceleration for the same force.
///
/// SPECIAL VALUES:
///   mass = 1.0  → Normal dynamic object (responds to forces).
///   mass = 10.0 → Heavy object (hard to push).
///   mass = 0.0  → STATIC / immovable object (walls, floor).
///                  Division by zero is prevented in systems.
///   mass < 0.0  → INVALID. Don't use negative mass.
#[derive(Component, Debug, Clone, Copy)]
pub struct Mass {
    /// The mass value in arbitrary mass units.
    pub value: f32,
}

impl Default for Mass {
    fn default() -> Self {
        Self { value: 1.0 }
    }
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
        Self {
            total_force: Vec2::ZERO,
        }
    }
}
