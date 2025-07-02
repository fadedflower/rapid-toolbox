<template>
    <div class="category-panel flex flex-col">
        <div class="toolbar flex justify-space-between align-center">
            <span class="toolbar-title no-select">Category</span>
            <Button
                class="toolbar-btn"
                icon="pi pi-plus-circle"
                variant="text"
                size="small"
                v-tooltip.bottom="{ value: 'Add category', class: 'btn-tooltip', showDelay: 700 }"
                @click="showAddDialog"
            />
        </div>
        <ScrollPanel class="category-list-scroll-panel scroll-panel" v-if="categoriesShown.length > 0">
            <ul class="category-list flex flex-col">
                <CategoryListItem
                    v-for="ct in categoriesShown"
                    :key="ct"
                    draggable="true"
                    :category="ct"
                    :selected="ct === selectedCategory"
                    :is-dragging="draggedCategory !== null"
                    @select="selectedCategory = ct"
                    @contextmenu="onContextMenu($event, ct)"
                    @dragstart="onDragStart($event, ct)"
                    @dragend="draggedCategory = null"
                    @dragover="onDragOver($event, ct)"
                    @drop="onDrop($event, ct)"
                />
            </ul>
        </ScrollPanel>
        <div class="flex justify-center align-center height-full" v-else>
            <span class="empty-placeholder-text no-select">No categories available</span>
        </div>
    </div>
    <Dialog class="width-dialog dialog-no-select" v-model:visible="addDialogVisible" modal header="Add category">
        <InputText class="width-full" size="small" v-model="dialogCategoryName" placeholder="Name" autofocus autocomplete="off" />
        <template #footer>
            <Button label="Cancel"  size="small" severity="secondary" @click="addDialogVisible = false" />
            <Button label="Add"  size="small" :disabled="dialogCategoryName.trim() === '' || categories.includes(dialogCategoryName)" @click="addCategory" />
        </template>
    </Dialog>
    <Dialog class="width-dialog dialog-no-select" v-model:visible="renameDialogVisible" modal header="Rename category">
        <InputText class="width-full" size="small" v-model="dialogCategoryName" placeholder="Name" autofocus autocomplete="off" />
        <template #footer>
            <Button label="Cancel" size="small" severity="secondary" @click="renameDialogVisible = false" />
            <Button label="Rename" size="small" :disabled="dialogCategoryName.trim() === '' || categories.includes(dialogCategoryName)" @click="renameCategory" />
        </template>
    </Dialog>
    <ContextMenu ref="category-menu" :model="categoryMenuItems" />
</template>

<script setup lang="ts">
import { ref, useTemplateRef, computed, onMounted, watch } from "vue";
import { useConfirm } from "primevue/useconfirm";
import { invoke } from "@tauri-apps/api/core";
import type { MenuItem } from "primevue/menuitem";
import { DnDItem, DropEffect } from "../types";
import { messageDialog } from "../util";
import { useSingleMenu } from "../stores";
import CategoryListItem from "./CategoryListItem.vue";
const confirm = useConfirm();
const singleMenu = useSingleMenu();
const menuId = "category-menu";

const categories = ref<string[]>([]);
const dialogCategoryName = ref("");
const selectedCategory = defineModel<string | null>({ default: null });
onMounted(async () => {
    categories.value = await invoke<string[]>("get_category_list");
    if (categories.value.length > 0) {
        selectedCategory.value = categories.value[0];
    }
});

const addDialogVisible = ref(false);
const showAddDialog = () => {
    dialogCategoryName.value = "";
    addDialogVisible.value = true;
};
const addCategory = async () => {
    if (await invoke<boolean>("add_category", { category: dialogCategoryName.value.trim() })) {
        categories.value.push(dialogCategoryName.value.trim());
        addDialogVisible.value = false;
    }
};

const renameDialogVisible = ref(false);
const showRenameDialog = () => {
    dialogCategoryName.value = selectedContextMenuCategory.value!;
    renameDialogVisible.value = true;
};
const renameCategory = async () => {
    if (selectedCategory.value === selectedContextMenuCategory.value) {
        selectedCategory.value = dialogCategoryName.value.trim();
    }
    if (await invoke<boolean>("rename_category", { category: selectedContextMenuCategory.value!, newCategory: dialogCategoryName.value.trim() })) {
        categories.value[categories.value.indexOf(selectedContextMenuCategory.value!)] = dialogCategoryName.value.trim();
        renameDialogVisible.value = false;
    }
};

const updateCategories = async (newCategories: string[]) => {
    if (await invoke<boolean>("update_categories", { newCategories })) {
        categories.value = newCategories;
    }
};

