# 🧱 Collision Detection: Finding Overlaps

> **"Collision detection answers one question: 'Are these two things touching?' The answer is always some clever math."** 🎯

---

## 🎯 The Two Phases

Collision detection has two phases for performance:

```
Phase 1: BROAD PHASE 🗺️        Phase 2: NARROW PHASE 🔍
┌──────────────────────┐       ┌──────────────────────┐
│ "Which pairs MIGHT   │       │ "Are these two       │
│  be colliding?"      │  ──►  │  actually colliding?"│
│                      │       │                      │
│ Fast & approximate   │       │ Slow & precise       │
│ Uses bounding boxes  │       │ Uses actual shapes   │
│ O(n) or O(n log n)   │       │ O(pairs in broad)    │
└──────────────────────┘       └──────────────────────┘
```

---

## 📦 Axis-Aligned Bounding Box (AABB)

The simplest and fastest collision shape:

```rust
/// 📦 An Axis-Aligned Bounding Box
///
/// "Axis-Aligned" means the box CANNOT rotate — its edges are
/// always parallel to the x and y axes.
///
/// This makes collision checks INCREDIBLY fast:
/// Just compare min/max coordinates!
#[derive(Debug, Clone, Copy)]
struct Aabb {
    /// 📍 Minimum corner (bottom-left)
    min: Vec2,
    /// 📍 Maximum corner (top-right)
    max: Vec2,
}

impl Aabb {
    fn new(center: Vec2, half_width: f32, half_height: f32) -> Self {
        Self {
            min: Vec2::new(center.x - half_width, center.y - half_height),
            max: Vec2::new(center.x + half_width, center.y + half_height),
        }
    }
    
    /// 🧱 Check if two AABBs overlap
    /// This is THE simplest collision check in all of game physics!
    ///
    /// Think of it as: "Is there a gap on any axis?"
    /// If there's a gap on ANY axis, they're NOT colliding.
    /// If there's NO gap on ALL axes, they ARE colliding.
    fn overlaps(&self, other: &Aabb) -> bool {
        // Check X axis: is there a gap?
        // No gap on X means the projections overlap
        self.min.x <= other.max.x && self.max.x >= other.min.x
        // Check Y axis: is there a gap?
        && self.min.y <= other.max.y && self.max.y >= other.min.y
        // ✅ Both axes overlap → collision!
    }
    
    /// 📏 Get the overlap amount on each axis
    /// Useful for pushing objects apart
    fn overlap_amount(&self, other: &Aabb) -> Vec2 {
        let overlap_x = (self.max.x - other.min.x)
            .min(other.max.x - self.min.x);
        let overlap_y = (self.max.y - other.min.y)
            .min(other.max.y - self.min.y);
        Vec2::new(overlap_x, overlap_y)
    }
}
```

```
AABB Overlap Check:

  X Axis (separate):          X & Y Axes (overlap):
  
     ┌────┐                        ┌────┐
     │ A  │               Y        │ A  │
     └────┘               ▲       └────┘
           ┌────┐         │         ┌────┐
           │ B  │         │         │ B  │
           └────┘         └──►     └────┘
  
  X gap → no collision     X overlap ✅ + Y overlap ✅ = COLLISION!
```

---

## 🔵 Circle Collision

```rust
/// 🔵 Circle collider
#[derive(Debug, Clone, Copy)]
struct Circle {
    center: Vec2,
    radius: f32,
}

impl Circle {
    /// 🎯 Circle-vs-Circle collision
    ///
    /// Two circles collide if the distance between their centers
    /// is less than the sum of their radii.
    ///
    /// This is the SECOND fastest collision check!
    fn overlaps(&self, other: &Circle) -> bool {
        // Vector between centers
        let diff = self.center - other.center;
        
        // 📏 Distance squared (avoid sqrt!)
        let dist_sq = diff.length_squared();
        
        // Sum of radii (squared for comparison)
        let radius_sum = self.radius + other.radius;
        let radius_sum_sq = radius_sum * radius_sum;
        
        // 💥 Collision if centers are closer than combined radii
        dist_sq <= radius_sum_sq
    }
    
    /// 📐 Get collision normal (direction to push apart)
    fn collision_normal(&self, other: &Circle) -> Vec2 {
        let diff = other.center - self.center;
        let dist = diff.length();
        
        if dist < 0.0001 {
            // Circles are perfectly overlapping — pick a direction
            Vec2::X
        } else {
            diff / dist  // Unit vector from self to other
        }
    }
    
    /// 📏 Penetration depth
    fn penetration(&self, other: &Circle) -> f32 {
        let diff = self.center - other.center;
        let dist = diff.length();
        let radius_sum = self.radius + other.radius;
        
        (radius_sum - dist).max(0.0)  // How far they overlap
    }
}

/// 🧪 Tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn circles_overlap() {
        let a = Circle { center: Vec2::ZERO, radius: 5.0 };
        let b = Circle { center: Vec2::new(3.0, 0.0), radius: 5.0 };
        assert!(a.overlaps(&b));  // Centers 3 apart, radii sum to 10 → overlap!
    }
    
    #[test]
    fn circles_dont_overlap() {
        let a = Circle { center: Vec2::ZERO, radius: 5.0 };
        let b = Circle { center: Vec2::new(20.0, 0.0), radius: 5.0 };
        assert!(!a.overlaps(&b));  // Centers 20 apart, radii sum to 10 → no overlap!
    }
}
```

