use reqwest::{Client, Error};

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

pub async fn get_completion(prompt: &str) -> Result<(), Error> {
    let client = Client::new();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        panic!("OPENAI_API_KEY must be set"); // TODO handle this gracefully
    });

    let payload = serde_json::json!({
        "model": "gpt-3.5-turbo", // TODO refactor to variable
        "messages": [
            { "role": "user", "content": prompt },
        ],
    }).to_string();

    let response = client
        .post(OPENAI_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(payload)
        .send()
        .await?;
    let body = response.text().await?;
    let completion: serde_json::Value = serde_json::from_str(&body)
        .expect("Failed to parse response from OpenAI API"); // TODO handle gracefully
    println!("{:?}", completion["choices"][0]["message"]["content"].as_str().unwrap_or(""));
    Ok(())
}
