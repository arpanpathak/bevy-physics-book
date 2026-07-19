# ⚙️ Setting Up: From Zero to a Working Bevy Physics Project

> **Before we write a single line of physics code, we need a working Rust development environment and a Bevy project. This chapter walks you through EVERY step - from installing Rust to seeing your first physics object fall on screen. No prior Rust or Bevy experience required.** 🔨

---

## 🎯 Prerequisites - What You Need

To follow this book, you need:
- A computer (Windows, Mac, or Linux) with internet access
- About 2GB of free disk space (Rust + dependencies take space)
- A text editor or IDE (VS Code recommended, but any will work)
- The ability to open a terminal/command prompt
- **No prior Rust experience required** - we'll install it together

---

## 📥 Step 0: Install Rust (If You Haven't Already)

If you already have Rust installed, skip to Step 1. If not, here's how:

### For Mac and Linux:

Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This downloads and runs the Rust installer. When prompted:
1. Type `1` and press Enter (this chooses the default installation)
2. Wait for the installation to complete (1-2 minutes)
3. Close and reopen your terminal, OR run: `source $HOME/.cargo/env`

### For Windows:

1. Go to https://rustup.rs in your web browser
2. Download `rustup-init.exe` (the big orange button)
3. Run the installer
4. Select "Default installation" when prompted
5. After installation, open a NEW command prompt or PowerShell

### Verify Rust Is Installed

Run these commands to confirm everything works:

```bash
# Check Rust version (should show rustc 1.8x or later)
rustc --version

# Check Cargo version (Cargo is Rust's build tool)
cargo --version
```

If you see version numbers, you're good. If you get "command not found," restart your terminal or log out and back in.

### What Did We Just Install?

| Tool | What It Does | Why We Need It |
|------|-------------|----------------|
| `rustc` | The Rust **compiler** - turns `.rs` files into executable programs | Without it, our code is just text |
| `cargo` | Rust's **package manager and build tool** - creates projects, downloads dependencies, runs tests | Bevy is a dependency. Cargo downloads it and all 200+ crates it needs |
| `rustup` | The Rust **version manager** - lets you switch between Rust versions | Ensures you can update to the latest Rust when needed |

---

## 🆕 Step 1: Create the Project

Now let's create the Bevy physics project:

```bash
# Create a new Rust project called "my_physics_game"
cargo new my_physics_game

# Move into the project directory
cd my_physics_game
```

Let's see what `cargo new` created:

```bash
# List all files (including hidden ones)
ls -la

# Output should look like:
#   Cargo.toml    <- The project configuration file
#   src/          <- Source code directory
#     main.rs     <- The main entry point file
#   .git/         <- Git repository (for version control)
```

### What Are These Files?

```
my_physics_game/
+-- Cargo.toml          📋 THE RECIPE CARD
|                       Contains: project name, version, dependencies list.
|                       When you add "bevy = 0.15" here, Cargo downloads
|                       the entire Bevy engine and all its dependencies.
|
+-- src/
|   +-- main.rs         📝 THE ENTRY POINT
|                       Contains the main() function. When you run
|                       "cargo run", this is the file that executes.
|                       Initially contains: fn main() { println!("Hello!"); }
|
+-- .git/               📦 VERSION CONTROL (optional for us)
                        Git repository for tracking changes to your code.
                        Created automatically by cargo new.
```

---

## 📦 Step 2: Add Bevy as a Dependency

Open `Cargo.toml` in your text editor. It currently looks like this:

```toml
[package]
name = "my_physics_game"
version = "0.1.0"
edition = "2021"
description = "🎮 A physics playground built with Bevy & Rust"

# NOTE: There are NO dependencies yet.
# The [dependencies] section doesn't exist - we ADD it.
```

**Change it to this:**

