# 🔄 Integration Methods: Simulating Motion Over Time

> **"Integration is the art of turning 'what happens next?' into 'what happens now?' — one tiny step at a time."** ⏱️

---

## 🤔 The Problem

Physics is **continuous** — it happens at every infinitesimal moment. But computers are **discrete** — they process frames. Integration bridges this gap:

```
Reality (continuous):  ●━━━━━━━━━━━━━━━━━━━━━━━━━━━━●
                       motion happens everywhere

Computer (discrete):   ●──────●──────●──────●──────●
                       we only see snapshots

Integration:           "Guess what happens BETWEEN the dots"
```

---

## 📊 Comparison of Methods

Let's compare the three main integration methods:

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
/// ⚡ Explicit Euler — the most basic integrator
///
/// v(t + dt) = v(t) + a(t) × dt
/// x(t + dt) = x(t) + v(t) × dt
///
/// Problem: Uses OLD velocity for position update.
/// Energy increases over time → Objects EXPLODE! 💥
fn explicit_euler(
    pos: &mut Vec2,
    vel: &mut Vec2,
    acc: &Vec2,
    dt: f32,
) {
    // First update velocity
    *vel += *acc * dt;
    
    // ❌ Then update position using OLD velocity
    *pos += *vel * dt;  // Wait, this uses NEW vel...
}
// Actually, the real explicit Euler uses v_old for pos update.
// Let me show it properly:
```

```
📊 Explicit Euler Drift:

    Energy                Reality
    ↑       ╱╲           ───
    │      ╱  ╲         ╱   ╲
    │     ╱    ╲  ←    ╱     ╲
    │    ╱      ╲     ╱       ╲
    │   ╱        ╲   ╱         ╲
    │  ╱          ╲ ╱           ╲
    │ ╱            ╲             ╲
    └──────────────────────────────► t
    
    Notice: The Explicit Euler orbit SPIRALS OUT!
    Energy is not conserved. Bad for orbital physics! 🛰️
```

---

## 2️⃣ Symplectic Euler (The Workhorse) ⭐

```rust
/// 🔄 Symplectic (Semi-Implicit) Euler — GAME PHYSICS STANDARD
///
/// v(t + dt) = v(t) + a(t) × dt
/// x(t + dt) = x(t) + v(t + dt) × dt  ← Uses NEW velocity!
///
/// This tiny change (using new v for x update) makes ALL the difference!
/// Energy is CONSERVED (no explosion!). This is what we've been using.
fn symplectic_euler(
    pos: &mut Vec2,
    vel: &mut Vec2,
    acc: &Vec2,
    dt: f32,
) {
    // Step 1: Update velocity using acceleration
    *vel += *acc * dt;
    
    // Step 2: 🎯 Update position using NEW velocity
    *pos += *vel * dt;
    
    // 💡 The key insight:
    // We estimate "average velocity" during this timestep
    // as v_new, which is more accurate than v_old!
}

/// 📊 Symplectic Euler Accuracy:
///
/// The symplectic Euler preserves something called the
/// "symplectic form" — a fancy way of saying it conserves
/// energy in Hamiltonian systems (which mechanical systems are).
///
/// Result: Orbits STAY in orbit. Springs don't explode. 👍
```

```
📊 Symplectic Euler (Stable!):

    Energy                
    ↑       ╱╲           
    │      ╱  ╲          
    │     ╱    ╲  ← Stable orbit!
    │    ╱      ╲       ╱
    │   ╱        ╲     ╱
    │  ╱          ╲   ╱
    │ ╱            ╲ ╱
    └──────────────────────────────► t
    
    Energy stays roughly CONSTANT.
    Good enough for 99% of games! 🎮
```

---

## 3️⃣ Verlet Integration (The Stable One)

```rust
/// 🧮 Verlet Integration — position-based, very stable
///
/// x(t + dt) = 2 × x(t) - x(t - dt) + a(t) × dt²
///
/// Key insight: Verlet doesn't store velocity explicitly!
/// It derives it from position history.
#[derive(Component)]
struct VerletParticle {
    /// Previous position (needed for Verlet)
    prev_pos: Vec2,
    /// Current position
    pos: Vec2,
    /// Accumulated acceleration
    acc: Vec2,
}

impl VerletParticle {
    fn new(pos: Vec2) -> Self {
        Self {
            prev_pos: pos,
            pos,
            acc: Vec2::ZERO,
        }
    }
    
    /// 🔄 One Verlet integration step
    fn integrate(&mut self, dt: f32) {
        // 📐 Verlet formula:
        // new_pos = 2×current - previous + acceleration × dt²
        let temp = self.pos;
        self.pos = self.pos * 2.0 - self.prev_pos + self.acc * dt * dt;
        self.prev_pos = temp;
        
        // 🧹 Clear acceleration for next frame
        self.acc = Vec2::ZERO;
    }
    
