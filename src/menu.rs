use bevy::prelude::*;

use crate::{AppState, ButtonMaterials};

pub struct MenuPlugin;

struct MenuData {
    button_entity: Entity,
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
    let button_entity = cmds.spawn_bundle(ButtonBundle {
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
    }).id();

    cmds.insert_resource(MenuData{button_entity});
}

fn play_button(mut state: ResMut<State<AppState>>, 
               mut query: Query<(&Interaction, &mut UiColor),
                                (Changed<Interaction>, With<Button>)>,
               button_colors: Res<ButtonMaterials>,               
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = button_colors.clicked.clone();
                state.set(AppState::Game).unwrap();
            },
            Interaction::Hovered => {
                *color = button_colors.hovered.clone();
            },
            Interaction::None => {
                *color = button_colors.none.clone();
            }
        }
    }
}

fn teardown_menu(mut cmds: Commands, menu_data: Res<MenuData>) {
    cmds.entity(menu_data.button_entity).despawn_recursive();
}

