mod constants;
mod gravity_point;
mod input;
mod particle;
mod particle_spawner;

use bevy::{input::InputPlugin, prelude::*};
use bevy_rapier2d::prelude::*;
use constants::*;
use gravity_point::*;
use input::*;
use particle::*;
use particle_spawner::*;

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
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ParticlePlugin)
        .add_plugins(GravityPointPlugin)
        .add_plugins(ParticleSpawnerPlugin)
        .add_plugins(MyInputPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vect::ZERO;

    commands.spawn(Camera2dBundle::default());
}
