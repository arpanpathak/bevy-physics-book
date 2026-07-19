# 💥 Dynamics: Forces & Newton's Laws

> **"Kinematics describes WHAT motion looks like  -  'the object fell down.' Dynamics explains WHY  -  'gravity exerted a force of 9.8 N/kg downward.' The equation F = ma is the bridge between the two."** 💪

---

## 🎯 The Bridge Between Chapters

In the previous chapter (Kinematics), we learned: `velocity += acceleration × dt; position += velocity × dt`. But we had a mystery ingredient: **Where does acceleration come from?**

```
KINEMATICS:  "Given acceleration, find velocity and position."
DYNAMICS:    "Find acceleration from forces and mass."

  The pipeline:
  
  Forces --> Sum forces --> a = F/m --> Integrate --> Move!
  (causes)    (accumulate)  (Newton II)  (kinematics)  (result)
```

Dynamics answers: **what forces are acting on this object, and how do they change its motion?**

---

## 📜 Newton's Three Laws (The Full Explanation)

### 1️⃣ Law of Inertia  -  "Objects Resist Change"

```
An object at rest stays at rest.
An object in motion stays in motion (same speed, same direction).
UNLESS acted upon by a net external force.

WHAT THIS MEANS IN CODE:
  • If no forces act on an object, velocity stays constant.
  • This is why we CLEAR forces each frame  -  without new forces,
    nothing should accelerate.
  • Inertia is NOT a force  -  it's the ABSENCE of acceleration
    when no net force exists.

GAME EXAMPLE: A puck on an ice rink (no friction)
  • Frame 1: puck moving at (50, 0), no forces -> velocity stays (50, 0)
  • Frame 2: puck moving at (50, 0), no forces -> velocity stays (50, 0)
  • Frame 100: STILL moving at (50, 0) -> infinite slide!
  
  In a real game, we ADD friction (a force) to make it stop.
```

### 2️⃣ F = ma  -  "The Central Equation"

```
The net force on an object equals its mass times its acceleration.

  F = m × a     ->     a = F / m     (the form we actually use)
  
  Force is a VECTOR. Mass is a SCALAR.
  Acceleration is a VECTOR in the SAME direction as the net force.

INTUITION:
  Push an empty shopping cart:   it accelerates easily (low mass)
  Push a full shopping cart:     it accelerates slowly (high mass)  
  Push a wall:                   it doesn't accelerate at all (∞ mass -> static)

  SAME PUSH, DIFFERENT MASSES -> DIFFERENT ACCELERATIONS.

CODE IMPACT:
  • mass = 1.0  -> a = F (force and acceleration are equal numerically)
  • mass = 10.0 -> a = F/10 (heavy object, small acceleration)
  • mass = 0.0  -> a = undefined! We treat this as "static object"
```

### 3️⃣ Action-Reaction  -  "Forces Are Symmetric"

```
For every action force, there's an equal and opposite reaction force.

This means forces ALWAYS come in pairs:
  • You push on a wall -> the wall pushes back with EQUAL force
  • The ground pushes up on you with the same force gravity pulls down
  • When objects collide, the force on A = -force on B

CODE IMPACT:
  • In collision response: impulse_on_A = -impulse_on_B
  • This conserves momentum: m₁·v₁ + m₂·v₂ = constant (before = after)
  • Without this, objects would gain or lose momentum from nothing
```

---

## ⚡ The Force Accumulation Pipeline

Every frame, every physics object follows this EXACT pipeline:

```
+-----------------------------------------------------------------+
|                    ONE FRAME OF PHYSICS                          |
+-----------------------------------------------------------------+
|                                                                   |
|  1️⃣  CLEAR FORCES                                               |
|      ForceAccumulator = (0, 0)                                   |
|      ↳ Why? Forces don't persist between frames. If you pushed  |
|        an object last frame, it's still moving (inertia), but   |
|        the PUSH is gone. Only velocity persists.                 |
|                                                                   |
|  2️⃣  ACCUMULATE FORCES                                          |
|      ForceAccumulator += gravity_force                           |
|      ForceAccumulator += drag_force                              |
|      ForceAccumulator += player_input_force                      |
|      ForceAccumulator += collision_impulse                       |
|      ↳ Every system that generates a force adds it here.        |
|        At the end, ForceAccumulator = TOTAL force this frame.    |
|                                                                   |
|  3️⃣  F = ma  ->  a = F / m                                      |
|      Acceleration = ForceAccumulator / Mass                      |
|      ↳ This is Newton's Second Law. Converts force into         |
|        the acceleration that kinematics needs.                   |
|                                                                   |
|  4️⃣  INTEGRATE (kinematics)                                     |
|      Velocity += Acceleration × dt                               |
|      Position += Velocity × dt                                   |
|      ↳ Now the acceleration becomes motion.                      |
|        This is the kinematics chapter in action.                 |
|                                                                   |
+-----------------------------------------------------------------+
```

