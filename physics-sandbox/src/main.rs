//! # 🎮 Physics Sandbox
//!
//! A complete, interactive physics simulation built with Bevy.
//!
//! ## Controls
//! - 🖱️ **Click** — Spawn a ball at cursor position
//! - ◀️ ▶️ **Arrow keys** — Adjust horizontal/vertical gravity
//! - **C** — Clear all objects
//! - **R** — Reset gravity
//! - **E** — Explosion (spawn 20 balls)
//! - **F3** — Toggle debug visualization
//!
//! Run with: `cargo run`

use bevy::prelude::*;

// ═══════════════════════════════════════════════════
// 📍 COMPONENTS
// ═══════════════════════════════════════════════════

#[derive(Component, Clone, Copy)]
struct Position(Vec2);

#[derive(Component, Clone, Copy, Default)]
struct Velocity(Vec2);

#[derive(Component, Clone, Copy, Default)]
struct Acceleration(Vec2);

#[derive(Component, Clone, Copy)]
struct Mass(f32);
impl Default for Mass { fn default() -> Self { Self(1.0) } }

#[derive(Component, Clone, Copy, Default)]
struct ForceAccumulator(Vec2);

#[derive(Component, Clone)]
enum Collider {
    Circle { radius: f32 },
}

// ═══════════════════════════════════════════════════
// ⚙️ RESOURCES
// ═══════════════════════════════════════════════════

#[derive(Resource)]
struct PhysicsSettings {
    gravity: Vec2,
    fixed_dt: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self { gravity: Vec2::new(0.0, -500.0), fixed_dt: 1.0 / 60.0 }
    }
}

#[derive(Resource, Default)]
struct DebugMode(bool);

#[derive(Resource, Default)]
struct SandboxState {
    spawn_radius: f32,
    spawn_mass: f32,
}

// ═══════════════════════════════════════════════════
// 🔄 SYSTEMS
// ═══════════════════════════════════════════════════

fn clear_forces(mut query: Query<&mut ForceAccumulator>) {
    for mut f in query.iter_mut() { f.0 = Vec2::ZERO; }
}

fn apply_gravity(
    mut query: Query<(&Mass, &mut ForceAccumulator)>,
    settings: Res<PhysicsSettings>,
) {
    for (mass, mut forces) in query.iter_mut() {
        forces.0 += settings.gravity * mass.0;
    }
}

fn integrate(
    mut query: Query<(&ForceAccumulator, &Mass, &mut Acceleration, &mut Velocity, &mut Position)>,
    settings: Res<PhysicsSettings>,
) {
    let dt = settings.fixed_dt;
    for (forces, mass, mut acc, mut vel, mut pos) in query.iter_mut() {
        acc.0 = if mass.0 > 0.0 { forces.0 / mass.0 } else { Vec2::ZERO };
        vel.0 += acc.0 * dt;
        pos.0 += vel.0 * dt;
    }
}

fn spawn_ball(commands: &mut Commands, pos: Vec2, radius: f32, mass: f32) {
    let hue = (mass / 5.0).clamp(0.0, 1.0) * 0.7;
    let color = Color::hsl(hue * 360.0, 0.8, 0.5);
    commands.spawn((
        Position(pos),
        Velocity::default(),
        Acceleration::default(),
        Mass(mass),
        ForceAccumulator::default(),
        Collider::Circle { radius },
        SpriteBundle {
            sprite: Sprite::from_color(color, Vec2::splat(radius * 2.0)),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        },
    ));
}

