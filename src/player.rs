use crate::actions::Actions;
use crate::loading::ModelAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::time::FixedTimestep;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub attack_delay: i32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player))
            .add_system_set(SystemSet::on_update(GameState::Spawned).with_system(move_player))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.6))
                    .with_system(attack_delay),
            );
    }
}

fn spawn_player(mut commands: Commands, models: Res<ModelAssets>) {
    commands
        .spawn_bundle(SceneBundle {
            scene: models.player_model.clone(),
            transform: Transform::from_xyz(10.0, 0.04, 10.0)
                .with_scale(Vec3 {
                    x: 0.0075,
                    y: 0.0075,
                    z: 0.0075,
                })
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 1.5)),
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player { attack_delay: 0 });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let speed = 10.0;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        0.0,
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
    );

    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}

fn attack_delay(mut player_query: Query<&mut Player, With<Player>>) {
    for mut player in &mut player_query {
        player.attack_delay = std::cmp::max(0, player.attack_delay - 1);
    }
}
