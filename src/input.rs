use crate::gravity_point::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct MyInputPlugin;
impl Plugin for MyInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_gravity_point);
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
        return Vec2::new(position.x - 500.0, -position.y + 250.0);
    } else {
        return Vec2::new(0.0, 0.0);
    }
}