```toml
[package]
name = "my_physics_game"
version = "0.1.0"
edition = "2021"

[dependencies]
# 🎯 Bevy - the game engine
# Version 0.15 is what this book targets.
# The * means "latest compatible version in the 0.15 range"
bevy = "0.15"
```

Now let's test that everything works:

```bash
# Build the project (this downloads Bevy and compiles it)
# First build takes 5-15 minutes depending on your internet speed
cargo build
```

**What happens during `cargo build`:**

```
Step 1: Cargo reads Cargo.toml
Step 2: Sees "bevy = 0.15" in [dependencies]
Step 3: Contacts crates.io (Rust's package registry)
Step 4: Downloads Bevy source code (~100+ crates)
Step 5: Downloads Bevy's dependencies (another ~100+ crates)
Step 6: Compiles everything (this takes the longest)
Step 7: Creates the executable in target/debug/

Subsequent builds are MUCH faster - Cargo caches everything.
```

> ⚠️ **First build tip:** Go make coffee. Seriously. Bevy compiles over 200 dependent crates on the first build. Subsequent builds take seconds, not minutes.

---

## 🏗️ Step 3: Create the Physics Module Structure

Now let's create the folder structure for our physics engine:

```bash
# Create the physics module directory
mkdir -p src/physics

# Create empty module files (we'll fill them in shortly)
touch src/physics/mod.rs
touch src/physics/components.rs
touch src/physics/systems.rs
```

Your project structure should now look like:

```
my_physics_game/
+-- Cargo.toml                  # Dependencies
+-- src/
|   +-- main.rs                 # Entry point
|   +-- physics/
|       +-- mod.rs              # Physics plugin definition
|       +-- components.rs       # Position, Velocity, Mass, etc.
|       +-- systems.rs          # Integration, forces, rendering sync
```

### Why This Structure?

Each file has one job. This is called **separation of concerns**:

```
src/
+-- main.rs              🎬 THE ORCHESTRATOR
|                        Only sets up the app and spawns entities.
|                        Doesn't know HOW physics works - only that it exists.
|
+-- physics/             🧠 THE PHYSICS ENGINE (reusable module)
    +-- mod.rs           📝 THE PUBLIC API
    |                    Exposes PhysicsPlugin. External code just adds it.
    |                    "What can outsiders see?" -> Only the plugin.
    |
    +-- components.rs    📍 THE VOCABULARY
    |                    Defines Position, Velocity, Mass, ForceAccumulator.
    |                    PURE DATA. No logic. Just structs with fields.
    |                    "What concepts exist?" -> Position, Velocity, etc.
    |
    +-- systems.rs       🔄 THE GRAMMAR
                         Defines how components transform each frame.
                         PURE LOGIC. Functions that read/write components.
                         "What happens each frame?" -> Integrate, sync, etc.
```

> 💡 **The key insight:** If you separate data from logic, you can:
> 1. Change the logic without changing the data (and vice versa)
> 2. Test the logic in isolation (pass in test data, check output)
> 3. Reuse the logic with different data (different games, same physics)
> 4. Run the logic in parallel (no shared mutable state - the ECS guarantee)

---

## 📝 Step 4: Components - The Deep Dive

This is where we define the VOCABULARY of our physics engine. Every concept in physics becomes a Rust struct with `#[derive(Component)]`. 

**Important rule:** Components are PURE DATA. No logic. No methods that modify other components. All logic lives in systems (Step 5). This separation is what makes ECS fast and parallelizable.

---

### Position: Where Is the Object?

**What it represents:** A location in 2D space. Position is a VECTOR from the world origin (0,0) to the entity's location. In physics terms: `r(t)` - the object's location at time t.

**Why a separate component from Bevy's Transform?**
- Transform bundles position + rotation + scale together. Physics only needs position.
- Transform is connected to Bevy's scene graph - changing it triggers parent/child hierarchy updates.
- Carrying rotation + scale through every physics iteration wastes CPU cache bandwidth.
- By separating, the physics pipeline is independent of the rendering pipeline.

