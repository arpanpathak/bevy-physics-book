# 💥 Dynamics: Forces & Newton's Laws

> **"Kinematics describes WHAT the motion is. Dynamics explains WHY it happens. Forces are the 'why.'"** 💪

---

## 📜 Newton's Three Laws

```
╔══════════════════════════════════════════════════════════╗
║                 NEWTON'S LAWS OF MOTION                  ║
╠══════════════════════════════════════════════════════════╣
║                                                          ║
║  1️⃣  LAW OF INERTIA                                      ║
║     "Objects keep doing what they're doing unless        ║
║      a force changes it."                                ║
║     → A stationary ball stays still                     ║
║     → A moving ball keeps moving in a straight line     ║
║                                                          ║
║  2️⃣  F = ma                                              ║
║     "Force equals mass times acceleration."              ║
║     → Push gently → small acceleration                  ║
║     → Push hard → big acceleration                      ║
║     → Heavy object → same push = less acceleration      ║
║                                                          ║
║  3️⃣  ACTION = REACTION                                   ║
║     "For every action, there's an equal opposite         ║
║      reaction."                                          ║
║     → Jump: you push Earth, Earth pushes you            ║
║     → You move up, Earth moves... imperceptibly         ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝
```

---

## ⚡ Force Accumulation: The Physics Pipeline

```rust
/// 🏗️ The complete physics pipeline:
///
/// 1. CLEAR forces     → Reset acceleration to zero
/// 2. APPLY forces     → Sum all forces: gravity, drag, thrust, collisions
/// 3. INTEGRATE        → a → v → x (kinematics!)
/// 4. DETECT collisions → Find overlapping pairs
/// 5. RESOLVE collisions → Apply impulses to separate objects
///
/// Every frame, every physics object goes through this pipeline!

/// 💥 Component that accumulates forces each frame
#[derive(Component, Default)]
pub struct ForceAccumulator {
    /// Sum of all forces applied this frame
    pub forces: Vec2,
}

impl ForceAccumulator {
    /// ➕ Add a force vector
    pub fn apply(&mut self, force: Vec2) {
        self.forces += force;
    }
    
    /// 🧹 Clear all forces (called at end of frame)
    pub fn clear(&mut self) {
        self.forces = Vec2::ZERO;
    }
}
```

---

## 🌍 Types of Forces

### 🏋️ Gravity

```rust
/// 🌍 Gravity: The most universal force
///
/// F = m × g
/// Where g is the gravitational acceleration (9.81 m/s² on Earth)
fn apply_gravity(
    mut query: Query<(&Mass, &mut ForceAccumulator)>,
    settings: Res<PhysicsSettings>,
) {
    for (mass, mut forces) in query.iter_mut() {
        // F = m × g  — gravity pulls down
        let gravity_force = settings.gravity * mass.0;
        forces.apply(gravity_force);
    }
}

/// 💡 On Earth: g = 9.81 m/s² downward
/// On Moon:   g = 1.62 m/s²
/// On Mars:   g = 3.72 m/s²
/// "Mario gravity": g = 50-100 m/s² (feels snappier!)
/// 
/// Game feel tip: Use 1.5x-2x real gravity for more
/// satisfying, snappier gameplay!
```

### 🌬️ Drag / Air Resistance

```rust
/// 🌬️ Linear Drag: F = -b × v
///
/// Proportional to velocity — like moving through honey
/// Simple, stable, good for most games
fn apply_linear_drag(
    mut query: Query<(&Velocity, &mut ForceAccumulator, &LinearDamping)>,
) {
    for (vel, mut forces, damping) in query.iter_mut() {
        // Drag opposes velocity: if moving right, drag pushes left
        let drag_force = -vel.0 * damping.0;
        forces.apply(drag_force);
    }
}

/// 🌊 Quadratic Drag: F = -0.5 × ρ × v² × Cd × A
///
/// Proportional to velocity SQUARED — like real air resistance
/// More realistic, used in racing games
fn apply_quadratic_drag(
    mut query: Query<(&Velocity, &mut ForceAccumulator, &Aerodynamics)>,
) {
    for (vel, mut forces, aero) in query.iter_mut() {
        let speed_sq = vel.0.length_squared();
        let direction = vel.0.normalize_or_zero();
        
        // F_drag = -½ × ρ × v² × Cd × A × direction
        let drag_magnitude = 0.5 * aero.air_density * speed_sq * aero.drag_coefficient * aero.area;
        let drag_force = -direction * drag_magnitude;
        
        forces.apply(drag_force);
    }
}

#[derive(Component)]
pub struct Aerodynamics {
    pub air_density: f32,      // ρ (rho) — 1.225 kg/m³ at sea level
    pub drag_coefficient: f32, // Cd — 0.47 for sphere, 0.04 for streamlined
    pub area: f32,             // A — cross-sectional area
}

impl Default for Aerodynamics {
    fn default() -> Self {
        Self {
            air_density: 1.225,
            drag_coefficient: 0.47, // Sphere-like
            area: 1.0,
        }
    }
}
```

