use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::{AppState, ButtonMaterials};
use crate::game::ui::{ScoreCount, QuestionCount};
use crate::game::token::{Token, On, SideLength};
use crate::game::load::Rounds;

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
pub struct QuestionSlot;
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
struct NewRound; // TODO: would love to remove, but SystemSet and Stages funky ATM,
                 //       both on my and Bevy's end

impl Plugin for CheckPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SubmitPressed>()
           .add_event::<NewRound>()
           .add_system_set(
               SystemSet::on_update(AppState::Game).with_system(spawn_questionblock)
                                                   .with_system(spawn_answerblock)
                                                   .with_system(submit_button)
                                                   .with_system(submit_visible)
                                                   .with_system(submit_tokens)
                                                   .with_system(update_round)
                                                   .with_system(update_q_and_a))
           .add_system_set(
               SystemSet::on_exit(AppState::Game).with_system(teardown_blocks));
    }
}

// Spawns a 'questionblock' in the QuestionSlot
fn spawn_questionblock(question_slot: Query<(Entity, &GlobalTransform, &Node), 
                        With<QuestionSlot>>,
                    rounds: Res<Rounds>,
                    asset_server: Res<AssetServer>,
                    mut cmds: Commands,
) {
    for (slot_id, slot_gt, slot_node) in question_slot.iter() {
        if slot_gt.translation.x != 0. || slot_gt.translation.y != 0. {
            let question_t = slot_gt.translation 
                + Vec3::new(OFFSET_X, OFFSET_Y, 0.);

            cmds.spawn_bundle(Text2dBundle {
               text: Text::with_section(
                    rounds.questions[rounds.round_number].text.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                        font_size: 40.,
                        color: Color::BLACK,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                transform: Transform {
                    translation: question_t,
                    ..Default::default()
                },
                text_2d_bounds: Text2dBounds {
                    size: Size::new(slot_node.size.x, slot_node.size.y),
                },
                ..Default::default()
            }).insert(QuestionText);
            
            cmds.entity(slot_id).remove::<QuestionSlot>();
        }
    }
}

// Spawns an 'answerblock' in every AnswerSlot
fn spawn_answerblock(answer_slots: Query<(Entity, &GlobalTransform, &Node),
                        With<AnswerSlot>>,
                     asset_server: Res<AssetServer>,
                     rounds: Res<Rounds>,
                     mut cmds: Commands,
) {
    let palette = [AnswerColor(Color::RED), AnswerColor(Color::GREEN), 
                   AnswerColor(Color::BLUE), AnswerColor(Color::YELLOW)];

    for (i, (slot_id, answer_gt, answer_node)) in answer_slots.iter().enumerate() {
        // Mega scuffed, but only way around my poor programming and Bevy's poor
        // frame-update dispatch decisions
        let question = &rounds.questions[rounds.round_number];

        if answer_gt.translation.x != 0. || answer_gt.translation.y != 0. {
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
                                    font: asset_server
                                              .load("fonts/PublicSans-Medium.ttf"),
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
                    text_2d_bounds: Text2dBounds {
                        size: Size::new(answer_node.size.x, answer_node.size.y),
                    },
                    ..Default::default()
                }).insert(AnswerText);
            });

            cmds.entity(slot_id).remove::<AnswerSlot>();
        }
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
                 mut new_round: EventWriter<NewRound>,
                 tokens: Query<&On, With<Token>>,
                 answer_blocks: Query<&Truth, With<AnswerBlock>>,
                 mut score_count: Query<(&mut Text, &mut ScoreCount)>,
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

        let (mut text, mut score) = score_count.single_mut();
        score.0 += correct;
        text.sections[0].value = format!("Score: {}", score.0);

        new_round.send(NewRound);
    }
}

fn update_round(mut new_round: EventReader<NewRound>,
                mut rounds: ResMut<Rounds>,
                mut q_count: Query<(&mut Text, &mut QuestionCount)>,
) {
    if new_round.iter().last().is_some() {
        rounds.round_number += 1;
        
        if rounds.round_number < rounds.round_max {
            let (mut text, mut question) = q_count.single_mut();
            question.0 += 1;
            text.sections[0].value = format!("Question: {}/2", question.0);
        }
    }
}

// Updates QuestionText and AnswerText for a new rounds when SubmitPressed
fn update_q_and_a(mut qa_text: ParamSet<(
                      Query<&mut Text, With<QuestionText>>,
                      Query<(&mut Text, &Parent), With<AnswerText>>,
                  )>,
                  mut truths: Query<&mut Truth>,
                  rounds: Res<Rounds>,
) {
    if rounds.is_changed() && rounds.round_number < rounds.round_max {
        let new_q = &rounds.questions[rounds.round_number];

        // Update QuestionText 
        for mut question_text in qa_text.p0().iter_mut() {
            question_text.sections[0].value = new_q.text.clone();
        }

        // Update AnswerText and Truth
        for (i, (mut a_text, a_parent)) in qa_text.p1().iter_mut().enumerate() {
            a_text.sections[0].value = new_q.answers[i].text.clone();
            if let Ok(mut a_truth) = truths.get_mut(a_parent.0) {
                a_truth.0 = new_q.answers[i].truth;
            }
        }
    }
}

// Removes all blocks and children thereof
fn teardown_blocks(answer_query: Query<Entity, With<AnswerBlock>>,
                   question_query: Query<Entity, With<QuestionText>>,
                   mut cmds: Commands,
) {
    for answer_id in answer_query.iter() {
        cmds.entity(answer_id).despawn_recursive();
    }

    let question_id = question_query.single();
    cmds.entity(question_id).despawn();
}