```
Circle-Circle Collision:

    Colliding:                     Not Colliding:
    
      ╱‾‾‾╲                         ╱‾‾‾╲          ╱‾‾‾╲
     ╱     ╲                       ╱     ╲        ╱     ╲
    │   A   │                      │  A   │      │   B  │
     ╲     ╱                       ╲     ╱        ╲     ╱
      ╲___╱                         ╲___╱          ╲___╱
      ╱‾‾‾╲                          ↑
     ╱     ╲                      dist > r₁ + r₂
    │   B   │                     
     ╲     ╱                      No collision! ❌
      ╲___╱
        ↑
    dist ≤ r₁ + r₂
    Collision! ✅
```

---

## 📐 Circle vs AABB

```rust
/// 🔵📦 Circle vs AABB collision
///
/// This is a common case: circular player vs rectangular walls
fn circle_vs_aabb(circle: &Circle, aabb: &Aabb) -> bool {
    // STEP 1: Find the closest point on the AABB to the circle center
    let closest_x = circle.center.x.clamp(aabb.min.x, aabb.max.x);
    let closest_y = circle.center.y.clamp(aabb.min.y, aabb.max.y);
    
    // STEP 2: Vector from closest point to circle center
    let diff = circle.center - Vec2::new(closest_x, closest_y);
    
    // STEP 3: Check if distance is less than circle radius
    diff.length_squared() <= circle.radius * circle.radius
}

/// 💡 Key insight: "Closest point on rectangle to circle"
/// We clamp the circle center to the rectangle bounds.
/// If the clamped point is within the circle's radius → collision!
```

```
Circle vs AABB:

    ┌──────────────────────┐
    │                      │
    │        ●──┐          │
    │      circle│←closest │
    │        │  │ point    │
    │        │  │          │
    │        ▼  │          │
    └────────┼──┘──────────┘
             │
             dist < radius?
             Yes → collision! ✅
```

---

## 🔺 Separating Axis Theorem (SAT)

The **SAT** is the most general 2D collision test. It works for ANY convex polygon:

```rust
/// 🔺 Separating Axis Theorem
///
/// THEOREM: Two convex shapes do NOT overlap if there exists
/// a line (axis) where their projections don't overlap.
///
/// In practice: Check every edge of both polygons as a potential
/// separating axis. If all axes overlap → collision!
struct Polygon {
    vertices: Vec<Vec2>,
}

impl Polygon {
    /// 📐 Get all edge normals (potential separating axes)
    fn edge_normals(&self) -> Vec<Vec2> {
        let mut normals = Vec::with_capacity(self.vertices.len());
        
        for i in 0..self.vertices.len() {
            let j = (i + 1) % self.vertices.len();
            
            // Edge vector
            let edge = self.vertices[j] - self.vertices[i];
            
            // 🔄 Perpendicular (normalized) — this is our axis!
            let normal = Vec2::new(-edge.y, edge.x).normalize();
            normals.push(normal);
        }
        
        normals
    }
    
    /// 📏 Project polygon onto an axis
    /// Returns (min, max) of the projection
    fn project_onto_axis(&self, axis: Vec2) -> (f32, f32) {
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        
        for vertex in &self.vertices {
            // ⭐ Dot product = projection length onto axis
            let proj = vertex.dot(axis);
            min = min.min(proj);
            max = max.max(proj);
        }
        
        (min, max)
    }
    
    /// 💥 SAT collision test
    fn sat_overlaps(&self, other: &Polygon) -> bool {
        // Check ALL axes from BOTH polygons
        let mut axes = self.edge_normals();
        axes.extend(other.edge_normals());
        
        for axis in axes {
            let (min_a, max_a) = self.project_onto_axis(axis);
            let (min_b, max_b) = other.project_onto_axis(axis);
            
            // ❌ Gap found on this axis → no collision
            if max_a < min_b || max_b < min_a {
                return false;
            }
            // ✅ Overlap on this axis → continue checking
        }
        
        // ✅ Overlap on ALL axes → COLLISION!
        true
    }
}
```