### Why Forces MUST Be Cleared Each Frame

```rust
/// ❌ THE BUG: Not clearing forces.
///
/// Frame 1: gravity applies (0, -500) to the force accumulator.
///          Total force = (0, -500). a = F/m = (0, -500). ✅ Good.
///
/// Frame 2: gravity applies (0, -500) AGAIN.
///          But we DIDN'T clear! Total force = (0, -500) + (0, -500) = (0, -1000).
///          a = F/m = (0, -1000). The object accelerates TWICE as fast! ❌
///
/// Frame 3: Total force = (0, -1500). Even faster! ❌
/// Frame 4: Total force = (0, -2000). RUNAWAY PHYSICS! 💥
///
/// ✅ THE FIX: Clear forces at the START of each frame.
///            Every frame starts with a clean slate.
pub fn clear_force_accumulators(
    mut force_query: Query<&mut ForceAccumulator>,
) {
    for mut force_accumulator in force_query.iter_mut() {
        force_accumulator.clear(); // Set to (0, 0)  -  FRESH START
    }
}
```

---

## 🌍 Types of Forces (With Full Derivations)

### 🏋️ Gravity: The Universal Force

```rust
/// GRAVITY applies a constant downward force to all objects with mass.
///
/// MATHEMATICAL DERIVATION:
///   F_gravity = m × g
///   
///   Where g = gravitational acceleration (a VECTOR pointing downward).
///   On Earth: g ≈ (0, -9.81) m/s²
///   In game units: g ≈ (0, -500) to (0, -1000) pixels/s²
///
///   Since a = F/m = (m × g) / m = g, ALL objects fall at the SAME rate
///   regardless of mass! A feather and a boulder fall identically
///   (in vacuum  -  air resistance changes this).
///
///   This is why astronauts on the Moon dropped a hammer and feather
///   together  -  they hit the ground simultaneously.
pub fn apply_gravity_to_all_objects(
    mut force_query: Query<(&Mass, &mut ForceAccumulator)>,
    physics_settings: Res<PhysicsSettings>,
) {
    for (mass, mut force_accumulator) in force_query.iter_mut() {
        // F = m × g  -  gravity force is proportional to mass
        let gravitational_force = physics_settings.gravity * mass.0;
        force_accumulator.add_force(gravitational_force);
    }
}

/// 💡 GAME FEEL: Adjust gravity for the desired feel.
///
///   Realistic gravity (Earth):    g = (0, -9.81)  -> slow, floaty
///   Platformer gravity (Mario):   g = (0, -60)    -> snappy, responsive
///   Moon gravity:                  g = (0, -1.62)  -> floaty jumps
///   "Juice" gravity:              g = (0, -100)   -> dramatic falls
///
/// There's no "correct" gravity for a game  -  only what feels right.
```

### 🌬️ Linear Drag (Damping): Why Things Stop

```rust
/// LINEAR DRAG applies a force that opposes velocity.
///
/// MATHEMATICAL DERIVATION:
///   F_drag = -b × v
///
///   Where:
///     b = damping coefficient (how much resistance)
///     v = current velocity
///     - = opposite direction (if moving right, drag pushes left)
///
///   This creates EXPONENTIAL DECAY of velocity:
///     v(t) = v₀ × e^(-b × t / m)
///
///   After one second:  v = v₀ × e^(-b/m)
///   After two seconds: v = v₀ × e^(-2b/m)
///   After three seconds: v ≈ 0 (effectively stopped)
///
///   Higher b -> faster decay -> objects stop sooner.
pub fn apply_linear_drag_to_objects(
    mut force_query: Query<(&Velocity, &mut ForceAccumulator, &LinearDamping)>,
) {
    for (velocity, mut force_accumulator, damping) in force_query.iter_mut() {
        // Drag force opposes velocity. Moving right? Drag pushes left.
        // Faster movement? Stronger drag.
        let drag_force = -velocity.0 * damping.coefficient;
        force_accumulator.add_force(drag_force);
    }
}

/// TUNING GUIDE for damping coefficient:
///
///   b = 0.0    -> No drag. Object slides forever (in space).
///   b = 0.1    -> Very subtle drag. Ice-like movement.
///   b = 1.0    -> Noticeable drag. Movement feels "heavy."
///   b = 5.0    -> Strong drag. Like moving through honey.
///   b = 10.0   -> Extreme drag. Stops almost immediately.
///
/// For most games, start with b = 0.5 and adjust until it feels right.
#[derive(Component)]
pub struct LinearDamping {
    /// Drag coefficient. Higher = more resistance.
    pub coefficient: f32,
}

impl Default for LinearDamping {
    fn default() -> Self {
        Self { coefficient: 0.5 }
    }
}
```