**The struct:**

```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub Vec2);
```

That's it. A single `Vec2` wrapped in a newtype. `Vec2` holds the x and y coordinates.

**Usage examples:**
- `Position::new(400.0, 300.0)` - center of an 800×600 screen
- `Position::new(0.0, 0.0)` - the origin (world center)
- `Position(Vec2::ZERO)` - same as above, using default

---

### Velocity: How Fast and Which Way?

**What it represents:** The RATE OF CHANGE of position over time. If velocity is (50, 0), the entity moves RIGHT at 50 units every second.

**Physical intuition:** Think of velocity as "where the object WANTS to go next."
- `(50, 0)` = "move right at 50 px/s"
- `(0, -200)` = "move down at 200 px/s" (falling)
- `(0, 0)` = "stay still"

**The formula this enables:**
```
new_position = old_position + velocity × delta_time
```

This is THE fundamental equation of motion. Everything in game physics builds on it.

**The struct:**

```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity(pub Vec2);
```

Velocity and Position are BOTH `Vec2`. They can be added together because they're the SAME type. This is the beauty of vectors - position and velocity live in the same mathematical space.

---

### Acceleration: What Forces Are Acting?

**What it represents:** The RATE OF CHANGE of velocity. If acceleration is (0, -500), the velocity decreases by 500 px/s every second. This is what gravity does - it continuously increases your downward speed.

**The connection to Newton's Second Law:**
```
F = m × a   ->   a = F / m
```
Acceleration is where FORCES live. Gravity, drag, thrust, wind - they all produce acceleration, which changes velocity, which changes position.

**The pipeline:**
```
Forces -> a = F/m -> v += a×dt -> x += v×dt
(cause)  (Newton)  (integrate)  (integrate)
```

**Critical:** Acceleration is RECALCULATED every frame. It does NOT persist. Each frame:
1. Clear acceleration to zero
2. Apply all forces (gravity, drag, etc.)
3. Compute a = F/m
4. Integrate into velocity
5. Clear for next frame

**The struct:**

```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Acceleration(pub Vec2);
```

---

### Mass: How Hard Is It to Move This Object?

**What it represents:** INERTIA - resistance to changing velocity. Newton's Second Law: `F = m × a`, so `a = F / m`. Higher mass = same force produces less acceleration.

**Practical values:**
- `mass = 1.0` -> Normal dynamic object. Force = acceleration numerically.
- `mass = 10.0` -> Heavy object. Same force gives 1/10 the acceleration.
- `mass = 0.0` -> STATIC object. NEVER moves. Walls, floors, pillars. Handled as special case to avoid division by zero.

**The mass canceling trick:** Since `a = F/m` and gravity's `F = m × g`, we get `a = (m × g) / m = g`. The MASS CANCELS OUT. All objects fall at the same rate regardless of mass. A feather and a boulder fall identically in vacuum.

**The struct:**

```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Mass(pub f32);
```

---

### ForceAccumulator: The Scratch Paper for Forces

**What it represents:** A temporary buffer that collects ALL forces acting on an entity during a single frame. Forces are ADDED to this buffer by various systems (gravity, drag, input), then the TOTAL is divided by mass to get acceleration.

**Why clear every frame?** Forces are like pushes. If you push a box across the floor:
- WHILE you're pushing: force is present -> box accelerates
- AFTER you stop: force is GONE -> box slows (friction)
- But the VELOCITY persists (inertia) until friction stops it

If forces were NOT cleared, every force would be "always on." Gravity applied once would stay forever, accumulating MORE each frame. Objects would accelerate to infinite speed.

**The struct:**

```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct ForceAccumulator {
    pub total_force: Vec2,
}

impl ForceAccumulator {
    pub fn add_force(&mut self, force: Vec2) {
        self.total_force += force;
    }
    pub fn clear(&mut self) {
        self.total_force = Vec2::ZERO;
    }
}
```

