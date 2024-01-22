use rustpython_vm::builtins::{PyBaseExceptionRef, PyStr};

use crate::error::{Error, Result};

pub struct Interpreter(rustpython_vm::Interpreter);

impl Interpreter {
    pub fn new() -> Self {
        let interpreter = create_interpreter();
        Self(interpreter)
    }

    pub fn import(&self, dir_path: &str, module_name: &str) -> Result<()> {
        self.0.enter(|vm| {
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

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import() -> crate::Result<()> {
        Interpreter::new().import("test_assets", "example")
    }

    #[test]
    fn test_import_wrong_path() {
        if let Err(Error::Python(msg)) = Interpreter::new().import("", "example") {
            assert!(msg.contains("ModuleNotFoundError"), "Got {}", msg);
        } else {
            panic!("should return Error::Python");
        }
    }

    #[test]
    fn test_import_wrong_module() {
        if let Err(Error::Python(msg)) = Interpreter::new().import("", "foo") {
            assert!(msg.contains("ModuleNotFoundError"), "Got {}", msg);
        } else {
            panic!("should return Error::Python");
        }
    }

    #[test]
    fn test_import_cached_module() -> crate::Result<()> {
        let interpreter = Interpreter::new();
        interpreter.import("test_assets", "example")?;
        interpreter.import("", "example")?;
        Ok(())
    }
}
