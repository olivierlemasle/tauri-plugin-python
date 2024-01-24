use rustpython_vm::{
    builtins::{PyBaseExceptionRef, PyStr},
    py_serde, PyResult, VirtualMachine,
};

use crate::error::{Error, Result};

pub struct Interpreter(rustpython_vm::Interpreter);

impl Interpreter {
    pub fn new() -> Self {
        let interpreter = create_interpreter();
        Self(interpreter)
    }

    pub fn import(&self, dir_path: &str, module_name: &str) -> Result<()> {
        self.0.enter(|vm| {
            vm.insert_sys_path(vm.new_pyobj(dir_path)).to_err(vm)?;

            let module_name = PyStr::new_ref(module_name, &vm.ctx);
            let _module = vm.import(&module_name, None, 0).to_err(vm)?;
            Ok(())
        })
    }

    pub fn call_function(
        &self,
        module_name: &str,
        function_name: &str,
        args: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        self.0.enter(|vm| {
            let posargs: Vec<_> = args
                .into_iter()
                .map(|x| py_serde::deserialize(vm, x).unwrap())
                .collect();

            let module_name = PyStr::new_ref(module_name, &vm.ctx);
            let module = vm.import(&module_name, None, 0).to_err(vm)?;
            let function_name = PyStr::new_ref(function_name, &vm.ctx);
            let function = module.get_attr(&function_name, vm).to_err(vm)?;
            let result = function.call(posargs, vm).to_err(vm)?;

            let json = py_serde::serialize(vm, &result, serde_json::value::Serializer)?;
            Ok(json)
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

trait ToErr<T> {
    fn to_err(self, vm: &VirtualMachine) -> Result<T>;
}

impl<T> ToErr<T> for PyResult<T> {
    fn to_err(self, vm: &VirtualMachine) -> Result<T> {
        self.map_err(|err: PyBaseExceptionRef| {
            let mut s = String::new();
            vm.write_exception(&mut s, &err).unwrap();
            Error::Python(s)
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn import() -> crate::Result<()> {
        Interpreter::new().import("test_assets", "example")
    }

    #[test]
    fn import_wrong_path() {
        if let Err(Error::Python(msg)) = Interpreter::new().import("", "example") {
            assert!(msg.contains("ModuleNotFoundError"), "Got {}", msg);
        } else {
            panic!("should return Error::Python");
        }
    }

    #[test]
    fn import_wrong_module() {
        if let Err(Error::Python(msg)) = Interpreter::new().import("test_assets", "foo") {
            assert!(msg.contains("ModuleNotFoundError"), "Got {}", msg);
        } else {
            panic!("should return Error::Python");
        }
    }

    #[test]
    fn import_cached_module() -> crate::Result<()> {
        let interpreter = Interpreter::new();
        interpreter.import("test_assets", "example")?;
        interpreter.import("", "example")?;
        Ok(())
    }

    #[test]
    fn call_function() -> crate::Result<()> {
        let interpreter = Interpreter::new();
        interpreter.import("test_assets", "example")?;

        let args = vec![];
        let result = interpreter.call_function("example", "foo", args)?;
        assert_eq!(result, json!("baz"));

        let args = vec![json!(2), json!(3)];
        let result = interpreter.call_function("example", "multiply", args)?;
        assert_eq!(result, json!(6));

        let args = vec![json!(3), json!("bar")];
        let result = interpreter.call_function("example", "multiply", args)?;
        assert_eq!(result, json!("barbarbar"));

        let args = vec![json!(2), json!("foo")];
        let result = interpreter.call_function("example", "create_dict", args)?;
        assert_eq!(
            result,
            json!({"first": 2, "second": "foo", "array": [2, "foo"]})
        );

        Ok(())
    }

    #[test]
    #[should_panic]
    fn nonexiting_function() {
        let interpreter = Interpreter::new();
        interpreter.import("test_assets", "example").unwrap();
        interpreter
            .call_function("example", "abcd", vec![])
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn not_a_function() {
        let interpreter = Interpreter::new();
        interpreter.import("test_assets", "example").unwrap();
        interpreter.call_function("example", "bar", vec![]).unwrap();
    }

    #[test]
    #[should_panic]
    fn wrong_args() {
        let interpreter = Interpreter::new();
        interpreter.import("test_assets", "example").unwrap();
        interpreter
            .call_function("example", "multiply", vec![json!(1)])
            .unwrap();
    }
}
