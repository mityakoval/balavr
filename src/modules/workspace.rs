use adw::prelude::Cast;
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use hyprland::data::{Workspace, Workspaces};
use hyprland::dispatch;
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use hyprland::prelude::*;

pub fn widget() -> gtk::Widget {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);

    let active_workspace = Workspace::get_active().unwrap();

    let workspaces = Workspaces::get()
        .unwrap();

    for (i, workspace) in workspaces.into_iter().enumerate() {
        let btn = gtk::Button::with_label(&i.to_string());

        btn.add_css_class("workspace-btn");

        if (workspace.id == active_workspace.id) {
            btn.add_css_class("active");
        }

        btn.connect_clicked(move |_| {
            let result = dispatch!(MoveToWorkspace, WorkspaceIdentifierWithSpecial::Id(workspace.id), None);
            match result {
                Ok(_) => {}
                Err(err) => { println!("{}", err); }
            }
        });

        row.append(&btn)
    }

    row.upcast()
}