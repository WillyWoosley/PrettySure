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
struct ApiResponse {
    response_code: ResponseCode,
    results: Vec<ApiQuestion>,
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
fn insert_trivia(mut cmds: Commands, mut appstate: ResMut<State<AppState>>) {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://opentdb.com/api.php?amount=2&type=multiple");

    let res = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => return,
    };

    let api_res = match res.json::<ApiResponse>() {
        Ok(parsed) => parsed,
        Err(_) => return,
    };
    
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

    cmds.insert_resource(Rounds {
        round_number: 0,
        round_max: questions.len(),
        questions
    });

    appstate.set(AppState::Game).unwrap();
}

