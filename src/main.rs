mod constants;
mod gravity_point;
mod input;
mod particle;
mod particle_spawner;

use bevy::prelude::*;
use bevy_fps_counter::{FpsCounter, FpsCounterPlugin};
use bevy_rapier2d::prelude::*;
use constants::*;
use gravity_point::*;
use input::*;
use particle::*;
use particle_spawner::*;

use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping};

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
        .add_plugins(FpsCounterPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ParticlePlugin)
        .add_plugins(GravityPointPlugin)
        .add_plugins(ParticleSpawnerPlugin)
        .add_plugins(MyInputPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, (acc_text, vel_text, part_text))
        .run();
}

#[derive(Component)]
struct AccelerationText;

#[derive(Component)]
struct VelocityText;

#[derive(Component)]
struct ParticleText;

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut diags_state: ResMut<FpsCounter>,
) {
    rapier_config.gravity = Vect::ZERO;

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..Default::default()
        },
        BloomSettings::NATURAL,
    ));

    diags_state.enable();

    commands
        .spawn(
            TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
        )
        .insert(AccelerationText);

    commands
        .spawn(
            TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(35.0),
                left: Val::Px(10.0),
                ..default()
            }),
        )
        .insert(VelocityText);

    commands
        .spawn(
            TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(60.0),
                left: Val::Px(10.0),
                ..default()
            }),
        )
        .insert(ParticleText);
    commands.spawn(
        TextBundle::from_section(
            "Press (R) to reset",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(85.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn acc_text(mut acceleration_text: Query<&mut Text, With<AccelerationText>>) {
    let mut acceleration_text = acceleration_text.single_mut();

    unsafe {
        acceleration_text.sections[0].value = format!(
            "(W/S)Max Acceleration:{}",
            MAX_ACCELERATION.round().to_string()
        );
    }
}

fn vel_text(mut velocity_text: Query<&mut Text, With<VelocityText>>) {
    let mut velocity_text = velocity_text.single_mut();
    unsafe {
        velocity_text.sections[0].value =
            format!("(A/D)Max Velocity:{}", MAX_VELOCITY.round().to_string());
    }
}

fn part_text(
    mut particle_text: Query<&mut Text, With<ParticleText>>,
    particles: Query<Entity, With<Particle>>,
) {
    let mut particle_text = particle_text.single_mut();

    let mut particles_list: Vec<Entity> = Vec::new();
    for particle in &particles {
        particles_list.push(particle)
    }
    particle_text.sections[0].value =
        format!("(Q/E)Particles:{}", particles_list.len().to_string());
}
