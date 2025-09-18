use gtk::{glib, Align};
use gtk::prelude::{Cast, WidgetExt};

fn set_time(label: &gtk::Label) {
    let now = chrono::Local::now();
    label.set_text(&now.format("%a %d %b %H:%M").to_string());
}

pub fn widget() -> gtk::Widget {
    let label = gtk::Label::new(None);
    label.add_css_class("clock-label");
    label.set_halign(Align::End);

    set_time(&label);


    glib::timeout_add_seconds_local(
        1,
    glib::clone!(@weak label => @default-return glib::ControlFlow::Break, move || {
            set_time(&label);
            glib::ControlFlow::Continue
        })
    );


    label.upcast()
}