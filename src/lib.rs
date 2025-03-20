use tauri::{
    path::BaseDirectory,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime,
};

use std::{fs, sync::Mutex};

pub use error::{Error, Result};
pub use models::*;
use python::*;

mod commands;
mod error;
mod models;
mod python;

pub struct Python<R: Runtime> {
    app_handle: AppHandle<R>,
}

impl<R: Runtime> Python<R> {
    pub fn add_resource_path_to_sys_path(&self, path: &str) -> Result<()> {
        let resolved = self
            .app_handle
            .path()
            .resolve(path, BaseDirectory::Resource)
            .map_err(|_| Error::Resolve(path.to_string()))?;

        /*if !fs::metadata(&resolved)?.is_dir() {
            return Err(Error::NotADir(path.to_string()));
        }*/

        let interpreter = self.app_handle.state::<State>().inner().0.lock().unwrap();
        let path = &resolved.to_string_lossy();
        interpreter.add_to_sys_path(path)
    }

    pub fn import(&self, module_name: &str) -> Result<()> {
        let interpreter = self.app_handle.state::<State>().inner().0.lock().unwrap();
        interpreter.import(module_name)
    }

    pub fn call_function(
        &self,
        module_name: &str,
        function_name: &str,
        args: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let interpreter = self.app_handle.state::<State>().inner().0.lock().unwrap();
        interpreter.call_function(module_name, function_name, args)
    }
}

#[derive(Default)]
struct State(Mutex<Interpreter>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the python APIs.
pub trait PythonExt<R: Runtime> {
    fn python(&self) -> &Python<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PythonExt<R> for T {
    fn python(&self) -> &Python<R> {
        self.state::<Python<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("python")
        .invoke_handler(tauri::generate_handler![
            commands::add_resource_path_to_sys_path,
            commands::import,
            commands::call_function
        ])
        .setup(|app, _api| {
            let python = Python {
                app_handle: app.clone(),
            };
            app.manage(python);

            // manage state so it is accessible by the commands
            app.manage(State::default());
            Ok(())
        })
        .build()
}
