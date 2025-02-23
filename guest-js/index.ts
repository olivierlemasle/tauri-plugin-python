import { invoke } from "@tauri-apps/api/core";

export async function addResourcePathToSysPath(path: string) {
  return await invoke<void>("plugin:python|add_resource_path_to_sys_path", {
    path,
  });
}

export async function importModule(moduleName: string) {
  return await invoke<void>("plugin:python|import", { moduleName });
}

export async function callFunction(
  moduleName: string,
  functionName: string,
  args: Array<any>
) {
  return await invoke<any>("plugin:python|call_function", {
    moduleName,
    functionName,
    args,
  });
}
