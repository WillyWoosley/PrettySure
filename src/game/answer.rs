use bevy::prelude::*;

use crate::{AppState, ButtonMaterials};

pub struct CheckPlugin;

#[derive(Component)]
pub struct SubmitButton;
#[derive(Component)]
pub struct AnswerText;
#[derive(Component)]
pub struct QuestionText;

struct SubmitPressed;

impl Plugin for CheckPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SubmitPressed>()
           .add_system_set(
               SystemSet::on_update(AppState::Game).with_system(submit_button)
                                                   .with_system(update_question)
                                                   .with_system(update_answers));
    }
}

fn submit_button(mut submit_pressed: EventWriter<SubmitPressed>,
                 mut submit_query: Query<(&Interaction, &mut UiColor),
                                         (Changed<Interaction>, With<SubmitButton>)>,
                 button_colors: Res<ButtonMaterials>,
) {
    for (interaction, mut color) in submit_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = button_colors.clicked.clone();
                submit_pressed.send(SubmitPressed);
            },
            Interaction::Hovered => {
                *color = button_colors.hovered.clone();
            },
            Interaction::None => {
                *color = button_colors.none.clone();
            },
        }
    }
}

fn update_question(mut submit_pressed: EventReader<SubmitPressed>,
                    mut question_query: Query<&mut Text, With<QuestionText>>,
) {
    if submit_pressed.iter().last().is_some() {
        for mut question in question_query.iter_mut() {
            question.sections[0].value = String::from("Question updated");
        }
    }
}

fn update_answers(mut submit_pressed: EventReader<SubmitPressed>,
                  mut answer_query: Query<&mut Text, With<AnswerText>>,
) {
    if submit_pressed.iter().last().is_some() {
        for mut answer in answer_query.iter_mut() {
            answer.sections[0].value = String::from("Answer updated");
        }
    }
}

