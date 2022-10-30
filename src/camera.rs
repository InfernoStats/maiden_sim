use crate::actions::Actions;
use crate::player::Player;
use crate::GameState;

use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use smooth_bevy_cameras::{
    controllers::orbit::{
        ControlEvent, OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin,
    },
    LookTransform, LookTransformPlugin,
};

pub struct CameraPlugin;

#[derive(Component)]
pub struct Camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LookTransformPlugin)
            .add_plugin(OrbitCameraPlugin::new(true))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_camera))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(follow_player))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(control_camera))
            .add_system_set(SystemSet::on_update(GameState::Spawned).with_system(follow_player))
            .add_system_set(SystemSet::on_update(GameState::Spawned).with_system(control_camera));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(-2.0, 10.0, 5.0),
            Vec3::new(10.0, 1.0, 10.0),
        ))
        .insert(PickRaycastSource::default())
        .insert(Name::new("Camera"))
        .insert(Camera);
}

fn follow_player(
    actions: Res<Actions>,
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut LookTransform, With<Camera>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let player = player.single();
    let mut transform = camera.single_mut();

    let new_transform = Vec3 {
        x: player.translation.x,
        y: transform.target.y,
        z: player.translation.z,
    };
    let transform_delta = transform.target - new_transform;

    transform.eye -= transform_delta;
    transform.target = new_transform;
}

fn control_camera(
    mut events: EventWriter<ControlEvent>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    controllers: Query<&OrbitCameraController>,
) {
    let controller = match controllers.get_single() {
        Ok(c) => c,
        Err(_) => return,
    };

    let OrbitCameraController {
        mouse_rotate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        pixels_per_line,
        ..
    } = *controller;

    let cursor_delta = mouse_motion_events
        .iter()
        .fold(Vec2::ZERO, |acc, event| acc + event.delta);

    if mouse_buttons.pressed(MouseButton::Middle) {
        events.send(ControlEvent::Orbit(mouse_rotate_sensitivity * cursor_delta));
    }

    let mut scalar = 1.0;
    for event in mouse_wheel_events.iter() {
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / pixels_per_line,
        };
        scalar = get_scroll_scale(scroll_amount, mouse_wheel_zoom_sensitivity);
    }

    events.send(ControlEvent::Zoom(scalar));
}

// It appears that WASM scrolling works *very* differently to native applications
#[cfg(target_arch = "wasm32")]
fn get_scroll_scale(scroll_amount: f32, _mouse_wheel_zoom_sensitivity: f32) -> f32 {
    if scroll_amount >= 1.0 {
        0.85
    } else if scroll_amount <= -1.0 {
        1.15
    } else {
        1.0
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_scroll_scale(scroll_amount: f32, mouse_wheel_zoom_sensitivity: f32) -> f32 {
    1.0 - scroll_amount * mouse_wheel_zoom_sensitivity
}
