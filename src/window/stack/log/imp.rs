// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/log/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use std::ffi::OsStr;

use crate::{
    util::{read_line_utf8_async_to_buffer, scripts_dir},
    window::NoidWelcomeWindow,
};

use super::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/ch-naseem/NoidWelcome/ui/stack/log.ui")]
pub struct StackPageLog {
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
impl StackPageLog {
    #[template_callback]
    fn on_button_return_clicked(&self) {
        if let Some(widget) = &self.obj().ancestor(NoidWelcomeWindow::static_type())
            && let Ok(window) = widget.clone().downcast::<NoidWelcomeWindow>()
        {
            let window = window.imp();

            window.stack.set_visible_child_name("main");
        }

        self.text_view_log.buffer().set_text("");
        self.label_title.set_label("");
        self.button_return.set_visible(false)
    }

    #[template_callback]
    fn on_button_cancel_clicked(&self) {
        self.box_confirmation.set_visible(false);
        self.text_view_log
            .buffer()
            .set_text("Canceled system tweak.");
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
impl ObjectSubclass for StackPageLog {
    const NAME: &'static str = "StackPageLog";
    type Type = super::StackPageLog;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for StackPageLog {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for StackPageLog {}
impl BoxImpl for StackPageLog {}
