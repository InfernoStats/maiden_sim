#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{
    AmbientLight, App, ClearColor, Color, NonSend, StandardMaterial, WindowDescriptor,
};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
use maiden::GamePlugin;
use std::io::Cursor;
use winit::window::Icon;

pub const WINDOW_HEIGHT: f32 = 540.0;
pub const WINDOW_WIDTH: f32 = 960.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "Maiden Simulator".to_string(),
            canvas: Some("#maiden".to_owned()),
            ..Default::default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.8 / 1.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins_with(DefaultPickingPlugins::build(RaycastBackend), |group| {
            group.disable::<CustomHighlightingPlugin<StandardMaterial>>()
        })
        .add_plugin(EguiPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(set_window_icon)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/images/app_icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