### 🧲 Spring Force (Hooke's Law): Elasticity

```rust
/// SPRING FORCE pulls an object toward an anchor point.
///
/// MATHEMATICAL DERIVATION (Hooke's Law):
///   F_spring = -k × (x - x₀)
///
///   Where:
///     k = spring constant (stiffness)
///     x = current position
///     x₀ = rest position (where the spring is relaxed)
///     (x - x₀) = DISPLACEMENT from rest
///
///   The force is PROPORTIONAL to displacement. Pull it twice as far?
///   It pulls back twice as hard.
///
///   This creates SIMPLE HARMONIC MOTION:
///     x(t) = A × cos(ωt + φ)
///     where ω = √(k/m) = natural frequency
///     and T = 2π/ω = period of oscillation
///
///   Higher k -> faster oscillation, stiffer feel.
///   Higher m -> slower oscillation, heavier feel.
pub fn apply_spring_force_to_attached_object(
    object_position: Vec2,
    spring_anchor: Vec2,
    spring_stiffness: f32,
    force_accumulator: &mut ForceAccumulator,
) {
    // Step 1: Compute the displacement from rest position.
    let displacement_from_anchor = object_position - spring_anchor;
    
    // Step 2: Compute the spring force using Hooke's Law.
    // The force ALWAYS points toward the anchor (hence the negative).
    let spring_force = -displacement_from_anchor * spring_stiffness;
    
    // Step 3: Apply the force.
    force_accumulator.add_force(spring_force);
}
```

```
SPRING OSCILLATION VISUALIZED:

  Position vs Time for a Spring:
  
  x
  |   \\      \\      \\
  |  \  \    \  \    \  \
  | \    \  \    \  \    \
  -+------\\------\\------\--> t
  |        \      \        \
  |         \    \          \
  |          \  \            \
  |           \\              \
  
  WITHOUT DAMPING: Oscillates forever (theoretical)
  WITH DAMPING: Gradually settles at rest position
  
  In games, ALWAYS add damping to springs, or they'll
  oscillate indefinitely and look broken.
```

---

## 🧮 The Bridge System: F = ma -> a = F/m

```rust
/// THE CRITICAL BRIDGE between Dynamics and Kinematics.
///
/// This system takes the accumulated forces and converts them
/// into acceleration using Newton's Second Law.
///
/// After this system runs, the kinematics systems can use the
/// acceleration to update velocity and position.
///
/// The formula: a = F / m
///
/// Special cases:
///   mass = 0 (static): a = 0  -  immovable object
///   mass < 0: invalid  -  don't do this
pub fn convert_forces_to_acceleration(
    mut physics_query: Query<(
        &ForceAccumulator,
        &Mass,
        &mut Acceleration,
    )>,
) {
    for (force_accumulator, mass, mut acceleration) in physics_query.iter_mut() {
        if mass.0 > 0.0 {
            // Standard case: a = F / m
            // 
            // Example: F = (0, -1000), m = 2.0
            //   a = (0, -1000) / 2.0 = (0, -500)
            // The object accelerates downward at 500 px/s²
            acceleration.0 = force_accumulator.total_force() / mass.0;
        } else {
            // mass = 0.0 -> static/infinite mass object
            // These objects NEVER accelerate  -  they're immovable.
            // Think: walls, floors, pillars, the ground.
            acceleration.0 = Vec2::ZERO;
        }
    }
}
```

---

## 🔄 Complete Force Pipeline Trace

Let's trace ONE object through ONE complete physics frame:

