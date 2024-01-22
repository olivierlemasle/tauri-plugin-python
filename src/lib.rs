use tauri::{
    path::BaseDirectory,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime,
};

use std::sync::Mutex;

pub use models::*;
pub use python::*;

mod commands;
mod error;
mod models;
mod python;

pub use error::{Error, Result};

pub struct Python<R: Runtime> {
    app_handle: AppHandle<R>,
}

impl<R: Runtime> Python<R> {
    pub fn import(&self, path: String) -> Result<()> {
        let (dir_path, module_name) = self
            .app_handle
            .path()
            .resolve(&path, BaseDirectory::Resource)
            .as_ref()
            .map(|p| {
                let dir_path = p
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();

                let module_name = p.file_stem().unwrap().to_string_lossy().to_string();

                (dir_path, module_name)
            })
            .map_err(|_| Error::Resolve(path))?;

        let interpreter = self.app_handle.state::<State>().inner().0.lock().unwrap();
        interpreter.import(&dir_path, &module_name)
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
        .invoke_handler(tauri::generate_handler![commands::import])
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
