import {invoke} from "@tauri-apps/api/core";

export async function changeDone(status: boolean, id: number) {
    console.log("Invoking change_status", { status, id });
    try {
        const ok = await invoke("change_status", { status, id });
        console.log("Update successful:", ok);
        return ok;
    } catch (err) {
        console.error("Error invoking change_status:", err);
    }
}
