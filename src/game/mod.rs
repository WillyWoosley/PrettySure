use bevy::prelude::*;

use crate::game::ui::UiPlugin;
use crate::game::answer::CheckPlugin;
use crate::game::token::TokenPlugin;
use crate::game::trivia::TriviaPlugin;

pub struct GamePlugin;

mod ui;
mod answer;
mod token;
mod trivia;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiPlugin)
           .add_plugin(CheckPlugin)
           .add_plugin(TokenPlugin)
           .add_plugin(TriviaPlugin);
    }
}

