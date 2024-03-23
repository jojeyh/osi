mod message;
mod network;
mod audio;

use gtk::gdk::Display;
use gtk::glib::clone;
use gtk::{prelude::*, CssProvider, Orientation, ScrolledWindow};
use gtk::{glib, Box, Application, ApplicationWindow, Button, Entry};

use ndarray::{arr0, Array, Array0, Array1, Array2, Array3};
use network::{get_completion, get_transcription};
use audio::record;
use message::Message;
use ort::{inputs, GraphOptimizationLevel, Session, Value};

const APP_ID: &str = "org.nemea.osi";

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
		// Currently not using hte command entry
        command_entry.set_text("");
        tokio::spawn(async move {
            let audio_data = record().await.unwrap_or_else(|_| {
                panic!("Failed to record audio");
            });
            let transcription = get_transcription(audio_data)
                .await.unwrap_or("".to_string());
            let completion = get_completion(&transcription).await;
            let output = std::process::Command::new("bash")
                .arg("-c")
                .arg(completion)
                .output()
                .expect("Failed to execute command");
            let output_str = String::from_utf8(output.stdout).unwrap();
            println!("{}", output_str);
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
