// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/main/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use gtk::prelude::WidgetExt;

use crate::{tweak::Tweak, util::autostart_file, window::Window};

use super::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/ch-naseem/NoidWelcome/ui/stack/main.ui")]
pub struct StackPageMain {
    #[template_child]
    pub switch_autostart: TemplateChild<gtk::Switch>,

    #[template_child]
    pub button_get_software: TemplateChild<gtk::Button>,

    #[template_child]
    pub button_system_update: TemplateChild<gtk::Button>,

    #[template_child]
    pub button_virt_manager: TemplateChild<gtk::Button>,

    #[template_child]
    pub button_oxidize_system: TemplateChild<gtk::Button>,
}

#[gtk::template_callbacks]
impl StackPageMain {
    #[template_callback]
    fn on_button_get_software_clicked(&self) {
        let window = self.obj().root().and_downcast::<Window>().unwrap();

        window.emit_by_name::<()>("navigate", &[&"get-software"])
    }

    #[template_callback]
    fn on_button_system_update_clicked(&self) {
        self.obj().handle_tweak(Tweak::SystemUpdate);
    }

    #[template_callback]
    fn on_button_virt_manager_clicked(&self) {
        self.obj().handle_tweak(Tweak::VirtManager);
    }

    #[template_callback]
    fn on_button_oxidize_system_clicked(&self) {
        self.obj().handle_tweak(Tweak::OxidizeSystem);
    }

    #[template_callback]
    fn on_switch_autostart_state_set(&self, state: bool) -> glib::Propagation {
        let autostart_file = autostart_file();

        if state {
            std::fs::copy(
                std::path::PathBuf::from(
                    "/usr/share/applications/com.ch_naseem.NoidWelcome.desktop",
                ),
                &autostart_file,
            )
            .unwrap();
        } else {
            std::fs::remove_file(&autostart_file).unwrap();
        }

        glib::Propagation::Proceed
    }
}

#[glib::object_subclass]
impl ObjectSubclass for StackPageMain {
    const NAME: &'static str = "StackPageMain";
    type Type = super::StackPageMain;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for StackPageMain {
    fn constructed(&self) {
        self.parent_constructed();

        // Handle autostart
        self.switch_autostart.set_active(autostart_file().exists());
    }
}

impl WidgetImpl for StackPageMain {}
impl BoxImpl for StackPageMain {}
