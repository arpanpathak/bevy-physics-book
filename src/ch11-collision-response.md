# 🤝 Collision Response: Making Things Bounce

> **"Detection tells you something is wrong. Response makes it right — with velocity, impulses, and a little bit of magic."** 💥

---

## 🎯 The Response Pipeline

After detecting a collision, we must **resolve** it:

```
Collision Detected! 💥
        │
        ▼
┌─────────────────────┐
│ 1. SEPARATE         │  ← Push objects apart (position correction)
│    (Fix overlap)    │
└─────────────────────┘
        │
        ▼
┌─────────────────────┐
│ 2. COMPUTE IMPULSE  │  ← Calculate velocity change
│    (Fix velocity)   │
└─────────────────────┘
        │
        ▼
┌─────────────────────┐
│ 3. APPLY IMPULSE    │  ← Update velocities
│    (Make it bounce) │
└─────────────────────┘
        │
        ▼
    All resolved! ✅
```

---

## 💥 Impulse-Based Collision Response

```rust
/// 🤝 Collision resolution using impulses
///
/// An "impulse" is an instantaneous change in velocity.
/// Think of it as a force applied over an infinitely short time.
/// Perfect for game physics!

/// 📐 Compute the impulse between two colliding objects
fn resolve_collision(
    // Collision data
    normal: Vec2,         // Direction to push entity_a away
    penetration: f32,     // How much they overlap
    
    // Entity A
    pos_a: &mut Vec2,
    vel_a: &mut Vec2,
    mass_a: f32,
    
    // Entity B
    pos_b: &mut Vec2,
    vel_b: &mut Vec2,
    mass_b: f32,
    
    // Material properties
    restitution: f32,     // Bounciness (0 = inelastic, 1 = perfectly elastic)
    friction: f32,        // Surface roughness
) {
    // ─── STEP 1: POSITION CORRECTION ───
    // Push objects apart proportional to their masses
    // Heavier objects move less
    let total_mass = mass_a + mass_b;
    
    if total_mass > 0.0 {
        let ratio_a = mass_b / total_mass;  // A moves proportional to B's mass
        let ratio_b = mass_a / total_mass;  // B moves proportional to A's mass
        
        // Push apart along the collision normal
        *pos_a += normal * penetration * ratio_a;
        *pos_b -= normal * penetration * ratio_b;
    }
    
    // ─── STEP 2: RELATIVE VELOCITY ───
    // How fast are they approaching each other?
    let rel_vel = *vel_a - *vel_b;
    let rel_vel_along_normal = rel_vel.dot(normal);
    
    // ❌ Don't resolve if they're separating
    if rel_vel_along_normal > 0.0 {
        return;
    }
    
    // ─── STEP 3: IMPULSE MAGNITUDE ───
    // j = -(1 + e) × v_rel · n / (1/m₁ + 1/m₂)
    //
    // Where:
    //   e = restitution (bounciness)
    //   v_rel = relative velocity
    //   n = collision normal
    //   m₁, m₂ = masses
    
    let inv_mass_a = if mass_a > 0.0 { 1.0 / mass_a } else { 0.0 };
    let inv_mass_b = if mass_b > 0.0 { 1.0 / mass_b } else { 0.0 };
    let inv_mass_sum = inv_mass_a + inv_mass_b;
    
    if inv_mass_sum == 0.0 {
        return;  // Both are immovable
    }
    
    // 💥 The impulse magnitude!
    let j = -(1.0 + restitution) * rel_vel_along_normal / inv_mass_sum;
    
    // ─── STEP 4: APPLY NORMAL IMPULSE ───
    // v_new = v_old + (j × n) / m
    let impulse = normal * j;
    *vel_a += impulse * inv_mass_a;
    *vel_b -= impulse * inv_mass_b;
    
    // ─── STEP 5: FRICTION (tangential impulse) ───
    // Friction opposes sliding motion along the surface
    let tangent = rel_vel - normal * rel_vel_along_normal;
    
    if tangent.length_squared() > 0.0001 {
        let tangent = tangent.normalize();
        let rel_vel_tangential = rel_vel.dot(tangent);
        
        // Friction impulse (clamped by Coulomb's law: F_friction ≤ μ × F_normal)
        let jt = -rel_vel_tangential / inv_mass_sum;
        let max_friction = j * friction;
        let jt = jt.clamp(-max_friction, max_friction);
        
        let friction_impulse = tangent * jt;
        *vel_a += friction_impulse * inv_mass_a;
        *vel_b -= friction_impulse * inv_mass_b;
    }
}
```

