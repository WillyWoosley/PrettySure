use bevy::prelude::*;

use crate::{AppState, ButtonMaterials};
use crate::game::token::SideLength;

// Hardcoded for now for predetermined screen size
const OFFSET_X: f32 = -400.;
const OFFSET_Y: f32 = -300.;

pub struct CheckPlugin;

#[derive(Component)]
pub struct SubmitButton;
#[derive(Default, Component)]
pub struct Answer;
#[derive(Component)]
struct AnswerText;
#[derive(Component)]
pub struct AnswerSlot;
#[derive(Component)]
pub struct QuestionText;

#[derive(Default, Bundle)]
struct AnswerBundle {
    answer: Answer,
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
                                                   .with_system(update_question)
                                                   .with_system(update_answers));
    }
}

// Spawns an 'answerblock' in every added AnswerSlot
fn spawn_answerblock(answer_slots: Query<(&GlobalTransform, &Node), Added<AnswerSlot>>,
                     asset_server: Res<AssetServer>,                     
                     mut cmds: Commands,
) {
    for (answer_gt, answer_node) in answer_slots.iter() {
        let answer_t = answer_gt.translation + Vec3::new(OFFSET_X, OFFSET_Y, 0.);

        // Whole Bundle
        cmds.spawn_bundle(AnswerBundle {
            answer: Answer,
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
                    color: Color::rgb(0.25, 0.25, 0.75),
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
                            value: "Temporary answer text".to_string(),
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
}

fn submit_button(mut submit_pressed: EventWriter<SubmitPressed>,
                 mut submit_query: Query<(&Interaction, &mut UiColor),
                                         (Changed<Interaction>, With<SubmitButton>)>,
                 button_colors: Res<ButtonMaterials>,
) {
    for (interaction, mut color) in submit_query.iter_mut() {
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

