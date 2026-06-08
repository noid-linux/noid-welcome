// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use super::*;
use crate::util::{autostart_file, scripts_dir};

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/ch-naseem/NoidWelcome/ui/window.ui")]
pub struct NoidWelcomeWindow {
    #[template_child]
    pub switch_autostart: TemplateChild<gtk::Switch>,

    #[template_child]
    pub button_system_update: TemplateChild<gtk::Button>,

    #[template_child]
    pub button_virt_manager: TemplateChild<gtk::Button>,

    #[template_child]
    pub button_oxidize_system: TemplateChild<gtk::Button>,
}

#[gtk::template_callbacks]
impl NoidWelcomeWindow {
    #[template_callback]
    fn on_switch_autostart_state_set(&self, state: bool) -> glib::Propagation {
        let autostart_file = autostart_file();

        if state {
            std::fs::copy(
                std::path::PathBuf::from("/usr/share/applications/noid-welcome.desktop"),
                &autostart_file,
            )
            .unwrap();
        } else {
            std::fs::remove_file(&autostart_file).unwrap();
        }

        glib::Propagation::Proceed
    }

    #[template_callback]
    fn on_button_system_update_clicked(&self) {
        let _ = std::process::Command::new(scripts_dir().join("system_update.sh"))
            .spawn()
            .expect("failed to update your system");
    }

    #[template_callback]
    fn on_button_virt_manager_clicked(&self) {
        let _ = std::process::Command::new(scripts_dir().join("virt_manager.sh"))
            .spawn()
            .expect("failed to update your system");
    }

    #[template_callback]
    fn on_button_oxidize_system_clicked(&self) {
        let _ = std::process::Command::new(scripts_dir().join("oxidize_system.sh"))
            .spawn()
            .expect("failed to update your system");
    }
}

#[glib::object_subclass]
impl ObjectSubclass for NoidWelcomeWindow {
    const NAME: &'static str = "NoidWelcomeWindow";
    type Type = super::NoidWelcomeWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for NoidWelcomeWindow {
    fn constructed(&self) {
        self.parent_constructed();

        // Handle autostart
        self.switch_autostart.set_active(autostart_file().exists());
    }
}

impl WidgetImpl for NoidWelcomeWindow {}
impl WindowImpl for NoidWelcomeWindow {}
impl ApplicationWindowImpl for NoidWelcomeWindow {}