---

### The Complete Data Flow

Here's how the four components interact every single frame. Read top to bottom:

```
+----------------------------------------------------------------+
|                    ONE PHYSICS TIMESTEP                         |
+----------------------------------------------------------------+
|                                                                  |
|  +--------------------+                                          |
|  | ForceAccumulator   |  <-- Gravity adds (0, -500) * mass      |
|  |  = (0, -1000)      |  <-- Drag adds -velocity * damping      |
|  +---------+----------+                                          |
|            |                                                     |
|            v   a = F / m = (0, -1000) / 2.0 = (0, -500)         |
|            |                                                     |
|  +--------------------+                                          |
|  | Acceleration       |  = (0, -500) px/s^2                      |
|  +---------+----------+                                          |
|            |                                                     |
|            v   v += a * dt = (0, 0) + (0, -500) * 0.01667       |
|            |                                                     |
|  +--------------------+                                          |
|  | Velocity           |  = (0, -8.33) px/s                       |
|  +---------+----------+                                          |
|            |                                                     |
|            v   x += v * dt = (0, 300) + (0, -8.33) * 0.01667    |
|            |                                                     |
|  +--------------------+                                          |
|  | Position           |  = (0, 299.86) px                        |
|  +--------------------+                                          |
|                                                                  |
|  Then: Acceleration is cleared to (0, 0) for next frame          |
+------------------------------------------------------------------+
```

---

> 💡 **Full source code for this chapter:** [code-examples/ch02-setup/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch02-setup)
> 
> The runnable project includes Cargo.toml, main.rs, and the complete physics module in separate files.

---

## 🔬 Deep Dive: What "Integration" Actually Means (Trace Through With Real Numbers)

This is the single most important concept in game physics. I'm going to explain it THREE ways:
1. **An everyday analogy** (driving a car)
2. **The math** (simple arithmetic, I promise)
3. **The code** (how it looks in Rust)

After this, you will NEVER look at motion the same way again.

---

### The Analogy: Driving a Car

You're driving a car. Your foot is on the gas pedal.

```
The gas pedal is FORCE.      -> How hard you push it.
The car's speed is VELOCITY. -> How fast you're going.
The road position is POSITION. -> Where you are on the road.
```

**Let's trace 5 seconds of driving, 1 second at a time:**

```
Second 0: You're stopped at a red light.
  - Gas pedal: not pressed     (force = 0)
  - Speed: 0 mph               (velocity = 0)
  - Position: mile marker 0    (position = 0)

Second 1: The light turns green. You press the gas gently.
  - Gas pedal: lightly pressed  (force = small)
  - Speed increases to: 5 mph   (velocity = 5)
  - You've moved to: mile 0.08  (position = 0.08)
  - Why 0.08? Average speed during this second was (0+5)/2 = 2.5 mph.
    2.5 mph for 1 second = 0.00069 miles ≈ 0.08 miles in game units.

Second 2: You press the gas harder.
  - Gas pedal: medium press     (force = medium)
  - Speed increases to: 15 mph  (velocity = 15)
  - You've moved to: mile 0.36  (position = 0.36)
  - Average speed: (5+15)/2 = 10 mph -> moved 0.28 miles this second.

Second 3: You floor it.
  - Gas pedal: floored          (force = maximum)
  - Speed increases to: 30 mph  (velocity = 30)
  - Position: mile 1.01         (position = 1.01)

Second 4: You see a red light ahead. You lift your foot off the gas.
  - Gas pedal: released         (force = 0)
  - Speed drops to: 25 mph      (velocity = 25 - drag slows you)
  - Position: mile 1.47         (position = 1.47)

Second 5: You brake gently.
  - Brake pedal: pressed        (force = negative/braking)
  - Speed drops to: 10 mph      (velocity = 10)
  - Position: mile 1.65         (position = 1.65)
```

**Look at the pattern. Every second, three things happen:**

