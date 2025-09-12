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
        .invoke_handler(tauri::generate_handler![install_xbps_packages])
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
