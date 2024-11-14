mod commands;

use commands::submit_command;
use commands::open_file_dialog;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
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
      .invoke_handler(tauri::generate_handler![submit_command, open_file_dialog])
      //.invoke_handler(tauri::generate_handler![open_file_dialog])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