```
1. The PEDAL (force) changes the SPEED (velocity).
2. The SPEED (velocity) changes the POSITION (position).
3. The NEW speed is used to compute the NEXT position.

This is EXACTLY what our physics code does, 60 times per second!
```

---

### The Math: Two Lines That Control Everything

The car example above follows EXACTLY two mathematical rules:

```
Rule 1: new_speed     = old_speed     + (pedal_force / car_mass) × time
Rule 2: new_position  = old_position  + new_speed × time
```

Let's verify with the car example, second by second:

```
SECOND 1:
  old_speed = 0 mph
  pedal = 10 (gas), mass = 2 (car is heavy), time = 1 second
  
  new_speed = 0 + (10/2) × 1 = 5 mph  ✓
  new_position = 0 + 5 × 1 = 5         ✓ (scaled down to 0.08 miles earlier)

SECOND 2:
  old_speed = 5 mph
  pedal = 20 (more gas), mass = 2, time = 1
  
  new_speed = 5 + (20/2) × 1 = 15 mph  ✓
  new_position = 5 + 15 × 1 = 20       ✓ (scaled to 0.36 miles)
  
Notice: we used NEW SPEED (15) for position, not the old speed (5).
This is SYMPLECTIC integration. If we used old speed:
  position = 5 + 5 × 1 = 10 --- WRONG! We'd be behind where we actually are.
```

**The two rules in physics notation:**

```
v(t + dt) = v(t) + a × dt         <- Rule 1: force changes velocity
x(t + dt) = x(t) + v(t+dt) × dt   <- Rule 2: velocity changes position
                                     (note: uses v(t+dt), the NEW velocity!)
```

In game code, these become TWO LINES:

```rust
// The ENTIRE physics engine, in two lines:
velocity += acceleration * delta_time;           // Rule 1
position += velocity * delta_time;                // Rule 2 (uses NEW velocity!)
```

**That's it. Everything else in this book -- forces, collisions, constraints -- is just figuring out what `acceleration` should be. The motion itself is always these two lines.**

---

### Trace It: Exactly What Happens in 60 Frames of a Falling Object

Let's watch a ball fall for 1 second (60 frames) with real numbers:

```
INITIAL STATE:
  position.y = 300          (ball starts at y=300, near the top of the screen)
  velocity.y = 0            (ball is not moving yet -- dropped from rest)
  gravity = -500            (gravity pulls down at 500 pixels/second^2)
  dt = 1/60 ~ 0.01667      (each frame is about 16.67 milliseconds)

Frame |  velocity  |  position  |  What's happening
------+------------+------------+-------------------------------------------
  0   |     0.00   |   300.00   |  Ball released. Starts falling.
  1   |    -8.33   |   299.86   |  Gravity kicked in. Speed = -8.33
  2   |   -16.67   |   299.58   |  Faster. Fell 0.28 pixels this frame
  3   |   -25.00   |   299.17   |  Even faster.
  4   |   -33.33   |   298.61   |  Speed is building up linearly.
  5   |   -41.67   |   297.92   |  Position curve is parabolic.
  10  |   -83.33   |   290.28   |  Half a second in. Falling fast.
  15  |  -125.00   |   276.39   |
  20  |  -166.67   |   256.94   |  Midpoint of the fall.
  30  |  -250.00   |   193.06   |  Ball is moving visibly fast.
  40  |  -333.33   |   105.56   |  Really zooming now.
  50  |  -416.67   |   -15.28   |  Past the bottom of the screen!
  60  |  -500.00   |  -158.33   |  1 second elapsed. Off screen.

KEY OBSERVATIONS:
  1. Velocity INCREASES by 8.33 EVERY frame. (-500 x 0.01667 = -8.33)
     This is LINEAR growth. Each frame adds the same amount.

  2. Position CHANGES by velocity x dt each frame.
     Since velocity grows, position changes MORE each frame.
     This is QUADRATIC/PARABOLIC growth - like a snowball rolling downhill.

  3. After 60 frames (1 second):
     velocity = -500 pixels/second (terminal speed from gravity alone)
     position changed by = 0 + (-500) x 1^2 / 2 = -250 pixels
     Check: 1/2 x (-500) x 1^2 = -250. And 300 - 250 = 50.
     But our table shows -158 at frame 60!
     
     Why the difference? Because we're using DISCRETE integration (frames),
     not the continuous formula. The discrete version is an APPROXIMATION.
     With smaller dt, it gets closer to the exact answer.
     With dt = 1/120 (120 FPS), the error is halved.

THIS IS WHY WE USE SMALL TIMESTEPS!
More frames per second = more accurate physics.
But also more computation. Trade-offs everywhere in game dev.
```

