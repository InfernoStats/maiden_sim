use crate::config::Config;
use crate::loading::ModelAssets;
use crate::map::MAP_WIDTH;
use crate::player::Player;
use crate::spawn_point::{generate_spawn_points, SpawnPoint};
use crate::spell::Spell;
use crate::GameState;
use bevy::{pbr::NotShadowCaster, prelude::*, time::FixedTimestep};
use bevy_mod_picking::prelude::*;

pub struct MatomenosPlugin;

enum FrozenState {
    NotFrozen,
    ShouldFreeze,
    Frozen,
}

#[derive(Component)]
pub struct Matomenos {
    frozen: FrozenState,
    color_timer: Timer,
    color_handle: Handle<StandardMaterial>,
}

enum ActionState {
    NotSpawned,
    Spawned,
    Moving,
}

pub struct CurrentSpawn {
    spawn_delay: Timer,
    spawns: Vec<SpawnPoint>,
    state: ActionState,
    pub leaks: i32,
    pub rerun: bool,
}

impl Default for CurrentSpawn {
    fn default() -> CurrentSpawn {
        CurrentSpawn {
            spawn_delay: Timer::from_seconds(3.0, false),
            spawns: Vec::new(),
            state: ActionState::NotSpawned,
            leaks: 0,
            rerun: false,
        }
    }
}

impl Plugin for MatomenosPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentSpawn>()
            .add_event::<NylocasClicked>()
            .add_system_set(
                SystemSet::on_update(GameState::Spawned)
                    .with_system(spawn_nylos)
                    .with_system(draw_freeze)
                    .with_system(NylocasClicked::handle_events),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Spawned)
                    .with_run_criteria(FixedTimestep::step(0.6))
                    .with_system(move_nylos),
            )
            .add_system_set(SystemSet::on_resume(GameState::Playing).with_system(reset));
    }
}

fn spawn_nylos(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut current_spawn: ResMut<CurrentSpawn>,
    models: Res<ModelAssets>,
    config: ResMut<Config>,
) {
    match current_spawn.state {
        ActionState::Spawned | ActionState::Moving => return,
        ActionState::NotSpawned => (),
    }

    if current_spawn.spawn_delay.tick(time.delta()).finished() {
        if !current_spawn.spawns.is_empty() {
            for spawn_point in &current_spawn.spawns {
                spawn_single_nylo(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &models,
                    *spawn_point,
                );
            }
        } else {
            let spawn_points = generate_spawn_points(2 * config.players);

            for spawn_point in spawn_points {
                current_spawn.spawns.push(spawn_point);
                spawn_single_nylo(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &models,
                    spawn_point,
                );
            }
        }

        current_spawn.state = ActionState::Spawned;
    }
}

fn spawn_single_nylo(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    models: &Res<ModelAssets>,
    spawn: SpawnPoint,
) {
    let color_handle = materials.add(Color::NONE.into());

    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform::from_xyz(
            spawn.x, 0.75, spawn.y,
        )))
        .insert(meshes.add(shape::Cube { size: 1.75 }.into()))
        .insert(color_handle.clone())
        .insert(NotShadowCaster)
        .insert(PickRaycastTarget::default())
        .insert_bundle(PickableBundle::default())
        .forward_events::<PointerClick, NylocasClicked>()
        .insert(Name::new("Matomenos"))
        .insert(Matomenos {
            frozen: FrozenState::NotFrozen,
            color_timer: Timer::from_seconds(4.0 * 0.6, false),
            color_handle: color_handle,
        })
        .with_children(|commands| {
            commands.spawn_bundle(SceneBundle {
                scene: models.matomenos_model.clone(),
                transform: Transform::from_xyz(0.0, -0.75, 0.0)
                    .with_scale(Vec3 {
                        x: 0.0075,
                        y: 0.0075,
                        z: 0.0075,
                    })
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI * 1.5)),
                ..Default::default()
            });
        });
}