### 🏗️ Normal / Contact Force

```rust
/// 🏗️ Ground contact force — prevents falling through floor
///
/// This is an example of a CONSTRAINT force:
/// It pushes just hard enough to prevent penetration
fn apply_ground_contact(
    mut query: Query<(&mut Position, &mut Velocity, &mut ForceAccumulator)>,
) {
    for (mut pos, mut vel, mut forces) in query.iter_mut() {
        let ground_y = -300.0;  // Ground plane
        
        if pos.0.y < ground_y {
            // 🛑 Push back above ground
            pos.0.y = ground_y;
            
            // ⛔ Stop downward velocity
            if vel.0.y < 0.0 {
                vel.0.y = 0.0;
            }
            
            // 🏗️ Apply normal force (cancel gravity)
            // This keeps the object on the ground
            forces.apply(Vec2::new(0.0, 9.81)); // Cancel gravity's effect
        }
    }
}
```

### 🧲 Spring Force (Hooke's Law)

```rust
/// 🌸 Spring force: F = -k × (x - rest_length)
///
/// Hooke's Law: The force is proportional to displacement
/// k = spring constant (stiffness)
/// Higher k = stiffer spring = faster oscillation
#[derive(Component)]
pub struct Spring {
    pub anchor: Vec2,        // Where the spring is attached
    pub rest_length: f32,    // Natural length of spring
    pub stiffness: f32,      // k — how stiff (higher = stiffer)
    pub damping: f32,        // b — how much energy is lost
}

fn apply_spring_force(
    mut query: Query<(&Position, &mut ForceAccumulator, &Spring)>,
) {
    for (pos, mut forces, spring) in query.iter_mut() {
        // 📐 Vector from anchor to current position
        let displacement = pos.0 - spring.anchor;
        let current_length = displacement.length();
        
        if current_length < 0.001 {
            continue;  // At rest — no force
        }
        
        // 🧭 Direction of the spring
        let direction = displacement / current_length;
        
        // 🏋️ How far from rest length?
        let stretch = current_length - spring.rest_length;
        
        // 💥 Hooke's Law: F = -k × stretch
        let spring_force = -spring.stiffness * stretch;
        
        // 🛑 Damping: F = -b × (velocity along spring direction)
        // We need velocity for this — omitted for simplicity
        let total_force = spring_force;
        
        // Apply force along spring direction
        forces.apply(direction * total_force);
    }
}
```

---

## 🧮 F = ma → a = F/m

The most important equation. Here's how we use it:

```rust
/// 🔄 Convert accumulated forces into acceleration
///
/// This is the BRIDGE between dynamics (forces) and kinematics (motion)
fn resolve_forces_to_acceleration(
    mut query: Query<(&ForceAccumulator, &Mass, &mut Acceleration)>,
) {
    for (forces, mass, mut acc) in query.iter_mut() {
        // 🧮 Newton's Second Law: a = F / m
        if mass.0 > 0.0 {
            acc.0 = forces.forces / mass.0;
        } else {
            // Infinite mass = static object (wall, floor, etc.)
            acc.0 = Vec2::ZERO;
        }
    }
}
```

---

## 🎮 Complete Force System Example

