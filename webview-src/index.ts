import { invoke } from "@tauri-apps/api/core";

export async function importModule(modulePath: string) {
  return await invoke("plugin:python|import", { modulePath });
}
