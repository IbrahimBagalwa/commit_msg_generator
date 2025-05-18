use reqwest::Client;
use serde_json::json;
use serde::{ Deserialize, Serialize };

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}
#[derive(Deserialize)]
struct ResponseChoice {
    message: MessageContent,
}
#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    choices: Vec<ResponseChoice>,
}

pub async fn generator_commit_msg(diff: String) -> String {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let endpoint = std::env::var("OPENAI_API_ENDPOINT").expect("OPENAI_API_ENDPOINT not set");

    let client = Client::new();
    let messages = vec![
        Message {
            role: "system".into(),
            content: "You are an assistant that generates concise and descriptive commit messages based on git diff.".into(),
        },
        Message {
            role: "user".into(),
            content: format!("Generate a commit message for the following git diff:\n\n{}", diff),
        }
    ];

    let request_body =
        json!({
        "messages":messages,
        "temperature":0.7,
        "top_p":0.95,
        "frequency_penalty":0.0,
        "presence_penalty":0.0
    });

    let res = client
        .post(endpoint.clone())
        .header("api-key", api_key)
        .header("Content-type", "application/json")
        .json(&request_body)
        .send().await
        .expect(&format!("Request to {} failed", endpoint));

    if res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        panic!("{} Error: {} - {}", endpoint, status, text);
    }

    let response: ApiResponse = res.json().await.expect("Failed to parse the response");
    response.choices.first().expect("No response").message.content.clone()
}
