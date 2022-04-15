use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use serde::Deserialize;
use html_escape::decode_html_entities;
use rand::Rng;
use reqwest::Client;
use futures_lite::future;
use async_compat::Compat;

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

#[derive(Default)]
pub struct Rounds {
    pub round_number: usize,
    pub round_max: usize,
    pub questions: Vec<Question>,
}

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

#[derive(Default)]
pub struct SessionId {
    id: Option<String>,
}

#[derive(Default)]
struct SiteData {
    session_id: SessionId,
    rounds: Rounds,
}

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SessionId {id: None})
           .add_system_set(
               SystemSet::on_enter(AppState::Load).with_system(spawn_trivia_get))
           .add_system_set(
               SystemSet::on_update(AppState::Load).with_system(insert_trivia));
    }
}

// Spawns an Async call to retrieve trivia data
fn spawn_trivia_get(thread_pool: Res<AsyncComputeTaskPool>,
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

// Awaits completion of HTTP requests and inserts Rounds (and potentially a SessionId
// when done
fn insert_trivia(mut question_task: Query<(Entity, &mut Task<SiteData>)>,
                 mut session_id: ResMut<SessionId>,
                 mut appstate: ResMut<State<AppState>>,
                 mut cmds: Commands,
) {
    for (entity, mut task) in question_task.iter_mut() {
        if let Some(site_data) = future::block_on(future::poll_once(&mut *task)) {
            // Set SessionId if previously unset
            if session_id.id.is_none() {
                session_id.id = site_data.session_id.id;
            }

            // Insert Rounds and finish AppState::Load
            cmds.insert_resource(site_data.rounds);
            cmds.entity(entity).remove::<Task<Rounds>>();
            appstate.set(AppState::Game).unwrap();
        }
    }
}

// Async function that handles HTTP queries to OpenTDB
//
// TODO: Better error handling
async fn retrieve_questions(session_id: Option<String>) -> SiteData {
    let client = Client::new();
    let mut site_data = SiteData::default();

    // Retrieve and set a SessionId if not already set
    if session_id.is_none() {
        let session_res = match client.get(
            "https://opentdb.com/api_token.php?command=request"
        ).send().await {
            Ok(response) => response,
            Err(_) => return SiteData::default(),
        };
        
        let api_res = match session_res.json::<ApiIdResponse>().await {
            Ok(parsed) => parsed,
            Err(_) => return SiteData::default(),
        };

        site_data.session_id = SessionId {
            id: Some(api_res.token)
        };
    } else {
        site_data.session_id.id = session_id;
    }
   
    // Retrieve trivia questions
    let res = match client.get(
        format!("https://opentdb.com/api.php?amount=2&type=multiple&token={}",
                site_data.session_id.id.as_ref().unwrap()))
        .send().await {
        Ok(response) => response,
        Err(_) => return SiteData::default(),
    };

    let api_res = match res.json::<ApiQResponse>().await {
        Ok(parsed) => parsed,
        Err(_) => return SiteData::default(),
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
    
    site_data
}

