use bevy::prelude::*;
use serde::Deserialize;

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
struct SessionId {
    id: Option<String>,
}

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Load).with_system(insert_trivia));
    }
}

// Pulls trivia questions from OpenTDB over HTTP and inserts them as the 'Rounds'
// resource
//
// TODO: Better error handling
fn insert_trivia(mut cmds: Commands,
                 mut appstate: ResMut<State<AppState>>,
                 mut session_id: Local<SessionId>,
) {
    let client = reqwest::blocking::Client::new();

    // Retrieve and set a SessionId if not already set
    if session_id.id.is_none() {
        let session_res = match client.get(
            "https://opentdb.com/api_token.php?command=request"
        ).send() {
            Ok(response) => response,
            Err(_) => return,
        };
        
        let api_res = match session_res.json::<ApiIdResponse>() {
            Ok(parsed) => parsed,
            Err(_) => return,
        };

        session_id.id = Some(api_res.token);
    }

    // Retrieve trivia questions
    let res = match client.get(
        format!("https://opentdb.com/api.php?amount=2&type=multiple&token={}", 
                session_id.id.as_ref().unwrap()
        ),
    ).send() {
        Ok(response) => response,
        Err(_) => return,
    };

    let api_res = match res.json::<ApiQResponse>() {
        Ok(parsed) => parsed,
        Err(_) => return,
    };
    
    // Format retrieved questions
    let mut questions = Vec::new();
    for api_q in api_res.results {
        // TODO: Need to randomize how Answer are inserted, and also maybe some sort
        //       of panic in (I think not possible?) change that we receive anything
        //       other than four answers
        let answers = [
            Answer {
                text: api_q.correct_answer,
                truth: true,
            },
            Answer {
                text: api_q.incorrect_answers[0].clone(),
                truth: false,
            },
            Answer {
                text: api_q.incorrect_answers[1].clone(),
                truth: false,
            },
            Answer {
                text: api_q.incorrect_answers[2].clone(),
                truth: false,
            },
        ];

        questions.push(
            Question {
                text: api_q.question,
                answers,
            }
        );
    }

    // Actually insert the trivia questions and move to next State
    cmds.insert_resource(Rounds {
        round_number: 0,
        round_max: questions.len(),
        questions
    });

    appstate.set(AppState::Game).unwrap();
}

