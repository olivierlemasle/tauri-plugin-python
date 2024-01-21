import { invoke } from '@tauri-apps/api/core'

export async function execute() {
  return await invoke('plugin:python|execute')
}
