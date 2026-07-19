# 🏃 Kinematics: The Geometry of Motion

> **Kinematics is the study of motion WITHOUT asking what causes it. It answers three questions: WHERE are you? How FAST are you going? Is your speed CHANGING? Dynamics (next chapter) asks WHY. Here, we just DESCRIBE.** 🎯

---

## 🎯 What Is Motion, Really?

Look at a car driving down the street. In your mind, you naturally understand:

- **Where** the car is right now (it's passing the blue house)
- **How fast** it's going (about 30 mph)
- **Whether** it's speeding up or slowing down (it's braking for the stop sign)

These three intuitions map EXACTLY to the three kinematic quantities. Let's make them precise.

---

## 📐 The Three Quantities of Motion

### 1. Position: "Where Are You?"

Position is the simplest. It's a LOCATION in space.

```
In 1D (a number line):
  -5  -4  -3  -2  -1   0   1   2   3   4   5
                      ▲
                      │
                  You are here: position = 0

In 2D (a screen):
          ↑ y
          │
    300 ─ ● player         ← Position = (200, 300)
          │
          └───────────────► x
              200
```

**Position is always measured RELATIVE to an origin (0,0).** In games, the origin might be the center of the screen, the bottom-left corner, or the starting point of the level.

**In our physics engine, position is a Vec2.** It's stored as a component on each entity.

---

### 2. Velocity: "How Fast and Which Way?"

Velocity tells you two things at once:
- **Speed** (magnitude) = how fast position is changing
- **Direction** = which way position is changing

```
Velocity = (50, 0) means:
  Speed = 50 units per second
  Direction = to the right (positive X)

Velocity = (0, -100) means:
  Speed = 100 units per second
  Direction = downward (negative Y)

Velocity = (35.4, 35.4) means:
  Speed = √(35.4² + 35.4²) = 50 units per second
  Direction = diagonal (up and right at 45°)
```

**Here's the critical mental model:** Velocity is NOT "how far you move." Velocity is "how far you WOULD move in one second, if you kept going at this exact rate."

```
Imagine you take a photo of a moving car. The photo FREEZES time.
In that frozen instant, the car has a speed (let's say 60 mph).
That speed is the VELOCITY.

If the car could maintain that exact speed for one full second,
it would travel 60 mph × 1 second = 88 feet.

That 88 feet is the DISTANCE. The "60 mph" is the VELOCITY.
```

**In our physics engine, velocity is a Vec2 component added to position every frame:**

```
position += velocity × delta_time

If velocity = (50, 0) and delta_time = 1/60 second:
  position += (50, 0) × 0.01667
  position += (0.833, 0)
  
The entity moves 0.833 pixels to the right this frame.
Over 60 frames (1 second), it moves 50 pixels total.
```

---

### 3. Acceleration: "Is Your Speed Changing?"

Acceleration is the RATE OF CHANGE of velocity. If velocity is "how fast position changes," acceleration is "how fast VELOCITY changes."

```
POSITION:  "Where am I?"     → measured in pixels
VELOCITY:  "How fast is my   → measured in pixels PER SECOND
           position changing?"
ACCELERATION: "How fast is   → measured in pixels PER SECOND PER SECOND
              my velocity          (pixels/second²)
              changing?"
```

**Real-world examples of acceleration:**

| Situation | What Happens | Acceleration |
|-----------|-------------|--------------|
| Dropping a ball | Velocity increases downward every second | -9.81 m/s² (gravity) |
| Slamming brakes | Velocity decreases every second | Positive (opposite to motion) |
| Turning a corner | Velocity CHANGES DIRECTION (even if speed constant) | Perpendicular to motion |
| Car cruising at 60 mph | Velocity stays constant | 0 m/s² (no acceleration) |

**Counterintuitive fact: Turning IS acceleration.** Even if your speed stays the same, changing DIRECTION is acceleration. This is because velocity is a VECTOR - changing either magnitude OR direction counts as acceleration.