const confirmRemoval = () => {
    confirm.require({
        message: `Do you want to remove category "${selectedContextMenuCategory.value}"?`,
        header: "Remove category",
        icon: "pi pi-exclamation-circle",
        rejectLabel: 'Cancel',
        rejectProps: {
            label: 'Cancel',
            severity: 'secondary',
            outlined: true,
            size: 'small'
        },
        acceptProps: {
            label: 'Remove',
            severity: 'danger',
            size: 'small'
        },
        accept() {
            if (selectedCategory.value === selectedContextMenuCategory.value) {
                if (categories.value.length === 1)
                    selectedCategory.value = "";
                else
                    selectedCategory.value = categories.value[0] === selectedCategory.value ? categories.value[1] : categories.value[0];
            }
            updateCategories(categories.value.filter(c => c !== selectedContextMenuCategory.value));
            selectedContextMenuCategory.value = null;
        }
    });
};

const categoryMoveUp = () => {
    const index = categories.value.indexOf(selectedContextMenuCategory.value!);
    let newCategories = categories.value;
    [newCategories[index], newCategories[index - 1]] = [newCategories[index - 1], newCategories[index]];
    updateCategories(newCategories);
};
const categoryMoveDown = () => {
    const index = categories.value.indexOf(selectedContextMenuCategory.value!);
    let newCategories = categories.value;
    [newCategories[index], newCategories[index + 1]] = [newCategories[index + 1], newCategories[index]];
    updateCategories(newCategories);
};

const selectedContextMenuCategory = ref<string | null>(null);
const categoryMenu = useTemplateRef("category-menu");
const categoryMenuItems = ref<MenuItem[]>([
    { label: "Rename", icon: "pi pi-pencil", command: showRenameDialog },
    { label: "Move up", icon: "pi pi-angle-up", command: categoryMoveUp,
        visible: () => categories.value.indexOf(selectedContextMenuCategory.value!) !== 0
    },
    { label: "Move down", icon: "pi pi-angle-down", command: categoryMoveDown,
        visible: () => categories.value.indexOf(selectedContextMenuCategory.value!) !== categories.value.length - 1
    },
    { separator: true },
    { label: "Remove", icon: "pi pi-trash", command: confirmRemoval }
]);
const onContextMenu = (event: PointerEvent, category: string) => {
    selectedContextMenuCategory.value = category;
    singleMenu.open(menuId);
    categoryMenu.value?.show(event);
};
watch(singleMenu.getMenuId, newId => {
    if (newId !== menuId) {
        categoryMenu.value?.hide();
    }
});

const categoriesOnDrag = ref<string[]>([]);
const draggedCategory = ref<string | null>(null);
const categoriesShown = computed<string[]>(() => draggedCategory.value === null ? categories.value : categoriesOnDrag.value);
const onDragStart = (event: DragEvent, categoryName: string) => {
    let dndItem: DnDItem = { type: "category", name: categoryName };
    event.dataTransfer?.setData("dnditem", JSON.stringify(dndItem));
    draggedCategory.value = categoryName;
    categoriesOnDrag.value = categories.value.slice();
};
const onDragOver = (event: DragEvent, categoryName: string) => {
    event.preventDefault();
    let dropEffect: DropEffect = "none";
    if (draggedCategory.value) {
        dropEffect = "move";
        if (categoryName !== draggedCategory.value) {
            const draggedIndex = categoriesOnDrag.value.indexOf(draggedCategory.value!);
            const overIndex = categoriesOnDrag.value.indexOf(categoryName);
            //move categories
            categoriesOnDrag.value.splice(draggedIndex, 1);
            categoriesOnDrag.value.splice(overIndex, 0, draggedCategory.value!);
        }
    } else if (event.dataTransfer?.types.includes("dnditem") && selectedCategory.value !== categoryName) {
        dropEffect = "copy";
    }
    if (event.dataTransfer) {
        event.dataTransfer.dropEffect = dropEffect;
    }
};
const onDrop = async (event: DragEvent, categoryName: string) => {
    event.preventDefault();
    // Check if the order has changed
    if (draggedCategory.value && categoriesOnDrag.value[0] !== categories.value[0]) {
        updateCategories(categoriesOnDrag.value);
    } else if (event.dataTransfer?.types.includes("dnditem")) {
        let dndItem: DnDItem = JSON.parse(event.dataTransfer.getData("dnditem"));
        if (dndItem.type === "app" && selectedCategory.value !== categoryName) {
            if (!await invoke<boolean>("add_app_to_category", { app: dndItem.name, category: categoryName })) {
                messageDialog(confirm, "Add app", `The app "${dndItem.name}" already exists in category "${categoryName}".`, "warning");
            }
        }
    }
};
</script>

<style scoped>
.category-panel {
    width: 100%;
    background-color: rgb(255 255 255 / .13);
}
.category-list {
    margin: 0;
    padding: 0;
    list-style-type: none;
}
.category-list-scroll-panel { min-height: 0; }
</style>