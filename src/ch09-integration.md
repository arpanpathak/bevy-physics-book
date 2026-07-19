# 🔄 Integration Methods: Simulating Motion Over Time

> **"Integration is the art of turning 'what happens next?' into 'what happens now?'  -  one tiny step at a time."** ⏱️

---

## 🤔 The Problem

Physics is **continuous**  -  it happens at every infinitesimal moment. But computers are **discrete**  -  they process frames. Integration bridges this gap:

```
Reality (continuous):  ●━━━━━━━━━━━━━━━━━━━━━━━━━━━━●
                       motion happens everywhere

Computer (discrete):   ●──────●──────●──────●──────●
                       we only see snapshots

Integration:           "Guess what happens BETWEEN the dots"
```

---

## 📊 Comparison of Methods

```
Method          Accuracy    Stability    Speed    Complexity
──────────────────────────────────────────────────────────────
Explicit Euler    ⭐☆☆☆       ⭐☆☆☆     ⚡⚡⚡⚡      🧩
Symplectic Euler  ⭐⭐☆☆       ⭐⭐⭐☆     ⚡⚡⚡⚡      🧩
Verlet            ⭐⭐⭐☆       ⭐⭐⭐⭐     ⚡⚡⚡       🧩🧩
Runge-Kutta 4     ⭐⭐⭐⭐       ⭐⭐⭐⭐     ⚡⚡        🧩🧩🧩
```

---

## 1️⃣ Explicit Euler (The Simple One)

