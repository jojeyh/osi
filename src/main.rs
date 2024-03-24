mod message;
mod network;
mod recorder;
mod utils;
mod components;

use gtk::glib::Propagation;
use gtk::{prelude::*, EventControllerKey};
use gtk::gdk::{Display, Key};
use gtk::{CssProvider, Label, Orientation, PolicyType, ScrolledWindow};
use gtk::{glib, Box, Application, ApplicationWindow};

use components::line::Line;

const APP_ID: &str = "org.nemea.osi";
const LINE_MARGIN: i32 = 10;

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
    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.set_hexpand(true);
    vbox.set_vexpand(true);

    // Add a spacer to push the command line to the bottom
    let spacer = Label::new(None);
    spacer.set_vexpand(true);
    vbox.append(&spacer); 

    let line = Line::new();

    vbox.append(&line.widget);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&vbox)
        .build();
    scrolled_window.set_hscrollbar_policy(PolicyType::Never);
    scrolled_window.set_vexpand(true);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("osi")
        .child(&scrolled_window)
        .build();
    window.set_default_size(1000, 600);

    window.present();
}