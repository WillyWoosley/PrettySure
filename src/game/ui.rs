use bevy::prelude::*;

use crate::AppState;
use crate::game::answer::{
    AnswerSlot,
    QuestionText, 
    SubmitButton,
};
use crate::game::token::TokenSlot;
use crate::game::load::Rounds;

pub struct UiPlugin;

#[derive(Component)]
struct UiRoot;
#[derive(Component)]
pub struct QuestionCount(pub u8);
#[derive(Component)]
pub struct ScoreCount(pub u8);
#[derive(Component)]
struct ScoreCard;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
               SystemSet::on_enter(AppState::Game).with_system(setup_ui))
           .add_system_set(
               SystemSet::on_update(AppState::Game).with_system(final_scorecard)
                                                   .with_system(return_to_menu))
           .add_system_set(
               SystemSet::on_exit(AppState::Game).with_system(teardown_ui));
    }
}

fn setup_ui(mut cmds: Commands,
            asset_server: Res<AssetServer>,
) {
    cmds.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    })
    .with_children(|parent| {
        // Left Border
        parent.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexEnd,
                size: Size::new(Val::Percent(15.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        }).with_children(|parent| {
            // Counter Text Container
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(25.)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            }).with_children(|parent| {
                // Question Counter Text
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Question: 1/2",
                        TextStyle {
                            font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                            font_size: 24.,
                            color: Color::BLACK,
                        },
                        Default::default()
                    ),
                    ..Default::default()
                }).insert(QuestionCount(1));

                // Score Counter Text
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Score: 0",
                        TextStyle {
                            font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                            font_size: 24.,
                            color: Color::BLACK,
                        },
                        Default::default()
                    ),
                    ..Default::default()
                }).insert(ScoreCount(0));
            });

            // Token Slots
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexEnd,
                    size: Size::new(Val::Percent(100.), Val::Percent(75.)),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            }).with_children(|parent| {
                for _ in 0..5 {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(2.5)),
                            size: Size::new(Val::Percent(50.), Val::Percent(20.)),
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    }).insert(TokenSlot);
                }
            });
        });
        
        // Gameboard Content
        // Content Containter
        parent.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                flex_wrap: FlexWrap::Wrap,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(70.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        }).with_children(|parent| {
            // Question Container
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(30.)),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            }).with_children(|parent| {
                // Question Text
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        String::new(), 
                        TextStyle {
                            font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                            font_size: 40.,
                            color: Color::BLACK,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                }).insert(QuestionText);
            });
            
            for _ in 0..2 {
                // Row Answer Container
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        padding: Rect::all(Val::Px(5.)),
                        size: Size::new(Val::Percent(100.), Val::Percent(30.)),
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                }).with_children(|parent| {
                    for _ in 0..2 {
                        // Answer Box
                        parent.spawn_bundle(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: Rect::all(Val::Px(5.)),
                                padding: Rect::all(Val::Px(5.)),
                                size: Size::new(Val::Percent(50.), Val::Percent(100.)),
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        }).insert(AnswerSlot);
                    }
                });
            }

            // Submit Button
            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(20.), Val::Percent(8.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                visibility: Visibility {
                    is_visible: false,
                },
                ..Default::default()
            }).with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Submit?",
                        TextStyle {
                            font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                            font_size: 30.,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                }).insert(SubmitButton);
            }).insert(SubmitButton);
        });

        // Right Border
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(15.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()

        });
    }).insert(UiRoot);
}

// Spawns a final scorecard when all rounds are completed
fn final_scorecard(score_q: Query<&ScoreCount>,
                   windows: Res<Windows>,
                   asset_server: Res<AssetServer>,
                   mut cmds: Commands,
                   rounds: Res<Rounds>,
) {
    if rounds.is_changed() && rounds.round_number == rounds.round_max {
        let window = windows.get_primary().unwrap();
        let x_dim = window.width() / 2.;
        let y_dim = window.height() / 2.;
        let score = score_q.single();

        cmds.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::PURPLE,
                custom_size: Some(Vec2::new(x_dim, y_dim)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(x_dim - 5., y_dim - 5.)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., 11.),
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn_bundle(Text2dBundle {
                transform: Transform::from_xyz(0., 0., 55.),
                text: Text {
                    sections: vec![
                        TextSection {
                            value: format!("Final Score: {} Points!\n", score.0),
                            style: TextStyle {
                                font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                                font_size: 24.,
                                color: Color::BLACK,
                            },
                        },
                        TextSection {
                            value: String::from("Click anywhere to continue..."),
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
            });
        }).insert(ScoreCard);
    }
}

// Returns to the main menu on any left click if the game is over
fn return_to_menu(rounds: Res<Rounds>, 
                  mouse_button: Res<Input<MouseButton>>,
                  mut appstate: ResMut<State<AppState>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) &&
       rounds.round_number == rounds.round_max {
        appstate.set(AppState::Menu).unwrap();
    }
}

// Despawns all elements of the Ui 
fn teardown_ui(ui_q: Query<Entity, With<UiRoot>>,
               scorecard_q: Query<Entity, With<ScoreCard>>,
               mut cmds: Commands,
) {
    let root = ui_q.single();
    cmds.entity(root).despawn_recursive();

    let scorecard = scorecard_q.single();
    cmds.entity(scorecard).despawn_recursive();
}