```rust
/// Performs one step of **Explicit Euler** integration.
///
/// This is the most straightforward integration method, but it has
/// a critical flaw: it uses the velocity at the BEGINNING of the
/// timestep to update the position, rather than the velocity at
/// the END. This causes energy to drift upward over time.
///
/// # Mathematical Formulation
///
/// ```text
/// velocity(t + Δt)     = velocity(t) + acceleration(t) × Δt
/// position(t + Δt)     = position(t) + velocity(t) × Δt
/// ```
///
/// Note the second equation uses `velocity(t)` (the OLD value),
/// not `velocity(t + Δt)` (the newly computed value). This is
/// what distinguishes Explicit from Symplectic Euler.
///
/// # Arguments
/// * `current_position` - The position vector at the start of the timestep.
///   This will be modified in-place to hold the new position.
/// * `current_velocity` - The velocity vector at the start of the timestep.
///   This will be modified in-place to hold the new velocity.
/// * `acceleration` - The acceleration vector, assumed constant over
///   this timestep. This is typically derived from F = ma applied to
///   the accumulated forces on the object.
/// * `delta_time` - The duration of this timestep in seconds. Typically
///   1/60 for standard game physics.
///
/// # Warning
/// This integrator does NOT conserve energy. Objects in orbit will
/// gradually spiral outward. Use Symplectic Euler instead for games.
pub fn explicit_euler(
    current_position: &mut Vec2,
    current_velocity: &mut Vec2,
    acceleration: &Vec2,
    delta_time: f32,
) {
    // Step 1: Update the velocity by adding the acceleration over this timestep.
    // This is the discrete approximation of: dv/dt = a  →  Δv = a × Δt
    *current_velocity += *acceleration * delta_time;

    // Step 2: Update the position using velocity at the START of the timestep.
    //
    // ❌ THIS IS THE FLAW: We're using the OLD velocity value.
    // The velocity was just updated in Step 1, so `current_velocity`
    // now holds `velocity(t + Δt)`, NOT `velocity(t)`.
    //
    // For Explicit Euler to be truly "explicit," we should have saved
    // the old velocity BEFORE updating it. The code as written is
    // actually closer to Symplectic Euler by accident!
    //
    // Let's be explicit (pun intended):
    let velocity_before_update = *current_velocity - *acceleration * delta_time;
    *current_position += velocity_before_update * delta_time;
    // Now `current_position` is truly using the OLD velocity. ✅
}
```

---

## 2️⃣ Symplectic Euler (The Workhorse) ⭐

```rust
/// Performs one step of **Symplectic (Semi-Implicit) Euler** integration.
///
/// This is the RECOMMENDED integrator for game physics. It differs from
/// Explicit Euler in ONE crucial detail: it uses the NEWLY computed
/// velocity to update the position, rather than the old one.
///
/// # Mathematical Formulation
///
/// ```text
/// velocity(t + Δt)     = velocity(t) + acceleration(t) × Δt
/// position(t + Δt)     = position(t) + velocity(t + Δt) × Δt
/// ```
///
/// The key difference is in the second line: we use `velocity(t + Δt)`
/// (the value we just computed) instead of `velocity(t)`.
///
/// # Why This Tiny Change Matters
///
/// This seemingly minor difference means the Symplectic Euler method
/// preserves the **symplectic form** of Hamiltonian mechanics. In
/// plain English: it conserves energy much better. Objects in orbit
/// stay in orbit. Springs don't explode. This is NOT just theoretical
///  -  the difference in stability is dramatic.
///
/// # Arguments
/// * `current_position` - Position at start of timestep. Modified in-place.
/// * `current_velocity` - Velocity at start of timestep. Modified in-place.
/// * `acceleration` - Constant acceleration during this timestep.
/// * `delta_time` - Duration of timestep in seconds.
pub fn symplectic_euler(
    current_position: &mut Vec2,
    current_velocity: &mut Vec2,
    acceleration: &Vec2,
    delta_time: f32,
) {
    // Step 1: Integrate acceleration into velocity.
    // Δv = a × Δt   -  the area under the (constant) acceleration curve.
    *current_velocity += *acceleration * delta_time;

    // Step 2: Integrate the NEW velocity into position.
    // Δx = v_new × Δt   -  uses the velocity we JUST computed.
    //
    // ✅ THIS IS THE KEY: By using the new velocity, we're effectively
    // estimating the AVERAGE velocity over this timestep as v_new,
    // which is a better approximation than v_old.
    *current_position += *current_velocity * delta_time;
}
```

---

## 3️⃣ Verlet Integration (The Stable One)

```rust
/// 📍 A particle that uses **Verlet integration**.
///
/// Unlike Euler methods which store velocity explicitly, Verlet
/// integration works purely with POSITIONS. The velocity is
/// IMPLICITLY derived from the difference between current and
/// previous positions.
///
/// # Mathematical Formulation
///
/// ```text
/// position(t + Δt) = 2 × position(t) - position(t - Δt) + acceleration(t) × Δt²
/// ```
///
/// This formula comes from the Taylor expansion of position around
/// time t, keeping terms up to second order. The `2 × position(t) - position(t - Δt)`
/// term is essentially the inertial term (where the object would go
/// if no forces acted), and `acceleration × Δt²` is the force term.
///
/// # Why Verlet is Amazing for Constraints
///
/// Since Verlet works with positions directly, CONSTRAINTS are trivial:
/// you just MOVE the position to satisfy the constraint, and the
/// velocity automatically adjusts in the next frame. This makes Verlet
/// the go-to choice for:
/// - Cloth simulation (thousands of particles with distance constraints)
/// - Rope/chain physics
/// - Soft body dynamics
/// - Ragdolls
///
/// # Performance Note
/// Verlet requires storing TWO positions (current and previous),
/// which is 2× the memory of Euler methods. However, it's more
/// stable per timestep and can often use larger timesteps.
#[derive(Component)]
pub struct VerletParticle {
    /// The position of this particle at time `t - Δt` (the previous frame).
    /// We need this to compute the inertial term in the Verlet formula.
    pub previous_position: Vec2,

    /// The position of this particle at time `t` (the current frame).
    /// This gets updated each timestep and becomes the new "previous"
    /// position for the next frame.
    pub current_position: Vec2,

