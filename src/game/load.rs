use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::text::Text2dBounds;
use serde::Deserialize;
use html_escape::decode_html_entities;
use rand::Rng;
use reqwest::Client;
use futures_lite::future;
use async_compat::Compat;

use std::time::Duration;

use crate::AppState;

pub struct LoadPlugin;

#[derive(Default, Component)]
pub struct Answer {
    pub text: String,
    pub truth: bool,
}
#[derive(Default, Component)]
pub struct Question {
    pub text: String,
    pub answers: [Answer; 4],
}
#[derive(Component)]
struct LoadBar;
#[derive(Component)]
struct LoadText {
    timer: Timer,
    dots: u8,
}
#[derive(Component)]
struct ErrorCard;

#[derive(Default)]
pub struct Rounds {
    pub round_number: usize,
    pub round_max: usize,
    pub questions: Vec<Question>,
}
#[derive(Default)]
pub struct SessionId {
    id: Option<String>,
}
#[derive(Default)]
struct SiteData {
    session_id: SessionId,
    rounds: Rounds,
}

struct GetError; 

#[derive(Deserialize)]
struct ResponseCode(u8);
#[derive(Deserialize)]
#[allow(dead_code)]
struct ApiQuestion {
    category: String,
    r#type: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}
#[derive(Deserialize)]
#[allow(dead_code)]
struct ApiQResponse {
    response_code: ResponseCode,
    results: Vec<ApiQuestion>,
}
#[derive(Deserialize)]
#[allow(dead_code)]
struct ApiIdResponse {
    response_code: ResponseCode,
    response_message: String,
    token: String,
}

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SessionId {id: None})
           .add_event::<GetError>()
           .add_system_set(
               SystemSet::on_enter(AppState::Load).with_system(spawn_load_task)
                                                  .with_system(spawn_loadscreen))
           .add_system_set(
               SystemSet::on_update(AppState::Load).with_system(insert_trivia)
                                                   .with_system(update_loadscreen)
                                                   .with_system(spawn_errorcard)
                                                   .with_system(error_to_menu))
           .add_system_set(
               SystemSet::on_exit(AppState::Load).with_system(teardown_loadscreen));
    }
}

// Spawns an Async call to retrieve trivia data
fn spawn_load_task(thread_pool: Res<AsyncComputeTaskPool>,
                    session_id: Res<SessionId>,
                    mut cmds: Commands,
) {
    let id = session_id.id.clone();
    let trivia_get = thread_pool.spawn(async move {
        let site_data = Compat::new(async {
            retrieve_questions(id).await
        }).await;

        site_data
    });

    cmds.spawn().insert(trivia_get);
}

// Spawns some basic loading text
fn spawn_loadscreen(asset_server: Res<AssetServer>, mut cmds: Commands) {
    cmds.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                String::from("Loading. . ."),
                TextStyle {
                    font: asset_server.load("fonts/PublicSans-Medium.ttf"),
                    font_size: 64.,
                    color: Color::BLACK,
                },
                Default::default(),
            ),
            ..Default::default()
        }).insert(LoadText {
            timer: Timer::from_seconds(0.5, true),
            dots: 3,
        });
    }).insert(LoadBar);
}

// Awaits completion of HTTP requests and inserts Rounds (and potentially a SessionId
// when done
fn insert_trivia(mut question_task: Query<(Entity, &mut Task<Result<SiteData, ()>>)>,
                 mut error_writer: EventWriter<GetError>,
                 mut session_id: ResMut<SessionId>,
                 mut appstate: ResMut<State<AppState>>,
                 mut cmds: Commands,
) {
    for (entity, mut task) in question_task.iter_mut() {
        if let Some(site_res) = future::block_on(future::poll_once(&mut *task)) {
            match site_res {
                // Site data successfully retrieved
                Ok(site_data) => {
                    // Set SessionId if previously unset
                    if session_id.id.is_none() {
                        session_id.id = site_data.session_id.id;
                    }

                    // Insert Rounds and finish AppState::Load
                    cmds.insert_resource(site_data.rounds);
                    appstate.set(AppState::Game).unwrap();
                },
                // Something went wrong along the way
                Err(_) => {
                    error_writer.send(GetError);
                },
            }

            cmds.entity(entity).remove::<Task<Result<SiteData, ()>>>();
        }
    }
}

