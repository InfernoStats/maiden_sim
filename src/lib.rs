mod actions;
mod camera;
mod config;
mod loading;
mod maiden;
mod map;
mod matomenos;
mod menu;
mod player;
mod spawn_point;

use crate::actions::ActionsPlugin;
use crate::camera::CameraPlugin;
use crate::config::ConfigPlugin;
use crate::loading::LoadingPlugin;
use crate::maiden::MaidenPlugin;
use crate::map::MapPlugin;
use crate::matomenos::MatomenosPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Spawned,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(ConfigPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(MaidenPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(MatomenosPlugin)
            .add_plugin(CameraPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