---

### The Wrong Way: What Happens If You Use Old Velocity

Let me show you why using the OLD velocity (Explicit Euler) instead of the NEW velocity (Symplectic Euler) causes EXPLOSIONS:

```
BOTH methods start the same:

Frame 0: velocity = 0, position = 0
Frame 1: velocity += -500 x 0.01667 = -8.33

NOW THE DIFFERENCE:

SYMPLECTIC (correct): position += NEW velocity = -8.33
  position = 0 + (-8.33) x 0.01667 = -0.139
  Energy stays constant. ✓

EXPLICIT (wrong):     position += OLD velocity = 0
  position = 0 + 0 x 0.01667 = 0  <- The ball DIDN'T MOVE this frame!
  Energy artificially LOW.
  
Frame 2:
  SYMPLECTIC: velocity = -16.67, position = -0.417  ✓
  EXPLICIT:   velocity = -16.67, position = -0.139  <- Still behind!

After 60 frames (1 second) of an ideal orbit:
  SYMPLECTIC: Orbit stays stable. Energy conserved. ✓
  EXPLICIT:   Orbit spiraled outward by 5%! Energy INCREASED! ✗

THE INTUITION:
  Explicit Euler underestimates motion (uses old, slower velocity).
  This adds energy to the system each frame.
  After thousands of frames, energy has doubled, tripled, etc.
  Objects fly off to infinity! 💥
  
  Symplectic Euler estimates average velocity (new velocity ~ average).
  Energy stays balanced. Orbits stay in orbit. Springs don't explode.
```

---

### The Code: How It Looks in Rust

Now that you understand the WHY, here's the code that implements it:

The TWO-LINE integration that powers all of physics.

 Input:  velocity, position (current state)
         acceleration (from F = ma)
         delta_time (timestep, typically 1/60 second)
 Output: velocity, position (next state, modified in-place)

```rust
fn symplectic_euler_step(
    velocity: &mut Vec2,
    position: &mut Vec2,
    acceleration: &Vec2,
    delta_time: f32,
) {
    // Rule 1: Acceleration changes velocity.
    // v_new = v_old + a x dt
    //
    // Example: v = (0, 0), a = (0, -500), dt = 0.01667
    //   v_new = (0, 0) + (0, -500) x 0.01667 = (0, -8.33)
    *velocity += *acceleration * delta_time;
    
    // Rule 2: Velocity changes position. Uses NEW velocity!
    // x_new = x_old + v_new x dt
    //
    // Example: x = (0, 300), v_new = (0, -8.33), dt = 0.01667
    //   x_new = (0, 300) + (0, -8.33) x 0.01667 = (0, 299.86)
    *position += *velocity * delta_time;
}
```

And here's the full Bevy system that calls it for every physics entity every frame:

The integration system registered with Bevy.
 It runs every frame, for every entity that has all four physics components.

