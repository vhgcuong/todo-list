import {invoke} from "@tauri-apps/api/core";
import {Todo} from "../types/models.ts";

export async function fetchAll(): Promise<Todo[]> {
    try {
        const todos = await invoke<Todo[]>("todos");
        console.log("Fetch successful:", todos);
        return todos;
    } catch (err) {
        console.error("Error invoking todos:", err);
        return [];
    }
}

export async function changeDone(status: boolean, id: number): Promise<boolean> {
    console.log("Invoking change_status", { status, id });
    try {
        const ok = await invoke<boolean>("change_status", { status, id });
        console.log("Update successful:", ok);
        return ok;
    } catch (err) {
        console.error("Error invoking change_status:", err);
        return false;
    }
}

export async function add(text: string, status: boolean) {
    console.log("Invoking", { status, text });
    try {
        const todo = await invoke("add", { status, text });
        console.log("Create successful:", todo);
        return todo;
    } catch (err) {
        console.error("Error invoking add:", err);
        return null;
    }
}