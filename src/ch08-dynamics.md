# 💥 Dynamics: Forces & Newton's Laws

> **"Kinematics describes WHAT motion looks like. Dynamics explains WHY it happens. The answer is always forces — and one equation: F = ma."** 💪

---

## 🎯 The Bridge Between Chapters

In the last chapter, we learned kinematics: `vel += acc × dt; pos += vel × dt`. But we had a problem — **where does acceleration come from?**

```
Kinematics: "If I know the acceleration, I can find velocity and position."
Dynamics:   "Here's how to FIND the acceleration."

  Dynamics input ──► Acceleration ──► Velocity ──► Position
  (Forces, mass)      (F = ma)         (integrate)   (integrate)
```

Dynamics bridges from "what forces act on an object" to "how does it accelerate." The bridge is **Newton's Second Law**.

---

## 📜 Newton's Three Laws (The Complete Picture)

### 1️⃣ Law of Inertia — "Objects Resist Change"

```
An object at rest stays at rest.
An object in motion stays in motion (same speed, same direction).
UNLESS acted upon by a net external force.

What this means for games:
  • A stationary box stays put until you push it
  • A sliding puck keeps sliding (in space — no friction)
  • Velocity doesn't change without a force
  • This is WHY we clear forces each frame — without new
    forces, nothing should accelerate

Code consequence:
  vel += acc * dt  →  If acc = 0, vel doesn't change ✅
```

### 2️⃣ F = ma — "The Central Equation"

```
The net force on an object equals its mass times its acceleration.

  F = ma     →     a = F/m     (the form we actually use)

What this means for games:
  • Apply a force → object accelerates in that direction
  • Double the force → double the acceleration
  • Double the mass → HALF the acceleration
  • Zero mass → infinite acceleration (we handle this as "static")

INTUITION:
  Push a shopping cart (low mass): it accelerates easily 🛒
  Push a truck (high mass): barely moves 🚛
  Same push, different masses → different accelerations.
```

### 3️⃣ Action-Reaction — "Forces Come in Pairs"

```
For every action force, there's an equal and opposite reaction force.

When a box sits on a table:
  • Gravity pulls the box DOWN on the table
  • The table pushes the box UP with EQUAL force
  → The box doesn't move (net force = 0)

When you jump:
  • You push DOWN on the Earth
  • Earth pushes UP on you
  → You go up, Earth goes... imperceptibly down (it's heavy)

Code consequence:
  When entity A collides with entity B:
    impulse_B_on_A = -impulse_A_on_B  (equal and opposite!)
```

---

## ⚡ The Force Pipeline

Every frame, every physics object follows this pipeline:

```
┌─────────────────────────────────────────────────────────────────┐
│                    ONE FRAME OF PHYSICS                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  1️⃣ CLEAR:     ForceAccumulator = (0, 0)                       │
│     ↳ Old forces are GONE. Every frame starts fresh.             │
│                                                                   │
│  2️⃣ ACCUMULATE: Sum ALL forces acting on this object            │
│     ↳ ForceAccumulator += gravity_force                          │
│     ↳ ForceAccumulator += drag_force                             │
│     ↳ ForceAccumulator += thrust_force                           │
│     ↳ ForceAccumulator += collision_force                        │
│                                                                   │
│  3️⃣ CONVERT:    a = F / m  (Newton's Second Law)                │
│     ↳ Acceleration = total_force / mass                          │
│                                                                   │
│  4️⃣ INTEGRATE:  vel += a × dt;  pos += vel × dt                │
│     ↳ This is kinematics — now we know the acceleration!         │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Why CLEAR forces? A Critical Detail

```rust
/// ❌ WITHOUT CLEARING:
/// Frame 1: gravity applies (0, -500) to acceleration
/// Frame 2: gravity applies (0, -500) AGAIN
///          But old force is still there! → (0, -1000)
/// Frame 3: gravity applies AGAIN → (0, -1500)
/// Result: acceleration keeps GROWING → RUNAWAY PHYSICS! 💥

/// ✅ WITH CLEARING:
/// Frame 1: clear → (0, 0), apply gravity → (0, -500) ✅
/// Frame 2: clear → (0, 0), apply gravity → (0, -500) ✅
/// Frame 3: clear → (0, 0), apply gravity → (0, -500) ✅
/// Result: CONSTANT acceleration (as gravity should be) ✅
```

---

## 🌍 Types of Forces in Detail

### 🏋️ Gravity: The Universal Constant

```rust
/// GRAVITY — the simplest force
///
/// F_gravity = m × g
///
/// Where g is the gravitational field strength:
///   Earth:     g = (0, -9.81) m/s²
///   "Mario":   g = (0, -50) to (0, -100) m/s²  ← game feel!
///   Space:     g = (0, 0) m/s²
///
/// KEY INSIGHT: All objects fall at the same rate!
/// Since a = F/m = (m×g)/m = g, mass CANCELS OUT.
/// A feather and a boulder fall identically (in vacuum).
fn apply_gravity(
    mut query: Query<(&Mass, &mut ForceAccumulator)>,
    settings: Res<PhysicsSettings>,
) {
    for (mass, mut forces) in query.iter_mut() {
        // F = m × g
        forces.0 += settings.gravity * mass.0;
    }
}
```

### 🌬️ Drag: Why Things Eventually Stop

```rust
/// LINEAR DRAG: F_drag = -b × v
///
/// Proportional to velocity. Always OPPOSES motion.
/// Like moving through honey — the faster you go, the harder it pushes back.
///
/// KEY INSIGHT: Drag creates TERMINAL VELOCITY.
/// As velocity increases, drag increases until drag = gravity.
/// At that point: net force = 0 → no more acceleration → terminal velocity!
///
///   gravity = m × g (constant)
///   drag = -b × v (grows with velocity)
///   At terminal velocity: m×g = b×v_terminal  →  v_terminal = m×g / b

