use gtk::{glib, Align};
use gtk::prelude::{Cast, WidgetExt};

pub fn widget() -> gtk::Widget {
    let label = gtk::Label::new(None);
    label.add_css_class("clock-label");
    label.set_halign(Align::End);
    let label_clone = label.clone();

    let update_time = move || {
        let now = chrono::Local::now();
        label_clone.set_text(&now.format("%a %d %b %H:%M:%S").to_string());
        label_clone.set_halign(Align::End);
        glib::ControlFlow::Continue
    };

    update_time();

    glib::timeout_add_seconds_local(
        1,
        update_time
    );


    label.upcast()
}