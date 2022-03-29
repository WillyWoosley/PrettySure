use bevy::prelude::*;

use crate::{AppState, ButtonMaterials};
use crate::game::token::{Token, On, SideLength};
use crate::game::trivia::{Rounds};

// Hardcoded for now for predetermined screen size
const OFFSET_X: f32 = -400.;
const OFFSET_Y: f32 = -300.;

pub struct CheckPlugin;

#[derive(Component)]
pub struct SubmitButton;
#[derive(Default, Component)]
pub struct AnswerBlock;
#[derive(Component)]
struct AnswerText;
#[derive(Default, Component, Clone, Copy)]
pub struct AnswerColor(pub Color);
#[derive(Default, Component)]
pub struct Truth(pub bool);
#[derive(Component)]
pub struct AnswerSlot;
#[derive(Component)]
pub struct QuestionText;

#[derive(Default, Bundle)]
struct AnswerBundle {
    answer_block: AnswerBlock,
    color: AnswerColor,
    truth: Truth,
    side_length: SideLength,
    transform: Transform,
    global_transform: GlobalTransform,
}

struct SubmitPressed;

impl Plugin for CheckPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SubmitPressed>()
           .add_system_to_stage(CoreStage::Last, spawn_answerblock)
           .add_system_set(
               SystemSet::on_update(AppState::Game).with_system(submit_button)
                                                   .with_system(submit_visible)
                                                   .with_system(submit_tokens)
                                                   .with_system(update_q_and_a));
    }
}

// Spawns an 'answerblock' in every added AnswerSlot
fn spawn_answerblock(answer_slots: Query<(&GlobalTransform, &Node), Added<AnswerSlot>>,
                     asset_server: Res<AssetServer>,
                     mut rounds: ResMut<Rounds>,
                     mut cmds: Commands,
) {
    let palette = [AnswerColor(Color::RED), AnswerColor(Color::GREEN), 
                   AnswerColor(Color::BLUE), AnswerColor(Color::YELLOW)];
    let question = &rounds.questions[rounds.round_number];

    for (i, (answer_gt, answer_node)) in answer_slots.iter().enumerate() {
        let answer_t = answer_gt.translation + Vec3::new(OFFSET_X, OFFSET_Y, 0.);

        // Whole Bundle
        cmds.spawn_bundle(AnswerBundle {
            answer_block: AnswerBlock,
            color: palette[i],
            truth: Truth(question.answers[i].truth),
            side_length: SideLength {
                x_len: answer_node.size.x,
                y_len: answer_node.size.y,
            },
            transform: Transform {
                translation: answer_t,
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            // Answer Sprite
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: palette[i].0,
                    custom_size: Some(answer_node.size),
                    ..Default::default()
                },
                ..Default::default()
            });
            
            // Answer Text
            parent.spawn_bundle(Text2dBundle {
                transform: Transform::from_xyz(0., 0., 0.5),
                text: Text {
                    sections: vec![
                        TextSection {
                            value: question.answers[i].text.clone(),
                            style: TextStyle {
                                font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                                font_size: 24.,
                                color: Color::BLACK,
                            },
                        },
                    ],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
                ..Default::default()
            }).insert(AnswerText);
        });
    }

    // Update to first round
    if answer_slots.iter().last().is_some() {
        rounds.round_number += 1;
    }
}

fn submit_button(mut submit_pressed: EventWriter<SubmitPressed>,
                 mut submit_query: Query<(&Visibility, &Interaction, &mut UiColor),
                                         (Changed<Interaction>, With<SubmitButton>)>,
                 button_colors: Res<ButtonMaterials>,
) {
    for (visibility, interaction, mut color) in submit_query.iter_mut() {
        if visibility.is_visible {
            match interaction {
                Interaction::Clicked => {
                    *color = button_colors.clicked;
                    submit_pressed.send(SubmitPressed);
                },
                Interaction::Hovered => {
                    *color = button_colors.hovered;
                },
                Interaction::None => {
                    *color = button_colors.none;
                },
            }
        }
    }
}

// Checks to see whether the visibility of the submit button should be updated
fn submit_visible(token_query: Query<Option<&On>, With<Token>>,
                  mut submit_query: Query<&mut Visibility,
                      With<SubmitButton>>,
) {
    for mut submit_visibility in submit_query.iter_mut() {
        if submit_visibility.is_visible {
            if token_query.iter().filter(|on| on.is_none()).count() != 0 {
                submit_visibility.is_visible = false;
            }
        } else {
            if token_query.iter().filter(|on| on.is_none()).count() == 0 {
                submit_visibility.is_visible = true;
            }
        }
    }
}

// Determines whether tokens were on a correct or incorrect answer when submitted
fn submit_tokens(mut submit_pressed: EventReader<SubmitPressed>,
                 tokens: Query<&On, With<Token>>,
                 answer_blocks: Query<&Truth, With<AnswerBlock>>,
) {
    if submit_pressed.iter().last().is_some() {
        let mut correct = 0;
        for token_on in tokens.iter() {
            if let Ok(answer_truth) = answer_blocks.get(token_on.0) {
                if answer_truth.0 {
                    correct += 1;
                }
            }
        }

        info!("Got {} correct guesses!", correct);
    }
}

// Updates QuestionText and AnswerText for a new rounds when SubmitPressed
fn update_q_and_a(mut submit_pressed: EventReader<SubmitPressed>,
                  mut qa_text: QuerySet<(
                      QueryState<&mut Text, With<QuestionText>>,
                      QueryState<&mut Text, With<AnswerText>>,
                  )>,
                  mut rounds: ResMut<Rounds>,
) {
    if submit_pressed.iter().last().is_some() {
        let new_q = &rounds.questions[rounds.round_number];

        // Update QuestionText 
        for mut question_text in qa_text.q0().iter_mut() {
            question_text.sections[0].value = new_q.text.clone();
        }

        // Update AnswerText
        for (i, mut answer_text) in qa_text.q1().iter_mut().enumerate() {
            answer_text.sections[0].value = new_q.answers[i].text.clone();
        }
        
        // Update and check rounds remaining
        rounds.round_number += 1;
        if rounds.round_number == rounds.round_max {
            // TODO: Signal the end of the game, for now just ceases updates
            rounds.round_number -= 1;
        }
    }
}

