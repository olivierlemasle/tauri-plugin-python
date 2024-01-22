use tauri::{command, AppHandle, Runtime};

use crate::{PythonExt, Result};

#[command]
pub(crate) async fn import<R: Runtime>(app: AppHandle<R>, module_path: String) -> Result<()> {
    app.python().import(module_path)
}