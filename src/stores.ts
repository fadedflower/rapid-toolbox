import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { AppMetadata } from "./types";

export const useSingleMenu = defineStore("singleMenu", () => {
    const menuId = ref("");
    const open = (id: string) => {
        menuId.value = id;
    }
    const getMenuId = () => menuId.value;
    return { menuId, open, getMenuId };
});

export const useAppList = defineStore("appList", () => {
    const apps = ref<AppMetadata[]>([]);
    const reloadApps = async () => {
        apps.value = await invoke<AppMetadata[]>("get_all_app_list");
        apps.value.sort((a, b) => Intl.Collator(undefined, { numeric: true }).compare(a.name, b.name));
    };
    return { apps, reloadApps };
});