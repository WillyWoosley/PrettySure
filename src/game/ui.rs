use bevy::prelude::*;

use crate::AppState;

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
        // left border
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(15.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::rgb(0.5, 0.5, 0.5).into(),
            ..Default::default()

        }).with_children(|parent| {
            // Text Container
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            }).with_children(|parent| {
                // Question Text
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
                
                // Token Text
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
        
        //right border
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

