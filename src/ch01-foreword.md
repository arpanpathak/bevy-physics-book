# 🎮 Rust Game Development Physics Math ⚡

## A Comprehensive Guide to Building Physics Engines with Bevy & Rust

> **"Game physics is the art of lying convincingly  -  making the virtual world feel real, one elegant equation at a time."** 🎯

---

![Physics Math Banner](https://img.shields.io/badge/Rust-Physics%20Math-dea584?style=for-the-badge&logo=rust&logoColor=white)
![Bevy](https://img.shields.io/badge/Bevy-0.15-7B42ED?style=for-the-badge&logo=bevy&logoColor=white)
![Status](https://img.shields.io/badge/Status-Comprehensive-2ea44f?style=for-the-badge)

---

## 📖 Table of Contents

| #  | Chapter | ⭐ Highlights |
|----|---------|--------------|
| 01 | 🚀 [Foreword & Index](ch01-foreword.md) | Navigation guide, how to read this book |
| 02 | ⚙️ [Setting Up Your Bevy Physics Playground](ch02-setup.md) | Cargo setup, Bevy boilerplate |
| 03 | 🧮 [Vector Mathematics: The Language of Space](ch03-vectors.md) | 2D/3D vectors, operations, applications |
| 04 | 🔢 [Matrices & Transformations](ch04-matrices.md) | Rotation, scaling, translation, composition |
| 05 | 🌀 [Quaternions: Rotations Without Gimbal Lock](ch05-quaternions.md) | Quaternion math, slerp, orientation |
| 06 | 📐 [Trigonometry for Game Physics](ch06-trigonometry.md) | Sin/cos/tan, angles, projectile motion |
| 07 | 🏃 [Kinematics: The Geometry of Motion](ch07-kinematics.md) | Position, velocity, acceleration curves |
| 08 | 💥 [Dynamics: Forces & Newton's Laws](ch08-dynamics.md) | Force accumulation, gravity, friction, drag |
| 09 | 🔄 [Integration Methods: Simulating Motion Over Time](ch09-integration.md) | Euler, Verlet, RK4 comparisons |
| 10 | 🧱 [Collision Detection: Finding Overlaps](ch10-collision-detection.md) | AABB, Circle, SAT, Raycasting |
| 11 | 🤝 [Collision Response: Making Things Bounce](ch11-collision-response.md) | Impulse resolution, restitution, friction |
| 12 | 🔗 [Constraints & Joints](ch12-constraints.md) | Springs, distance constraints, ragdolls |
| 13 | 📦 [Spatial Partitioning: Optimization at Scale](ch13-spatial-partitioning.md) | Grid, Quadtree, BVH |
| 14 | 🏗️ [Bevy ECS Physics Architecture](ch14-ecs-architecture.md) | Systems, resources, bundles, plugins |
| 15 | 🎮 [Mini Physics Sandbox: Putting It All Together](ch15-physics-sandbox.md) | Complete working game example |
| 16 | 📚 [Appendix: Rust Patterns & References](ch16-appendix.md) | Cheat sheets, further reading |

---

## 🤔 Who Is This Book For?

This book is for **Rust game developers** who want to understand the *mathematical soul* of physics simulation. You'll learn:

- ✅ **The math**  -  vectors, matrices, quaternions, calculus for games
- ✅ **The code**  -  clean, idiomatic Rust with Bevy's ECS
- ✅ **The architecture**  -  how to structure physics engines that scale
- ✅ **The intuition**  -  why things work (and when they break)

> **Prerequisites:** Basic Rust knowledge. No prior physics or Bevy experience required  -  we build from the ground up! 🌱

---

## 🧭 How to Read This Book

```
Mathematical Foundations (Ch 3-6)
        |
        v
Kinematics & Dynamics (Ch 7-8)
        |
        v
Integration Methods (Ch 9)
        |
        v
Collision Systems (Ch 10-11)
        |
        v
Advanced Topics (Ch 12-13)
        |
        v
Architecture & Demo (Ch 14-15)
```

Each chapter builds on the previous. Code examples use **Bevy 0.15+** and are **copy-paste runnable**. 🏃‍♂️

---

## 🎯 What Makes Good Game Physics?

```
        Realism
           |
    +-----------------------+
    |   "Good Enough"       |
    |   Physics Zone        |
    +-----------------------+
           |
    +-----------------------+
    |   Performance         |  <- Always the bottleneck!
    +-----------------------+
```

**The Golden Rule of Game Physics:** ⭐

> *"It doesn't need to be physically accurate  -  it needs to be **physically plausible** and **fun**."*

Real physics engines (like NASA's) use double-precision matrix decompositions with femtosecond timesteps. **Game physics** uses floats, cheats, and Euler integration  -  and that's perfectly fine! 🎮

---

## 🛠️ Conventions Used

```rust
// 📝 Comments like this explain WHY, not WHAT
// 💡 Pro-tips give you deeper insight
// ⚠️ Warnings highlight common pitfalls
// 🔥 Advanced topics you can skip on first read

// Code blocks marked with file names show complete files
// Inline `code` references types, functions, or variables
```

---

## 🚀 Quick Start For The Impatient

Want to skip the explanations and just see something move on screen? Follow these exact steps:

### 0. Install Rust (if you haven't already)

Open a terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then close and reopen your terminal. Verify it worked:

```bash
rustc --version   # Should show rustc 1.8x or later
cargo --version   # Should show cargo 1.8x or later
```

### 1. Create a new Rust project

Open a terminal and run:

```bash
cargo new my_physics_game
cd my_physics_game
```

### 2. Add Bevy as a dependency

Open `Cargo.toml` and replace its contents with:

```toml
[package]
name = "my_physics_game"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.15"
```

### 3. Write the code

Open `src/main.rs` and replace its contents with:

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, physics_step)
        .add_systems(Update, render_position)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Position(Vec2::new(0.0, 0.0)),
        Velocity(Vec2::new(50.0, 100.0)),
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.5, 0.2), Vec2::new(20.0, 20.0)),
            ..default()
        },
    ));
}

fn physics_step(mut query: Query<(&Velocity, &mut Position)>, time: Res<Time>) {
    for (vel, mut pos) in query.iter_mut() {
        pos.0 += vel.0 * time.delta_secs();
    }
}

fn render_position(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(pos.0.x, pos.0.y, 0.0);
    }
}
```

### 4. Run it

```bash
# First build takes 5-15 minutes (downloading + compiling Bevy)
cargo run
```

A window will appear with an orange square moving diagonally across a black background.

> **That's it.** An orange square moves across the screen. The rest of this book explains what each piece does and how to build a complete physics engine from here. The full runnable code is at [code-examples/ch02-setup/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch02-setup).

**Ready? Let's dive in!** 🏊‍♂️

---

> **[Next Chapter: Setting Up Your Bevy Physics Playground ->](ch02-setup.md)**
