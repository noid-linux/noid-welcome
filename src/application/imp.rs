// SPDX-License-Identifier: GPL-3.0-or-later
/* src/application/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use crate::window::NoidWelcomeWindow;

use super::*;

#[derive(Debug, Default)]
pub struct NoidWelcomeApplication {}

#[glib::object_subclass]
impl ObjectSubclass for NoidWelcomeApplication {
    const NAME: &'static str = "NoidWelcomeApplication";
    type Type = super::NoidWelcomeApplication;
    type ParentType = gtk::Application;
}

impl ObjectImpl for NoidWelcomeApplication {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_accels_for_action("app.quit", &["<primary>q"]);
    }
}

impl ApplicationImpl for NoidWelcomeApplication {
    fn activate(&self) {
        let application = self.obj();

        let window = application.active_window().unwrap_or_else(|| {
            let window = NoidWelcomeWindow::new(&*application);
            window.upcast()
        });

        window.present();
    }
}

impl GtkApplicationImpl for NoidWelcomeApplication {}
