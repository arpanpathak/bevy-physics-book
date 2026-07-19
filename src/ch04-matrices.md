# 🔢 Matrices & Transformations

> **"A matrix is a function that takes a vector and returns a vector  -  but it's a special kind of function: LINEAR. Scaling, rotation, reflection, shear  -  all of these are just matrix multiplications. A single 3×3 grid of numbers represents ANY spatial relationship in 2D."** 🔄

---

## 🎯 Why Matrices Exist: The Composition Problem

Imagine you have a game object that needs to be:
1. **Scaled** to make it twice as big
2. **Rotated** to face a certain direction
3. **Translated** to a specific position in the world

Without matrices, you'd do this:

❌ WITHOUT MATRICES: Tedious, error-prone, non-composable.

```rust
fn transform_point_without_matrices(
    point: Vec2,
    scale_factor: Vec2,
    rotation_angle: f32,
    translation: Vec2,
) -> Vec2 {
    // Step 1: Scale
    let scaled_point = Vec2::new(point.x * scale_factor.x, point.y * scale_factor.y);
    
    // Step 2: Rotate (using the 2D rotation formula)
    let cos_angle = rotation_angle.cos();
    let sin_angle = rotation_angle.sin();
    let rotated_point = Vec2::new(
        scaled_point.x * cos_angle - scaled_point.y * sin_angle,
        scaled_point.x * sin_angle + scaled_point.y * cos_angle,
    );
    
    // Step 3: Translate
    let final_point = rotated_point + translation;
    
    final_point
}
```

Every time you want to apply these transformations to a point, you must manually sequence them. Want to apply them to a DIFFERENT point? You repeat the entire sequence. Want to COMBINE them with another transformation? You rewrite everything.

**Matrices solve this by ENCODING the entire transformation into a single object:**

✅ WITH MATRICES: Compose once, apply anywhere.

```rust
use bevy::prelude::*;

// Build the transformation matrix ONCE.
let scale_matrix = Mat3::from_scale(Vec2::new(2.0, 2.0));
let rotation_matrix = Mat3::from_angle(std::f32::consts::FRAC_PI_4);
let translation_matrix = Mat3::from_translation(Vec2::new(100.0, 50.0));

// Compose into a SINGLE matrix: T × R × S
let transformation = translation_matrix * rotation_matrix * scale_matrix;

// Now apply to ANY number of points:
let point_a = transformation.transform_point2(Vec2::new(10.0, 0.0));
let point_b = transformation.transform_point2(Vec2::new(-10.0, 5.0));
let point_c = transformation.transform_point2(Vec2::new(0.0, -20.0));
// All three points share the SAME transformation, computed ONCE.
```

**This is the core reason matrices exist in game physics: they make composition trivial.** You build the transformation once, combine it with a single multiplication, and apply it everywhere.

---

## 🧠 The Deep Insight: Columns Are Basis Vectors

A 3×3 matrix in 2D has three columns. Each column is a VECTOR that tells you where one of the original axes goes:

```
+------------------------------------------------------------+
|                 WHAT THE COLUMNS MEAN                      |
+------------------------------------------------------------+
|                                                            |
|  Column 0 (first column)  = Where the X-axis goes          |
|  Column 1 (second column) = Where the Y-axis goes          |
|  Column 2 (third column)  = Where the ORIGIN goes          |
|                                                            |
|  When you multiply: mat × (x, y, 1)                        |
|                                                            |
|  You're computing:  x × column_0 + y × column_1 + 1 × column_2
|                                                            |
|  This is a LINEAR COMBINATION of the columns!              |
+------------------------------------------------------------+
```

Let's verify this with concrete examples:

The identity matrix: columns are the STANDARD basis vectors.
 Scale matrix: columns are just the axes, but LONGER.
 Rotation matrix: columns are the axes, but ROTATED.

