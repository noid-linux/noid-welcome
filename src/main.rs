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

    // Present window
    window.present();
}