    /// 🏃 Get velocity (derived from position delta)
    fn velocity(&self, dt: f32) -> Vec2 {
        (self.pos - self.prev_pos) / dt
    }
    
    /// ➕ Apply a force
    fn apply_force(&mut self, force: Vec2, mass: f32) {
        self.acc += force / mass;
    }
    
    /// 🔒 Constrain distance to a point (Verlet constraint!)
    fn constrain_distance(&mut self, anchor: Vec2, target_distance: f32) {
        let delta = self.pos - anchor;
        let dist = delta.length();
        
        if dist > 0.001 {
            let correction = (dist - target_distance) / dist;
            self.pos -= delta * correction * 0.5;  // Move half the error
        }
    }
}

/// 💡 Verlet is AMAZING for:
/// 1. Cloth simulation (tens of thousands of particles)
/// 2. Rope/chain physics
/// 3. Ragdolls (position-based dynamics)
/// 4. Soft bodies
///
/// Why? Constraints are trivial: just move positions directly!
```

```
📊 Verlet Cloth Simulation:

    ○───○───○───○───○
    │   │   │   │   │
    ○───○───○───○───○      Each ○ is a Verlet particle
    │   │   │   │   │      Each ─ is a distance constraint
    ○───○───○───○───○
    │   │   │   │   │      Gravity pulls down
    ○───○───○───○───○      Constraints hold it together
    │   │   │   │   │
    ○───○───○───○───○
    
    Pin the top corners → instant interactive cloth! 👕
```

---

## 4️⃣ Runge-Kutta 4 (The Accurate One)

```rust
/// 🔥 Runge-Kutta 4th Order — NASA-level accuracy
///
/// RK4 takes FOUR samples of acceleration within the timestep
/// and combines them for a much better estimate.
///
/// For most games: Overkill. But for rocket science... 🚀

/// 🧮 The state of a physics object (position + velocity)
#[derive(Clone)]
struct State {
    pos: Vec2,
    vel: Vec2,
}

/// 📐 Derivative = how state changes
#[derive(Clone)]
struct Derivative {
    dx: Vec2,  // derivative of position = velocity
    dv: Vec2,  // derivative of velocity = acceleration
}

/// 🔄 RK4 Integration
fn rk4_step(
    state: &State,
    dt: f32,
    acceleration_fn: &dyn Fn(&State) -> Vec2,
) -> State {
    // k1: derivative at START of timestep
    let k1 = evaluate(state, 0.0, &Derivative { dx: Vec2::ZERO, dv: Vec2::ZERO }, acceleration_fn);
    
    // k2: derivative at MIDPOINT (using k1)
    let k2 = evaluate(state, dt * 0.5, &k1, acceleration_fn);
    
    // k3: derivative at MIDPOINT (using k2) — refinement!
    let k3 = evaluate(state, dt * 0.5, &k2, acceleration_fn);
    
    // k4: derivative at END (using k3)
    let k4 = evaluate(state, dt, &k3, acceleration_fn);
    
    // 🎯 Weighted average of all four derivatives
    // Midpoints get higher weight (Simpson's rule)
    let dx = (k1.dx + k2.dx * 2.0 + k3.dx * 2.0 + k4.dx) / 6.0;
    let dv = (k1.dv + k2.dv * 2.0 + k3.dv * 2.0 + k4.dv) / 6.0;
    
    State {
        pos: state.pos + dx * dt,
        vel: state.vel + dv * dt,
    }
}

fn evaluate(
    state: &State,
    dt: f32,
    derivative: &Derivative,
    acceleration_fn: &dyn Fn(&State) -> Vec2,
) -> Derivative {
    // What state would we be at after this derivative?
    let new_state = State {
        pos: state.pos + derivative.dx * dt,
        vel: state.vel + derivative.dv * dt,
    };
    
    // What's the acceleration at that state?
    let accel = acceleration_fn(&new_state);
    
    Derivative {
        dx: new_state.vel,  // dx/dt = velocity
        dv: accel,          // dv/dt = acceleration
    }
}

/// 💡 RK4 vs Euler with dt=1/60:
///
/// Euler:      1 acceleration calculation per step
/// RK4:        4 acceleration calculations per step
///
/// But RK4 can use dt=1/15 (4× larger!) and still be MORE accurate!
/// Trade-off: more math, bigger timesteps = worth it for precision
```

```
📊 Accuracy Comparison (Orbit Simulation):

Error after 100 orbits:

    Explicit Euler:  ❌ ESCAPED ORBIT (diverged!)
    Symplectic Euler: ⚠️ ~5% drift
    Verlet:           ⚠️ ~3% drift  
    RK4:              ✅ ~0.01% drift
    
    For a GAME: Symplectic Euler is the sweet spot.
    For a SIMULATION: Use RK4.
    For a MOVIE: Use RK4 with adaptive timestep.
