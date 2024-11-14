use std::collections::HashMap;
use std::fs::File;
use rfd::FileDialog;
use puml::puml::code_generators::java::generate_java_code;
use puml::puml::core_parser::parser::parse;

// My commands
#[tauri::command]
pub fn submit_command(path: String/*, source_code_strategy: SourceCodeStrategy*/) -> HashMap<String, String> {
    let path = path.trim();
    let file = File::open(path).expect(format!("Failed to read file: {}", path).as_str());
    //parse(file, source_code_strategy)
    parse(file, generate_java_code)
}

#[tauri::command]
pub fn open_file_dialog() -> String {
    let file_path = FileDialog::new()
        .set_title("Select a File")
        .pick_file();
    return file_path.unwrap().to_string_lossy().to_string();
}
