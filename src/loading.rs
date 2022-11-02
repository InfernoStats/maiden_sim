use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<ImageAssets>()
                .with_collection::<ModelAssets>()
                .continue_to_state(GameState::Menu),
        );
    }
}

#[derive(AssetCollection)]
pub struct ModelAssets {
    #[asset(path = "models/room.glb#Scene0")]
    pub room_model: Handle<Scene>,
    #[asset(path = "models/player.glb#Scene0")]
    pub player_model: Handle<Scene>,
    #[asset(path = "models/maiden.glb#Scene0")]
    pub maiden_model: Handle<Scene>,
    #[asset(path = "models/matomenos.glb#Scene0")]
    pub matomenos_model: Handle<Scene>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "images/ice_barrage.png")]
    pub spell_sprite: Handle<Image>,
    #[asset(path = "images/ice_barrage_selected.png")]
    pub spell_selected_sprite: Handle<Image>,
}
