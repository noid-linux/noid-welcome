use std::path::PathBuf;

const AUTOSTART_FILE_PATH: &str = ".config/autostart/noid-welcome.desktop";
const NOID_WELCOME_HOME: &str = "/opt/noid-welcome";

#[tauri::command]
fn disable_startup() -> Result<(), String> {
    let home = std::env::var("HOME").unwrap();

    let autostart_file = PathBuf::from(home).join(&AUTOSTART_FILE_PATH);

    if autostart_file.exists() {
        std::fs::remove_file(autostart_file).ok();
    }

    Ok(())
}

#[tauri::command]
async fn run_tweak(tweak: &str) -> Result<String, String> {
    let mut args = vec!["--hold", "-e", "bash", "-ic"];

    let tweak = match tweak {
        "oxidize_system" => format!("{}/scripts/oxidize-system.sh", &NOID_WELCOME_HOME),
        "install_qemu_vmm" => format!("{}/scripts/qemu-virt-manager.sh", &NOID_WELCOME_HOME),
        _ => return Err("Failed to run tweak".into()),
    };

    args.push(&tweak);

    let status = std::process::Command::new("alacritty")
        .args(&args)
        .spawn()
        .map_err(|e| format!("Failed to run tweak: {}", e))?
        .wait()
        .map_err(|e| format!("Failed to run tweak: {}", e))?;

    Ok(status.to_string())
}

#[tauri::command]
fn install_xbps_packages(packages: Vec<String>) -> Result<String, String> {
    let status = std::process::Command::new("alacritty")
        .args(["--hold", "-e", "sudo", "xbps-install", "-S"])
        .args(packages)
        .spawn()
        .map_err(|e| format!("Failed to install packages: {}", e))?
        .wait()
        .map_err(|e| format!("Failed to install packages: {}", e))?;

    Ok(status.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            install_xbps_packages,
            disable_startup,
            run_tweak
        ])
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
