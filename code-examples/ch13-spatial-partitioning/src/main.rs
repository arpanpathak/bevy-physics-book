//! # 📦 Chapter 13: Spatial Partitioning
//! Run with: `cargo run -p ch13`
use bevy::prelude::*;
use std::collections::HashMap;
fn main() {
    let cell_size = 100.0;
    let mut grid: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
    let objects = vec![
        (1u32, Vec2::new(150.0, 150.0)),
        (2, Vec2::new(160.0, 140.0)),
        (3, Vec2::new(500.0, 500.0)),
    ];
    for (id, pos) in &objects {
        let cell = ((pos.x / cell_size).floor() as i32, (pos.y / cell_size).floor() as i32);
        grid.entry(cell).or_default().push(*id);
    }
    let query_pos = Vec2::new(155.0, 145.0);
    let query_cell = ((query_pos.x / cell_size).floor() as i32, (query_pos.y / cell_size).floor() as i32);
    let mut nearby = Vec::new();
    for dx in -1..=1 { for dy in -1..=1 {
        if let Some(ents) = grid.get(&(query_cell.0 + dx, query_cell.1 + dy)) {
            nearby.extend(ents);
        }
    }}
    println!("🗺️ Objects near ({:.0}, {:.0}): IDs {:?}", query_pos.x, query_pos.y, nearby);
    let total_pairs: usize = objects.len() * objects.len();
    let grid_pairs = nearby.len();
    println!("📊 Brute force would check {total_pairs} pairs; grid checks {grid_pairs}");
    println!("✅ Spatial partitioning complete!");
}
