use bevy::prelude::*;

use crate::game::ui::UiPlugin;
use crate::game::answer::CheckPlugin;

pub struct GamePlugin;

mod ui;
mod answer;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiPlugin)
           .add_plugin(CheckPlugin);
    }
}

