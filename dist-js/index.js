import { invoke } from '@tauri-apps/api/core';

async function addResourcePathToSysPath(path) {
    return await invoke("plugin:python|add_resource_path_to_sys_path", {
        path,
    });
}
async function importModule(moduleName) {
    return await invoke("plugin:python|import", { moduleName });
}
async function callFunction(moduleName, functionName, args) {
    return await invoke("plugin:python|call_function", {
        moduleName,
        functionName,
        args,
    });
}

export { addResourcePathToSysPath, callFunction, importModule };
