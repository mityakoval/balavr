use std::fs::File;
use adw::prelude::*;
use gtk::{gdk, style_context_add_provider_for_display, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk4_layer_shell as ls;
use gtk4_layer_shell::{Edge, KeyboardMode, LayerShell};
use gtk::SystemSetting::Display;
use crate::modules;
pub fn create_for_monitor(app: &adw::Application, monitor: &gdk::Monitor) -> adw::ApplicationWindow {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .default_width(monitor.width_mm())
        .title("Balavr")
        .resizable(false)
        .decorated(false)
        .build();

    window.init_layer_shell();
    window.set_size_request(monitor.geometry().width(), 10);
    window.set_layer(ls::Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, false);
    window.set_keyboard_mode(KeyboardMode::None);
    window.set_exclusive_zone(10);
    window.set_monitor(Some(monitor));

    let overlay = adw::ToastOverlay::new();
    let root = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    root.add_css_class("balavr-root");
    // root.set_margin_bottom(6);
    // root.set_margin_top(6);
    root.set_margin_start(6);
    root.set_margin_end(6);


    // Sections
    let left = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let center = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let right = gtk::Box::new(gtk::Orientation::Horizontal, 2);

    center.set_hexpand(true);
    center.set_halign(gtk::Align::Center);

    //Modules
    if let Some(workspaces_widget) = modules::workspace::widget() {
        left.append(&workspaces_widget)
    }
    right.append(&modules::clock::widget());

    root.append(&left);
    root.append(&gtk::Separator::new(gtk::Orientation::Vertical));
    root.append(&center);
    root.append(&gtk::Separator::new(gtk::Orientation::Vertical));
    root.append(&right);

    overlay.set_child(Some(&root));
    window.set_content(Some(&overlay));
    window
}

pub fn load_css() {
    let style_css = include_str!("style.css");
    let provider = CssProvider::new();
    provider.load_from_data(style_css);
    style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    )
}