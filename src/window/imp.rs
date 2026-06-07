// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use gio::glib::home_dir;

use super::*;
use crate::config::SCRIPTSDIR;

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

#[glib::object_subclass]
impl ObjectSubclass for NoidWelcomeWindow {
    const NAME: &'static str = "NoidWelcomeWindow";
    type Type = super::NoidWelcomeWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for NoidWelcomeWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let autostart_file = home_dir()
            .join(".config")
            .join("autostart")
            .join("noid-welcome.desktop");

        // Handle autostart
        let switch_autostart = &self.switch_autostart;

        switch_autostart.set_active(autostart_file.exists());

        switch_autostart.connect_state_set(move |_, state| {
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
        });

        // Handle system tweaks
        let scripts_dir = std::path::PathBuf::from(SCRIPTSDIR);

        let scripts_dir_clone = scripts_dir.clone();
        self.button_system_update.connect_clicked(move |_| {
            let _ = std::process::Command::new(scripts_dir_clone.join("system_update.sh"))
                .spawn()
                .expect("failed to update your system")
                .wait();
        });

        let scripts_dir_clone = scripts_dir.clone();
        self.button_virt_manager.connect_clicked(move |_| {
            let _ = std::process::Command::new(scripts_dir_clone.join("virt_manager.sh"))
                .spawn()
                .expect("failed to install virt-manager")
                .wait();
        });

        let scripts_dir_clone = scripts_dir.clone();
        self.button_oxidize_system.connect_clicked(move |_| {
            let _ = std::process::Command::new(scripts_dir_clone.join("oxidize_system.sh"))
                .spawn()
                .expect("failed to oxidize system")
                .wait();
        });
    }
}

impl WidgetImpl for NoidWelcomeWindow {}
impl WindowImpl for NoidWelcomeWindow {}
impl ApplicationWindowImpl for NoidWelcomeWindow {}
