// SPDX-License-Identifier: GPL-3.0-or-later
/* src/util.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

use crate::config::SCRIPTSDIR;

pub fn autostart_file() -> std::path::PathBuf {
    gio::glib::home_dir()
        .join(".config")
        .join("autostart")
        .join("noid-welcome.desktop")
}

pub fn scripts_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(SCRIPTSDIR)
}