```rust
/// 🚀 A player-controller character with multiple forces

#[derive(Component)]
struct PlayerController {
    /// Thrust force strength
    thrust_power: f32,
    /// Is the player on the ground?
    grounded: bool,
}

/// 🎮 Apply forces based on input
fn player_force_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&PlayerController, &mut ForceAccumulator)>,
) {
    for (controller, mut forces) in query.iter_mut() {
        let mut input_force = Vec2::ZERO;
        
        // 🏃 Horizontal movement
        if keyboard.pressed(KeyCode::KeyA) { input_force.x -= 1.0; }
        if keyboard.pressed(KeyCode::KeyD) { input_force.x += 1.0; }
        
        // 🦘 Jump (only if grounded)
        if keyboard.just_pressed(KeyCode::Space) && controller.grounded {
            input_force.y = 300.0;  // Jump impulse!
        }
        
        forces.apply(input_force * controller.thrust_power);
    }
}

/// 🏗️ Build the complete force system pipeline
fn physics_step(
    // System order matters!
    mut clear_forces: Query<&mut ForceAccumulator>,
    query_gravity: Query<(&Mass, &mut ForceAccumulator)>,
    query_drag: Query<(&Velocity, &mut ForceAccumulator, &LinearDamping)>,
    query_fma: Query<(&ForceAccumulator, &Mass, &mut Acceleration)>,
    query_integrate: Query<(&mut Position, &mut Velocity, &Acceleration)>,
    settings: Res<PhysicsSettings>,
) {
    let dt = settings.fixed_dt;
    
    // 1️⃣ CLEAR forces from last frame
    for mut f in clear_forces.iter_mut() {
        f.clear();
    }
    
    // 2️⃣ APPLY gravity (force)
    for (mass, mut forces) in query_gravity.iter() {
        forces.apply(settings.gravity * mass.0);
    }
    
    // 3️⃣ APPLY drag (force)
    for (vel, mut forces, damping) in query_drag.iter() {
        forces.apply(-vel.0 * damping.0);
    }
    
    // 4️⃣ F = ma → a = F/m
    for (forces, mass, mut acc) in query_fma.iter() {
        acc.0 = if mass.0 > 0.0 {
            forces.forces / mass.0
        } else {
            Vec2::ZERO
        };
    }
    
    // 5️⃣ INTEGRATE: a → v → x
    for (mut pos, mut vel, acc) in query_integrate.iter() {
        vel.0 += acc.0 * dt;
        pos.0 += vel.0 * dt;
    }
}
```

---

## 📊 Force Reference

| Force Type | Formula | Bevy Implementation | Use Case |
|-----------|---------|-------------------|----------|
| Gravity | F = mg | `forces.apply(gravity * mass)` | Universal |
| Linear Drag | F = -bv | `forces.apply(-vel * damping)` | Simple damping |
| Quadratic Drag | F = -½ρv²CdA | Complex (above) | Realistic air |
| Spring (Hooke) | F = -kx | `-stiffness * displacement` | Elastic objects |
| Normal | Fn = mg cos(θ) | Ground collision | Ground contact |
| Friction | Ff = μFn | Collision response | Surfaces |
| Buoyancy | Fb = ρVg | Water physics | Floating |

---

## 🎯 Chapter Summary

```
Dynamics = WHY things move

    Forces are applied → Accumulated → Divided by mass → Acceleration
         (F₁ + F₂ + ...)            (F = ma)             (a)
    
    Then kinematics takes over:
    a → integrate → v → integrate → x

    The pipeline:
    ╔════════╗   ╔══════════╗   ╔════════════╗
    ║ CLEAR  ║ → ║ APPLY    ║ → ║ F = ma     ║
    ║ forces ║   ║ forces   ║   ║ a = F/m    ║
    ╚════════╝   ╚══════════╝   ╚════════════╝
                                      ↓
                                 ╔════════════╗
                                 ║ INTEGRATE  ║
                                 ║ a → v → x  ║
                                 ╚════════════╝
```

> **Key Takeaway:** Forces are the CAUSE, acceleration is the EFFECT. The pipeline is: gather forces → divide by mass → integrate → move. Never skip the "clear forces" step — ghost forces from last frame create the most mysterious bugs! 🐛

---

**[← Previous: Kinematics](ch07-kinematics.md)** | **[Next: Integration Methods →](ch09-integration.md)**
