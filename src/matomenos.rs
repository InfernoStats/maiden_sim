use crate::config::Config;
use crate::loading::ModelAssets;
use crate::map::MAP_WIDTH;
use crate::player::Player;
use crate::spawn_point::{generate_spawn_points, SpawnPoint};
use crate::GameState;
use bevy::{pbr::NotShadowCaster, prelude::*, time::FixedTimestep};
use bevy_mod_picking::{highlight::InitialHighlight, prelude::*};

pub struct MatomenosPlugin;

#[derive(Component)]
pub struct Matomenos {
    frozen: bool,
}

pub struct CurrentSpawn {
    move_delay: bool,
    has_spawned: bool,
    spawn_delay: Timer,
    spawns: Vec<SpawnPoint>,
    pub leaks: i32,
    pub rerun: bool,
}

impl Default for CurrentSpawn {
    fn default() -> CurrentSpawn {
        CurrentSpawn {
            move_delay: true,
            has_spawned: false,
            spawn_delay: Timer::from_seconds(3.0, false),
            spawns: Vec::new(),
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
    if current_spawn.has_spawned {
        return;
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

        current_spawn.has_spawned = true;
    }
}

fn spawn_single_nylo(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    models: &Res<ModelAssets>,
    spawn: SpawnPoint,
) {
    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform::from_xyz(
            spawn.x, 0.75, spawn.y,
        )))
        .insert(meshes.add(shape::Cube { size: 1.75 }.into()))
        .insert(materials.add(Color::NONE.into()))
        .insert(NotShadowCaster)
        .insert(PickRaycastTarget::default())
        .insert_bundle(PickableBundle::default())
        .forward_events::<PointerClick, NylocasClicked>()
        .insert(InitialHighlight {
            initial: materials.add(Color::NONE.into()),
        })
        .insert(HighlightOverride {
            hovered: Some(materials.add(Color::NONE.into())),
            pressed: Some(materials.add(Color::NONE.into())),
            selected: Some(materials.add(Color::NONE.into())),
        })
        .insert(Name::new("Matomenos"))
        .insert(Matomenos { frozen: false })
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
    mut query: Query<(Entity, &Matomenos, &mut Transform), With<Matomenos>>,
    mut current_spawn: ResMut<CurrentSpawn>,
) {
    if !current_spawn.has_spawned {
        return;
    }

    if current_spawn.move_delay {
        current_spawn.move_delay = false;
        return;
    }

    for (entity, nylo, mut transform) in query.iter_mut() {
        if nylo.frozen {
            continue;
        }

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

pub fn reset(
    mut commands: Commands,
    mut matomenos: Query<Entity, With<Matomenos>>,
    mut current_spawn: ResMut<CurrentSpawn>,
) {
    for entity in matomenos.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    current_spawn.move_delay = true;
    current_spawn.has_spawned = false;
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
        mut player: Query<&mut Player, With<Player>>,
        mut query: Query<(&Transform, &mut Matomenos), With<Matomenos>>,
        current_spawn: Res<CurrentSpawn>,
    ) {
        let mut player = player.single_mut();
        if current_spawn.move_delay || player.attack_delay != 0 {
            return;
        }

        for event in events.iter() {
            let (target_x, target_z) = match query.get(event.0) {
                Ok((t, _)) => (t.translation.x, t.translation.z),
                Err(_) => return,
            };

            // For every nylo on the map, search in a 3x3 area for positions next to the target
            for (transform, mut matomenos) in query.iter_mut() {
                let (x, z) = (transform.translation.x, transform.translation.z);
                if f32::abs(target_x - x) <= 1.0 && f32::abs(target_z - z) <= 1.0 {
                    matomenos.frozen = true;
                    player.attack_delay = 5;
                }
            }
        }
    }
}
