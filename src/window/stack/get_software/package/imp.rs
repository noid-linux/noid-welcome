// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/get_software/package/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use std::cell::{Cell, RefCell};

use super::*;

#[derive(Debug, Default, glib::Properties)]
#[properties(wrapper_type = super::PackageObject)]
pub struct PackageObject {
    #[property(get, set)]
    install: Cell<bool>,

    #[property(get, set)]
    icon: RefCell<String>,

    #[property(get, set)]
    pkgname: RefCell<String>,

    #[property(get, set)]
    short_desc: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for PackageObject {
    const NAME: &'static str = "PackageObject";
    type Type = super::PackageObject;
}

#[glib::derived_properties]
impl ObjectImpl for PackageObject {
    fn constructed(&self) {
        self.parent_constructed();
    }
}
