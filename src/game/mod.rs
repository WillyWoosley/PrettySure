use bevy::prelude::*;

use crate::game::ui::UiPlugin;
use crate::game::answer::CheckPlugin;
use crate::game::token::TokenPlugin;

pub struct GamePlugin;

mod ui;
mod answer;
mod token;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiPlugin)
           .add_plugin(CheckPlugin)
           .add_plugin(TokenPlugin);
    }
}

