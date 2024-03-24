mod message;
mod network;
mod recorder;
mod utils;
mod components;

use std::process::Command;

use gtk::glib::spawn_future_local;
use gtk::{prelude::*, Align};
use gtk::gdk::Display;
use gtk::{CssProvider, Label, Orientation, PolicyType, ScrolledWindow};
use gtk::{glib, Box, Application, ApplicationWindow};
use gtk::glib::clone;

use components::line::Line;

use crate::network::get_completion;

const APP_ID: &str = "org.nemea.osi";

fn main() -> glib::ExitCode {
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
    let (sender, receiver) = async_channel::bounded(1);

    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.set_hexpand(true);
    vbox.set_vexpand(true);

    // Add a spacer to push the command line to the bottom
    let spacer = Label::new(None);
    spacer.set_vexpand(true);
    vbox.append(&spacer); 

    // First cmdline prompt
    let sender_clone = sender.clone();
    let line = Line::new(sender_clone);
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

    spawn_future_local(clone!(@weak vbox => async move {
        while let Ok(s) = receiver.recv().await {
            println!("Received: {}", s);
            let line = Line::new(sender.clone());
            let output = Command::new("bash")
                .arg("-c")
                .arg(&s)
                .output()
                .expect("Failed to execute command");
            let output_str = std::str::from_utf8(&output.stdout).unwrap();
            let output_box = Box::new(Orientation::Horizontal, 0);
            output_box.set_margin_bottom(10);
            output_box.set_margin_top(10);
            output_box.set_margin_start(10);
            output_box.set_margin_end(10);
            output_box.set_halign(Align::Start);
            let label = Label::new(Some(output_str));
            output_box.append(&label);
            vbox.append(&output_box);
            vbox.append(&line.widget);
        }
    }));
}