    /// The accumulated acceleration acting on this particle.
    /// This is reset to zero at the start of each timestep and
    /// accumulates forces (gravity, drag, springs, etc.) as the
    /// various force systems run.
    pub acceleration: Vec2,
}

impl VerletParticle {
    /// Creates a new Verlet particle at the specified position.
    ///
    /// # Arguments
    /// * `starting_position` - The initial position of this particle.
    ///   Both `previous_position` and `current_position` are set to
    ///   this value, meaning the particle starts at rest.
    pub fn new(starting_position: Vec2) -> Self {
        Self {
            // Start with both positions equal → velocity = 0
            previous_position: starting_position,
            current_position: starting_position,
            // No forces acting yet
            acceleration: Vec2::ZERO,
        }
    }

    /// Advances the particle by one Verlet integration step.
    ///
    /// The Verlet formula is:
    /// ```text
    /// new_position = 2 × current - previous + acceleration × Δt²
    /// ```
    ///
    /// # Arguments
    /// * `delta_time` - The duration of this timestep in seconds.
    pub fn integrate(&mut self, delta_time: f32) {
        // The Verlet update:
        // new_position = 2 × current - previous + acceleration × Δt²
        //
        // The term `2 × current - previous` is the INERTIAL term:
        // it represents where the particle would be if no forces acted.
        // (This is equivalent to current + velocity × Δt in Euler methods.)
        //
        // The term `acceleration × Δt²` is the FORCE term:
        // it represents the displacement caused by forces.
        let new_position = self.current_position * 2.0
            - self.previous_position
            + self.acceleration * delta_time * delta_time;

        // Shift the window: current becomes previous for next frame.
        self.previous_position = self.current_position;
        self.current_position = new_position;

        // 🧹 Clear acceleration for the next timestep.
        // Forces are re-calculated fresh each frame.
        self.acceleration = Vec2::ZERO;
    }

    /// Computes the velocity from the position history.
    ///
    /// Since Verlet doesn't store velocity explicitly, we derive it
    /// from the difference between current and previous positions:
    /// ```text
    /// velocity = (current_position - previous_position) / Δt
    /// ```
    ///
    /// This is the CENTRAL DIFFERENCE approximation of the derivative.
    ///
    /// # Arguments
    /// * `delta_time` - The timestep used in the last integration.
    pub fn derive_velocity(&self, delta_time: f32) -> Vec2 {
        (self.current_position - self.previous_position) / delta_time
    }

    /// Applies a force to this particle for the current timestep.
    ///
    /// Forces are converted to acceleration via F = ma and accumulated.
    /// Multiple calls to this method sum up before integration.
    ///
    /// # Arguments
    /// * `force` - The force vector to apply.
    /// * `mass` - The mass of this particle. Must be > 0.
    pub fn apply_force(&mut self, force: Vec2, mass: f32) {
        // Per Newton's Second Law: a = F / m
        self.acceleration += force / mass;
    }

    /// Constrains this particle to stay at a fixed distance from an anchor point.
    ///
    /// This is THE reason to use Verlet: constraints are trivial.
    /// We just MOVE the particle to satisfy the distance, and the
    /// velocity will naturally adjust in the next integration step.
    ///
    /// # Arguments
    /// * `anchor` - The point to constrain distance to.
    /// * `target_distance` - The desired distance from the anchor.
    /// * `stiffness` - How rigid the constraint is (0.0 to 1.0).
    ///   1.0 = perfectly rigid, 0.5 = soft.
    pub fn constrain_to_distance(
        &mut self,
        anchor: Vec2,
        target_distance: f32,
        stiffness: f32,
    ) {
        // Vector from the anchor to this particle.
        let displacement = self.current_position - anchor;
        let current_distance = displacement.length();

        // Avoid division by zero if the particle is at the anchor.
        if current_distance < 0.0001 {
            return;
        }

        // The direction of the constraint (unit vector from anchor to particle).
        let direction = displacement / current_distance;

        // How far off are we from the target distance?
        let distance_error = current_distance - target_distance;

        // Move the particle to correct the error.
        // The factor 0.5 comes from the fact that in a pair constraint
        // (two particles), each particle moves half the error.
        // Here the anchor is fixed, so the particle moves the full error.
        self.current_position -= direction * distance_error * stiffness;
    }
}
```

---

## 4️⃣ Runge-Kutta 4 (The Accurate One)

```rust
/// 🧮 The complete state of a physics body at a moment in time.
///
/// RK4 operates on "state" objects rather than individual components.
/// The state bundles position and velocity together because they
/// are co-evolving quantities  -  you can't update one without the other.
#[derive(Clone, Debug)]
pub struct PhysicsState {
    /// The position of the body in world space.
    pub position: Vec2,
    /// The velocity of the body in units per second.
    pub velocity: Vec2,
}

