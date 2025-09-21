use adw::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, glib};

use crate::ui;

pub fn run() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id("dev.mityakoval.balavr")

        .build();

    app.connect_activate(build);
    app.run()
}

fn build(app: &adw::Application) {
    let display = gdk::Display::default().expect("error initializing display");

    let monitors = display.monitors();

    for i in 0..monitors.n_items() {
        if let Some(obj) = monitors.item(i) {
            if let Ok(monitor) = obj.downcast::<gdk::Monitor>() {
                let win = ui::bar::create_for_monitor(app, &monitor);
                win.present()
            }
        }
    }
}
