use std::path::Path;
use adw::prelude::Cast;
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use hyprland::data::{Workspace, Workspaces};
use hyprland::dispatch;
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use hyprland::prelude::*;
use hyprland::shared::WorkspaceId;

pub struct WorkspacesWidget {
    buttons: Vec<WorkspaceBtn>,
    pub widget: gtk::Widget
}

impl WorkspacesWidget {
    pub fn new(buttons: Vec<>) -> Self {
        buttons
    }
}

struct WorkspaceBtn {
    workspace_id: WorkspaceId,
    btn: gtk::Button,
    active: bool
}

impl WorkspaceBtn {
    pub fn new(gtk_button: gtk::Button, workspace_id: WorkspaceId) -> Self {
        Self {
            workspace_id,
            btn: gtk_button,
            active: false
        }
    }
}

pub fn widget() -> Option<WorkspacesWidget> {
    if !hypr_ipc_available() {
        eprintln!("Hyprland IPC unavailable (no socket) — hiding workspace widget");
        return None;
    }

    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);

    if let Ok(active_workspace) = Workspace::get_active() {
        let workspaces = Workspaces::get()
            .unwrap();

        let mut buttons: Vec<WorkspaceBtn> = Vec::new();

        for (i, workspace) in workspaces.into_iter().enumerate() {
            let btn = gtk::Button::with_label(&(i + 1).to_string());

            let mut workspace_btn = WorkspaceBtn::new(btn, workspace.id);

            let btn_clone = workspace_btn.btn.clone();

            workspace_btn.btn.add_css_class("workspace-btn");

            if workspace.id == active_workspace.id {
                workspace_btn.btn.add_css_class("active");
                workspace_btn.active = true;
            }

            workspace_btn.btn.connect_clicked(move |_| {
                let result = dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(workspace.id));
                btn_clone.add_css_class("active");

                match result {
                    Ok(_) => {}
                    Err(err) => { println!("{}", err); }
                }
            });

            row.append(&workspace_btn.btn);

            buttons.push(workspace_btn);
        }

        Some(
            WorkspacesWidget {
                buttons,
                widget: row.upcast()
            }
        )
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