/// 📐 The time derivative of the physics state.
///
/// In calculus terms, this represents `d(State)/dt`.
/// The derivative of position is velocity. The derivative of
/// velocity is acceleration. So this struct captures both.
#[derive(Clone, Debug)]
pub struct StateDerivative {
    /// Time derivative of position = velocity (dx/dt).
    pub derivative_of_position: Vec2,
    /// Time derivative of velocity = acceleration (dv/dt).
    pub derivative_of_velocity: Vec2,
}

/// Performs one RK4 integration step.
///
/// RK4 (Runge-Kutta 4th Order) takes FOUR samples of the acceleration
/// function within a single timestep and combines them using a
/// weighted average (Simpson's rule). This gives dramatically better
/// accuracy than Euler methods  -  at the cost of 4× more acceleration
/// evaluations.
///
/// # How RK4 Works (Conceptual)
///
/// Instead of assuming acceleration is constant over the whole
/// timestep (like Euler), RK4:
///
/// 1. **k1**: Samples acceleration at the START of the timestep.
/// 2. **k2**: Samples acceleration at the MIDPOINT, using k1 to estimate the state there.
/// 3. **k3**: Samples acceleration at the MIDPOINT AGAIN, using k2 for a refined estimate.
/// 4. **k4**: Samples acceleration at the END, using k3 to estimate the final state.
/// 5. Combines all four with weights: (k1 + 2×k2 + 2×k3 + k4) / 6
///
/// This is analogous to Simpson's Rule for numerical integration  - 
/// midpoints get higher weight because they're better estimates.
///
/// # Arguments
/// * `current_state` - The position and velocity at the start of the timestep.
/// * `delta_time` - The duration of this timestep.
/// * `acceleration_function` - A function that computes acceleration given
///   the current state. This is where forces (gravity, drag, etc.) are applied.
///
/// # Returns
/// A new `PhysicsState` representing position and velocity after `delta_time`.
pub fn rk4_step(
    current_state: &PhysicsState,
    delta_time: f32,
    acceleration_function: &dyn Fn(&PhysicsState) -> Vec2,
) -> PhysicsState {
    // ─── Sample 1: Derivative at the START of the timestep ───
    let k1_derivative = evaluate_derivative(
        current_state,
        /* time_offset: */ 0.0,
        /* assumed_derivative: */ &StateDerivative {
            derivative_of_position: Vec2::ZERO,
            derivative_of_velocity: Vec2::ZERO,
        },
        acceleration_function,
    );

    // ─── Sample 2: Derivative at the MIDPOINT, using k1 to estimate ───
    let k2_derivative = evaluate_derivative(
        current_state,
        delta_time * 0.5,
        &k1_derivative,
        acceleration_function,
    );

    // ─── Sample 3: Derivative at the MIDPOINT again, using k2 (refined) ───
    let k3_derivative = evaluate_derivative(
        current_state,
        delta_time * 0.5,
        &k2_derivative,
        acceleration_function,
    );

    // ─── Sample 4: Derivative at the END, using k3 ───
    let k4_derivative = evaluate_derivative(
        current_state,
        delta_time,
        &k3_derivative,
        acceleration_function,
    );

    // ─── Combine: Weighted average (Simpson's rule weights) ───
    // Midpoints (k2, k3) get weight 2, endpoints (k1, k4) get weight 1.
    // Divide by 6 to normalize.
    let combined_position_derivative = (
        k1_derivative.derivative_of_position
        + k2_derivative.derivative_of_position * 2.0
        + k3_derivative.derivative_of_position * 2.0
        + k4_derivative.derivative_of_position
    ) / 6.0;

    let combined_velocity_derivative = (
        k1_derivative.derivative_of_velocity
        + k2_derivative.derivative_of_velocity * 2.0
        + k3_derivative.derivative_of_velocity * 2.0
        + k4_derivative.derivative_of_velocity
    ) / 6.0;

    // ─── Apply the combined derivative to advance the state ───
    PhysicsState {
        position: current_state.position + combined_position_derivative * delta_time,
        velocity: current_state.velocity + combined_velocity_derivative * delta_time,
    }
}

