mod message;

use gtk::gdk::Display;
use gtk::glib::clone;
use gtk::{prelude::*, CssProvider, Orientation, ScrolledWindow};
use gtk::{glib, Box, Application, ApplicationWindow, Button, Entry};
use reqwest::Error;

use message::Message;

const APP_ID: &str = "org.nemea.osi";
const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

#[tokio::main]
async fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("src/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display"), 
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

async fn get_completion(prompt: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();
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

fn build_ui(app: &Application) {
    let scrolled_window = ScrolledWindow::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

	let command_entry = Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    command_entry.set_placeholder_text(Some("Enter command..."));

    let command_button = Button::builder()
        .label("Command")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let message = Message::new("Arrakis was made to train the faithful");
    scrolled_window.set_child(Some(&message.text_view));
    scrolled_window.set_size_request(600, 800);

    command_button.connect_clicked(clone!(@strong command_entry => move |_| {
        let prompt = command_entry.clone().text().to_string();
        command_entry.set_text("");
        tokio::spawn(async move {
            let _ = get_completion(&prompt).await;
        });
    }));

    let command_box = Box::new(Orientation::Horizontal, 1);
    command_box.append(&command_entry);
    command_box.append(&command_button);

    let vbox = Box::new(Orientation::Vertical, 1);
    vbox.append(&scrolled_window);
    vbox.append(&command_box);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("osi")
        .child(&vbox)
        .build();

    window.present();
}
