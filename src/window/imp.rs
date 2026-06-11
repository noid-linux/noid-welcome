// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use std::sync::LazyLock;

use gio::glib::subclass::Signal;

use super::*;

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

        self.obj().connect_navigate(glib::clone!(
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
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: LazyLock<Vec<Signal>> = LazyLock::new(|| {
            vec![
                Signal::builder("navigate")
                    .param_types([String::static_type()])
                    .build(),
            ]
        });

        SIGNALS.as_ref()
    }
}

impl WidgetImpl for NoidWelcomeWindow {}
impl WindowImpl for NoidWelcomeWindow {}
impl ApplicationWindowImpl for NoidWelcomeWindow {}