```rust
let identity_matrix = Mat3::IDENTITY;
// Column 0: (1, 0, 0) -> X-axis stays at (1, 0)  -  unchanged.
// Column 1: (0, 1, 0) -> Y-axis stays at (0, 1)  -  unchanged.
// Column 2: (0, 0, 1) -> Origin stays at (0, 0)  -  unchanged.
// Result: point = 1×X + 0×Y + 0×origin = (x, y)  -  THE SAME POINT.

let scale_matrix = Mat3::from_scale(Vec2::new(2.0, 3.0));
// Column 0: (2, 0, 0) -> X-axis now points to (2, 0)  -  twice as long.
// Column 1: (0, 3, 0) -> Y-axis now points to (0, 3)  -  three times as long.
// Result: point = x×(2,0) + y×(0,3) = (2x, 3y)  -  stretched!

let rotation_matrix = Mat3::from_angle(std::f32::consts::FRAC_PI_4);
// Column 0: (cos45°, sin45°, 0) ≈ (0.707, 0.707, 0)
// Column 1: (-sin45°, cos45°, 0) ≈ (-0.707, 0.707, 0)
// Result: point = x×(0.707,0.707) + y×(-0.707,0.707)
//        = (0.707x - 0.707y, 0.707x + 0.707y)  -  rotated 45°!
//
// 🔑 KEY INSIGHT: The columns of a rotation matrix are ORTHONORMAL.
// They're unit vectors perpendicular to each other. This is what
// makes rotation "rigid"  -  it preserves distances and angles.
```

---

## 🔧 The Three Fundamental Transformations (In Depth)

### 1️⃣ Translation  -  Moving Without Rotating

TRANSLATION moves a point by adding a constant offset.

 Matrix form:
   [1  0  tx]   [x]   [x + tx]
   [0  1  ty] × [y] = [y + ty]
   [0  0  1 ]   [1]   [1     ]

 The identity submatrix (top-left 2×2) means rotation and scale
 are both identity  -  we're ONLY moving.

 Geometric meaning:
   Column 0: (1, 0, 0)  -  X-axis unchanged
   Column 1: (0, 1, 0)  -  Y-axis unchanged
   Column 2: (tx, ty, 1)  -  Origin moves to (tx, ty)!

```rust
pub fn create_translation_matrix(offset: Vec2) -> Mat3 {
    Mat3::from_translation(offset)
}

// Usage:
let move_right_and_up = create_translation_matrix(Vec2::new(100.0, 50.0));
let original_point = Vec2::new(10.0, 20.0);
let moved_point = move_right_and_up.transform_point2(original_point);
// moved_point = (10 + 100, 20 + 50) = (110, 70)
```

### 2️⃣ Rotation  -  Spinning Around the Origin

ROTATION spins a point around the origin (0,0) by angle θ.

 Matrix form:
   [cosθ  -sinθ  0]   [x]   [x·cosθ - y·sinθ]
   [sinθ   cosθ  0] × [y] = [x·sinθ + y·cosθ]
   [0      0     1]   [1]   [1              ]

 WHY THIS FORMULA?

 Consider a point at angle φ from the x-axis, at distance r:
   x = r·cos(φ)
   y = r·sin(φ)

 After rotating by θ, the point is at angle (φ + θ):
   x' = r·cos(φ + θ) = r·cos(φ)·cos(θ) - r·sin(φ)·sin(θ)
   y' = r·sin(φ + θ) = r·sin(φ)·cos(θ) + r·cos(φ)·sin(θ)

 Substituting x = r·cos(φ) and y = r·sin(φ):
   x' = x·cos(θ) - y·sin(θ)   <- THE ROTATION FORMULA
   y' = x·sin(θ) + y·cos(θ)

 The columns are literally:
   Column 0: (cosθ, sinθ)  -  where the X axis points after rotation
   Column 1: (-sinθ, cosθ)  -  where the Y axis points after rotation

 Note that these two columns are always PERPENDICULAR (dot product = 0)
 and both have length 1. This is what makes rotation "rigid."

