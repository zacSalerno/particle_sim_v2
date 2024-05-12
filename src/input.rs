use crate::particle::*;
use crate::particle_spawner::*;
use crate::{gravity_point::*, HEIGHT, WIDTH};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_fps_counter::FpsCounterText;

pub struct MyInputPlugin;
impl Plugin for MyInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_gravity_point, check_for_reset));
    }
}

fn spawn_gravity_point(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Query<'_, '_, &Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        make_gravity_point(
            &mut commands,
            &mut meshes,
            &mut materials,
            cursor_position(window),
        )
    }
    // println!("Cursor Position: {:?}", cursor_position(window))
}

fn cursor_position(q_windows: Query<&Window, With<PrimaryWindow>>) -> Vec2 {
    if let Some(position) = q_windows.single().cursor_position() {
        return Vec2::new(position.x - (WIDTH / 2.0), -position.y + (HEIGHT / 2.0));
    } else {
        return Vec2::new(0.0, 0.0);
    }
}

fn check_for_reset(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    commands: Commands,
    entities: Query<Entity, (Without<Camera>, Without<Window>, Without<FpsCounterText>)>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        reset_sim(commands, entities, meshes, materials);
    }
}

// fn reset_sim(
// mut commands: Commands,
// entities: Query<Entity, (With<Particle>, With<GravityPoint>)>,
// meshes: ResMut<Assets<Mesh>>,
// materials: ResMut<Assets<ColorMaterial>>,
// ) {
// for entity in &entities {
// commands.entity(entity).despawn();
// }

// }

fn reset_sim(
    mut commands: Commands,
    entities: Query<Entity, (Without<Camera>, Without<Window>, Without<FpsCounterText>)>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }

    spawn_particles(commands, meshes, materials);
}