```rust
/// INITIAL STATE:
///   Position: (0, 300)
///   Velocity: (0, 0)          -  starting from rest
///   Mass: 2.0
///   Gravity: (0, -500)         -  500 px/s² downward
///   Damping: 0.1
///   dt: 1/60 ≈ 0.01667

pub fn trace_complete_physics_frame() {
    // --- START OF FRAME ---
    
    // Step 1: CLEAR forces
    //   ForceAccumulator = (0, 0)  <- No leftover forces
    
    // Step 2: APPLY GRAVITY
    //   F_gravity = m × g = 2.0 × (0, -500) = (0, -1000)
    //   ForceAccumulator = (0, -1000)
    
    // Step 3: APPLY DRAG
    //   F_drag = -b × v = -0.1 × (0, 0) = (0, 0)
    //   No drag initially because velocity is zero.
    //   ForceAccumulator = (0, -1000) + (0, 0) = (0, -1000)
    
    // Step 4: F = ma -> a = F/m
    //   a = (0, -1000) / 2.0 = (0, -500)
    //   Acceleration = (0, -500)
    
    // Step 5: INTEGRATE (kinematics)
    //   v += a × dt = (0, 0) + (0, -500) × 0.01667 = (0, -8.33)
    //   x += v × dt = (0, 300) + (0, -8.33) × 0.01667 = (0, 299.86)
    //
    //   New Velocity: (0, -8.33)   -  falling slowly
    //   New Position: (0, 299.86)  -  slightly lower
    
    // --- FRAME 2 ---
    //   Clear: ForceAccumulator = (0, 0)
    //   Gravity: (0, -1000)
    //   Drag: -0.1 × (0, -8.33) = (0, 0.833)  <- upward! Opposing fall!
    //   Total: (0, -1000) + (0, 0.833) = (0, -999.167)
    //   a = (0, -999.167) / 2.0 = (0, -499.58)
    //   v = (0, -8.33) + (0, -499.58) × 0.01667 = (0, -16.66)
    //   x = (0, 299.86) + (0, -16.66) × 0.01667 = (0, 299.58)
    //
    //   Velocity increased to -16.66 (falling faster)
    //   Drag is starting to oppose the fall
    
    // --- FRAME 60 (~1 second) ---
    //   Terminal velocity approach: drag ≈ gravity
    //   Velocity ≈ (0, -500)  -  falling at constant speed
    //   Drag = -0.1 × (0, -500) = (0, 50)
    //   Gravity = (0, -1000)
    //   Net force = (0, -50)  -  almost zero!
    //   Object falls at constant speed from here on.
    
    println!("Complete physics trace available above.");
}
```

---

## 🎯 Chapter Summary

```
DYNAMICS = WHY THINGS MOVE

  The complete pipeline (MEMORIZE THIS SEQUENCE):
  
  ++
  |  1. CLEAR:      ForceAccumulator = (0, 0)               |
  |  2. ACCUMULATE: ForceAccumulator += Forces (gravity,     |
  |                   drag, thrust, collisions, ...)         |
  |  3. CONVERT:    Acceleration = ForceAccumulator / Mass   |
  |  4. INTEGRATE:  Velocity += Acceleration × dt            |
  |                  Position += Velocity × dt                |
  ++
  
  F = ma is the ENGINE:
  • Forces are the INPUT (what we control)
  • Acceleration is the OUTPUT (what kinematics uses)
  • Mass is the SCALING FACTOR (heavier = less responsive)
  
  FAILURE MODES:
  • Not clearing forces: runaway acceleration (objects rocket off)
  • mass = 0 without special handling: division by zero (NaN!)
  • Forgetting drag: objects never stop (endless sliding)
  • Misunderstanding F = ma: a = F/m, NOT a = F × m!
```

> **Dynamics is WHERE GAME FEEL COMES FROM. The same game with gravity = -200 vs gravity = -2000 feels COMPLETELY different. Drag of 0.1 vs 10.0 changes whether movement feels like ice or mud. Forces aren't just physics  -  they're the VOCABULARY of game feel. Tune them, don't just copy them.** 💪

> 💡 **Full source code for this chapter:** [code-examples/ch08-dynamics/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch08-dynamics)
> 
> The runnable project includes Cargo.toml, main.rs, and complete module files.

---

**[<- Previous: Kinematics](ch07-kinematics.md)** | **[Next: Integration Methods ->](ch09-integration.md)**