```rust
pub fn create_rotation_matrix(angle_radians: f32) -> Mat3 {
    Mat3::from_angle(angle_radians)
}

// Visual verification:
let rotate_90 = create_rotation_matrix(std::f32::consts::FRAC_PI_2); // 90°
let point_on_x_axis = Vec2::new(10.0, 0.0);
let after_rotation = rotate_90.transform_point2(point_on_x_axis);
// after_rotation ≈ (0, 10)  -  the point moved from the X axis to the Y axis ✅

// What about a diagonal point?
let diagonal = Vec2::new(1.0, 1.0);
let rotated_diagonal = rotate_90.transform_point2(diagonal);
// = (1·0 - 1·1, 1·1 + 1·0) = (-1, 1)
// The point (1,1) rotated 90° to (-1,1)  -  try drawing it!
```

### 3️⃣ Scaling  -  Stretching Along Axes

SCALING stretches or shrinks a point along the x and y axes.

 Matrix form:
   [sx  0   0]   [x]   [x·sx]
   [0   sy  0] × [y] = [y·sy]
   [0   0   1]   [1]   [1   ]

 Special cases:
   sx = sy = 2.0  -> Uniform scale (everything gets 2× bigger)
   sx = 2, sy = 1 -> Stretch horizontally only
   sx = -1, sy = 1 -> Mirror/reflect across the Y axis
   sx = sy = 0.5  -> Shrink to half size

```rust
pub fn create_scale_matrix(factors: Vec2) -> Mat3 {
    Mat3::from_scale(factors)
}

let double_size = create_scale_matrix(Vec2::splat(2.0));
let stretch_horizontal = create_scale_matrix(Vec2::new(2.0, 1.0));
let mirror_x = create_scale_matrix(Vec2::new(-1.0, 1.0));
```

---

## 🔗 Composition: The Reason Matrices Exist

The real power of matrices isn't in individual transformations  -  it's in **combining them**.

### How Matrix Multiplication Works

When you multiply two matrices, each element of the result is a **dot product** of a row from the first matrix and a column from the second:

