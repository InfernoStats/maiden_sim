use crate::loading::ModelAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MapPlugin;

pub const MAP_WIDTH: usize = 24;

#[derive(Component)]
pub struct Map;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_map));
    }
}

fn spawn_map(mut commands: Commands, models: Res<ModelAssets>) {
    commands
        .spawn_bundle(SceneBundle {
            scene: models.room_model.clone(),
            transform: Transform::from_xyz(10.9, -2.275, 14.8).with_scale(Vec3 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            }),
            ..Default::default()
        })
        .insert(Name::new("Map"))
        .insert(Map);
}
