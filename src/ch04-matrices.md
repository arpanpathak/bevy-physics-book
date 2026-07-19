# 🔢 Matrices & Transformations

> **"A matrix is a function that takes a vector and returns a vector — but it's a special kind of function: LINEAR. Scaling, rotation, reflection, shear — all of these are just matrix multiplications."** 🔄

---

## 🎯 The Core Idea: A Matrix IS a Transformation

A matrix isn't just a grid of numbers. A matrix **IS a transformation** — when you multiply it by a vector, you get a new, transformed vector.

```
    ┌         ┐   ┌   ┐     ┌         ┐
    │ a  b  tx │   │ x │     │  x'  │
    │ c  d  ty │ × │ y │  =  │  y'  │
    │ 0  0  1  │   │ 1 │     │  1   │
    └         ┘   └   ┘     └         ┘
         ↑           ↑           ↑
    The matrix      Input      Output
    (WHAT to do)   (WHERE)    (RESULT)
```

**The profound insight:** The COLUMNS of a matrix are the **basis vectors** of the transformed coordinate system. When you multiply, you're expressing the input vector in a NEW coordinate system:

```
Result = input.x × column_1 + input.y × column_2 + column_3

Where:
  column_1 = where the X-axis goes after transformation
  column_2 = where the Y-axis goes after transformation
  column_3 = where the origin goes (translation)
```

Let's see this concretely:

```rust
let identity = Mat3::IDENTITY;
// Column 0: (1, 0, 0) → X axis stays at (1,0)
// Column 1: (0, 1, 0) → Y axis stays at (0,1)
// Column 2: (0, 0, 1) → Origin stays at (0,0)
// Result: NO transformation

let scale = Mat3::from_scale(Vec2::new(2.0, 3.0));
// Column 0: (2, 0, 0) → X axis is now TWICE as long
// Column 1: (0, 3, 0) → Y axis is THREE times as long
// Column 2: (0, 0, 1) → Origin unchanged
// Result: points get stretched

let rotate = Mat3::from_angle(std::f32::consts::FRAC_PI_4);
// Column 0: (cos45°, sin45°, 0) → X axis rotated 45° upward
// Column 1: (-sin45°, cos45°, 0) → Y axis rotated 45° as well
// Result: the whole coordinate system spins!
```

---

## 🔧 The Three Fundamental Transformations

### 1️⃣ Translation — Moving Things Around

```rust
/// 🏃 TRANSLATION:
/// [1  0  tx]   [x]   [x + tx]
/// [0  1  ty] × [y] = [y + ty]
/// [0  0  1 ]   [1]   [1     ]

// Translation is just "add a vector" disguised as a matrix.
// The matrix form exists so we can COMBINE it with rotation/scale.
let t = Mat3::from_translation(Vec2::new(100.0, 50.0));

let point = Vec2::new(10.0, 20.0);
let moved = t.transform_point2(point);  // = (110, 70)
```

### 2️⃣ Rotation — Spinning Things Around

```rust
/// 🌀 ROTATION:
/// [cosθ  -sinθ  0]   [x]   [x·cosθ - y·sinθ]
/// [sinθ   cosθ  0] × [y] = [x·sinθ + y·cosθ]
/// [0      0     1]   [1]   [1              ]

let r = Mat3::from_angle(std::f32::consts::FRAC_PI_4);
let point = Vec2::new(10.0, 0.0);
let rot = r.transform_point2(point);  // ≈ (7.07, 7.07)
```

**Why this formula?** The rotation matrix comes from the trigonometric identities for the sum of angles. When you rotate a point (x, y) by angle θ, the new coordinates are:

```
x' = x·cos(θ) - y·sin(θ)   ← Both axes contribute!
y' = x·sin(θ) + y·cos(θ)   ← This is the "cross-talk" of rotation
```

The columns are literally `(cosθ, sinθ)` and `(-sinθ, cosθ)` — the rotated X and Y axes.

### 3️⃣ Scaling — Stretching Things

```rust
/// 📏 SCALING:
/// [sx  0   0]   [x]   [x·sx]
/// [0   sy  0] × [y] = [y·sy]
/// [0   0   1]   [1]   [1   ]

let s = Mat3::from_scale(Vec2::new(2.0, 0.5));
let scaled = s.transform_point2(Vec2::new(10.0, 20.0));  // = (20, 10)
// Twice as wide, half as tall
```

---

## 🔗 Composition: The Real Power

**The magic: multiplying matrices COMBINES their transformations.**

```rust
let t = Mat3::from_translation(Vec2::new(100.0, 50.0));
let r = Mat3::from_angle(std::f32::consts::FRAC_PI_4);
let s = Mat3::from_scale(Vec2::new(2.0, 2.0));

// COMBINE:
let transform = t * r * s;
// Read right-to-left: Scale → Rotate → Translate

let result = transform.transform_point2(Vec2::new(10.0, 0.0));
```

