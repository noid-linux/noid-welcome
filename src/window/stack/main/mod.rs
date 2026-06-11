// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/main/mod.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use gio::prelude::*;
use gtk::glib;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct StackPageMain(ObjectSubclass<imp::StackPageMain>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::ConstraintTarget, gtk::Buildable, gtk::Accessible;
}
