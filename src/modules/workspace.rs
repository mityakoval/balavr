use std::path::Path;
use adw::prelude::Cast;
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use hyprland::data::{Workspace, Workspaces};
use hyprland::dispatch;
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use hyprland::prelude::*;

pub fn widget() -> Option<gtk::Widget> {
    if !hypr_ipc_available() {
        eprintln!("Hyprland IPC unavailable (no socket) — hiding workspace widget");
        return None;
    }

    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);

    if let Ok(active_workspace) = Workspace::get_active() {
        let workspaces = Workspaces::get()
            .unwrap();

        for (i, workspace) in workspaces.into_iter().enumerate() {
            let btn = gtk::Button::with_label(&(i + 1).to_string());

            btn.add_css_class("workspace-btn");

            if workspace.id == active_workspace.id {
                btn.add_css_class("active");
            }

            btn.connect_clicked(move |_| {
                let result = dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(workspace.id));
                match result {
                    Ok(_) => {}
                    Err(err) => { println!("{}", err); }
                }
            });

            row.append(&btn)
        }

        Some(row.upcast())
    } else {
        None
    }
}

fn hypr_ipc_available() -> bool {
    let his = match std::env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(v) if !v.is_empty() => v,
        _ => return false,
    };
    let xdg = match std::env::var("XDG_RUNTIME_DIR") {
        Ok(v) if !v.is_empty() => v,
        _ => return false,
    };
    Path::new(&format!("{}/hypr/{}/.socket.sock", xdg, his)).exists()
}