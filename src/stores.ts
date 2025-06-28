import { defineStore } from "pinia";
import { ref } from "vue";

export const useSingleMenu = defineStore("singleMenu", () => {
    const menuId = ref("");
    const open = (id: string) => {
        menuId.value = id;
    }
    const getMenuId = () => menuId.value;
    return { menuId, open, getMenuId };
});