use tauri_plugin_python::PythonExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
