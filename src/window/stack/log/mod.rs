// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/log/mod.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use gio::glib::closure_local;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct StackPageLog(ObjectSubclass<imp::StackPageLog>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::ConstraintTarget, gtk::Buildable, gtk::Accessible;
}

impl StackPageLog {
    pub fn connect_return<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "return",
            true,
            closure_local!(|obj: Self| {
                f(&obj);
            }),
        )
    }
}
