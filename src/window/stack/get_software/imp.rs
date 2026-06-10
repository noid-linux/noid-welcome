// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/get_software/imp.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use std::{ffi::OsStr, sync::LazyLock};

use gio::glib::subclass::Signal;
use gtk::prelude::*;

use crate::util::read_line_utf8_async_to_buffer;

use super::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/com/ch-naseem/NoidWelcome/ui/stack/get-software.ui")]
pub struct StackPageGetSoftware {
    #[template_child]
    pub label_title: TemplateChild<gtk::Label>,

    #[template_child]
    pub text_view_log: TemplateChild<gtk::TextView>,

    #[template_child]
    pub column_view_packages: TemplateChild<gtk::ColumnView>,

    #[template_child]
    pub list_store_packages: TemplateChild<gio::ListStore>,

    #[template_child]
    pub factory_install: TemplateChild<gtk::SignalListItemFactory>,

    #[template_child]
    pub box_confirmation: TemplateChild<gtk::Box>,

    #[template_child]
    pub button_return: TemplateChild<gtk::Button>,
}

#[gtk::template_callbacks]
impl StackPageGetSoftware {
    #[template_callback]
    fn on_button_cancel_clicked(&self) {
        self.obj().emit_by_name::<()>("navigate", &[&"main"])
    }

    #[template_callback]
    fn on_button_return_clicked(&self) {
        self.text_view_log.buffer().set_text("");
        self.button_return.set_visible(false);
        self.obj().emit_by_name::<()>("navigate", &[&"main"]);
        self.obj().toggle_column_view();
        self.box_confirmation.set_visible(true);
    }

    #[template_callback]
    fn on_button_proceed_clicked(&self) {
        let packages = self
            .list_store_packages
            .into_iter()
            .map(|p| p.unwrap().downcast::<PackageObject>().unwrap())
            .filter(|p| p.install())
            .map(|p| p.pkgname())
            .collect::<Vec<String>>()
            .join(" ");

        let cmd = format!("xbps-install -S && xbps-install -y {packages}");
        let argv = &[
            OsStr::new("pkexec"),
            OsStr::new("bash"),
            OsStr::new("-c"),
            OsStr::new(&cmd),
        ];

        self.obj().toggle_column_view();
        self.box_confirmation.set_visible(false);

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
                #[strong(rename_to = stack_page)]
                self.obj(),
                move || {
                    let stack_page = stack_page.imp();

                    stack_page.button_return.set_visible(true);

                    let buffer = stack_page.text_view_log.buffer();
                    buffer.insert(&mut buffer.end_iter(), "Completed successfully\n");
                }
            ),
        );
    }
}

#[glib::object_subclass]
impl ObjectSubclass for StackPageGetSoftware {
    const NAME: &'static str = "StackPageGetSoftware";
    type Type = super::StackPageGetSoftware;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for StackPageGetSoftware {
    fn constructed(&self) {
        self.parent_constructed();

        let factory_install = &self.factory_install;

        factory_install.connect_setup(|_, object| {
            let list_item = object.downcast_ref::<gtk::ListItem>().unwrap();
            let check_button = gtk::CheckButton::new();
            list_item.set_child(Some(&check_button));
        });

        factory_install.connect_bind(|_, object| {
            let list_item = object.downcast_ref::<gtk::ListItem>().unwrap();
            let check_button = list_item
                .child()
                .and_downcast::<gtk::CheckButton>()
                .unwrap();
            let package = list_item.item().and_downcast::<PackageObject>().unwrap();

            package
                .bind_property("install", &check_button, "active")
                .bidirectional()
                .sync_create()
                .build();
        });

        let list_store_packages = &self.list_store_packages;
        list_store_packages.append(&PackageObject::new(
            true,
            "brave",
            "Web browser that blocks ads and trackers by default",
        ));

        let custom_packages: Vec<(bool, &str, &str)> = vec![
            (
                false,
                "brave-origin",
                "Minimalist browser from the makers of Brave",
            ),
            (
                false,
                "librewolf",
                "Fork of Firefox, focused on privacy, security and freedom",
            ),
            (
                false,
                "cinny-desktop",
                "Yet another matrix client for desktop",
            ),
            (
                false,
                "freetube",
                "Open source desktop YouTube player built with privacy in mind",
            ),
            (
                false,
                "grayjay",
                "Stream and download content from various sources",
            ),
            (
                false,
                "spotify-client",
                "Proprietary music streaming client",
            ),
            (false, "discord", "Chat and VOIP application"),
            (
                false,
                "obsidian",
                "Private and flexible writing app that adapts to the way you think",
            ),
        ];

        custom_packages.iter().for_each(|pkg| {
            list_store_packages.append(&PackageObject::new(pkg.0, pkg.1, pkg.2));
        });
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: LazyLock<Vec<Signal>> = LazyLock::new(|| {
            vec![
                Signal::builder("navigate")
                    .param_types([String::static_type()])
                    .build(),
                Signal::builder("install-packages")
                    .param_types([gtk::StringList::static_type()])
                    .build(),
            ]
        });

        SIGNALS.as_ref()
    }
}

impl WidgetImpl for StackPageGetSoftware {}
impl BoxImpl for StackPageGetSoftware {}
