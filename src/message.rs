use gtk::prelude::*;
use gtk::TextView;

pub struct Message {
    pub text_view: TextView,
}

impl Message {
    pub fn new(text: &str) -> Self {
        let text_view = TextView::new();
        text_view.set_editable(false);
        text_view.set_cursor_visible(false);
        let buffer = text_view.buffer();
        buffer.set_text(text);

        // Add a CSS class to the TextView
        text_view.add_css_class("message");
        text_view.add_css_class("elevated");

        // Add the CssProvider to the TextView
        Self { text_view }
    }
}