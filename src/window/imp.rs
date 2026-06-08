// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use std::ffi::OsStr;

use super::*;
use crate::util::{read_line_utf8_async_to_buffer, scripts_dir};

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/ch-naseem/NoidWelcome/ui/window.ui")]
pub struct NoidWelcomeWindow {
    #[template_child]
    pub stack: TemplateChild<gtk::Stack>,

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
    }
}

impl WidgetImpl for NoidWelcomeWindow {}
impl WindowImpl for NoidWelcomeWindow {}
impl ApplicationWindowImpl for NoidWelcomeWindow {}
