// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/mod.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

pub use stack::{log::StackPageLog, main::StackPageMain};

mod imp;
mod stack;

glib::wrapper! {
    pub struct NoidWelcomeWindow(ObjectSubclass<imp::NoidWelcomeWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::ConstraintTarget, gtk::Buildable, gtk::Accessible, gtk::ShortcutManager, gtk::Root, gtk::Native;
}

impl NoidWelcomeWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