fn apply_linear_drag(
    mut query: Query<(&Velocity, &mut ForceAccumulator, &LinearDamping)>,
) {
    for (vel, mut forces, damping) in query.iter_mut() {
        // Drag opposes velocity: v = (5, 0) → drag = (-5b, 0)
        forces.0 += -vel.0 * damping.0;
    }
}
```

### 🧲 Springs: Hooke's Law

```rust
/// SPRING FORCE: F = -k × (x - x₀)
///
/// k = spring constant (stiffness)
/// x = current position
/// x₀ = rest position
///
/// The force is PROPORTIONAL to DISPLACEMENT.
/// Pull it further → it pulls back harder.
///
/// This creates OSCILLATION (if undertensioned):
///   Pull → spring pulls back → overshoots → pulls the other way → ...
///   Eventually settles at rest position (if damped)
fn apply_spring_force(
    pos: Vec2,
    anchor: Vec2,
    stiffness: f32,
) -> Vec2 {
    let displacement = pos - anchor;
    -displacement * stiffness  // Always pulls toward anchor
}
```

---

## 🔄 The Complete Pipeline in Action

Here's a complete trace of one physics step for a falling, damped object:

```
ENTITY: Position(0, 300), Velocity(0, 0), Mass(2.0)
WORLD:  gravity = (0, -500), damping = 0.1, dt = 1/60

STEP 1: CLEAR
  ForceAccumulator = (0, 0)  ← Start fresh

STEP 2: APPLY GRAVITY
  F_gravity = m × g = 2.0 × (0, -500) = (0, -1000)
  ForceAccumulator = (0, -1000)

STEP 3: APPLY DRAG
  F_drag = -b × v = -0.1 × (0, 0) = (0, 0)  ← No velocity yet
  ForceAccumulator = (0, -1000)

STEP 4: F = ma
  a = F / m = (0, -1000) / 2.0 = (0, -500)

STEP 5: INTEGRATE
  v += a × dt = (0, 0) + (0, -500) × 0.01667 = (0, -8.33)
  x += v × dt = (0, 300) + (0, -8.33) × 0.01667 = (0, 299.86)

RESULT: Object fell from y=300 to y=299.86
        Velocity is now -8.33 px/s (falling)
        
NEXT FRAME: Drag will be -0.1 × -8.33 = 0.833 (upward!)
            Gravity still pulls (0, -1000)
            Net force = (0, -1000) + (0, 0.833) = (0, -999.17)
            Slightly less than pure gravity → approaching terminal velocity
```

---

## 🧮 F = ma: The Bridge System

The critical system that connects dynamics to kinematics:

```rust
/// 🧮 THE BRIDGE: ForceAccumulator → Acceleration
///
/// This system does ONE thing: convert accumulated forces
/// into acceleration using F = ma.
///
/// After this runs, the kinematics system (next) can use
/// the acceleration to update velocity and position.
fn forces_to_acceleration(
    mut query: Query<(&ForceAccumulator, &Mass, &mut Acceleration)>,
) {
    for (forces, mass, mut acc) in query.iter_mut() {
        if mass.0 > 0.0 {
            // a = F / m — Newton's Second Law
            acc.0 = forces.0 / mass.0;
        } else {
            // mass = 0 → static object (infinite inertia)
            acc.0 = Vec2::ZERO;
        }
    }
}
```

---

## 📊 Force Reference

| Force | Formula | Behavior | Use Case |
|-------|---------|----------|----------|
| **Gravity** | `F = m × g` | Constant downward | Universal |
| **Linear Drag** | `F = -b × v` | Opposes velocity proportional to speed | Simple damping |
| **Quadratic Drag** | `F = -½ρv²CdA` | Opposes velocity proportional to SPEED² | Realistic air |
| **Spring** | `F = -k × Δx` | Pulls toward rest, proportional to stretch | Elasticity |
| **Normal** | `F ⊥ surface` | Prevents penetration | Ground contact |
| **Friction** | `F ≤ μ × F_normal` | Opposes sliding | Surface grip |
| **Buoyancy** | `F = ρ × V × g` | Upward, proportional to displaced volume | Floating |

---

## 🎯 Chapter Summary

```
DYNAMICS = WHY THINGS MOVE

  The pipeline (MEMORIZE THIS):
  
  ╔══════════════════════════════════════════════════╗
  ║  CLEAR → ACCUMULATE(F) → a=F/m → INTEGRATE     ║
  ║           ↓             ↓          ↓             ║
  ║      All forces      Newton's    kinema-         ║
  ║      this frame      Second Law  tics            ║
  ╚══════════════════════════════════════════════════╝

  F = ma is EVERYTHING:
  • m=0  → Static object (walls, floor)
  • m>0  → Dynamic object (affected by forces)
  • a=F/m → Bigger mass = less acceleration
  • Forces SUMMING means you can stack them modularly
  
  KEY INSIGHT: Forces always CLEAR at frame start.
  If you forget to clear, forces accumulate and
  your objects become rockets. 🚀
```

> **Dynamics is where the "game feel" comes from. Gravity of -500 vs -2000 makes the same game feel completely different. Drag of 0.1 vs 2.0 changes whether movement feels like ice or honey. Forces ARE game feel. Tune them, don't just copy them.** 💥

---

**[← Previous: Kinematics](ch07-kinematics.md)** | **[Next: Integration Methods →](ch09-integration.md)**