---

## 🎾 Restitution (Bounciness)

```rust
/// 🎾 Restitution coefficient e = how bouncy is the collision?
///
/// e = 0.0   → Perfectly inelastic (balls of clay, sticky)
/// e = 0.3   → Slightly bouncy (a basketball)
/// e = 0.5   → Moderately bouncy (a tennis ball)
/// e = 0.8   → Very bouncy (a superball)
/// e = 1.0   → Perfectly elastic (ideal, no energy loss)
///
/// Combined restitution (when two objects collide):
/// e_combined = min(e_a, e_b)  — conservative
/// e_combined = e_a × e_b       — common choice
/// e_combined = sqrt(e_a × e_b) — geometric mean

#[derive(Component)]
struct Material {
    restitution: f32,
    friction: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            restitution: 0.3,  // Slightly bouncy
            friction: 0.2,     // Slightly rough
        }
    }
}

// ⭐ Common game materials:
const MATERIALS: [(&str, f32, f32); 8] = [
    ("steel",    0.7, 0.3),   // Clang!
    ("rubber",   0.9, 0.8),   // Boing!
    ("wood",     0.3, 0.4),   // Thud.
    ("ice",      0.1, 0.03),  // Sliiiide
    ("mud",      0.0, 0.9),   // Splat.
    ("bouncy",   0.95, 0.1),  // BOING!
    ("dampened", 0.1, 0.5),   // Oof.
    ("perfect",  1.0, 0.0),   // Ideal physics
];
```

```
Bounce Heights (drop from 10m):

    e=1.0:   10m ───┐  ┌───┐  ┌───┐  ┌───  (bounces forever!)
                     │  │   │  │   │  │
    e=0.5:   10m ───┐  ┌─┐ ┌─┐ ┌─┐ ┌─┐┌─  (decays slowly)
                     │  │ │ │ │ │ │ │ ││
    e=0.0:   10m ───┐                                     (splat!)
                     │
    
    Game feel tip: Slightly lower restitution than real life.
    Players expect things to settle down eventually! 🎮
```

---

## 🏗️ Collision Response System

```rust
/// 💥 Complete collision response system using Bevy ECS
fn collision_response_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(&mut Position, &mut Velocity, &Mass, Option<&Material>)>,
) {
    // 📦 Collect all entities' data for quick lookup
    let mut entities: HashMap<Entity, (Vec2, Vec2, f32, f32, f32)> = HashMap::new();
    
    for (entity, pos, vel, mass, material) in query.iter() {
        let restitution = material.map_or(0.3, |m| m.restitution);
        let friction = material.map_or(0.2, |m| m.friction);
        entities.insert(entity, (pos.0, vel.0, mass.0, restitution, friction));
    }
    
    // 💥 Process each collision event
    for event in collision_events.read() {
        let (mut pos_a, mut vel_a, mass_a, mat_a) = 
            match get_mut!(query, event.entity_a) {
                Ok(data) => data,
                Err(_) => continue,
            };
        
        let (mut pos_b, mut vel_b, mass_b, mat_b) = 
            match get_mut!(query, event.entity_b) {
                Ok(data) => data,
                Err(_) => continue,
            };
        
        // Combine material properties
        let restitution = mat_a.map_or(0.3, |m| m.restitution)
            * mat_b.map_or(0.3, |m| m.restitution);
        let friction = (mat_a.map_or(0.2, |m| m.friction)
            + mat_b.map_or(0.2, |m| m.friction)) / 2.0;
        
        // 💥 Resolve the collision
        resolve_collision(
            event.normal,
            event.penetration,
            &mut pos_a.0,
            &mut vel_a.0,
            mass_a.0,
            &mut pos_b.0,
            &mut vel_b.0,
            mass_b.0,
            restitution,
            friction,
        );
    }
}
```

---

## 🧮 Energy & Momentum Conservation

