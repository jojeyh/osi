use std::sync::OnceLock;

use async_channel::Sender;
use gtk::gdk::Key;
use gtk::{prelude::*, Align, EventControllerKey, Label, TextView, WrapMode};
use gtk::{Box, Orientation};
use gtk::glib::{clone, Propagation};
use tokio::runtime::Runtime;

use crate::network::get_completion;

const WIDGET_MARGIN: i32 = 10;

pub struct Line {
    pub widget: Box,
}

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}

impl Line {
    pub fn new(sender: Sender<String>) -> Self {
        let current_dir = std::env::current_dir()
            .expect("Failed to get current directory"); // TODO handle error
        let current_dir_str = current_dir.to_str().unwrap();
        let cmdline_prompt = format!("{}> ", current_dir_str);
        let label = Label::new(Some(&cmdline_prompt));
        label.set_valign(Align::Start);

        let widget = Box::new(Orientation::Horizontal, 0);
        widget.set_margin_bottom(WIDGET_MARGIN);
        widget.set_margin_top(WIDGET_MARGIN);
        widget.set_margin_start(WIDGET_MARGIN + 5);
        widget.set_margin_end(WIDGET_MARGIN);
        widget.set_hexpand(true);

        let entry = TextView::new();
        entry.set_hexpand(true);
        entry.set_wrap_mode(WrapMode::WordChar);

        widget.append(&label);
        widget.append(&entry);

        let event_controller = EventControllerKey::new();
        event_controller.connect_key_pressed(clone!(@strong entry => move |_, key, _, _| {
            match key {
                Key::Return => {
                    let buffer = entry.buffer();
                    let start = buffer.start_iter();
                    let end = buffer.end_iter();
                    let text = buffer.text(&start, &end, false);

                    runtime().spawn(clone!(@strong sender => async move {
                        let completion = get_completion(&text).await;
                        sender.send(completion).await
                            .expect("Channel must be open.");
                    }));
                    entry.set_editable(false);
                    Propagation::Stop
                },
                _ => Propagation::Proceed,
            }
        }));
        entry.add_controller(event_controller);

        Self {
            widget,
        }
    }
}