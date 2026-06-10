// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

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
#[template(resource = "/com/ch-naseem/NoidWelcome/ui/window.ui")]
pub struct NoidWelcomeWindow {
    #[template_child]
    pub header_label: TemplateChild<gtk::Label>,

    #[template_child]
    pub stack: TemplateChild<gtk::Stack>,

    #[template_child]
    pub stack_page_main: TemplateChild<StackPageMain>,

    #[template_child]
    pub stack_page_log: TemplateChild<StackPageLog>,

    #[template_child]
    pub stack_page_get_software: TemplateChild<StackPageGetSoftware>,
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

        self.stack_page_main.connect_run_tweak(glib::clone!(
            #[weak(rename_to = window)]
            self.obj(),
            move |_widget, title, summary| {
                let window = window.imp();
                let stack_page_log = window.stack_page_log.imp();

                window.stack.set_visible_child_name("log");
                stack_page_log.box_confirmation.set_visible(true);
                window.header_label.set_label(title);

                let buffer = stack_page_log.text_view_log.buffer();
                buffer.set_text(HEADER);
                buffer.insert(&mut buffer.end_iter(), summary);
            }
        ));

        self.stack_page_main.connect_navigate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_widget, stackpage| {
                match stackpage {
                    "get-software" => window.obj().set_header_label(Some("Get software")),
                    _ => window.obj().set_header_label(None),
                }

                window.stack.set_visible_child_name(stackpage);
            }
        ));

        self.stack_page_log.connect_navigate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_, stackpage| {
                if stackpage == "main" {
                    window.obj().set_header_label(None);
                }

                window.stack.set_visible_child_name(stackpage);
            }
        ));

        self.stack_page_get_software.connect_navigate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_, stackpage| {
                if stackpage == "main" {
                    window.obj().set_header_label(None);
                }

                window.stack.set_visible_child_name(stackpage);
            }
        ));
    }
}

impl WidgetImpl for NoidWelcomeWindow {}
impl WindowImpl for NoidWelcomeWindow {}
impl ApplicationWindowImpl for NoidWelcomeWindow {}
