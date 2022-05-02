use bevy::prelude::*;

use crate::{ButtonMaterials, AppState};

pub struct HelpPlugin;

#[derive(Component)]
struct HelpScreen;

impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
                SystemSet::on_enter(AppState::Help).with_system(spawn_help_menu))
            .add_system_set(
                SystemSet::on_update(AppState::Help).with_system(back_button))
            .add_system_set(
                SystemSet::on_exit(AppState::Help).with_system(teardown_helpscreen));
    }
}

// Spawns help info text and back button
fn spawn_help_menu(asset_server: Res<AssetServer>, mut cmds: Commands) {
    cmds.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|parent| {
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
                        "Back",
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
    })
    .insert(HelpScreen);
}

// Click handler for back to AppState::Menu button
fn back_button(mut state: ResMut<State<AppState>>, 
               mut query: Query<(&Interaction, &mut UiColor),
                                (Changed<Interaction>, With<Button>)>,
               button_colors: Res<ButtonMaterials>,               
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = button_colors.clicked;
                state.set(AppState::Menu).unwrap();
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

// Tears down help screen
fn teardown_helpscreen(helpscreen_query: Query<Entity, With<HelpScreen>>,
                       mut cmds: Commands,
) {
    for help_id in helpscreen_query.iter() {
        cmds.entity(help_id).despawn_recursive();
    }
}