```
SAT Visualization:

    Two boxes, checking one axis:
    
          ┌────┐
          │ A  │
          └────┘
                ┌────┐
                │ B  │
                └────┘
    
    Projection onto axis:
    
    A:  ████████░░░░░░░░░
    B:  ░░░░░░░░████████
    
    ════ Gap! No collision on this axis → NO COLLISION! ✅
    
    If ALL axes have overlap → collision found! 💥
```

---

## 🎯 Raycasting

```rust
/// 🎯 Ray — an infinite line from origin in a direction
struct Ray {
    origin: Vec2,
    direction: Vec2,  // Should be normalized
}

impl Ray {
    /// 🎯 Ray vs Circle intersection
    /// Returns the closest hit point (if any)
    fn intersect_circle(&self, circle: &Circle) -> Option<Vec2> {
        // Vector from ray origin to circle center
        let oc = self.origin - circle.center;
        
        // 📐 Quadratic formula coefficients
        let a = self.direction.dot(self.direction);
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.dot(oc) - circle.radius * circle.radius;
        
        // Discriminant: b² - 4ac
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return None;  // No intersection
        }
        
        // Find closest intersection (smallest t > 0)
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        
        if t >= 0.0 {
            Some(self.origin + self.direction * t)
        } else {
            None  // Intersection is behind the ray
        }
    }
    
    /// 🎯 Ray vs AABB intersection (slab method)
    fn intersect_aabb(&self, aabb: &Aabb) -> Option<Vec2> {
        let inv_dir = Vec2::new(
            1.0 / self.direction.x,
            1.0 / self.direction.y,
        );
        
        // 🏗️ Slab intersections
        let t1 = (aabb.min.x - self.origin.x) * inv_dir.x;
        let t2 = (aabb.max.x - self.origin.x) * inv_dir.x;
        let t3 = (aabb.min.y - self.origin.y) * inv_dir.y;
        let t4 = (aabb.max.y - self.origin.y) * inv_dir.y;
        
        let tmin = t1.min(t2).max(t3.min(t4));
        let tmax = t1.max(t2).min(t3.max(t4));
        
        if tmax >= 0.0 && tmax >= tmin {
            let t = if tmin >= 0.0 { tmin } else { tmax };
            Some(self.origin + self.direction * t)
        } else {
            None
        }
    }
}

/// 💡 Raycasting uses:
/// - 🎯 Mouse picking ("what did I click on?")
/// - 👁️ Line of sight checks
/// - 🔫 Bullet trajectory
/// - 🕯️ Shadow casting
/// - 📡 AI perception
```

---

## 🏗️ Complete Collision System