```rust
/// 🧮 Physics invariants — these should be conserved
///
/// MONITOR THESE during development to catch bugs!

/// 📊 Total momentum should be conserved in all collisions
fn check_momentum_conservation(
    query: Query<(&Velocity, &Mass)>,
) {
    let mut total_momentum = Vec2::ZERO;
    
    for (vel, mass) in query.iter() {
        // p = m × v
        total_momentum += vel.0 * mass.0;
    }
    
    // ⭐ Total momentum should remain constant
    // (unless external forces are applied)
    println!("📊 Total momentum: ({:.2}, {:.2})", 
        total_momentum.x, total_momentum.y);
}

/// 📊 Kinetic energy check
fn check_kinetic_energy(
    query: Query<(&Velocity, &Mass)>,
) {
    let mut total_ke = 0.0;
    
    for (vel, mass) in query.iter() {
        // KE = ½ × m × v²
        let speed_sq = vel.0.length_squared();
        total_ke += 0.5 * mass.0 * speed_sq;
    }
    
    // ⭐ KE should not increase (conservative or dissipative)
    // If KE increases → energy is being CREATED → BUG!
    println!("📊 Kinetic energy: {:.2}", total_ke);
}
```

---

## 🎯 Practical: Stacking Boxes

Stacking requires special handling — gravity keeps pushing down:

```rust
/// 📦 Stacking boxes — keeping things stable
///
/// Problem: Box A sits on Box B. Gravity pulls A down.
/// Solution: Box A is ON TOP of Box B.
/// But Box B can't tell Box A to move up... OR CAN IT?
///
/// The trick: CONTACT POINTS with a "resting" threshold.

#[derive(Component)]
struct BoxStack {
    /// How many objects are resting on top of me
    supporting_count: u32,
}

/// 🧱 Resting contact detection
/// If velocity toward contact is very small, object is "resting"
fn detect_resting_contacts(
    mut query: Query<(
        Entity,
        &mut Position,
        &mut Velocity,
        &Mass,
        Option<&BoxStack>,
    )>,
) {
    let ground_y = -300.0;
    
    for (entity, mut pos, mut vel, mass, stack) in query.iter_mut() {
        if pos.0.y <= ground_y && vel.0.y < 0.0 {
            // 🛑 On the ground — stop downward velocity
            pos.0.y = ground_y;
            
            if vel.0.y < 0.0 {
                vel.0.y = 0.0;
            }
            
            // ✅ Object is resting — mark as such
            println!("📦 Entity {:?} is resting on ground", entity);
        }
    }
}

/// 💡 Stacking stability tips:
/// 1. Use position correction (push apart) BEFORE velocity change
/// 2. Use "slop" (small penetration threshold) to avoid jitter
/// 3. Apply damping to reduce oscillation
/// 4. Use multiple iterations (2-4) per physics step
/// 5. Consider using a "sleep" system for stationary objects

/// 😴 Physics sleep — skip simulation for resting objects
#[derive(Component)]
struct Sleeping {
    timer: f32,
}

fn physics_sleep(
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity, &mut Sleeping)>,
) {
    const SLEEP_THRESHOLD: f32 = 0.01;  // Speed below this = "asleep"
    const WAKE_THRESHOLD: f32 = 0.05;   // Speed above this = "awake"
    
    for (entity, vel, mut sleep) in query.iter_mut() {
        if vel.0.length_squared() < SLEEP_THRESHOLD {
            sleep.timer += 1.0 / 60.0;
            
            if sleep.timer > 1.0 {
                // 💤 Go to sleep (remove from physics query)
                commands.entity(entity).insert(Sleeping { timer: 0.0 });
            }
        } else if vel.0.length_squared() > WAKE_THRESHOLD {
            // ⚡ Wake up!
            sleep.timer = 0.0;
        }
    }
}
```

---

## 🎯 Chapter Summary

```rust
/// 📝 Collision response cheat sheet:

// 💥 The impulse formula:
// j = -(1 + e) · (v₁ - v₂) · n / (1/m₁ + 1/m₂)

// 📐 Position correction (prevent overlap):
// pos += normal × penetration × (mass_other / total_mass)

// 🎾 Restitution blending:
// e_combined = e₁ × e₂

// 🧊 Friction (Coulomb model):
// F_friction ≤ μ × F_normal
// j_tangential = clamp(jt, -μ·j, μ·j)

// 🎯 Key principles:
// 1. Always separate positions before changing velocities
// 2. Conserve momentum (it's a hard invariant!)
// 3. Use restitution for bounciness
// 4. Use friction for surface grip
// 5. Add "slop" tolerance to avoid jittery stacks
```

> **Key Takeaway:** Collision response is about applying **impulses** — instantaneous velocity changes along the collision normal. The magic formula `j = -(1+e)·v_rel·n / (1/m₁ + 1/m₂)` handles everything from splats to superballs. Add friction and position correction, and you've got a complete collision solver! 🏆

---

**[← Previous: Collision Detection](ch10-collision-detection.md)** | **[Next: Constraints & Joints →](ch12-constraints.md)**
