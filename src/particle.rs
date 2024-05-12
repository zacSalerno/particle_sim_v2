use crate::gravity_point::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
struct Particle;

#[derive(Component)]
struct Acceleration {
    value: Vect,
}

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_particles);
    }
}

pub fn spawn_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vect,
) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(Acceleration {
            value: Vec2::new(0.0, 0.0),
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(1.0)).into(),
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 0.0),
                ..Default::default()
            },
            material: materials.add(Color::hex("#e1c19b").unwrap()),
            ..default()
        })
        .insert(Particle);
}

fn move_particles(
    mut particles: Query<(&mut Velocity, &mut Acceleration, &Transform), With<Particle>>,
    gravity_points: Query<&Transform, With<GravityPoint>>,
) {
    for (mut velocity, mut acceleration, particle_transform) in particles.iter_mut() {
        for &gravity_point_transform in gravity_points.iter() {
            acceleration.value =
                gravity_point_transform.translation.xy() - particle_transform.translation.xy();
            // println!("Acceleration: {}", acceleration.value);
            // println!("Velocity: {}", velocity.linvel);
            // println!("Position: {}", particle_transform.translation);

            acceleration.value = acceleration.value.clamp_length(0.0, 0.5);

            velocity.linvel = velocity.linvel.clamp_length_max(100.0);
            velocity.linvel += acceleration.value;
        }
    }
}