```rust
/// 💥 Collision event — emitted when two entities collide
#[derive(Event)]
struct CollisionEvent {
    entity_a: Entity,
    entity_b: Entity,
    normal: Vec2,      // Push direction for A
    penetration: f32,  // How deep they overlap
}

/// 🧱 Broad-phase + narrow-phase collision detection
fn collision_detection_system(
    mut collision_events: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Position, &Collider)>,
) {
    // 👥 Collect all entities with colliders
    let entities: Vec<(Entity, Vec2, &Collider)> = query
        .iter()
        .map(|(e, pos, col)| (e, pos.0, col))
        .collect();
    
    // 🗺️ Broad phase: check all pairs (naive O(n²) for now)
    for i in 0..entities.len() {
        for j in (i + 1)..entities.len() {
            let (entity_a, pos_a, collider_a) = entities[i];
            let (entity_b, pos_b, collider_b) = entities[j];
            
            // 🔍 Narrow phase: check actual collision
            if let Some((normal, penetration)) = check_collision(
                pos_a, collider_a, pos_b, collider_b,
            ) {
                // 📢 Emit collision event
                collision_events.send(CollisionEvent {
                    entity_a,
                    entity_b,
                    normal,
                    penetration,
                });
            }
        }
    }
}

/// 🔍 Dispatches to the right collision check based on shape type
fn check_collision(
    pos_a: Vec2,
    collider_a: &Collider,
    pos_b: Vec2,
    collider_b: &Collider,
) -> Option<(Vec2, f32)> {
    match (collider_a, collider_b) {
        // 🔵🔵 Circle vs Circle
        (Collider::Circle { radius: r1 }, Collider::Circle { radius: r2 }) => {
            let diff = pos_b - pos_a;
            let dist_sq = diff.length_squared();
            let radius_sum = r1 + r2;
            
            if dist_sq <= radius_sum * radius_sum {
                let dist = dist_sq.sqrt();
                let normal = if dist > 0.0 {
                    diff / dist
                } else {
                    Vec2::X  // Perfect overlap fallback
                };
                let penetration = radius_sum - dist;
                Some((normal, penetration))
            } else {
                None
            }
        }
        
        // 🔵📦 Circle vs AABB
        (Collider::Circle { radius }, Collider::Aabb { half_width, half_height })
        | (Collider::Aabb { half_width, half_height }, Collider::Circle { radius }) => {
            // Closest point on AABB to circle center
            let closest_x = pos_a.x.clamp(pos_b.x - half_width, pos_b.x + half_width);
            let closest_y = pos_a.y.clamp(pos_b.y - half_height, pos_b.y + half_height);
            
            let diff = pos_a - Vec2::new(closest_x, closest_y);
            let dist_sq = diff.length_squared();
            
            if dist_sq <= radius * radius {
                let dist = dist_sq.sqrt();
                let normal = if dist > 0.0 { diff / dist } else { Vec2::X };
                let penetration = radius - dist;
                Some((normal, penetration))
            } else {
                None
            }
        }
        
        // 📦📦 AABB vs AABB
        (Collider::Aabb { half_width: w1, half_height: h1 },
         Collider::Aabb { half_width: w2, half_height: h2 }) => {
            let min_a = Vec2::new(pos_a.x - w1, pos_a.y - h1);
            let max_a = Vec2::new(pos_a.x + w1, pos_a.y + h1);
            let min_b = Vec2::new(pos_b.x - w2, pos_b.y - h2);
            let max_b = Vec2::new(pos_b.x + w2, pos_b.y + h2);
            
            // X and Y overlap
            let overlap_x = (max_a.x - min_b.x).min(max_b.x - min_a.x);
            let overlap_y = (max_a.y - min_b.y).min(max_b.y - min_a.y);
            
            if overlap_x > 0.0 && overlap_y > 0.0 {
                // Push apart along the axis of LEAST overlap
                // This is the Minimum Translation Vector (MTV)
                if overlap_x < overlap_y {
                    let sign = if pos_a.x < pos_b.x { -1.0 } else { 1.0 };
                    Some((Vec2::new(sign, 0.0), overlap_x))
                } else {
                    let sign = if pos_a.y < pos_b.y { -1.0 } else { 1.0 };
                    Some((Vec2::new(0.0, sign), overlap_y))
                }
            } else {
                None
            }
        }
    }
}
```

---

## 🎯 Chapter Summary

```
Collision Detection Flow:

    Entities with colliders
            │
            ▼
    🗺️ BROAD PHASE (quick rejection)
    "Could they be touching?"
    (AABB overlap check, spatial hash, etc.)
            │
        Maybe │ No
            │ 
            ▼
    🔍 NARROW PHASE (exact check)     ❌ Skip
    "Are they actually touching?"
    (Circle, AABB, SAT, etc.)
            │
        Yes │ No
            │ 
            ▼
    💥 COLLISION!                     ❌ Skip
    (Emit event, resolve)
    
    Shape combinations:
    • Circle vs Circle   → dist ≤ r₁ + r₂        ⚡ Fastest
    • AABB vs AABB       → axis overlaps          ⚡ Fast
    • Circle vs AABB     → clamp test             ⚡ Fast
    • Polygon vs Polygon → SAT (all axes)         🐢 Slower
```

> **Key Takeaway:** Collision detection is always a trade-off between speed and accuracy. Use simple shapes (circles, AABBs) for most things, SAT for complex polygons, and always pair it with broad-phase culling. The fastest collision check is the one you DON'T do! 🎯

---

**[← Previous: Integration Methods](ch09-integration.md)** | **[Next: Collision Response →](ch11-collision-response.md)**
