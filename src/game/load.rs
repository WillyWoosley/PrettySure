use bevy::prelude::*;

use crate::AppState;

pub struct LoadPlugin;

#[derive(Default, Component)]
pub struct Answer {
    pub text: String,
    pub truth: bool,
}
#[derive(Default, Component)]
pub struct Question {
    pub text: String,
    pub answers: [Answer; 4],
}

#[derive(Default)]
pub struct Rounds {
    pub round_number: usize,
    pub round_max: usize,
    pub questions: [Question; 2],
}

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Load).with_system(insert_trivia));
    }
}

// Placeholder logic, to be replaced when we start actually pulling real
// trivia questions
fn insert_trivia(mut cmds: Commands, mut appstate: ResMut<State<AppState>>) {
    let questions = [
        Question {
            text: String::from("Question 1"),
            answers: [
                Answer {
                    text: String::from("Answer 1.1"),
                    truth: true,
                },
                Answer {
                    text: String::from("Answer 1.2"),
                    truth: false,
                },
                Answer {
                    text: String::from("Answer 1.3"),
                    truth: false,
                },
                Answer {
                    text: String::from("Answer 1.4"),
                    truth: false,
                },
            ],
        },
        Question {
            text: String::from("Question 2"),
            answers: [
                Answer {
                    text: String::from("Answer 2.1"),
                    truth: false,
                },
                Answer {
                    text: String::from("Answer 2.2"),
                    truth: true,
                },
                Answer {
                    text: String::from("Answer 2.3"),
                    truth: false,
                },
                Answer {
                    text: String::from("Answer 2.4"),
                    truth: false,
                },
            ],
        },
    ];

    cmds.insert_resource(Rounds {
        round_number: 0,
        round_max: questions.len(),
        questions
    });

    appstate.set(AppState::Game).unwrap();
}