---

## 🔗 The Chain: How They Connect

Position, velocity, and acceleration form an unbroken chain:

```
Acceleration ──► changes ──► Velocity ──► changes ──► Position
 (external             (rate of change    (rate of change
  influence)            of velocity)       of position)
```

**In plain English:**

```
Your FOOT on the gas pedal = acceleration
This changes your SPEED = velocity
Your speed changes your LOCATION = position
```

**In math notation (don't worry, this is just the formal version):**

```
v(t) = d/dt [position(t)]    ← velocity is the DERIVATIVE of position
a(t) = d/dt [velocity(t)]    ← acceleration is the DERIVATIVE of velocity

Going backward (which is what our code does every frame):
position(t+dt) = position(t) + velocity(t) × dt   ← INTEGRATE velocity
velocity(t+dt) = velocity(t) + acceleration(t) × dt  ← INTEGRATE acceleration
```

### What "Derivative" and "Integral" Mean in Plain English

If calculus scares you, here's the ONLY thing you need to know:

```
A DERIVATIVE = "how fast is this thing changing RIGHT NOW?"
  → The derivative of position is velocity.
  → The derivative of velocity is acceleration.

An INTEGRAL = "add up all the tiny changes over time."
  → The integral of acceleration over time is velocity.
  → The integral of velocity over time is position.
```

**In game code, "integral" just means:** `value += change × delta_time`

```rust
// This IS an integral. You do calculus every frame without realizing it.
velocity += acceleration * delta_time;   // Integrate acceleration
position += velocity * delta_time;        // Integrate velocity
```

---

## 🧮 The SUVAT Equations: Formulas for Constant Acceleration

When acceleration is CONSTANT (like gravity, which doesn't change while you fall), we have exact formulas that describe ALL of motion:

```
SUVAT stands for:
  s = displacement (change in position)
  u = initial velocity (velocity at time 0)
  v = final velocity (velocity at time t)
  a = constant acceleration
  t = time
```

**The five equations (you only need the first two):**

| Equation | What It Tells You | Example |
|----------|-------------------|---------|
| `v = u + a × t` | Final velocity after time t | "After falling for 2 seconds, how fast are you going?" |
| `s = u × t + ½ × a × t²` | Displacement after time t | "After falling for 2 seconds, how far did you fall?" |
| `v² = u² + 2 × a × s` | Final velocity from displacement | "How fast will you hit the ground if you fall 10 meters?" |

### Trace a Jump With Real Numbers

Let's trace a Mario-style jump. The player jumps upward at 10 m/s. Gravity pulls down at -9.81 m/s².

```
QUESTION: How high does the player jump? How long does it take?

GIVEN:
  u = 10.0  (initial velocity, upward = positive)
  a = -9.81 (gravity, downward = negative)
  
STEP 1: Find the time when velocity reaches zero (peak of jump).
  v = u + a × t
  0 = 10 + (-9.81) × t
  t = -10 / -9.81 = 1.02 seconds
  
  The player reaches the peak of their jump after 1.02 seconds.

STEP 2: Find the height at that time.
  s = u × t + ½ × a × t²
  s = 10 × 1.02 + 0.5 × (-9.81) × 1.02²
  s = 10.2 + 0.5 × (-9.81) × 1.04
  s = 10.2 + (-5.10)
  s = 5.1 meters
  
  The player jumps 5.1 meters high.

STEP 3: Total time in the air.
  What goes up must come down. The fall takes the same time as the rise.
  Total air time = 1.02 × 2 = 2.04 seconds
  
  So a jump that reaches 5.1 meters takes about 2 seconds total.
```

```
Height vs Time for the jump:

  height (m)
    5 │    ╱╲        
    4 │   ╱  ╲       ← Peak at t=1.02s, height=5.1m
    3 │  ╱    ╲
    2 │ ╱      ╲
    1 │╱        ╲
    0 └────────────► time (s)
      0   1   2

  The shape is a PARABOLA. That's what "s = ut + ½at²" looks like.
  
  Velocity vs Time for the same jump:

  vel (m/s)
   10 │    ╱
    5 │   ╱         ← Crosses zero at t=1.02s (peak of jump)
    0 │  ╱───────► t
   -5 │ ╲
  -10 │  ╲
  
  The velocity DECREASES LINEARLY. That's what "v = u + at" looks like.
  At the peak, velocity is zero — the player is momentarily weightless.
```

---

## 🔮 The Killer Feature: Trajectory Prediction

Kinematics lets you PREDICT THE FUTURE. This is the single most powerful tool for game AI:

```rust
/// Given where something is NOW and how it's moving,
/// where will it be in `t` seconds?
///
/// This uses the SUVAT equation: s = ut + ½at²
/// applied to each axis (X and Y) independently.
fn predict_future_position(
    current_position: Vec2,
    current_velocity: Vec2,
    constant_acceleration: Vec2,  // Usually just gravity
    time_in_future: f32,          // How far ahead to look
) -> Vec2 {
    // s = ut + ½at² for each axis
    // x(t) = x₀ + vx × t + ½ × ax × t²
    // y(t) = y₀ + vy × t + ½ × ay × t²
    current_position
        + current_velocity * time_in_future
        + 0.5 * constant_acceleration * time_in_future * time_in_future
}
```

**How to use this for AI aiming (leading a target):**

```
Player is running to the right at 5 m/s.
Enemy wants to shoot where the player WILL be, not where he IS.

  Without prediction:      With prediction:
  
  Player ●───►             Player ●───►    ● (predicted position in 0.5s)
     ↑                         ↑           ↗
  Enemy shoots here ❌      Enemy aims here ✅
  (always misses)           (leads the target, always hits)
```

---

## 📝 The Code: Implementing Kinematics in Bevy

Now that you understand the PHYSICS, the code is almost trivial:

```rust
/// The ENTIRE kinematics engine, in two lines.
///
/// This runs every frame, for every physics entity.
/// It transforms acceleration into velocity, and velocity into position.
pub fn kinematics_step(
    velocity: &mut Vec2,
    position: &mut Vec2,
    acceleration: &Vec2,
    delta_time: f32,
) {
    // v_new = v_old + a × dt     (integrate acceleration)
    *velocity += *acceleration * delta_time;
    
    // x_new = x_old + v_new × dt  (integrate velocity — uses NEW velocity!)
    *position += *velocity * delta_time;
}
```

**That's it.** That's the entire physics of motion. Everything else in game physics — forces, collisions, constraints — is just figuring out what `acceleration` should be. The motion itself is always these two lines.

---

## 🎯 Chapter Summary

```
KINEMATICS = THE LANGUAGE OF MOTION

  POSITION:     "Where am I?"        → Vec2 in pixels
  VELOCITY:     "How fast and       → Vec2 in pixels/second
                 which way?"
  ACCELERATION: "How is my          → Vec2 in pixels/second²
                 velocity changing?"
  
  THE CHAIN:
    a ──∫──► v ──∫──► x
    (integrate)  (integrate)
    
    EVERY FRAME:
    vel += acc × dt
    pos += vel × dt    ← uses NEW velocity!
    
  SUVAT EQUATIONS (for constant acceleration):
    v = u + at
    s = ut + ½at²
    v² = u² + 2as
    
  KEY INSIGHT: All three quantities are the SAME thing
  at different levels of "zoom" on the time axis.
  Position is where you are. Velocity is how position
  CHANGES. Acceleration is how velocity CHANGES.
  That's all. Everything else is implementation.
```

> **The real value of kinematics isn't the formulas — it's the way of thinking. Every game physics problem reduces to: "What's the acceleration? OK, now integrate twice." Master that mental model, and you can simulate anything that moves.** 🏃

---

**[← Previous: Trigonometry](ch06-trigonometry.md)** | **[Next: Dynamics →](ch08-dynamics.md)**
