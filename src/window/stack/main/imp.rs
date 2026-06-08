// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/main/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use gio::glib::{object::Cast, types::StaticType};
use gtk::prelude::*;

use crate::{util::autostart_file, window::NoidWelcomeWindow};

use super::*;

const HEADER: &str = r#"
 _   _       _     _   _____                  _
| \ | |     (_)   | | |_   _|                | |
|  \| | ___  _  __| |   | |_      _____  __ _| | _____
| . ` |/ _ \| |/ _` |   | \ \ /\ / / _ \/ _` | |/ / __|
| |\  | (_) | | (_| |   | |\ V  V /  __/ (_| |   <\__ \
\_| \_/\___/|_|\__,_|   \_/ \_/\_/ \___|\__,_|_|\_\___/
"#;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/ch-naseem/NoidWelcome/ui/stack/main.ui")]
pub struct StackPageMain {
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
impl StackPageMain {
    #[template_callback]
    fn on_button_system_update_clicked(&self) {
        if let Some(widget) = &self.obj().ancestor(NoidWelcomeWindow::static_type())
            && let Ok(window) = widget.clone().downcast::<NoidWelcomeWindow>()
        {
            let window = window.imp();

            window.stack.set_visible_child_name("log");
            window.box_confirmation.set_visible(true);
            window.label_title.set_label("System update");

            let buffer = window.text_view_log.buffer();

            buffer.set_text(HEADER);
            buffer.insert(
                &mut buffer.end_iter(),
                r#"
This tweak will attempt to update your system using xbps package manager

    "#,
            );
        }
    }

    #[template_callback]
    fn on_button_virt_manager_clicked(&self) {
        if let Some(widget) = &self.obj().ancestor(NoidWelcomeWindow::static_type())
            && let Ok(window) = widget.clone().downcast::<NoidWelcomeWindow>()
        {
            let window = window.imp();

            window.stack.set_visible_child_name("log");
            window.box_confirmation.set_visible(true);
            window.label_title.set_label("Install virt-manager");

            let buffer = window.text_view_log.buffer();

            buffer.set_text(HEADER);
            buffer.insert(
                &mut buffer.end_iter(),
                r#"
This tweak will install virt-manager, in the process it will:
- Install these deps: qemu, virt-manager, virt-viewer, dnsmasq, vde2,
  bridge-utils, openbsd-netcat, libguestfs
- Enable these Runit services: libvirtd, virtlogd
- Modify these files: /etc/libvirt/libvirtd.conf, /etc/libvirt/qemu.conf
- Add your user to the libvirt user group.

And you will need to restart for changes to take effect.

"#,
            );
        }
    }

    #[template_callback]
    fn on_button_oxidize_system_clicked(&self) {
        if let Some(widget) = &self.obj().ancestor(NoidWelcomeWindow::static_type())
            && let Ok(window) = widget.clone().downcast::<NoidWelcomeWindow>()
        {
            let window = window.imp();

            window.stack.set_visible_child_name("log");
            window.box_confirmation.set_visible(true);
            window.label_title.set_label("Oxidize your system");

            let buffer = window.text_view_log.buffer();

            buffer.set_text(HEADER);
            buffer.insert(
                &mut buffer.end_iter(),
                r#"
This tweak will install Rust and some useful CLIs, in the process it will:
- Install rustup
- Install these CLIs: ripgrep, bat, fd-find, zoxide, eza, tealdeer,
  du-dust, bottom, cargo-update
- Modify these files: ~/.zshrc, ~/.bashrc

"#,
            );
        }
    }

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