fn move_nylos(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Matomenos, &mut Transform), With<Matomenos>>,
    mut current_spawn: ResMut<CurrentSpawn>,
) {
    match current_spawn.state {
        ActionState::NotSpawned => return,
        ActionState::Spawned => {
            current_spawn.state = ActionState::Moving;
            return;
        }
        ActionState::Moving => (),
    }

    for (entity, mut nylo, mut transform) in query.iter_mut() {
        match nylo.frozen {
            FrozenState::Frozen => continue,
            FrozenState::ShouldFreeze => {
                nylo.frozen = FrozenState::Frozen;
            }
            FrozenState::NotFrozen => {}
        };

        // Distance to Maiden's Southwest Tile
        let distance_x = 1.5 - transform.translation.x;
        let distance_z = 13.5 - transform.translation.z;

        // Calculate next tick's X and Z coordinates
        // - If there is no difference between Maiden's X/Z and the
        //   nylo's X/Z coordinate, maintain it
        // - If there is a difference, use the signum function to add
        //   or subtract 1 unit in that direction
        let x = if distance_x == 0.0 {
            transform.translation.x
        } else {
            transform.translation.x + distance_x.signum() * 1.0
        };

        let z = if distance_z == 0.0 {
            transform.translation.z
        } else {
            transform.translation.z + distance_z.signum() * 1.0
        };

        // If the nylo will run into Maiden on this tick, despawn it
        if (x >= 2.0 && x <= 8.0) && (z >= 8.0 && z <= MAP_WIDTH as f32 - 9.0) {
            commands.entity(entity).despawn_recursive();
            current_spawn.leaks += 1;
            continue;
        }

        // Update the nylo's position vector with the new X and Z coordinates
        transform.translation = Vec3::new(x, transform.translation.y, z);
    }
}

fn draw_freeze(
    time: Res<Time>,
    mut query: Query<&mut Matomenos>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for mut nylo in query.iter_mut() {
        match nylo.frozen {
            FrozenState::Frozen => {
                nylo.color_timer.tick(time.delta());
                if nylo.color_timer.just_finished() {
                    let mut color_mat = materials.get_mut(&nylo.color_handle).unwrap();
                    color_mat.base_color = Color::NONE;
                }
                if nylo.color_timer.finished() {
                    continue;
                }
                let mut color_mat = materials.get_mut(&nylo.color_handle).unwrap();
                color_mat.base_color = Color::rgba(1.0, 1.0, 1.0, nylo.color_timer.percent_left());
            }
            FrozenState::ShouldFreeze => {
                let mut color_mat = materials.get_mut(&nylo.color_handle).unwrap();
                color_mat.base_color = Color::rgba(1.0, 1.0, 1.0, nylo.color_timer.percent_left());
            }
            FrozenState::NotFrozen => {}
        };
    }
}

fn reset(
    mut commands: Commands,
    mut matomenos: Query<Entity, With<Matomenos>>,
    mut current_spawn: ResMut<CurrentSpawn>,
) {
    for entity in matomenos.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    current_spawn.state = ActionState::NotSpawned;
    current_spawn.spawn_delay.reset();

    if !current_spawn.rerun {
        current_spawn.spawns.clear();
    }

    current_spawn.leaks = 0;
    current_spawn.rerun = false;
}

struct NylocasClicked(Entity);
impl ForwardedEvent<PointerClick> for NylocasClicked {
    fn from_data(event_data: &PointerEventData<PointerClick>) -> NylocasClicked {
        NylocasClicked(event_data.listener())
    }
}
impl NylocasClicked {
    fn handle_events(
        mut events: EventReader<NylocasClicked>,
        mut nylos_query: Query<(&Transform, &mut Matomenos), With<Matomenos>>,
        mut spell_query: Query<&mut Spell>,
        mut player_query: Query<&mut Player, With<Player>>,
    ) {
        let mut spell = spell_query.single_mut();
        let mut player = player_query.single_mut();
        if player.attack_delay != 0 || !spell.is_active {
            return;
        }

        for event in events.iter() {
            let (target_x, target_z) = match nylos_query.get(event.0) {
                Ok((t, _)) => (t.translation.x, t.translation.z),
                Err(_) => continue,
            };

            // For every nylo on the map, search in a 3x3 area for positions next to the target
            for (transform, mut matomenos) in nylos_query.iter_mut() {
                match matomenos.frozen {
                    FrozenState::NotFrozen => (),
                    _ => continue,
                }

                let (x, z) = (transform.translation.x, transform.translation.z);
                if f32::abs(target_x - x) <= 1.0 && f32::abs(target_z - z) <= 1.0 {
                    matomenos.frozen = FrozenState::ShouldFreeze;
                    player.attack_delay = 5;
                    spell.is_active = false;
                }
            }
        }
    }
}
