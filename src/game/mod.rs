use bevy::prelude::*;

use crate::game::ui::UiPlugin;

pub struct GamePlugin;

mod ui;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiPlugin);
    }
}

