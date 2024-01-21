use tauri::{command, AppHandle, Runtime};

use crate::{PythonExt, Result};

#[command]
pub(crate) async fn import<R: Runtime>(app: AppHandle<R>, module_path: String) -> Result<String> {
    app.python().import(module_path)?;
    Ok("success".to_string())
}
