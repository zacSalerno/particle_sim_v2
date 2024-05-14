use crate::{constants::*, gravity_point::*, particle::*};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_fps_counter::FpsCounterText;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct MyInputPlugin;
impl Plugin for MyInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_gravity_point,
                check_for_reset,
                change_velocity,
                change_acceleration,
                remove_particles,
                add_particles,
            ),
        );
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
    entities: Query<
        Entity,
        (
            Without<Camera>,
            Without<Window>,
            Without<FpsCounterText>,
            Without<Text>,
        ),
    >,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        reset_sim(commands, entities);
    }
}

fn reset_sim(
    mut commands: Commands,
    entities: Query<
        Entity,
        (
            Without<Camera>,
            Without<Window>,
            Without<FpsCounterText>,
            Without<Text>,
        ),
    >,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }

    unsafe {
        MAX_VELOCITY = OG_MAX_VELOCITY;
        MAX_ACCELERATION = OG_MAX_ACCELERATION;
    }

    // spawn_particles(commands, meshes, materials);
}

fn change_velocity(keyboard_input: Res<ButtonInput<KeyCode>>) {
    unsafe {
        if keyboard_input.pressed(KeyCode::KeyA) {
            MAX_VELOCITY -= MAX_VELOCITY_CHANGE_RATE;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            MAX_VELOCITY += MAX_VELOCITY_CHANGE_RATE;
        }
    }
}

fn change_acceleration(keyboard_input: Res<ButtonInput<KeyCode>>) {
    unsafe {
        if keyboard_input.pressed(KeyCode::KeyS) {
            MAX_ACCELERATION -= MAX_ACCELERATION_CHANGE_RATE;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            MAX_ACCELERATION += MAX_ACCELERATION_CHANGE_RATE;
        }
    }
}

fn remove_particles(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    particles: Query<Entity, With<Particle>>,
    mut commands: Commands,
) {
    if particles.is_empty() {
        return;
    }

    if keyboard_input.pressed(KeyCode::KeyQ) {
        let mut particles_list: Vec<Entity> = Vec::new();
        for particle in &particles {
            particles_list.push(particle)
        }

        if particles_list.len() >= 10 {
            for i in 0..=10 {
                commands.entity(particles_list[i]).despawn();
            }
        } else {
            commands.entity(particles_list[0]).despawn();
        }
    }
}

fn add_particles(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if keyboard_input.pressed(KeyCode::KeyE) {
        let mut rng = rand::thread_rng();

        for _ in 0..=10 {
            let rand_position: Vect = Vec2::new(
                rng.gen_range((-WIDTH / 2.0)..=(WIDTH / 2.0)),
                rng.gen_range((-HEIGHT / 2.0)..=(HEIGHT / 2.0)),
            );
            spawn_particle(&mut commands, &mut meshes, &mut materials, rand_position);
        }
    }
}
