import {invoke} from "@tauri-apps/api/core";

export async function changeDone(isDone: boolean, id: number) {
    return await invoke("change_done", { is_done: isDone, id: id });
}
