/// # 🎮 Physics Setup Example
///
/// This is the MAIN ENTRY POINT for the physics demo.
/// It sets up a Bevy application with:
///   1. Default Bevy plugins (rendering, window, input, audio)
///   2. Our custom PhysicsPlugin (integration, forces, sync)
///   3. A startup system that spawns test objects
///
/// MODULE STRUCTURE:
///   src/
///     main.rs              - App setup and entity spawning
///     physics/
///       mod.rs             - PhysicsPlugin definition
///       components.rs      - Position, Velocity, Mass, etc.
///       systems.rs         - Integration, forces, rendering sync
///
/// Run with: `cargo run -p ch02`

use bevy::prelude::*;

// Import our physics module.
// The `mod` declaration tells Rust to look for `physics/mod.rs`.
mod physics;

/// The entry point for the application.
///
/// Bevy applications follow a BUILDER PATTERN:
///   1. Create an empty App with `App::new()`
///   2. Add plugins (bundles of systems + resources)
///   3. Add startup systems (run once at initialization)
///   4. Call `.run()` to enter the main loop
///
/// The main loop looks like:
///   while window.is_open() {
///       run_all_startup_systems();  // Only on first frame
///       run_all_update_systems();   // Every frame
///       render();                   // Draw everything
///   }
fn main() {
    App::new()
        // ─── ADD DEFAULT PLUGINS ───
        // DefaultPlugins includes: windowing, rendering, input, audio,
        // UI, asset system, and the core ECS scheduler.
        // Without this, nothing would be visible.
        .add_plugins(DefaultPlugins)

        // ─── ADD OUR PHYSICS PLUGIN ───
        // This registers all physics systems, components, and settings.
        // The plugin handles system ordering internally.
        // Everything physics-related is encapsulated in this ONE line.
        .add_plugins(physics::PhysicsPlugin)

        // ─── ADD STARTUP SYSTEMS ───
        // Startup systems run EXACTLY ONCE when the app starts.
        // Use them for: spawning entities, setting up initial state,
        // configuring resources that need computation.
        .add_systems(Startup, setup_scene)

        // ─── RUN THE APPLICATION ───
        // This enters Bevy's main loop and never returns
        // (until the window is closed).
        .run();
}

/// Spawns the initial scene: a camera and a test object.
///
/// This function runs ONCE at startup. It:
///   1. Creates a 2D camera so we can see things
///   2. Spawns a square with physics components and a visual sprite
///
/// The entity we spawn has BOTH physics components (Position, Velocity, Mass)
/// AND a visual component (SpriteBundle). The physics systems modify Position,
/// and the sync system copies Position → Transform for rendering.
fn setup_scene(mut commands: Commands) {
    // ─── SPAWN THE CAMERA ───
    // A 2D camera is required to see anything in a Bevy game.
    // Without it, the renderer doesn't know what to display.
    commands.spawn(Camera2dBundle::default());

    // ─── SPAWN A PHYSICS TEST OBJECT ───
    // We spawn an entity with BOTH physics and rendering components.
    // This uses Bevy's "bundle" pattern - we group the components
    // as a tuple and spawn them all at once.
    commands.spawn((
        // Physics components (defined in physics::components)
        physics::components::Position::new(0.0, 300.0),
        physics::components::Velocity::new(50.0, 100.0), // Moving up-right
        physics::components::Acceleration::default(),
        physics::components::Mass::default(),  // mass = 1.0
        physics::components::ForceAccumulator::default(),

        // Visual component (defined by Bevy)
        // This gives the entity a visible sprite on screen.
        SpriteBundle {
            sprite: Sprite::from_color(
                Color::srgb(1.0, 0.5, 0.2),  // Orange color
                Vec2::new(40.0, 40.0),        // 40×40 pixel square
            ),
            ..default()
        },
    ));

    // Print a confirmation message to the console.
    println!("🚀 Physics simulation initialized!");
    println!("📐 A test object has been spawned at (0, 300)");
    println!("🏃 It will move under the influence of gravity.");
}
