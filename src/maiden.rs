use crate::loading::ModelAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MaidenPlugin;

#[derive(Component)]
pub struct Maiden;

impl Plugin for MaidenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_maiden));
    }
}

fn spawn_maiden(mut commands: Commands, models: Res<ModelAssets>) {
    commands
        .spawn_bundle(SceneBundle {
            scene: models.maiden_model.clone(),
            transform: Transform::from_xyz(4.60, 0.25, 11.49)
                .with_scale(Vec3 {
                    x: 0.0075,
                    y: 0.0075,
                    z: 0.0075,
                })
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 4.5)),
            ..Default::default()
        })
        .insert(Name::new("Maiden"))
        .insert(Maiden);
}
