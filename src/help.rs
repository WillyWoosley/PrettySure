use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::{ButtonMaterials, AppState};

pub struct HelpPlugin;

#[derive(Component)]
struct HelpElem;

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
fn spawn_help_menu(asset_server: Res<AssetServer>,
                   windows: Res<Windows>,
                   mut cmds: Commands
) {
    let window = windows.get_primary().unwrap();
    let height = window.height();
    let width = window.width();

    // Help Text
    cmds.spawn_bundle(Text2dBundle {
       text: Text::with_section(
             "PrettySure is a trivia game where you place your bets upon \
              various answers to the question posed using your \"tokens.\"\n\n\
              A turn is played by using left click to drag your tokens, \
              located on the lefthand side of the screen, onto one of the four \
              answer boxes. The token will take on the color of the answer it \
              is on top of when properly placed. Once all five tokens have been \
              placed on top of an answer, a submit button will appear at the \
              bottom of the screen, allowing you to lock in your answer and see \
              the correct one highlighted.\n\nPlay through all ten questions, and \
              try to get as close as possible to the maximum score of 40 \
              points!",
            TextStyle {
                font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                font_size: 25.,
                color: Color::BLACK,
            },
            TextAlignment {
                vertical: VerticalAlign::Bottom,
                horizontal: HorizontalAlign::Center,
            },
        ),
        text_2d_bounds: Text2dBounds {
            size: Size::new(width - 50., height / 2.),
        },
        ..Default::default()
    })
    .insert(HelpElem);

    // Licensing Text
    cmds.spawn_bundle(Text2dBundle {
       text: Text::with_section(
             "All questions provided by OpenTDB under the Creative Commons Sharealike \
             License, 4.0 \n PublicSans font provided under the SIL Open Font License, \
             1.1.\n This work was produced using the Bevy game engine, and is licensed \
             under the GNU General Public License, v3.\nFurther information and the \
             full license text can be found at \
             https://github.com/WillyWoosley/PrettySure",
            TextStyle {
                font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                font_size: 15.,
                color: Color::BLACK,
            },
            TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Center,
            },
        ),
        transform: Transform {
            translation: Vec3::new(0., -120., 0.),
            ..Default::default()
        },
        text_2d_bounds: Text2dBounds {
            size: Size::new(width - 50., height / 2.),
        },
        ..Default::default()
    })
    .insert(HelpElem);

    // Back Button
    cmds.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(20.)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,

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
    .insert(HelpElem);
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
fn teardown_helpscreen(helpelem_query: Query<Entity, With<HelpElem>>,
                       mut cmds: Commands,
) {
    for help_id in helpelem_query.iter() {
        cmds.entity(help_id).despawn_recursive();
    }
}

