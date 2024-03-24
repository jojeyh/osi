use gtk::gdk::Key;
use gtk::glib::Propagation;
use gtk::{prelude::*, Align, EventControllerKey, Label, TextView, WrapMode};
use gtk::{Box, Orientation};

const WIDGET_MARGIN: i32 = 10;

pub struct Line {
    pub widget: Box,
}

impl Line {
    pub fn new() -> Self {
        let event_controller = EventControllerKey::new();
        event_controller.connect_key_pressed(|_, key, _, _| {
            match key {
                Key::Return => {
                    println!("Enter");
                    Propagation::Stop
                },
                _ => Propagation::Proceed,
            }
        });

        let current_dir = std::env::current_dir()
            .expect("Failed to get current directory"); // TODO handle error
        let current_dir_str = current_dir.to_str().unwrap();
        let cmdline_prompt = format!("{}> ", current_dir_str);
        let label = Label::new(Some(&cmdline_prompt));
        label.set_valign(Align::Start);

        let widget = Box::new(Orientation::Horizontal, 0);
        widget.set_margin_bottom(WIDGET_MARGIN);
        widget.set_margin_top(WIDGET_MARGIN);
        widget.set_margin_start(WIDGET_MARGIN + 10);
        widget.set_margin_end(WIDGET_MARGIN);
        widget.set_hexpand(true);

        let entry = TextView::new();
        entry.add_controller(event_controller);
        entry.set_hexpand(true);
        entry.set_wrap_mode(WrapMode::WordChar);

        widget.append(&label);
        widget.append(&entry);

        Self {
            widget,
        }
    }
}