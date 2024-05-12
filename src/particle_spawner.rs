use crate::constants::*;
use crate::particle::*;
use crate::PARTICLES_TO_SPAWN;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct ParticleSpawnerPlugin;
impl Plugin for ParticleSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_particles);
    }
}

pub fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..=PARTICLES_TO_SPAWN {
        let rand_position: Vect = Vec2::new(
            rng.gen_range((-WIDTH / 2.0)..=(WIDTH / 2.0)),
            rng.gen_range((-HEIGHT / 2.0)..=(HEIGHT / 2.0)),
        );
        spawn_particle(&mut commands, &mut meshes, &mut materials, rand_position);
    }
}
