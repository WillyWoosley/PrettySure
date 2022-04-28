use bevy::prelude::*;

use crate::{game::GamePlugin, menu::MenuPlugin};

mod menu;
mod game;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum AppState {
    Menu,
    Load,
    Game,
}

#[derive(Default)]
pub struct ButtonMaterials {
    none: UiColor,
    hovered: UiColor,
    clicked: UiColor,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "PrettySure".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_state(AppState::Menu)
        .add_startup_system(setup)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn_bundle(UiCameraBundle::default());
    cmds.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    cmds.insert_resource(ButtonMaterials {
        none: Color::rgb(0.15, 0.15, 0.15).into(),
        hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        clicked: Color::rgb(0.35, 0.75, 0.35).into(),
    });
}

