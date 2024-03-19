mod window;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

const APP_ID: &str = "org.gtk_rs.osi";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("osi.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
