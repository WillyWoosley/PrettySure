use bevy::prelude::*;

use crate::{game::GamePlugin, menu::MenuPlugin};

mod menu;
mod game;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum AppState {
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Trivia Game".to_string(),
            ..Default::default()
        })
        .add_state(AppState::Menu)
        .add_startup_system(setup)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn_bundle(OrthographicCameraBundle::new_2d());
    cmds.spawn_bundle(UiCameraBundle::default());
}