fn setup_sandbox(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // Ground
    commands.spawn((
        Position(Vec2::new(0.0, -300.0)),
        SpriteBundle {
            sprite: Sprite::from_color(Color::srgb(0.3, 0.3, 0.3), Vec2::new(1600.0, 40.0)),
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..default()
        },
    ));
    // Walls
    for (x, y, w, h) in [(-750.0, 0.0, 20.0, 600.0), (750.0, 0.0, 20.0, 600.0)] {
        commands.spawn((
            Position(Vec2::new(x, y)),
            SpriteBundle {
                sprite: Sprite::from_color(Color::srgb(0.4, 0.4, 0.4), Vec2::new(w, h)),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
        ));
    }
    // Initial test balls
    for i in 0..5 {
        spawn_ball(&mut commands, Vec2::new((i as f32 - 2.0) * 60.0, 200.0), 15.0, 1.0);
    }
    println!("🎮 Physics Sandbox ready!");
    println!("   🖱️ Click to spawn balls");
    println!("   ◀️ ▶️ ⬆️ ⬇️ Arrow keys for gravity");
    println!("   C=Clear, R=Reset gravity, E=Explosion, F3=Debug");
}

fn spawn_on_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    state: Res<SandboxState>,
    mut commands: Commands,
) {
    if !buttons.just_pressed(MouseButton::Left) { return; }
    let window = windows.single();
    let cursor = match window.cursor_position() { Some(p) => p, None => return };
    let (camera, cam_tf) = cameras.single();
    if let Some(pos) = camera.viewport_to_world_2d(cam_tf, cursor) {
        spawn_ball(&mut commands, pos, state.spawn_radius, state.spawn_mass);
    }
}

fn sandbox_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<PhysicsSettings>,
    mut commands: Commands,
    query: Query<Entity, With<Collider>>,
) {
    if keyboard.just_pressed(KeyCode::ArrowRight) { settings.gravity.x += 100.0; }
    if keyboard.just_pressed(KeyCode::ArrowLeft) { settings.gravity.x -= 100.0; }
    if keyboard.just_pressed(KeyCode::ArrowUp) { settings.gravity.y += 100.0; }
    if keyboard.just_pressed(KeyCode::ArrowDown) { settings.gravity.y -= 100.0; }
    if keyboard.just_pressed(KeyCode::KeyC) { for e in &query { commands.entity(e).despawn(); } }
    if keyboard.just_pressed(KeyCode::KeyR) { settings.gravity = Vec2::new(0.0, -500.0); }
    if keyboard.just_pressed(KeyCode::KeyE) {
        for i in 0..20 {
            let angle = (i as f32 / 20.0) * std::f32::consts::TAU;
            let pos = Vec2::new(angle.cos() * (50.0 + i as f32 * 3.0), angle.sin() * (50.0 + i as f32 * 3.0) + 100.0);
            spawn_ball(&mut commands, pos, 8.0, 0.5);
        }
    }
}

fn toggle_debug(keyboard: Res<ButtonInput<KeyCode>>, mut debug: ResMut<DebugMode>) {
    if keyboard.just_pressed(KeyCode::F3) {
        debug.0 = !debug.0;
        println!("🕵️ Debug: {}", if debug.0 { "ON" } else { "OFF" });
    }
}

fn render_sync(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut tf) in query.iter_mut() {
        tf.translation = Vec3::new(pos.0.x, pos.0.y, 0.0);
    }
}

fn debug_draw(debug: Res<DebugMode>, mut gizmos: Gizmos, query: Query<(&Position, &Velocity, &Collider)>) {
    if !debug.0 { return; }
    for (pos, vel, collider) in query.iter() {
        match collider {
            Collider::Circle { radius } => { gizmos.circle_2d(pos.0, *radius, Color::GREEN); }
        }
        if vel.0.length_squared() > 1.0 {
            gizmos.line_2d(pos.0, pos.0 + vel.0 * 0.5, Color::RED);
        }
    }
}

// ═══════════════════════════════════════════════════
// 🎬 ENTRY POINT
// ═══════════════════════════════════════════════════

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<PhysicsSettings>()
        .init_resource::<DebugMode>()
        .init_resource::<SandboxState>()
        // Physics systems (ordered)
        .add_systems(Update, (
            clear_forces,
            apply_gravity,
            integrate,
        ).chain())
        // Rendering syncing
        .add_systems(Update, render_sync)
        // Game systems
        .add_systems(Startup, setup_sandbox)
        .add_systems(Update, (spawn_on_click, sandbox_controls))
        // Debug
        .add_systems(Update, (toggle_debug, debug_draw))
        .run();
}
