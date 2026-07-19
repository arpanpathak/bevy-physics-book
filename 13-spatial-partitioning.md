# 📦 Spatial Partitioning: Optimization at Scale

> **"With 10 objects, check everything against everything (100 checks). With 1000 objects, that's 500,000 checks — you need spatial partitioning!"** 🗺️

---

## 🤔 The Problem

Naive collision detection checks **every pair**:

```rust
// ❌ O(n²) — doesn't scale!
for a in all_objects {
    for b in all_objects {
        if a != b && a.overlaps(&b) {
            // Handle collision...
        }
    }
}

// n=10   → 45 checks    ✅ Fine
// n=100  → 4,950 checks  ⚠️ Getting slow
// n=1000 → 499,500 checks ❌ Too slow!
// n=10000 → 49,995,000 checks 💀 Game Over!
```

**Spatial partitioning** solves this by only checking nearby objects.

---

## 1️⃣ Spatial Grid (Simple & Effective)

```rust
/// 🗺️ Spatial Hash Grid — divides space into cells
///
/// Only check collisions between objects in the SAME cell (or adjacent cells)
/// Objects far apart are never compared.

#[derive(Resource)]
struct SpatialGrid {
    /// Cell size (should be > largest object size)
    cell_size: f32,
    /// Grid cells: (cell_x, cell_y) → list of entities
    cells: HashMap<(i32, i32), Vec<Entity>>,
    /// Entity positions (for boundary checks)
    entity_positions: HashMap<Entity, Vec2>,
}

impl SpatialGrid {
    fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
            entity_positions: HashMap::new(),
        }
    }
    
    /// 🧮 Convert a world position to grid coordinates
    fn cell_coord(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }
    
    /// 📥 Insert an entity into the grid
    fn insert(&mut self, entity: Entity, pos: Vec2) {
        let cell = self.cell_coord(pos);
        self.cells.entry(cell).or_default().push(entity);
        self.entity_positions.insert(entity, pos);
    }
    
    /// 🧹 Clear the grid (rebuild every frame)
    fn clear(&mut self) {
        self.cells.clear();
        self.entity_positions.clear();
    }
    
    /// 🔍 Find potential collision pairs for an entity
    /// Only checks the entity's cell and 8 neighbors (3×3 region)
    fn find_pairs(&self, entity: Entity, pos: Vec2) -> Vec<Entity> {
        let center = self.cell_coord(pos);
        let mut nearby = Vec::new();
        
        // Check the 3×3 neighborhood of cells
        for dx in -1..=1 {
            for dy in -1..=1 {
                let neighbor = (center.0 + dx, center.1 + dy);
                if let Some(entities) = self.cells.get(&neighbor) {
                    for &other in entities {
                        if other != entity {
                            nearby.push(other);
                        }
                    }
                }
            }
        }
        
        nearby
    }
}

/// 🔄 System: rebuild spatial grid every frame
fn rebuild_spatial_grid(
    mut grid: ResMut<SpatialGrid>,
    query: Query<(Entity, &Position)>,
) {
    grid.clear();
    
    // Insert all entities into the grid
    for (entity, pos) in query.iter() {
        grid.insert(entity, pos.0);
    }
}

/// 🔍 Broad phase using spatial grid
fn broad_phase_spatial_grid(
    grid: Res<SpatialGrid>,
    query: Query<(Entity, &Position)>,
) -> Vec<(Entity, Entity)> {
    let mut pairs = Vec::new();
    let mut checked = HashSet::new();
    
    for (entity, pos) in query.iter() {
        let nearby = grid.find_pairs(entity, pos.0);
        
        for &other in &nearby {
            let pair = if entity.index() < other.index() {
                (entity, other)
            } else {
                (other, entity)
            };
            
            if checked.insert(pair) {
                pairs.push(pair);
            }
        }
    }
    
    pairs
}

/// 💡 Spatial Grid Performance:
///
/// Without grid:  n=1000 → ~500,000 checks
/// With grid:     n=1000 → ~9,000 checks (cell visitation)
///
/// That's 55× FASTER! And the advantage grows with n.
```

