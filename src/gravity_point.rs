use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub struct GravityPointPlugin;
impl Plugin for GravityPointPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Debug)]
pub struct GravityPoint;

pub fn make_gravity_point(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vect,
) {
    commands
        .spawn(Collider::ball(5.0))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(5.0)).into(),
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 0.0),
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.2, 0.9, 0.2)),
            ..default()
        })
        .insert(GravityPoint);
}
