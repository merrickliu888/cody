#![allow(dead_code, unused)]

use std::env;

use once_cell::sync::Lazy;
use reqwest;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

static COHERE_API_KEY: Lazy<String> = Lazy::new(|| env::var("COHERE_API_KEY").unwrap());

#[derive(Serialize)]
pub struct GenerateBodyLocal {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

impl Default for GenerateBodyLocal {
    fn default() -> Self {
        GenerateBodyLocal {
            model: "cody".to_string(),
            prompt: "".to_string(),
            stream: false,
        }
    }
}

#[derive(Deserialize)]
pub struct GenerateResponseLocal {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
}

pub fn generate_response_local(prompt: String) -> Result<String, String> {
    let url = "http://localhost:11434/api/generate";
    let client = reqwest::blocking::Client::new();
    let body = GenerateBodyLocal {
        prompt,
        ..Default::default()
    };
    let auth_token = format!("Bearer: {}", COHERE_API_KEY.as_str());

    let res = client.post(url).json(&body).send().map_err(|err| err.to_string())?;
    if !res.status().is_success() {
        return Err(format!("Request Failed: {}", res.status()));
    }

    let llm_generated_shell_commands = res.json::<GenerateResponseLocal>().unwrap().response;

    return Ok(llm_generated_shell_commands);
}

#[derive(Serialize)]
pub struct GenerateBodyOnline {
    pub message: String,
    pub stream: bool,
    pub preamble: String,
    pub temperature: f64,
}

impl Default for GenerateBodyOnline {
    fn default() -> Self {
        GenerateBodyOnline {
            message: "".to_string(),
            stream: false,
            preamble: "You translate natural language to Unix shell commands. Only output the shell commands.
                        Here are some limitations:

                        - Variables values must be surrounded by double quotes.
                        - There is no variable expansion.
                        - In order to use a variable, you must do ${<variable_name>}
                        - Do everything in one line.
                        - Basic shell functionality only."
                .to_string(),
            temperature: 0.0,
        }
    }
}

#[derive(Deserialize)]
pub struct GenerateResponseOnline {
    pub text: String,
}

pub fn generate_response_online(prompt: String) -> Result<String, String> {
    let url = "https://api.cohere.com/v1/chat";
    let client = reqwest::blocking::Client::new();
    let body = GenerateBodyOnline {
        message: prompt,
        ..Default::default()
    };
    let auth_token = format!("Bearer: {}", COHERE_API_KEY.as_str());

    let res = client
        .post(url)
        .header(AUTHORIZATION, auth_token)
        .json(&body)
        .send()
        .map_err(|err| err.to_string())?;
    if !res.status().is_success() {
        return Err(format!("Request Failed: {}", res.status()));
    }

    let llm_generated_shell_commands = res.json::<GenerateResponseOnline>().unwrap().text;

    return Ok(llm_generated_shell_commands);
}
