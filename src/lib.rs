use rustpython_vm::builtins::{PyBaseExceptionRef, PyStr};
use tauri::{
    path::BaseDirectory,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime,
};

use std::sync::Mutex;

pub use models::*;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

pub struct Python<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Python<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }

    pub fn import(&self, path: String) -> Result<()> {
        let (dir_path, module_name) = self
            .0
            .path()
            .resolve(&path, BaseDirectory::Resource)
            .as_ref()
            .map(|p| {
                let dir_path: String = p
                    .parent()
                    .map(|p| p.to_string_lossy().into())
                    .unwrap_or_default();

                let module_name = p.file_stem().unwrap().to_string_lossy().to_string();

                (dir_path, module_name)
            })
            .map_err(|_| Error::Resolve(path))?;

        println!("Module {module_name} in directory {dir_path}");

        let state = self.0.state::<State>().inner();
        state.0.lock().unwrap().enter(|vm| {
            let handle_err = |err: PyBaseExceptionRef| {
                let mut s = String::new();
                vm.write_exception(&mut s, &err).unwrap();
                Error::Python(s)
            };

            vm.insert_sys_path(vm.new_pyobj(dir_path))
                .map_err(handle_err)?;

            let module_name = PyStr::new_ref(module_name, &vm.ctx);
            let _module = vm.import(&module_name, None, 0).map_err(handle_err)?;
            Ok(())
        })
    }
}

struct State(Mutex<rustpython_vm::Interpreter>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the python APIs.
pub trait PythonExt<R: Runtime> {
    fn python(&self) -> &Python<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PythonExt<R> for T {
    fn python(&self) -> &Python<R> {
        self.state::<Python<R>>().inner()
    }
}

fn create_interpreter() -> rustpython_vm::Interpreter {
    #[allow(unused_mut)]
    let mut settings = rustpython_vm::Settings::default();

    #[cfg(not(feature = "freeze-stdlib"))]
    if let Ok(python_lib) = std::env::var("PYTHONLIB") {
        settings.path_list.push(python_lib);
    } else {
        println!("No PYTHONLIB");
    }

    rustpython_vm::Interpreter::with_init(settings, |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());

        #[cfg(feature = "freeze-stdlib")]
        vm.add_frozen(rustpython_pylib::FROZEN_STDLIB);
    })
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("python")
        .invoke_handler(tauri::generate_handler![commands::import])
        .setup(|app, _api| {
            let python = Python(app.clone());
            app.manage(python);

            // manage state so it is accessible by the commands
            let interpreter = create_interpreter();
            app.manage(State(Mutex::new(interpreter)));
            Ok(())
        })
        .build()
}
