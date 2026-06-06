// SPDX-License-Identifier: GPL-3.0-or-later
/* build.rs
 * Copyright (C) 2026 Naz <ndpm13@ch-naseem.com>
 */

fn main() {
    let scripts_dir = std::env::var("SCRIPTS_DIR")
        .unwrap_or_else(|_| "/usr/libexec/noid-welcome/scripts".to_string());

    println!("cargo:rustc-env=SCRIPTS_DIR={scripts_dir}");
    println!("cargo:rerun-if-env-changed=SCRIPTS_DIR");
}