// Simply animates the loading text
fn update_loadscreen(mut load_query: Query<(&mut LoadText, &mut Text)>,
                     time: Res<Time>,
) {
    for (mut load, mut text) in load_query.iter_mut() {
        if load.timer.tick(time.delta()).just_finished() {
            text.sections[0].value = match load.dots {
                0 => String::from("Loading.    "),
                1 => String::from("Loading. .  "),
                2 => String::from("Loading. . ."),
                _ => String::from("Loading     "),
            };
            load.dots = (load.dots + 1) % 4;
        }
    }
}

// Spawns a simple error card when an error is encountered trying during HTTP question 
// retrieval
fn spawn_errorcard(load_query: Query<Entity, With<LoadBar>>,
                   mut error_reader: EventReader<GetError>, 
                   asset_server: Res<AssetServer>,
                   windows: Res<Windows>,
                   mut cmds: Commands,
) {
    if error_reader.iter().next().is_some() {
        let window = windows.get_primary().unwrap();
        let x_dim = window.width() / 2.;
        let y_dim = window.height() / 2.;
        
        let load_id = load_query.single();
        cmds.entity(load_id).despawn_recursive();

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
                            value: String::from("An error occured while retrieving \
                                                your trivia questions. Please check \
                                                your internet connection and try \
                                                again.\n\n"),
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
                text_2d_bounds: Text2dBounds {
                    size: Size::new(x_dim, y_dim),
                },
                ..Default::default()
            });
        }).insert(ErrorCard);
    }
}

// Returns to Menu on a left click after an HTTP error occurs
fn error_to_menu(errorcard_query: Query<&ErrorCard>,
                 mouse_button: Res<Input<MouseButton>>,
                 mut appstate: ResMut<State<AppState>>,
) {
    if errorcard_query.iter().next().is_some() {
        if mouse_button.just_pressed(MouseButton::Left) {
            appstate.set(AppState::Menu).unwrap();
        }
    }
}

// Tears down loading text and errorcard, if either exists
fn teardown_loadscreen(loadbar_query: Query<Entity, With<LoadBar>>,
                       errorcard_query: Query<Entity, With<ErrorCard>>,
                       mut cmds: Commands,
) {
    for loadbar_id in loadbar_query.iter() {
        cmds.entity(loadbar_id).despawn_recursive();
    }

    for errorcard_id in errorcard_query.iter() {
        cmds.entity(errorcard_id).despawn_recursive();
    }
}

// Async function that handles HTTP queries to OpenTDB
async fn retrieve_questions(session_id: Option<String>) -> Result<SiteData, ()> {
    let client = match Client::builder().timeout(Duration::from_secs(20)).build() {
        Ok(client) => client,
        Err(_) => return Err(()),
    };
    let mut site_data = SiteData::default();

    // Retrieve and set a SessionId if not already set
    if session_id.is_none() {
        let session_res = match client.get(
            "https://opentdb.com/api_token.php?command=request"
        ).send().await {
            Ok(response) => response,
            Err(_) => return Err(()),
        };
        
        let api_res = match session_res.json::<ApiIdResponse>().await {
            Ok(parsed) => parsed,
            Err(_) => return Err(()),
        };

        site_data.session_id = SessionId {
            id: Some(api_res.token)
        };
    } else {
        site_data.session_id.id = session_id;
    }
   
    // Retrieve trivia questions
    let res = match client.get(
        format!("https://opentdb.com/api.php?amount=8&type=multiple&token={}",
                site_data.session_id.id.as_ref().unwrap())
    ).send().await {
        Ok(response) => response,
        Err(_) => return Err(()),
    };

    let api_res = match res.json::<ApiQResponse>().await {
        Ok(parsed) => parsed,
        Err(_) => return Err(()),
    };
    
    // Format retrieved questions
    let mut questions = Vec::new();
    for api_q in api_res.results {
        // Creates a random ordering of retrieved answers
        let mut answers = [Answer::default(), Answer::default(), 
                           Answer::default(), Answer::default()];
        let t_ind = rand::thread_rng().gen_range(0..4);
        let mut f_ind = 0;
        for (i, answer) in answers.iter_mut().enumerate() {
            if i == t_ind {
                answer.text = decode_html_entities(&api_q.correct_answer)
                                      .to_string();
                answer.truth = true;
            } else {
                answer.text = decode_html_entities(&api_q.incorrect_answers[f_ind])
                                      .to_string();
                answer.truth = false;
                f_ind += 1;
            }
        }

        questions.push(
            Question {
                text: decode_html_entities(&api_q.question).to_string(),
                answers,
            }
        );
    }

    site_data.rounds = Rounds {
        round_number: 0,
        round_max: questions.len(),
        questions,
    };
    
    Ok(site_data)
}

