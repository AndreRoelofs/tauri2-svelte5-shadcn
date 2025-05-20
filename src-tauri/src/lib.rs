mod commands;
use commands::default::{adjust_initial_window_size, read, resize_window, write};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// Runs the Tauri application.
///
/// # Panics
///
/// This function will panic if the Tauri application fails to run or if there's an error during setup.
pub fn run() {
    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![
            read,
            write,
            resize_window,
            adjust_initial_window_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
