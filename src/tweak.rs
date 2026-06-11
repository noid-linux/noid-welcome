// SPDX-License-Identifier: GPL-3.0-or-later
/* src/tweak.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use std::ffi::OsStr;

use gio::glib::{
    self,
    object::ObjectExt,
    subclass::types::{ObjectSubclassExt, ObjectSubclassIsExt},
};
use gtk::prelude::*;

use crate::{
    util::{read_line_utf8_async_to_buffer, scripts_dir},
    window::{NoidWelcomeWindow, StackPageLog},
};

const HEADER: &str = r#"
 _   _       _     _   _____                  _
| \ | |     (_)   | | |_   _|                | |
|  \| | ___  _  __| |   | |_      _____  __ _| | _____
| . ` |/ _ \| |/ _` |   | \ \ /\ / / _ \/ _` | |/ / __|
| |\  | (_) | | (_| |   | |\ V  V /  __/ (_| |   <\__ \
\_| \_/\___/|_|\__,_|   \_/ \_/\_/ \___|\__,_|_|\_\___/
"#;

#[derive(Debug, Copy, Clone)]
pub enum Tweak {
    SystemUpdate,
    VirtManager,
    OxidizeSystem,
}

impl Tweak {
    fn title(&self) -> &'static str {
        match self {
            Self::SystemUpdate => "System update",
            Self::VirtManager => "Install virt-manager",
            Self::OxidizeSystem => "Oxidize your system",
        }
    }

    fn summary(&self) -> &'static str {
        match self {
            Self::SystemUpdate => {
                r#"
This tweak will attempt to update your system using xbps package manager

"#
            }
            Self::VirtManager => {
                r#"
This tweak will install virt-manager, in the process it will:
- Install these deps: qemu, virt-manager, virt-viewer, dnsmasq, vde2,
  bridge-utils, openbsd-netcat, libguestfs
- Enable these Runit services: libvirtd, virtlogd
- Modify these files: /etc/libvirt/libvirtd.conf, /etc/libvirt/qemu.conf
- Add your user to the libvirt user group.

And you will need to restart for changes to take effect.

"#
            }
            Self::OxidizeSystem => {
                r#"
This tweak will install Rust and some useful CLIs, in the process it will:
- Install rustup
- Install these CLIs: ripgrep, bat, fd-find, zoxide, eza, tealdeer,
  du-dust, bottom, cargo-update
- Modify these files: ~/.zshrc, ~/.bashrc

"#
            }
        }
    }

    fn script(&self) -> std::path::PathBuf {
        let filename = match self {
            Self::SystemUpdate => "system_update.sh",
            Self::VirtManager => "virt_manager.sh",
            Self::OxidizeSystem => "oxidize_system.sh",
        };

        scripts_dir().join(filename)
    }

    pub fn prompt(&self, window: &NoidWelcomeWindow) {
        let window = window.imp();

        let stack_page_log = &window.stack_page_log.imp();
        let buffer = &stack_page_log.text_view_log.buffer();

        window.obj().emit_by_name::<()>("navigate", &[&"log"]);
        window.header_label.set_label(self.title());

        stack_page_log.obj().set_tweak(*self);
        stack_page_log.box_confirmation.set_visible(true);

        buffer.set_text(HEADER);
        buffer.insert(&mut buffer.end_iter(), self.summary());
    }

    pub fn run(&self, stack_page: &StackPageLog) {
        let stack_page = stack_page.imp();
        stack_page.box_confirmation.set_visible(false);

        let buffer = stack_page.text_view_log.buffer();

        let script = &self.script();
        let script = script.to_string_lossy();

        let argv = &[
            OsStr::new("pkexec"),
            OsStr::new("bash"),
            OsStr::new("-c"),
            OsStr::new(&*script),
        ];
        let subprocess = gio::Subprocess::newv(
            argv,
            gio::SubprocessFlags::STDOUT_PIPE.union(gio::SubprocessFlags::STDERR_MERGE),
        )
        .unwrap();
        let stream = gio::DataInputStream::new(&subprocess.stdout_pipe().unwrap());

        read_line_utf8_async_to_buffer(
            stream,
            buffer,
            glib::clone!(
                #[weak]
                stack_page,
                move || {
                    stack_page.button_return.set_visible(true);

                    let buffer = stack_page.text_view_log.buffer();
                    buffer.insert(&mut buffer.end_iter(), "Completed successfully\n");
                }
            ),
        );
    }
}