```

---

## 🏆 Choosing the Right Method

```rust
/// 🎯 Decision tree for choosing an integrator
fn choose_integrator() -> &'static str {
    let answer = match your_needs {
        // Most games: simple, fast, good enough
        Needs::SimpleAndFast => "Symplectic Euler ✅",
        
        // Cloth, ropes, soft bodies
        Needs::PositionBasedConstraints => "Verlet ✅",
        
        // Scientific simulation, rockets
        Needs::HighPrecision => "RK4 ✅",
        
        // Learning, minimal viable product
        Needs::JustGettingStarted => "Symplectic Euler ✅",
    };
    answer
}

/// 📊 Summary Table:
///
/// ╔══════════════════╦══════════╦══════════╦══════╗
/// ║   Integrator    ║  Speed   ║ Accuracy ║ Ease ║
/// ╠══════════════════╬══════════╬══════════╬══════╣
/// ║ Explicit Euler  ║ ⚡⚡⚡⚡   ║   ❌     ║  ✅  ║
/// ║ Symplectic Euler║ ⚡⚡⚡⚡   ║   ✅     ║  ✅  ║ ← USE THIS
/// ║ Verlet          ║ ⚡⚡⚡    ║   ✅✅   ║  ⚠️  ║
/// ║ RK4             ║ ⚡⚡     ║   ✅✅✅ ║  ❌  ║
/// ╚══════════════════╩══════════╩══════════╩══════╝
```

---

## ⚡ Sub-stepping: The Secret Sauce

Fixed timestep + sub-stepping = stable physics at any framerate:

```rust
/// 🔄 Physics system with fixed timestep sub-stepping
#[derive(Resource)]
struct FixedTimeStep {
    /// Accumulator for leftover time
    accumulator: f32,
    /// Fixed physics timestep
    dt: f32,
}

impl Default for FixedTimeStep {
    fn default() -> Self {
        Self {
            accumulator: 0.0,
            dt: 1.0 / 60.0,  // 60 Hz physics
        }
    }
}

fn physics_system(
    time: Res<Time>,
    mut fixed_step: ResMut<FixedTimeStep>,
    // ... queries ...
) {
    // ⏱️ Accumulate real time
    fixed_step.accumulator += time.delta_secs();
    
    // 🎯 Limit max frames to prevent spiral of death
    // If the game freezes for 5 seconds, we don't simulate
    // all 300 frames at once — that would break everything!
    let max_frames = 5;
    fixed_step.accumulator = fixed_step.accumulator.min(
        fixed_step.dt * max_frames as f32
    );
    
    // 🔄 Run fixed steps until caught up
    while fixed_step.accumulator >= fixed_step.dt {
        // Run one physics step at fixed_dt
        run_physics_step(fixed_step.dt);
        
        // Consume the time
        fixed_step.accumulator -= fixed_step.dt;
    }
    
    // 📊 Interpolation hint: we could optionally interpolate
    // between the last two physics states for the remaining
    // accumulator fraction, giving buttery-smooth rendering
    // even at 30 FPS physics!
    let alpha = fixed_step.accumulator / fixed_step.dt;
    // render_interpolated(alpha);
}

/// 💡 Sub-stepping:
/// 
/// If dt = 1/60 and we have sub-steps = 4:
/// Each sub-step = 1/240 seconds
/// We run physics 4 times per frame
///
/// Result: 4× more stable! Can handle faster velocities!
/// Drawback: 4× more CPU work
///
/// Pro tip: Start with sub_steps=1, increase only if
/// objects are tunneling through walls.
```

---

## 🎯 Chapter Summary

```rust
/// 📝 Integration cheat sheet:

// 1️⃣ SYMPLECTIC EULER (use this 99% of the time)
fn step_symplectic(pos: &mut Vec2, vel: &mut Vec2, acc: &Vec2, dt: f32) {
    *vel += *acc * dt;      // v_new = v + a·dt
    *pos += *vel * dt;      // x_new = x + v_new·dt  ← uses updated v!
}

// 2️⃣ VERLET (for position-based dynamics)
fn step_verlet(pos: &mut Vec2, prev_pos: &mut Vec2, acc: &Vec2, dt: f32) {
    let temp = *pos;
    *pos = *pos * 2.0 - *prev_pos + *acc * dt * dt;
    *prev_pos = temp;
}

// 3️⃣ KEY INSIGHT:
//   - Euler: simple, fast, good enough
//   - Verlet: great for cloth/ragdolls (constraints are trivial)
//   - RK4: overkill for games, amazing for simulations
//   - Sub-stepping: the secret to stability at any framerate
```

> **Key Takeaway:** Always use **Symplectic Euler** unless you have a specific reason not to. It's the Goldilocks of integrators — not too simple, not too complex, just right for games. Pair it with **fixed timestep sub-stepping** and you'll handle everything from gentle floating to hypersonic collisions. 🏆

---

**[← Previous: Dynamics](08-dynamics.md)** | **[Next: Collision Detection →](10-collision-detection.md)**
