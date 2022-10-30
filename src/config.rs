use crate::matomenos::CurrentSpawn;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct ConfigPlugin;

pub struct Config {
    pub players: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config { players: 1 }
    }
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Config>().add_system(config_ui);
    }
}

fn config_ui(
    mut egui_context: ResMut<EguiContext>,
    mut config: ResMut<Config>,
    mut state: ResMut<State<GameState>>,
    current_spawn: Res<CurrentSpawn>,
) {
    egui::Window::new("Config").show(egui_context.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut config.players, 1..=5).text("Players"));
        ui.label(format!("Leaks: {}", current_spawn.leaks));
        ui.horizontal(|ui| {
            let spawn = ui.button("Spawn");
            let reset = ui.button("Reset");
            match state.current() {
                GameState::Playing => {
                    if spawn.clicked() {
                        state.push(GameState::Spawned).unwrap();
                    }
                }
                GameState::Spawned => {
                    if reset.clicked() {
                        state.pop().unwrap();
                    }
                }
                _ => {}
            }
        });
    });
}