```
Spatial Grid Visualization:

    ┌────┬────┬────┬────┬────┐
    │    │    │    │    │    │
    ├────┼────┼────┼────┼────┤
    │    │ ○  │    │ ○  │    │
    ├────┼────┼────┼────┼────┤
    │    │    │ ●  │    │    │
    ├────┼────┼────┼────┼────┤
    │    │ ○  │    │    │    │
    ├────┼────┼────┼────┼────┤
    │    │    │    │    │    │
    └────┴────┴────┴────┴────┘
    
    ● = player (checking)
    ○ = potential collisions (same or adjacent cells)
    Empty cells = nothing to check!
    
    Player in (2,2): checks cells (1,1)-(3,3) = 9 cells
    Instead of checking all 25 objects in the world!
```

---

## 2️⃣ Quadtree (Adaptive Partitioning)

```rust
/// 🌳 Quadtree — adaptive spatial partitioning
///
/// Unlike the grid (which has fixed cell size), a quadtree
/// subdivides only where there are many objects.
/// Perfect for unevenly distributed scenes!

struct Quadtree {
    /// Boundaries of this node
    bounds: Aabb,
    /// Max objects before splitting
    capacity: usize,
    /// Objects in this node
    objects: Vec<(Entity, Vec2)>,
    /// Child nodes (subdivided)
    children: Option<Box<[Quadtree; 4]>>,
}

impl Quadtree {
    fn new(bounds: Aabb, capacity: usize) -> Self {
        Self {
            bounds,
            capacity,
            objects: Vec::new(),
            children: None,
        }
    }
    
    /// 📥 Insert an entity
    fn insert(&mut self, entity: Entity, pos: Vec2) {
        // Is this position inside our bounds?
        if !self.bounds.contains(pos) {
            return;
        }
        
        // If we have children, insert into the appropriate child
        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                child.insert(entity, pos);
            }
            return;
        }
        
        // Not full yet — just add here
        if self.objects.len() < self.capacity {
            self.objects.push((entity, pos));
            return;
        }
        
        // Full and no children — subdivide!
        self.subdivide();
        
        // Now insert into children
        for child in self.children.as_mut().unwrap().iter_mut() {
            for &(e, p) in &self.objects {
                child.insert(e, p);
            }
        }
        self.objects.clear();
        
        // Insert new object
        for child in self.children.as_mut().unwrap().iter_mut() {
            child.insert(entity, pos);
        }
    }
    
    /// ✂️ Subdivide into 4 children
    fn subdivide(&mut self) {
        let center = (self.bounds.min + self.bounds.max) * 0.5;
        let (min, max) = (self.bounds.min, self.bounds.max);
        
        self.children = Some(Box::new([
            // NW quadrant
            Quadtree::new(Aabb::new(Vec2::new(min.x, center.y), center, self.capacity), self.capacity),
            // NE quadrant
            Quadtree::new(Aabb::new(center, Vec2::new(max.x, center.y)), self.capacity),
            // SW quadrant
            Quadtree::new(Aabb::new(Vec2::new(min.x, min.y), center), self.capacity),
            // SE quadrant
            Quadtree::new(Aabb::new(center, Vec2::new(max.x, min.y)), self.capacity),
        ]));
    }
    
    /// 🔍 Find all potential collisions for a position
    fn query(&self, pos: Vec2, results: &mut Vec<Entity>) {
        if !self.bounds.contains(pos) {
            return;
        }
        
        // Add objects at this node
        for &(entity, _) in &self.objects {
            results.push(entity);
        }
        
        // Recursively check children
        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.query(pos, results);
            }
        }
    }
}
```

---

## 3️⃣ Bounding Volume Hierarchy (BVH)

