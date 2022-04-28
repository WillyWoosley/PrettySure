use bevy::prelude::*;

use crate::{AppState, ButtonMaterials};

pub struct MenuPlugin;

struct MenuData {
    menu_handle: Entity,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
                SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(AppState::Menu).with_system(play_button))
            .add_system_set(
                SystemSet::on_exit(AppState::Menu).with_system(teardown_menu));
    }
}

fn setup_menu(mut cmds: Commands, asset_server: Res<AssetServer>) {
    let menu_handle = cmds.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(256.), Val::Px(256.)),
                ..Default::default()
            },
            image: asset_server.load("logo.png").into(),
            ..Default::default()
        });

        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                    "PrettySure",
                    TextStyle {
                        font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                        font_size: 120.,
                        color: Color::BLACK,
                    },
                    Default::default()
                  ),
            ..Default::default()
        });

       parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                    "A trivia game about hedging your bets!",
                    TextStyle {
                        font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                        font_size: 35.,
                        color: Color::BLACK,
                    },
                    Default::default()
                  ),
            ..Default::default()
        });

        parent.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.), Val::Px(50.)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                          "Play",
                          TextStyle {
                              font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                              font_size: 40.,
                              color: Color::rgb(1., 1., 1.),
                          },
                          Default::default(),
                      ),
                ..Default::default()
            });
        });
    }).id();

    cmds.insert_resource(MenuData{menu_handle});
}

fn play_button(mut state: ResMut<State<AppState>>, 
               mut query: Query<(&Interaction, &mut UiColor),
                                (Changed<Interaction>, With<Button>)>,
               button_colors: Res<ButtonMaterials>,               
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = button_colors.clicked;
                state.set(AppState::Load).unwrap();
            },
            Interaction::Hovered => {
                *color = button_colors.hovered;
            },
            Interaction::None => {
                *color = button_colors.none;
            }
        }
    }
}

fn teardown_menu(mut cmds: Commands, menu_data: Res<MenuData>) {
    cmds.entity(menu_data.menu_handle).despawn_recursive();
}

