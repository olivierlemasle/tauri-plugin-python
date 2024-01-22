import { invoke } from "@tauri-apps/api/core";

export async function importModule(modulePath: string) {
  return await invoke<void>("plugin:python|import", { modulePath });
}
