use reqwest::{multipart, Client, Error};

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_TRANSCRIBE: &str = "https://api.openai.com/v1/audio/transcriptions";

pub async fn get_completion(task: &str) -> String {
    let client = Client::new();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        panic!("OPENAI_API_KEY must be set"); // TODO handle this gracefully
    });

    let payload = serde_json::json!({
        "model": "gpt-4", // TODO refactor to variable
        "messages": [
            { "role":       "system", 
              "content":    "You are a helpful assistant 
                            that generates bash commands to accomplish 
                            the task below. ONLY return the
                            text of the bash command."
            },
            { "role": "user", "content": format!("Task: {}" ,task) },
        ],
    }).to_string();

    let response = client
        .post(OPENAI_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(payload)
        .send()
        .await.unwrap_or_else(
            |_| panic!("Failed to get response from OpenAI API") // TODO handle gracefully
        );

    let json_data = response.text().await.unwrap_or_else(
        |_| panic!("Failed to get response body from OpenAI API") // TODO handle gracefully
    );
    let completion: serde_json::Value = serde_json::from_str(&json_data)
        .unwrap_or_else(
            |_| panic!("Failed to parse response body from OpenAI API")
        );
    /*  
        TODO check for errors in completion, check if choices.0.message.content
        exists and handle appropaiately
    */

    /*
        TODO For now this method is faulty because the model
        is not trained specifically to generate bash commands 
        and fails on simple tasks, it refused to open a new terminal
        which is a simple "gnome-terminal" command
        It also does not return ONLY the bash command or does so only 
        sometimes.  Cannot force the output
     */
    println!("{}", completion);
    let text = completion["choices"][0]["message"]["content"]
        .as_str().unwrap_or("");
    text.to_string()
}

pub async fn get_transcription(audio_data: Vec<f32>) -> Result<String, Error> {
    println!("{:?}", audio_data.len());
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    /*
        TODO Annoying hack to write audio data to a file so it can be sent to OpenAI,
        not even using a Cursor works, not sure why at the moment.  This is a 
        temporary fix writing file to disk and reading it back in.
     */
    let mut writer = hound::WavWriter::create(".tmp.wav", spec)
        .expect("Failed to create audio writer.");

    for sample in audio_data {
        writer.write_sample(sample)
            .expect("Failed to write sample to audio file.");
    }

    writer.finalize()
        .expect("Failed to finalize audio file.");

    let wav_bytes = std::fs::read(".tmp.wav")
        .expect("Failed to read audio file.");

    let client = Client::new();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        panic!("OPENAI_API_KEY must be set"); // TODO handle this gracefully
    });

    let audio_part = multipart::Part::bytes(wav_bytes)
        .file_name("tmp.wav")
        .mime_str("audio/wav").unwrap();
    let form = multipart::Form::new()
        .text("model", "whisper-1")
        .part("file", audio_part);

    let response = client.post(OPENAI_TRANSCRIBE)
        .bearer_auth(api_key)
        .multipart(form)
        .send()
        .await?;

    let payload = response.text().await?;
    let payload_without_newlines = payload.replace("\n", "");

    // TODO these errors need to be handled better you fucking loser
    let json: serde_json::Value = serde_json::from_str(&payload_without_newlines)
        .unwrap_or_else(|_| panic!("Failed to parse response body from OpenAI API"));
    println!("{:?}", json);
    let transcription = json["text"].as_str()
        .expect("Expected text to be a string")
        .to_string();

    Ok(transcription)
}
