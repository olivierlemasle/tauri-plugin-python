use tauri::{command, AppHandle, Runtime};

use crate::{PythonExt, Result};

#[command]
pub(crate) async fn import<R: Runtime>(app: AppHandle<R>, module_path: String) -> Result<()> {
    app.python().import(module_path)
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
