use adw::prelude::*;
use gtk::prelude::*;
use gtk::gdk;
use gtk4_layer_shell as ls;
use gtk4_layer_shell::{Edge, KeyboardMode, LayerShell};

use crate::modules;
pub fn create_for_monitor(app: &adw::Application, monitor: &gdk::Monitor) -> adw::ApplicationWindow {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Balavr")
        .resizable(false)
        .decorated(false)
        .build();

    window.init_layer_shell();
    window.set_layer(ls::Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_keyboard_mode(KeyboardMode::None);
    window.set_exclusive_zone(36);
    window.set_monitor(Some(monitor));

    let overlay = adw::ToastOverlay::new();
    let root = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    root.add_css_class("balavr-root");
    root.set_margin_bottom(6);
    root.set_margin_top(6);
    root.set_margin_start(6);
    root.set_margin_end(6);


    // Sections
    let left = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    let center = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    let right = gtk::Box::new(gtk::Orientation::Horizontal, 6);

    center.set_hexpand(true);
    center.set_halign(gtk::Align::Center);

    //Modules
    left.append(&modules::workspace::widget());

    window
}