```rust
pub fn integrate_positions_using_symplectic_euler(
    mut physics_query: Query<(
        &ForceAccumulator,
        &Mass,
        &mut Acceleration,
        &mut Velocity,
        &mut Position,
    )>,
    physics_settings: Res<PhysicsSettings>,
) {
    // Use the FIXED timestep, not the real frame delta.
    // This makes physics framerate-independent.
    let delta_time = physics_settings.fixed_delta_time;

    for (force_accumulator, mass, mut acceleration, mut velocity, mut position) in
        physics_query.iter_mut()
    {
        // Step 1: Newton's Second Law - a = F/m
        if mass.value > 0.0 {
            acceleration.value = force_accumulator.total_force / mass.value;
        } else {
            acceleration.value = Vec2::ZERO;
        }

        // Step 2: Symplectic Euler integration
        velocity.value += acceleration.value * delta_time;
        position.value += velocity.value * delta_time;  // Uses NEW velocity!
    }
}
```

---

### The PhysicsSettings Resource: World Settings

Global physics settings stored as a Bevy Resource (singleton).

 A Resource in Bevy is like a global variable - but managed by Bevy's
 scheduler so systems can safely read/write it without conflicts.

 Why a Resource and not a Component?
   - Gravity affects ALL objects equally (not per-entity)
   - Timestep is a WORLD setting, not an object property
   - There's only ONE physics world with one set of settings

 Why FIXED timestep and not the frame's delta time?
   If dt varied with framerate:
     30 FPS: dt = 33ms, objects jump TWICE as far per step
     120 FPS: dt = 8ms, objects move HALF as far per step
   Physics would run at DIFFERENT speeds on different computers!

   Fixed timestep: physics always advances by 1/60 second per step.
   If a frame takes 33ms (30 FPS): physics runs TWICE (catch up)
   If a frame takes 8ms (120 FPS): physics runs ONCE (just right)
   The RESULT is identical regardless of framerate!
 Gravity vector. Default: (0, -500) pixels/second^2 in 2D.
 Negative Y = pulls objects downward.
 Game feel: (0, -9.81) = realistic, (0, -500) = platformer snappy
 Fixed physics timestep. 1/60 = ~16.67ms for 60 Hz simulation.
 Standard choice: 1/60. Stable and compatible with 60 FPS rendering.

```rust
#[derive(Resource)]
pub struct PhysicsSettings {
    pub gravity: Vec2,
    
    pub fixed_delta_time: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -500.0),
            fixed_delta_time: 1.0 / 60.0,
        }
    }
}
```

---

## 🎬 Step 6: Main Entry Point  -  The Full Picture

Now let's understand the main.rs in its entirety:

```rust
use bevy::prelude::*;

mod physics;

fn main() {
    App::new()
        // --- DEFAULT PLUGINS ---
        // This ONE call enables: window, renderer, input, audio, time, UI
        // Without it, you'd have a headless physics simulation
        // (which is valid for server-side physics!)
        .add_plugins(DefaultPlugins)
        
        // --- CUSTOM PHYSICS PLUGIN ---
        // Our physics module exposes a Plugin that registers
        // all physics systems, resources, and events.
        // Adding it here = installing the physics engine.
        .add_plugins(physics::PhysicsPlugin)
        
        // --- STARTUP SYSTEMS ---
        // Systems that run ONCE when the app starts.
        // Use these for: spawning initial entities, setting up
        // resources that need computation, camera setup.
        .add_systems(Startup, setup)
        
        // --- UPDATE SYSTEMS ---
        // Systems that run EVERY FRAME.
        // Our physics systems are registered inside PhysicsPlugin,
        // but game-specific logic goes here.
        // Note: game systems should run AFTER physics to avoid
        // reading stale state.
        .add_systems(Update, (
            player_input,
            camera_follow,
        ).after(physics::PhysicsPlugin))
        
        // 🚀 ENTER THE MAIN LOOP
        .run();
}
```

### What Bevy Does After `.run()`