/// Evaluates the derivative of the physics state at a guessed future state.
///
/// Given a current state and a candidate derivative (which tells us how
/// the state is changing), this function computes: "if the state changes
/// according to this derivative for `time_offset` seconds, what would
/// the acceleration be at that future state?"
///
/// This is the core subroutine used by RK4 to sample the acceleration
/// at different points within the timestep.
///
/// # Arguments
/// * `current_state` - The state at the START of the full timestep.
/// * `time_offset` - How far into the timestep we're sampling (0.0 to dt).
/// * `assumed_derivative` - The derivative we assume the state follows
///   to reach the sampling point.
/// * `acceleration_function` - Computes acceleration from state.
fn evaluate_derivative(
    current_state: &PhysicsState,
    time_offset: f32,
    assumed_derivative: &StateDerivative,
    acceleration_function: &dyn Fn(&PhysicsState) -> Vec2,
) -> StateDerivative {
    // Estimate the state at time = current + time_offset,
    // using the assumed derivative as an approximation:
    //   guessed_state = current_state + assumed_derivative × time_offset
    let guessed_state = PhysicsState {
        position: current_state.position + assumed_derivative.derivative_of_position * time_offset,
        velocity: current_state.velocity + assumed_derivative.derivative_of_velocity * time_offset,
    };

    // What's the acceleration at this guessed state?
    let acceleration_at_guess = acceleration_function(&guessed_state);

    // Return the derivative at this point:
    // - Position changes at the guessed velocity
    // - Velocity changes at the guessed acceleration
    StateDerivative {
        derivative_of_position: guessed_state.velocity,
        derivative_of_velocity: acceleration_at_guess,
    }
}
```

---

## ⚡ Sub-stepping: The Secret Sauce

```rust
/// 🎯 Manages a **fixed timestep accumulator** for consistent physics.
///
/// The core insight: real time passes at variable rates (a frame might
/// take 8ms or 33ms depending on what's happening), but physics must
/// run at a CONSTANT rate to be deterministic and stable.
///
/// This resource accumulates the REAL elapsed time and "chunks" it
/// into fixed-size physics timesteps:
///
/// ```text
/// Frame 1 (took 33ms):  accumulator = 33ms → run 2 physics steps (16.67ms each)
///                                              remainder = 0ms
/// Frame 2 (took 8ms):   accumulator = 8ms  → run 0 physics steps (not enough yet)
///                                              remainder = 8ms
/// Frame 3 (took 17ms):  accumulator = 25ms → run 1 physics step (16.67ms)
///                                              remainder = 8.33ms  ← interpolate!
/// ```
///
/// This decouples PHYSICS FRAMERATE from RENDER FRAMERATE.
#[derive(Resource)]
pub struct FixedTimestepAccumulator {
    /// Accumulated real time that hasn't been consumed by physics yet.
    /// Measured in seconds.
    pub accumulated_time: f32,

