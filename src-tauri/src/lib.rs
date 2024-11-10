use std::collections::HashMap;
use puml::puml::core_parser::parser::{parse};
use puml::puml::code_generators::java::generate_java_code;

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
      .invoke_handler(tauri::generate_handler![submit_controller])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// My commands
#[tauri::command]
fn submit_controller(path: String) -> HashMap<String, String> {
    parse(path, generate_java_code)
}