1. **Builds the system graph** from all `.add_systems()` calls
2. **Determines parallel execution** groups (systems accessing different components can run simultaneously)
3. **Runs all Startup systems** (your `setup()` function creates the initial world state)
4. **Enters the render loop**: For each frame, run Update systems -> sync Commands -> render

---

## 🔬 The Complete Data Flow: Trace Through One Frame

Let's trace a complete frame from start to finish, showing exactly what data exists and how it transforms:

```
INITIAL STATE (Frame N, start):
--------------------------------
World:
  Entity(1):
    Position: (0.0, 300.0)
    Velocity: (0.0, 0.0)
    Acceleration: (0.0, 0.0)    <- Cleared at end of last frame
    Mass: 1.0

  Resource:
    PhysicsSettings.gravity: (0.0, -500.0)  <- 500 px/s² downward
    PhysicsSettings.fixed_dt: 0.01667        <- 1/60 second

FRAME N BEGINS:
----------------

STEP 1: clear_forces()
  Entity(1).Acceleration = (0.0, 0.0)  <- Already zero, just confirming

STEP 2: apply_gravity()
  Entity(1).Acceleration += (0.0, -500.0) × 1.0 (mass is implicit here)
  Entity(1).Acceleration = (0.0, -500.0)
  v
  Now acceleration says: "I'm accelerating downward at 500 px/s²"

STEP 3: integrate()
  Sub-step 1 (sub_dt = 0.01667):
    a = (0.0, -500.0)                      <- From F = ma
    v += (0.0, -500.0) × 0.01667
    v = (0.0, -8.33)                       <- Fell 8.33 px/s in this step
    x += (0.0, -8.33) × 0.01667
    x = (0.0, 299.86)                      <- Moved down 0.14 pixels
    a = (0.0, 0.0)                         <- Cleared for next sub-step

  (If substeps = 1, we're done. If substeps = 4,
   repeat 3 more times with smaller steps)

STEP 4: sync_to_render()
  Entity(1).Transform.translation = (0.0, 299.86, 0.0)  <- Bevy's renderer sees this

FRAME N ENDS:
----------------
  BOTTOM LINE: Entity moved from y=300 to y=299.86 (falling!)
  Next frame: velocity continues accumulating, object accelerates
  After ~60 frames (1 second): object has fallen ~250 pixels
```

---

## 🎯 What We Learned (The Deep Version)

| Concept | What It Means | Why It Matters |
|---------|---------------|----------------|
| **App** | A builder for the entire game | Everything is registered here  -  plugins, systems, resources |
| **DefaultPlugins** | Built-in window/render/input/audio | Provides the "runtime" for our physics to exist in |
| **Plugin** | A package of systems + resources | Encapsulates our physics engine as a reusable unit |
| **System** | A function that transforms ECS data | Stateless, parallelizable, scheduled by Bevy |
| **Query** | A pattern-matching access to components | Like a database query  -  find entities with specific components |
| **Component** | A piece of typed data on an entity | Position, Velocity, etc.  -  PURE DATA, NO LOGIC |
| **Resource** | Global singleton data | PhysicsSettings  -  affects all entities |
| **Commands** | Deferred spawn/despawn/modify | Batched for efficiency, executed at sync points |
| **Sync Point** | Where deferred ops are flushed | Stops parallel execution temporarily  -  MINIMIZE THESE |
| **Integration** | Transforming acceleration -> velocity -> position | The mathematical heart of physics simulation |
| **Fixed Timestep** | Physics runs at constant rate regardless of FPS | Deterministic, framerate-independent simulation |

---

> **Key Takeaway:** Bevy's architecture isn't just ceremony  -  it's a carefully designed system that enables parallel, cache-efficient, deterministic physics simulation. Every struct, every trait, every registration call serves a purpose in the data flow. Understanding this flow is the difference between "copying code" and "knowing how to build." 🏗️

---

**[<- Previous: Foreword & Index](ch01-foreword.md)** | **[Next: Vector Mathematics ->](ch03-vectors.md)**
