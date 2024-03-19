mod imp;

use glib::Object;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application};
use gtk::prelude::*;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder()
            .property("application", app)
            .build()
    }

    pub fn setup_callbacks(&self) {
        self.imp()
            .transcribe_btn
            .connect_clicked(|_| {
                eprintln!("Transcribe button clicked");
            });
    }
}