// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_python::PythonExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_python::init())
        .setup(|app| {
            app.python()
                .add_resource_path_to_sys_path("../../../test_assets")?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
