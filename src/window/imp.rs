// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use std::ffi::OsStr;

use super::*;
use crate::util::{autostart_file, read_line_utf8_async_to_buffer, scripts_dir};

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

        let tweak = &scripts_dir()
            .join("system_update.sh")
            .to_string_lossy()
            .to_string();

        let argv = &[
            OsStr::new("pkexec"),
            OsStr::new("bash"),
            OsStr::new("-c"),
            OsStr::new(tweak),
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
                    buffer.insert(&mut buffer.end_iter(), "System update complete\n");
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

        self.text_view_log.buffer().set_text(
            r#"
 _   _       _     _   _____                  _
| \ | |     (_)   | | |_   _|                | |
|  \| | ___  _  __| |   | |_      _____  __ _| | _____
| . ` |/ _ \| |/ _` |   | \ \ /\ / / _ \/ _` | |/ / __|
| |\  | (_) | | (_| |   | |\ V  V /  __/ (_| |   <\__ \
\_| \_/\___/|_|\__,_|   \_/ \_/\_/ \___|\__,_|_|\_\___/

This tweak will attempt to update your system using xbps package manager
"#,
        );
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
