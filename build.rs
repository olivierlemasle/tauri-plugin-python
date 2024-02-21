const COMMANDS: &[&str] = &["add_resource_path_to_sys_path", "import", "call_function"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
