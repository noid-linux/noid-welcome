// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/get_software/package/mod.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct PackageObject(ObjectSubclass<imp::PackageObject>);
}

impl PackageObject {
    pub fn new(install: bool, pkgname: &str, short_desc: &str) -> Self {
        glib::Object::builder()
            .property("install", install)
            .property(
                "icon",
                format!("/com/ch-naseem/NoidWelcome/images/packages/{pkgname}.svg"),
            )
            .property("pkgname", pkgname)
            .property("short_desc", short_desc)
            .build()
    }
}