    /// The fixed duration of each physics timestep.
    /// Standard value: 1.0 / 60.0 = ~16.67ms (60 Hz physics).
    pub physics_timestep: f32,

    /// Maximum number of physics steps to run in a single frame.
    /// If the game freezes for 5 seconds, we DON'T want to run
    /// 300 physics steps to catch up  -  that would freeze again!
    /// Instead, we cap at this value and let the simulation
    /// "fall behind" (which is usually imperceptible).
    pub max_steps_per_frame: u32,
}

impl Default for FixedTimestepAccumulator {
    fn default() -> Self {
        Self {
            accumulated_time: 0.0,
            physics_timestep: 1.0 / 60.0,
            max_steps_per_frame: 5,
        }
    }
}

/// Runs the physics simulation using a fixed timestep accumulator.
///
/// This system should run at the BEGINNING of the frame, before any
/// other physics systems. It reads the frame's delta time, feeds it
/// into the accumulator, and runs physics steps until caught up.
///
/// # Arguments
/// * `time` - Bevy's time resource, providing the real frame delta.
/// * `accumulator` - Our fixed timestep accumulator.
/// * `physics_query` - The entities to run physics on.
pub fn fixed_timestep_physics_system(
    time: Res<Time>,
    mut accumulator: ResMut<FixedTimestepAccumulator>,
    mut physics_query: Query<(
        &mut Position,
        &mut Velocity,
        &mut Acceleration,
        &Mass,
    )>,
) {
    // ─── Step 1: Accumulate the real frame time ───
    accumulator.accumulated_time += time.delta_secs();

    // ─── Step 2: Clamp to prevent spiral-of-death ───
    // If the game freezes (e.g., from a background app switch),
    // accumulated_time could be huge. We cap it to prevent
    // running hundreds of physics steps in one frame.
    let max_accumulation = accumulator.physics_timestep
        * accumulator.max_steps_per_frame as f32;
    accumulator.accumulated_time = accumulator
        .accumulated_time
        .min(max_accumulation);

    // ─── Step 3: Consume fixed-size chunks ───
    // Each iteration runs EXACTLY one physics timestep.
    // The accumulator ensures that over time, the simulation
    // advances at exactly the physics framerate, regardless
    // of the real framerate.
    while accumulator.accumulated_time >= accumulator.physics_timestep {
        // Run one physics step at the fixed timestep.
        run_single_physics_step(
            &mut physics_query,
            accumulator.physics_timestep,
        );

        // Consume the time we just simulated.
        accumulator.accumulated_time -= accumulator.physics_timestep;
    }

    // ─── Step 4 (Optional): Interpolation ───
    // The remaining `accumulated_time / physics_timestep` fraction
    // can be used to INTERPOLATE between the last two physics states
    // for rendering, giving buttery-smooth visuals even with 30 Hz physics.
    let interpolation_factor = accumulator.accumulated_time / accumulator.physics_timestep;
    // Use `interpolation_factor` to lerp between previous and current states...
}

