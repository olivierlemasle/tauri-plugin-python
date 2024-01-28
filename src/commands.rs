use tauri::{command, AppHandle, Runtime};

use crate::{PythonExt, Result};

#[command]
pub(crate) async fn add_resource_path_to_sys_path<R: Runtime>(
    app: AppHandle<R>,
    path: &str,
) -> Result<()> {
    app.python().add_resource_path_to_sys_path(path)
}

#[command]
pub(crate) async fn import<R: Runtime>(app: AppHandle<R>, module_name: &str) -> Result<()> {
    app.python().import(module_name)
}

#[command]
pub(crate) async fn call_function<R: Runtime>(
    app: AppHandle<R>,
    module_name: &str,
    function_name: &str,
    args: Vec<serde_json::Value>,
) -> Result<serde_json::Value> {
    app.python().call_function(module_name, function_name, args)
}
