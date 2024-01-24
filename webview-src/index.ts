import { invoke } from "@tauri-apps/api/core";

export async function importModule(modulePath: string) {
  return await invoke<void>("plugin:python|import", { modulePath });
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
