mod constants;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use constants::*;
use rand::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WIDTH, HEIGHT).into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, spawn_particle))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, move_particles)
        .run();
}

#[derive(Component, Debug)]
struct Particle;

#[derive(Component)]
struct Acceleration {
    value: Vect,
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vect::ZERO;

    commands.spawn(Camera2dBundle::default());
}

fn spawn_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(5.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Mass(0.0))
        .insert(Velocity {
            linvel: Vec2::new(-10.0, -10.0),
            angvel: 0.0,
        })
        .insert(Acceleration {
            value: Vec2::new(1.0, 1.0),
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(5.0)).into(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            material: materials.add(Color::hex("#e1c19b").unwrap()),
            ..default()
        })
        .insert(Particle);
}

fn move_particles(mut query: Query<(&mut Velocity, &Acceleration), With<Particle>>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.linvel += acceleration.value;
        velocity.angvel = 0.0;
        println!("Velocity: {}", velocity.linvel)
    }
}

fn spawn_gravity_point(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}
