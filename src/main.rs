/* main.rs
 *
 * Copyright 2026 Naz <ndpm13@ch-naseem.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::env::home_dir;
use std::path::PathBuf;

use gtk::{glib, Application};
use gtk::{prelude::*, ApplicationWindow};

const APP_ID: &str = "com.ch-naseem.NoidWelcome";

fn main() -> glib::ExitCode {
    // FIXME: this breaks outside of Cargo
    let resources = gio::Resource::load(format!(
        "{}/data/resources/resources.gresource",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    ))
    .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let builder = gtk::Builder::from_resource("/com/ch-naseem/NoidWelcome/ui/window.ui");

    // Create a window and set the title
    let window: ApplicationWindow = builder
        .object("window")
        .expect("Could not get window object");

    window.set_application(Some(app));

    // Autostart
    let switch_autostart: gtk::Switch = builder.object("autostart").unwrap();
    handle_autostart(switch_autostart);

    // System tweaks
    let scripts_dir = PathBuf::from(std::env!("SCRIPTS_DIR"));

    let btn_system_update: gtk::Button = builder.object("system_update").unwrap();
    handle_system_update(btn_system_update, scripts_dir.clone());

    let btn_virt_manager: gtk::Button = builder.object("virt_manager").unwrap();
    handle_virt_manager(btn_virt_manager, scripts_dir.clone());

    let btn_oxidize_system: gtk::Button = builder.object("oxidize_system").unwrap();
    handle_oxidize_system(btn_oxidize_system, scripts_dir.clone());

    // Present window
    window.present();
}

fn handle_autostart(s: gtk::Switch) {
    if let Some(home_dir) = home_dir() {
        let autostart_file = home_dir
            .join(".config")
            .join("autostart")
            .join("noid-welcome.desktop");

        s.set_active(autostart_file.exists());

        s.connect_state_set(move |_, state| {
            if state {
                std::fs::copy(
                    PathBuf::from("/usr/share/applications/noid-welcome.desktop"),
                    &autostart_file,
                )
                .unwrap();
            } else {
                std::fs::remove_file(&autostart_file).unwrap();
            }
            glib::Propagation::Proceed
        });
    }
}

fn handle_system_update(btn: gtk::Button, scripts_dir: PathBuf) {
    btn.connect_clicked(move |_| {
        let _ = std::process::Command::new(scripts_dir.join("system_update.sh"))
            .spawn()
            .expect("failed to update your system")
            .wait();
    });
}

fn handle_virt_manager(btn: gtk::Button, scripts_dir: PathBuf) {
    btn.connect_clicked(move |_| {
        let _ = std::process::Command::new(scripts_dir.join("virt_manager.sh"))
            .spawn()
            .expect("failed to update your system")
            .wait();
    });
}

fn handle_oxidize_system(btn: gtk::Button, scripts_dir: PathBuf) {
    btn.connect_clicked(move |_| {
        let _ = std::process::Command::new(scripts_dir.join("oxidize_system.sh"))
            .spawn()
            .expect("failed to update your system")
            .wait();
    });
}
