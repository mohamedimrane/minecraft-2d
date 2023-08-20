use bevy::prelude::*;

// PLUGINS

pub struct GameModePlugin;

impl Plugin for GameModePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(GameMode::default())
            // Systems
            .add_systems(Startup, switch_gamemode);
    }
}

// RESOURCES
#[derive(Resource, Default)]
pub enum GameMode {
    #[default]
    Creative,
    Survival,
}

// SYSTEMS

fn switch_gamemode(mut gamemode: ResMut<GameMode>, keys: Res<Input<KeyCode>>) {
    *gamemode = if keys.just_pressed(KeyCode::C) {
        GameMode::Creative
    } else if keys.just_pressed(KeyCode::V) {
        GameMode::Survival
    } else {
        return;
    }
}
