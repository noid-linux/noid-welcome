// SPDX-License-Identifier: GPL-3.0-or-later
/* src/tweak.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use std::ffi::OsStr;

use gio::glib;
use gtk::prelude::*;

use crate::util::{read_line_utf8_async_to_buffer, scripts_dir};

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

pub trait TweakLogger: glib::clone::Downgrade {
    fn set_tweak(&self, tweak: Tweak);
    fn show_confirmation(&self);
    fn hide_confirmation(&self);
    fn show_return(&self);
}

impl Tweak {
    pub fn title(&self) -> &'static str {
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

    pub fn prompt(&self, buffer: &gtk::TextBuffer, logger: &impl TweakLogger) {
        logger.set_tweak(*self);
        logger.show_confirmation();

        buffer.set_text(HEADER);
        buffer.insert(&mut buffer.end_iter(), self.summary());
    }

    pub fn run<F>(&self, buffer: gtk::TextBuffer, logger: &impl TweakLogger, on_done: F)
    where
        F: Fn() + 'static,
    {
        logger.hide_confirmation();

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

        read_line_utf8_async_to_buffer(stream, buffer, on_done);
    }
}