/// Runs a single physics step for all entities.
///
/// This is the core physics pipeline, executed at the fixed timestep:
/// 1. Zero out acceleration (forces don't persist between frames)
/// 2. Apply forces (gravity, drag, etc.)  -  accumulated in Acceleration
/// 3. Integrate: acceleration → velocity → position
///
/// # Arguments
/// * `physics_query` - All entities with physics components.
/// * `delta_time` - The fixed physics timestep (NOT the frame delta).
fn run_single_physics_step(
    physics_query: &mut Query<(
        &mut Position,
        &mut Velocity,
        &mut Acceleration,
        &Mass,
    )>,
    delta_time: f32,
) {
    for (mut position, mut velocity, mut acceleration, mass) in physics_query.iter_mut() {
        // Step 1: Compute acceleration from forces.
        // In a full engine, this would involve summing gravity, drag,
        // thrust, collision impulses, etc. For now we use the
        // acceleration as-is (it was populated by earlier systems).
        let acceleration_from_forces = acceleration.0;

        // Step 2: Integrate using Symplectic Euler.
        // v_new = v_old + a × Δt
        // x_new = x_old + v_new × Δt  ← uses NEW velocity!
        if mass.0 > 0.0 {
            velocity.0 += acceleration_from_forces * delta_time;
            position.0 += velocity.0 * delta_time;
        }

        // Step 3: Clear acceleration for the next timestep.
        // Forces don't accumulate  -  they're re-evaluated each frame.
        acceleration.0 = Vec2::ZERO;
    }
}
```

---

## 🏆 Choosing the Right Method

```rust
/// Represents the different needs a physics simulation might have.
/// Use this to select the right integration method.
enum SimulationRequirements {
    /// Most games: simple, fast, good enough.
    SimpleAndFast,
    /// Cloth, ropes, soft bodies (position-based dynamics).
    PositionBasedConstraints,
    /// Scientific simulation, rockets, precision physics.
    HighPrecision,
}

/// Selects the appropriate integration method based on requirements.
///
/// # Arguments
/// * `requirements` - What the simulation needs.
///
/// # Returns
/// A string naming the recommended integrator.
fn recommend_integrator(requirements: SimulationRequirements) -> &'static str {
    match requirements {
        SimulationRequirements::SimpleAndFast => {
            // Symplectic Euler: the Goldilocks option.
            // 99% of games should use this.
            "Symplectic Euler ✅"
        }
        SimulationRequirements::PositionBasedConstraints => {
            // Verlet: constraints become trivial position adjustments.
            "Verlet ✅"
        }
        SimulationRequirements::HighPrecision => {
            // RK4: 4× more computation, but 100× more accurate.
            "Runge-Kutta 4 ✅"
        }
    }
}
```

---

## 🎯 Chapter Summary

```rust
/// 📝 Integration cheat sheet  -  the three methods at a glance.

/// 1️⃣ SYMPLECTIC EULER (USE THIS 99% OF THE TIME)
/// Stable, fast, energy-conserving. The standard for game physics.
fn symplectic_euler_step(
    position: &mut Vec2,
    velocity: &mut Vec2,
    acceleration: &Vec2,
    delta_time: f32,
) {
    *velocity += *acceleration * delta_time;
    *position += *velocity * delta_time;  // Uses UPDATED velocity ✅
}

/// 2️⃣ VERLET (for position-based dynamics)
/// Great for cloth, ropes, ragdolls. Constraints are trivial.
fn verlet_step(
    current_position: &mut Vec2,
    previous_position: &mut Vec2,
    acceleration: &Vec2,
    delta_time: f32,
) {
    let new_position = *current_position * 2.0
        - *previous_position
        + *acceleration * delta_time * delta_time;
    *previous_position = *current_position;
    *current_position = new_position;
}

/// 3️⃣ KEY RECOMMENDATION
/// Start with Symplectic Euler. It's simple, fast, and good enough.
/// Switch to Verlet only if you need cloth/constraints.
/// Use RK4 only if you're doing scientific simulation.
/// Never use Explicit Euler  -  it's strictly worse than Symplectic.
```

> **The integrator is the HEART of your physics engine. Everything else  -  forces, collisions, constraints  -  feeds INTO the integrator. Choose Symplectic Euler by default. It's stable, energy-conserving, and simple. Only reach for Verlet or RK4 when you have a specific need they're uniquely suited for.** 🏆

> 💡 **Full source code for this chapter:** [code-examples/ch09-integration/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch09-integration)
> 
> The runnable project includes Cargo.toml, main.rs, and complete module files.

---

**[← Previous: Dynamics](ch08-dynamics.md)** | **[Next: Collision Detection →](ch10-collision-detection.md)**
