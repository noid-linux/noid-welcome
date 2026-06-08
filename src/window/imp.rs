// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use std::ffi::OsStr;

use super::*;
use crate::util::{autostart_file, read_line_utf8_async_to_buffer, scripts_dir};

const HEADER: &str = r#"
 _   _       _     _   _____                  _
| \ | |     (_)   | | |_   _|                | |
|  \| | ___  _  __| |   | |_      _____  __ _| | _____
| . ` |/ _ \| |/ _` |   | \ \ /\ / / _ \/ _` | |/ / __|
| |\  | (_) | | (_| |   | |\ V  V /  __/ (_| |   <\__ \
\_| \_/\___/|_|\__,_|   \_/ \_/\_/ \___|\__,_|_|\_\___/
"#;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/ch-naseem/NoidWelcome/ui/window.ui")]
pub struct NoidWelcomeWindow {
    #[template_child]
    pub stack: TemplateChild<gtk::Stack>,

    // main stack
    #[template_child]
    pub switch_autostart: TemplateChild<gtk::Switch>,

    #[template_child]
    pub button_system_update: TemplateChild<gtk::Button>,

    #[template_child]
    pub button_virt_manager: TemplateChild<gtk::Button>,

    #[template_child]
    pub button_oxidize_system: TemplateChild<gtk::Button>,

    // log stack
    #[template_child]
    pub label_title: TemplateChild<gtk::Label>,

    #[template_child]
    pub text_view_log: TemplateChild<gtk::TextView>,

    #[template_child]
    pub box_confirmation: TemplateChild<gtk::Box>,

    #[template_child]
    pub button_return: TemplateChild<gtk::Button>,
}

#[gtk::template_callbacks]
impl NoidWelcomeWindow {
    #[template_callback]
    fn on_button_return_clicked(&self) {
        self.stack.set_visible_child_name("main");
        self.text_view_log.buffer().set_text("");
        self.label_title.set_label("");
        self.button_return.set_visible(false)
    }

    #[template_callback]
    fn on_button_cancel_clicked(&self) {
        self.box_confirmation.set_visible(false);
        self.text_view_log
            .buffer()
            .set_text("Canceled system update.");
        self.button_return.set_visible(true)
    }

    #[template_callback]
    fn on_button_proceed_clicked(&self) {
        self.box_confirmation.set_visible(false);
        let tweak_filename = match self.label_title.label().as_str() {
            "System update" => "system_update.sh",
            "Install virt-manager" => "virt_manager.sh",
            "Oxidize your system" => "oxidize_system.sh",
            _ => panic!(),
        };

        let tweak_path = &scripts_dir()
            .join(tweak_filename)
            .to_string_lossy()
            .to_string();

        let argv = &[
            OsStr::new("pkexec"),
            OsStr::new("bash"),
            OsStr::new("-c"),
            OsStr::new(tweak_path),
        ];

        let subprocess = gio::Subprocess::newv(
            argv,
            gio::SubprocessFlags::STDOUT_PIPE.union(gio::SubprocessFlags::STDERR_MERGE),
        )
        .unwrap();
        let stream = gio::DataInputStream::new(&subprocess.stdout_pipe().unwrap());

        let buffer = self.text_view_log.buffer();

        read_line_utf8_async_to_buffer(
            stream,
            buffer,
            glib::clone!(
                #[strong(rename_to = window)]
                self.obj(),
                move || {
                    window.imp().button_return.set_visible(true);

                    let buffer = window.imp().text_view_log.buffer();
                    buffer.insert(&mut buffer.end_iter(), "Completed successfully\n");
                }
            ),
        );
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

    #[template_callback]
    fn on_button_system_update_clicked(&self) {
        self.stack.set_visible_child_name("log");
        self.box_confirmation.set_visible(true);
        self.label_title.set_label("System update");

        let buffer = self.text_view_log.buffer();

        buffer.set_text(&HEADER);
        buffer.insert(
            &mut buffer.end_iter(),
            r#"
This tweak will attempt to update your system using xbps package manager

"#,
        );
    }

    #[template_callback]
    fn on_button_virt_manager_clicked(&self) {
        self.stack.set_visible_child_name("log");
        self.box_confirmation.set_visible(true);
        self.label_title.set_label("Install virt-manager");

        let buffer = self.text_view_log.buffer();

        buffer.set_text(&HEADER);
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

    #[template_callback]
    fn on_button_oxidize_system_clicked(&self) {
        self.stack.set_visible_child_name("log");
        self.box_confirmation.set_visible(true);
        self.label_title.set_label("Oxidize your system");

        let buffer = self.text_view_log.buffer();

        buffer.set_text(&HEADER);
        buffer.insert(
            &mut buffer.end_iter(),
            r#"
This tweak will install Rust and some useful CLIs, in the process it will:
- Install rustup
- Install these CLIs: ripgrep, bat, fd-find, zoxide, eza, tealdeer,
  du-dust, bottom, cargo-update
- Modify these files: ~/.zshrc, ~/.bashrc

And you will need to restart for changes to take effect.

"#,
        );
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