Matrix multiplication in detail.

 [a b tx]   [e f tx']   [a·e + b·g   a·f + b·h   a·tx' + b·ty' + tx]
 [c d ty] × [g h ty'] = [c·e + d·g   c·f + d·h   c·tx' + d·ty' + ty]
 [0 0 1 ]   [0 0 1  ]   [0            0            1               ]

 The result is a NEW matrix that encodes BOTH transformations.
 Applying this combined matrix to a point is EQUIVALENT to applying
 the first matrix, then the second.

```rust
pub fn compose_transforms(
    translate: Mat3,
    rotate: Mat3,
    scale: Mat3,
) -> Mat3 {
    // Read right-to-left: Scale -> Rotate -> Translate
    translate * rotate * scale
}
```

### Step-by-Step Trace: What Happens to a Point

Let's trace what happens to a single point through a composed transform:

Given these transforms:
 Compose them:
 Apply to a point:
 Here's what EACH step does to the point:

```rust
let scale = Mat3::from_scale(Vec2::new(2.0, 2.0));
let rotate = Mat3::from_angle(std::f32::consts::FRAC_PI_4); // 45°
let translate = Mat3::from_translation(Vec2::new(100.0, 50.0));

let transform = translate * rotate * scale;

let input_point = Vec2::new(10.0, 0.0);
let result = transform.transform_point2(input_point);


// STEP 1: SCALE (the rightmost matrix is applied FIRST)
// scale × point = (2×10, 2×0) = (20, 0)
// The point doubles in size along the x-axis.

// STEP 2: ROTATE (the middle matrix)
// rotate × (20, 0) = (20·cos45° - 0·sin45°, 20·sin45° + 0·cos45°)
//                 = (20·0.707, 20·0.707)
//                 ≈ (14.14, 14.14)
// The scaled point rotates 45° counterclockwise.

// STEP 3: TRANSLATE (the leftmost matrix is applied LAST)
// translate × (14.14, 14.14) = (14.14 + 100, 14.14 + 50)
//                             = (114.14, 64.14)
// The rotated point moves to its final position.

// Final result: (114.14, 64.14) ✅
```

### THE CARDINAL RULE: T × R × S  -  Always

This order (Scale -> Rotate -> Translate) is NON-NEGOTIABLE for game physics:

Why T × R × S and NOT S × R × T?

 T × R × S (Correct):
   1. Scale at the origin (safe  -  no offset to distort)
   2. Rotate at the origin (clean rotation)
   3. Translate to final position (just moves the result)

 S × R × T (Wrong):
   1. Translate FIRST (moves away from origin)
   2. Rotate (swings the translated point in an arc around origin!)
   3. Scale (also scales the offset from origin  -  shearing!)

 The result: the object moves in a giant arc instead of
 cleanly rotating around its center. It looks BROKEN.
 ✅ Correct transform builder:
 ❌ Wrong transform builder (don't use this):

```rust
pub fn build_correct_transform(
    position: Vec2,
    rotation_radians: f32,
    scale_factor: Vec2,
) -> Mat3 {
    Mat3::from_translation(position)
        * Mat3::from_angle(rotation_radians)
        * Mat3::from_scale(scale_factor)
}

pub fn build_wrong_transform(
    position: Vec2,
    rotation_radians: f32,
    scale_factor: Vec2,
) -> Mat3 {
    Mat3::from_scale(scale_factor)
        * Mat3::from_angle(rotation_radians)
        * Mat3::from_translation(position) // <- Translation first = WRONG!
}
```

```
T × R × S (Correct):
  +------+     +------+     .---.
  |      |     |      |    (     )--.
  |scale |--> |rotate|--> |translate|--> *
  +------+     +------+     '---'  +----+
  Safe at      Clean           Just
  origin!      rotation!       moves it!

S × R × T (Wrong):
  +------+     .---.           \\
  |      |    \     \         \  \
  |tran. |-->\rotate \-->   \scale\ <- SHEAR!
  +------+    \     \       \      \
               '---'        \
  Rotates around origin, NOT around object center!
```

---

## 🔬 Homogeneous Coordinates: The Magic of the Extra Row

If you've been paying attention, you noticed our matrices are 3×3 even though we're working in 2D. This isn't wasteful  -  it's **necessary**.

### The Problem with 2×2 Matrices

A 2×2 matrix CAN represent rotation and scale:
 BUT a 2×2 matrix CANNOT represent translation!
 Try it: there's no place to put (tx, ty) in a 2×2 grid.

 You'd need to do translation separately:

```rust
let rotation_2d = Mat2::from_angle(std::f32::consts::FRAC_PI_4);
let scale_2d = Mat2::from_cols(
    Vec2::new(2.0, 0.0),
    Vec2::new(0.0, 2.0),
);

let rotated = rotation_2d * point;
let translated = rotated + Vec2::new(100.0, 50.0); // Manual addition!
// This breaks composition  -  you can't combine rotation and translation
// into a single operation.
```

### How 3×3 Fixes This: Homogeneous Coordinates

The "trick" is adding a third coordinate that's always 1:

   [a  b  tx]   [x]     [a·x + b·y + tx·1]
   [c  d  ty] × [y]  =  [c·x + d·y + ty·1]
   [0  0  1 ]   [1]     [0·x + 0·y + 1·1 ]

 The extra "1" in the input vector allows the translation
 terms (tx, ty) to CONTRIBUTE to the result.

 Without it, tx and ty would be multiplied by 0 and have no effect.

 Bevy handles this automatically:
 - transform_point2(p)  -> treats p as (p.x, p.y, 1)  -> INCLUDES translation
 - transform_vector2(v) -> treats v as (v.x, v.y, 0)  -> IGNORES translation
 💡 This is why velocity doesn't "move" when the object translates!
 If you have a velocity of (50, 0) and the object moves to a new
 position, the velocity DIRECTION stays the same  -  only rotation
 and scale affect it.

```rust
pub fn demonstrate_point_vs_vector(transform: Mat3) {
    let point = Vec2::new(10.0, 20.0);
    
    // For a POSITION: translation applies (the third component is 1)
    let transformed_position = transform.transform_point2(point);
    // = rotation_and_scale_of_point + translation
    
    // For a DIRECTION/VELOCITY: translation does NOT apply (third component is 0)
    let transformed_direction = transform.transform_vector2(point);
    // = rotation_and_scale_of_point ONLY
    
}
```

---

## 🔄 The Inverse: Undoing a Transformation

Every transformation matrix has an **inverse**  -  a matrix that undoes it:

If transform moves a point from A to B, then
 inverse moves it from B back to A.
 USES FOR THE INVERSE:

 1. World -> Local coordinate conversion.
    Given a point in world space, transform it to an object's local space:
    local_point = object_transform.inverse().transform_point2(world_point)

 2. Camera -> World conversion (mouse picking).
    world_position = camera_transform.inverse().transform_point2(screen_position)

 3. Physics constraint solving in local space (avoids coordinate confusion).

```rust
pub fn demonstrate_inverse() {
    let transform = Mat3::from_translation(Vec2::new(100.0, 50.0))
        * Mat3::from_angle(std::f32::consts::FRAC_PI_4)
        * Mat3::from_scale(Vec2::new(2.0, 2.0));
    
    let original_point = Vec2::new(10.0, 20.0);
    let transformed_point = transform.transform_point2(original_point);
    
    // Compute the inverse:
    let inverse_transform = transform.inverse();
    let recovered_point = inverse_transform.transform_point2(transformed_point);
    
    // recovered_point ≈ (10, 20)  -  identical within floating-point precision
    println!(
        "Original: ({:.1}, {:.1}), Recovered: ({:.1}, {:.1})",
        original_point.x, original_point.y,
        recovered_point.x, recovered_point.y
    );
}
```

---

## 🧱 Practical: Complete Physics Transform Component

A complete physics transform that encapsulates position, rotation, and scale.

 This is a standalone component separate from Bevy's Transform (which
 is tied to the rendering hierarchy). Physics operates on this directly,
 and we sync to Bevy's Transform for rendering.
 World-space position (where the object is).
 Rotation in radians (which way the object faces).
 0 = facing right, π/2 = facing up, π = facing left.
 Scale factor (how big the object is).
 (1, 1) = normal size, (2, 2) = double size.
 Creates a new transform at the given position.
 Builds the transformation matrix for this transform.

 The matrix encodes: translate × rotate × scale
 This single matrix can transform ANY number of points
 from local space to world space.
 Transforms a point from LOCAL space to WORLD space.

 Local space = relative to the object's center.
 World space = absolute position in the game world.

 Example: if the object is at (100, 50) and we have a gun
 mounted at local offset (20, 0), this tells us where the
 gun tip is in the world.
 Transforms a direction from LOCAL space to WORLD space.

 Unlike local_to_world, this does NOT apply translation.
 Use this for velocities, normals, and other direction vectors.

 Example: if the object is rotated 45°, a local direction
 of (1, 0) becomes world direction (cos45°, sin45°).
 Gets the "forward" direction in world space.
 Gets the "up" direction in world space.

```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct PhysicsTransform {
    pub translation: Vec2,
    
    pub rotation: f32,
    
    pub scale: Vec2,
}

impl PhysicsTransform {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            translation: Vec2::new(x, y),
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
    
    pub fn to_matrix(&self) -> Mat3 {
        Mat3::from_translation(self.translation)
            * Mat3::from_angle(self.rotation)
            * Mat3::from_scale(self.scale)
    }
    
    pub fn local_to_world(&self, local_point: Vec2) -> Vec2 {
        self.to_matrix().transform_point2(local_point)
    }
    
    pub fn local_direction_to_world(&self, local_direction: Vec2) -> Vec2 {
        self.to_matrix().transform_vector2(local_direction)
    }
    
    pub fn forward(&self) -> Vec2 {
        Vec2::new(self.rotation.cos(), self.rotation.sin())
    }
    
    pub fn up(&self) -> Vec2 {
        Vec2::new(-self.rotation.sin(), self.rotation.cos())
    }
}
```

---

## 🎯 Complete Example: Spaceship with Child Parts

Demonstrates transform hierarchies using matrix composition.

 Scene: A spaceship body with two wings and an engine.
 Each part is positioned RELATIVE to the body.
 When the body moves or rotates, the parts follow automatically.
 🖥️ System: Sync PhysicsTransform -> Bevy's Transform for rendering.

 This bridges our physics engine (which uses PhysicsTransform)
 with Bevy's renderer (which uses Transform).

```rust
pub fn spaceship_transform_example() {
    // --- Spaceship body transform ---
    let ship_world_position = Vec2::new(400.0, 300.0);
    let ship_rotation = std::f32::consts::FRAC_PI_6; // 30° tilt
    let ship_scale = Vec2::splat(1.0);
    
    let body_transform = Mat3::from_translation(ship_world_position)
        * Mat3::from_angle(ship_rotation)
        * Mat3::from_scale(ship_scale);
    
    // --- Child part local transforms ---
    // These are defined relative to the BODY's center.
    // They get composed with the body's global transform.
    let left_wing_local = Mat3::from_translation(Vec2::new(-30.0, 10.0));
    let right_wing_local = Mat3::from_translation(Vec2::new(30.0, 10.0));
    let engine_local = Mat3::from_translation(Vec2::new(0.0, -40.0));
    
    // --- Child part WORLD transforms ---
    // Compose: body × local = world position of the part
    let left_wing_world = body_transform * left_wing_local;
    let right_wing_world = body_transform * right_wing_local;
    let engine_world = body_transform * engine_local;
    
    // 💡 When the ship rotates by 45°, ALL child parts rotate with it
    // AUTOMATICALLY. The local offsets are transformed by the parent's
    // rotation matrix. This is the foundation of:
    //   - Skeletal animation (bones transform their children)
    //   - Robot arm kinematics (joint angles propagate)
    //   - Solar systems (planets orbit rotating suns)
    //   - First-person cameras (head moves with body)
}

pub fn sync_physics_to_render_system(
    mut query: Query<(&PhysicsTransform, &mut Transform)>,
) {
    for (physics_transform, mut render_transform) in query.iter_mut() {
        // Copy position (adding z=0 for 2D).
        render_transform.translation = Vec3::new(
            physics_transform.translation.x,
            physics_transform.translation.y,
            0.0,
        );
        
        // Copy rotation (use Z-axis rotation for 2D sprites).
        render_transform.rotation = Quat::from_rotation_z(physics_transform.rotation);
        
        // Copy scale (adding z=1 for 2D).
        render_transform.scale = Vec3::new(
            physics_transform.scale.x,
            physics_transform.scale.y,
            1.0,
        );
    }
}
```

---

## 🎯 Chapter Summary

```
MATRICES ARE COMPOSABLE TRANSFORMATIONS:

  +-----------------------------------------------------+
  |  A matrix is NOT a grid of numbers.                 |
  |  A matrix IS a transformation encoded as numbers.   |
  |                                                     |
  |  Columns = Where the basis vectors go               |
  |  Multiplication = Composition (combine transforms)  |
  |  T × R × S = The sacred order (Scale, Rotate, Move) |
  |  Inverse = Undo any transformation                  |
  |                                                     |
  |  transform_point2(p)  -> Position (includes translation)
  |  transform_vector2(v) -> Direction (NO translation)  |
  +-----------------------------------------------------+

  ONE matrix = ONE spatial relationship.
  Compose with multiplication. Apply with transform_point2.
  This is how every sprite, camera, and collision shape
  in your game gets positioned.
```

> **If vectors are nouns (the things), matrices are verbs (what happens to things). A single 3×3 matrix encodes translation, rotation, and scale. Multiplying matrices composes them. Applying a matrix to a vector moves it through space. This is the entire spatial mathematics of game engines. Master matrices, and you master how objects LIVE in your game world.** 🔢

> 💡 **Full source code for this chapter:** [code-examples/ch04-matrices/](https://github.com/arpanpathak/bevy-physics-book/tree/main/code-examples/ch04-matrices)
> 
> The runnable project includes Cargo.toml, main.rs, and complete module files.

---

**[<- Previous: Vector Mathematics](ch03-vectors.md)** | **[Next: Quaternions ->](ch05-quaternions.md)**
