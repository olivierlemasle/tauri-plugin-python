'use strict';

var core = require('@tauri-apps/api/core');

async function addResourcePathToSysPath(path) {
    return await core.invoke("plugin:python|add_resource_path_to_sys_path", {
        path,
    });
}
async function importModule(moduleName) {
    return await core.invoke("plugin:python|import", { moduleName });
}
async function callFunction(moduleName, functionName, args) {
    return await core.invoke("plugin:python|call_function", {
        moduleName,
        functionName,
        args,
    });
}

exports.addResourcePathToSysPath = addResourcePathToSysPath;
exports.callFunction = callFunction;
exports.importModule = importModule;
