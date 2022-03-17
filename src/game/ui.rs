use bevy::prelude::*;

use crate::AppState;

const ANSWER_COUNT: u8 = 4;

#[derive(Component)]
struct AnswerText;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game).with_system(setup_ui));
    }
}

fn setup_ui(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn_bundle(UiCameraBundle::default());

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
                size: Size::new(Val::Percent(15.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..Default::default()

        }).with_children(|parent| {
            // Counter Text Container
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
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
                        "Questions",
                        TextStyle {
                            font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                            font_size: 24.,
                            color: Color::WHITE,
                        },
                        Default::default()
                    ),
                    ..Default::default()
                });

                // Token Counter Text
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Tokens",
                        TextStyle {
                            font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                            font_size: 24.,
                            color: Color::WHITE,
                        },
                        Default::default()
                    ),
                    ..Default::default()
                });
            });
        });
        
        // Gameboard Content
        // Content Containter
        parent.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                flex_wrap: FlexWrap::Wrap,
                size: Size::new(Val::Percent(70.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()

        }).with_children(|parent| {
            // Question Container
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(30.)),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            }).with_children(|parent| {
                // Question Text
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Temporary question text",
                        TextStyle {
                            font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                            font_size: 40.,
                            color: Color::BLACK,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            });
            
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(70.)),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            }).with_children(|parent| {
            // Answer Containers
            for i in 0..ANSWER_COUNT {
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: Rect::all(Val::Px(5.)),
                        padding: Rect::all(Val::Px(5.)),
                        size: Size::new(Val::Px(300.), Val::Px(200.)),
                        ..Default::default()
                    },
                    color: Color::YELLOW.into(),
                    ..Default::default()
                }).with_children(|parent| {
                    // Answer Text
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            format!("Temp Text {}", i),
                            TextStyle {
                                font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                                font_size: 24.,
                                color: Color::BLACK,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            }
            });
        });

        // Right Border
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(15.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..Default::default()

        });
    });
}

