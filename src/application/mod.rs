// SPDX-License-Identifier: GPL-3.0-or-later
/* src/application/mod.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

mod imp;

glib::wrapper! {
    pub struct NoidWelcomeApplication(ObjectSubclass<imp::NoidWelcomeApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl NoidWelcomeApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/com/ch-naseem/NoidWelcome")
            .build()
    }
}