```rust
/// 🌳 Bounding Volume Hierarchy — tree of nested bounding boxes
///
/// Used by:
/// - Bevy's built-in rendering (frustum culling)
/// - Rapier physics engine
/// - Modern game engines (Unity, Unreal)

struct BvhNode {
    /// Bounding box of this node and all children
    bounds: Aabb,
    /// Leaf: entity data
    entity: Option<Entity>,
    /// Internal: child nodes
    left: Option<Box<BvhNode>>,
    right: Option<Box<BvhNode>>,
}

impl BvhNode {
    /// 🏗️ Build BVH from a list of entities (bottom-up)
    fn build(entities: &[(Entity, Vec2, f32)]) -> Self {
        if entities.len() == 1 {
            let (entity, pos, radius) = entities[0];
            return Self {
                bounds: Aabb {
                    min: *pos - Vec2::splat(*radius),
                    max: *pos + Vec2::splat(*radius),
                },
                entity: Some(entity),
                left: None,
                right: None,
            };
        }
        
        // Find the longest axis and sort
        let mut sorted = entities.to_vec();
        
        // Simple approach: split in half along longest axis
        let mid = sorted.len() / 2;
        let right_entities = sorted.split_off(mid);
        
        let left = BvhNode::build(&sorted);
        let right = BvhNode::build(&right_entities);
        
        // Merge bounds
        let bounds = left.bounds.merge(&right.bounds);
        
        Self {
            bounds,
            entity: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    
    /// 🔍 Ray intersection (very fast with BVH!)
    fn intersect_ray(&self, ray: &Ray) -> Option<(Entity, Vec2)> {
        // Quick reject: does the ray hit this node's bounds?
        if !ray.intersects_aabb(&self.bounds) {
            return None;
        }
        
        // Leaf: check actual entity
        if let Some(entity) = self.entity {
            // Check entity collision here
            return Some((entity, ray.origin + ray.direction * 0.0));
        }
        
        // Internal: check children (closest first)
        let left_hit = self.left.as_ref().and_then(|l| l.intersect_ray(ray));
        let right_hit = self.right.as_ref().and_then(|r| r.intersect_ray(ray));
        
        // Return closest hit
        match (left_hit, right_hit) {
            (Some(l), Some(r)) => {
                // Return whichever is closer (simplified)
                Some(l)  // Proper: compare distances
            }
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }
}
```

---

## 📊 Performance Comparison

| Method | Build Time | Query Time | Memory | Best For |
|--------|-----------|------------|--------|----------|
| `O(n²)` brute force | None | O(n²) | O(1) | < 50 objects |
| **Spatial Grid** ✅ | O(n) | O(n) | O(n) | Uniform distribution |
| **Quadtree** | O(n log n) | O(log n) | O(n) | Uneven distribution |
| **BVH** | O(n log n) | O(log n) | O(n) | Dynamic scenes |

```rust
/// 🎯 Recommendation by scene size:
fn recommend_partitioning(object_count: usize) -> &'static str {
    match object_count {
        0..=50 => "❌ None needed — brute force is fine!",
        51..=500 => "✅ Spatial Grid — simple and effective",
        501..=5000 => "✅ Quadtree — handles uneven distribution",
        5001.. => "✅ BVH — professional-grade, used in Rapier",
    }
}
```

---

## 🎯 Chapter Summary

```rust
/// 📝 Spatial partitioning cheat sheet:

// For < 50 objects: don't bother, O(n²) is fine
// For 50-500 objects: Spatial Grid (easiest!)
// For 500+ objects: Quadtree or BVH

/// 🗺️ Spatial Grid Rules:
// 1. Cell size should be > largest object size
// 2. Check 3×3 neighborhood (9 cells total)
// 3. Rebuild every frame (it's fast!)
// 4. Use HashMap<(i32, i32), Vec<Entity>>

/// 🌳 Quadtree Rules:
// 1. Capacity threshold: 4-16 objects per node
// 2. Subdivide only when full
// 3. Query is O(log n)
// 4. Great for open-world games

// 🎯 The goal: reduce O(n²) to O(n log n) or better!
```

> **Key Takeaway:** For 50+ physics objects, spatial partitioning isn't optional — it's essential. Start with a **spatial grid** (simplest), upgrade to a **quadtree** or **BVH** when needed. The 80/20 rule applies: a spatial grid gets you 80% of the benefit with 20% of the complexity. 🗺️

---

**[← Previous: Constraints & Joints](12-constraints.md)** | **[Next: Bevy ECS Architecture →](14-ecs-architecture.md)**
