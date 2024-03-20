use gtk::glib::clone;
use gtk::{prelude::*, Orientation};
use gtk::{glib, Box, Application, ApplicationWindow, Button, Entry};

const APP_ID: &str = "org.nemea.osi";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

// Generate build_ui function
fn build_ui(app: &Application) {
	let text_entry = Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    text_entry.set_placeholder_text(Some("Enter command..."));

    let command_button = Button::builder()
        .label("Command")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    command_button.connect_clicked(clone!(@weak text_entry => move |_| {
        println!("Command: {}", text_entry.text());
    }));

    let vbox = Box::new(Orientation::Vertical, 1);
    vbox.append(&text_entry);
    vbox.append(&command_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("osi")
        .child(&vbox)
        .build();

    window.present();
}