**What happens step by step:**
```
Input: (10, 0)
┌─ S: (10, 0) × 2 = (20, 0)           ← Scale
├─ R: (20, 0) rotated 45° ≈ (14.14, 14.14)  ← Rotate
└─ T: (14.14, 14.14) + (100, 50) = (114.14, 64.14)  ← Translate
Result: (114.14, 64.14) ✅
```

### The Critical Rule: ORDER MATTERS (A × B ≠ B × A)

```rust
// ✅ CORRECT: T × R × S — scale at origin, then rotate, then move
let correct = t * r * s;

// ❌ WRONG: S × R × T — translate FIRST, then rotate (swings in arc!)
let wrong = s * r * t;
```

```
T × R × S (Correct):
  ┌─────┐   ┌─────┐    .─.
  │     │   │     │   (   )─.
  │scale│→ │rotate│→ │translate│→ ●
  └─────┘   └─────┘   '─'   └────┘
  Safe!           Clean!       Just moves!

S × R × T (Wrong — translation first):
  ┌─────┐    .─.         ╱╲
  │     │   ╱   ╲       ╱  ╲
  │tran.│→ ╱rotate╲→  ╱scale╲ ← SHEAR!
  └─────┘   ╲     ╱   ╱      ╲
             '─'    ╱
  Rotates around origin, NOT object center!
```

**ALWAYS use T × R × S. Never S × R × T. This is the cardinal rule of game transforms.**

---

## 🔬 The Homogeneous Coordinate Trick

Why 3×3 for 2D? Why not 2×2?

```rust
/// 2×2 matrices handle rotation and scale:
let rotate_2d = Mat2::from_angle(angle);

/// BUT they CANNOT represent translation!
let rotated = rotate_2d * point;                    // Can rotate
let final_pos = rotated + Vec2::new(100.0, 50.0);   // ADD translation manually

/// 3×3 matrices include translation IN the multiplication:
let final_pos = mat3.transform_point2(point);  // Everything in one step!

/// WHY THIS WORKS:
/// The extra "1" at the end of the vector (homogeneous coordinate)
/// allows the translation terms (tx, ty) to contribute:
///
/// [a  b  tx]   [x]     [a·x + b·y + tx·1]
/// [c  d  ty] × [y]  =  [c·x + d·y + ty·1]
/// [0  0  1 ]   [1]     [0·x + 0·y + 1·1 ] = [1]
///
/// Without that "1," translation would always multiply by zero!
```

---

## 🔄 The Inverse: Undoing a Transformation

```rust
/// The inverse "undoes" whatever the matrix does:
let transform = Mat3::from_translation(Vec2::new(100.0, 50.0))
    * Mat3::from_angle(std::f32::consts::FRAC_PI_4);

let point = Vec2::new(200.0, 100.0);
let transformed = transform.transform_point2(point);

// Go back:
let inverse = transform.inverse();
let original = inverse.transform_point2(transformed);
// original ≈ (200, 100) ✅ (minus floating point error)
```

**Inverse is used for:**
1. Converting WORLD to LOCAL coordinates
2. Undoing camera transform for mouse picking
3. Solving physics constraints

---

## 🧱 Practical: Transform Hierarchy

```rust
/// 🚀 Spaceship with child parts
// Body's world transform:
let body = Mat3::from_translation(ship_pos)
    * Mat3::from_angle(ship_angle);

// Left wing LOCAL to body:
let wing_local = Mat3::from_translation(Vec2::new(-30.0, 0.0));

// Left wing WORLD = body × local:
let wing_world = body * wing_local;
// When ship rotates, wing rotates with it AUTOMATICALLY!
```

---

## 🎯 Chapter Summary

```
MATRICES TRANSFORM VECTORS:

  ┌──────────────────────────────────────┐
  │  T × R × S  (ALWAYS! Read R-to-L)   │
  │   │  │  │                            │
  │   │  │  └── S: Scale at origin       │
  │   │  └───── R: Rotate around origin  │
  │   └──────── T: Translate to position │
  └──────────────────────────────────────┘

  Columns = Where the basis axes go
  Multiplication = Composition
  Inverse = Undo the transformation
  transform_point2 = For positions (includes translation)
  transform_vector2 = For directions/velocities (NO translation)
```

> **A matrix IS a spatial relationship. When you write `m.transform_point2(p)`, you're saying "apply this relationship to this point." The matrix doesn't just encode numbers — it encodes geometry.** 🔢

---

**[← Previous: Vector Mathematics](ch03-vectors.md)** | **[Next: Quaternions →](ch05-quaternions.md)**
