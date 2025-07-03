<template>
    <div class="category-panel flex flex-col" @contextmenu="onContextMenu($event)">
        <div class="toolbar flex justify-space-between align-center">
            <span class="toolbar-title no-select">{{ t('CategoryList.title') }}</span>
            <div class="flex gap-4">
                <Button
                    class="toolbar-btn"
                    icon="pi pi-plus-circle"
                    variant="text"
                    size="small"
                    v-tooltip.bottom="{ value: t('CategoryList.addCategory'), class: 'btn-tooltip', showDelay: 700 }"
                    @click="showAddDialog"
                />
                <Button
                    class="toolbar-btn"
                    icon="pi pi-sort"
                    variant="text"
                    size="small"
                    v-tooltip.bottom="{ value: t('CategoryList.sortCategories'), class: 'btn-tooltip', showDelay: 700 }"
                    @click="sortCategories"
                />
            </div>
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
            <span class="empty-placeholder-text no-select">{{ t('CategoryList.emptyPlaceholder') }}</span>
        </div>
    </div>
    <Dialog class="width-dialog dialog-no-select" v-model:visible="addDialogVisible" modal :header="t('CategoryList.addCategory')">
        <InputText class="width-full" size="small" v-model="dialogCategoryName" :placeholder="t('CategoryList.dialogPlaceholder')" autofocus autocomplete="off" />
        <template #footer>
            <Button :label="t('DialogCommon.btnCancel')" size="small" severity="secondary" @click="addDialogVisible = false" />
            <Button :label="t('DialogCommon.btnAdd')" size="small" :disabled="dialogCategoryName.trim() === '' || categories.includes(dialogCategoryName)" @click="addCategory" />
        </template>
    </Dialog>
    <Dialog class="width-dialog dialog-no-select" v-model:visible="renameDialogVisible" modal :header="t('CategoryList.renameCategory')">
        <InputText class="width-full" size="small" v-model="dialogCategoryName" :placeholder="t('CategoryList.dialogPlaceholder')" autofocus autocomplete="off" />
        <template #footer>
            <Button :label="t('DialogCommon.btnCancel')" size="small" severity="secondary" @click="renameDialogVisible = false" />
            <Button :label="t('DialogCommon.btnRename')" size="small" :disabled="dialogCategoryName.trim() === '' || categories.includes(dialogCategoryName)" @click="renameCategory" />
        </template>
    </Dialog>
    <ContextMenu ref="category-menu" :model="categoryMenuItems" />
</template>

<script setup lang="ts">
import { ref, useTemplateRef, computed, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useConfirm } from "primevue/useconfirm";
import { invoke } from "@tauri-apps/api/core";
import type { MenuItem } from "primevue/menuitem";
import { DnDItem, DropEffect } from "../types";
import { useMessageDialog } from "../util";
import { useSingleMenu } from "../stores";
import CategoryListItem from "./CategoryListItem.vue";
const { t } = useI18n();
const confirm = useConfirm();
const messageDialog = useMessageDialog();
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
        if (categories.value.length === 1)
            selectedCategory.value = dialogCategoryName.value.trim();
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

const sortCategories = async () => {
    await updateCategories(categories.value.sort((a, b) => Intl.Collator(undefined, { numeric: true }).compare(a, b)));
}

const confirmRemoval = () => {
    confirm.require({
        message: t("CategoryList.msgConfirmRemoval", [selectedContextMenuCategory.value]),
        header: t("CategoryList.titleRemoveCategory"),
        icon: "pi pi-exclamation-circle",
        rejectLabel: t("DialogCommon.btnCancel"),
        rejectProps: {
            severity: "secondary",
            outlined: true,
            size: "small"
        },
        acceptLabel: t("DialogCommon.btnRemove"),
        acceptProps: {
            severity: "danger",
            size: "small"
        },
        accept() {
            if (selectedCategory.value === selectedContextMenuCategory.value) {
                if (categories.value.length === 1)
                    selectedCategory.value = null;
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
const categoryMenuItems = computed<MenuItem[]>(() => [
    { label: t("CategoryList.menuRename"), icon: "pi pi-pencil", visible: () => selectedContextMenuCategory.value !== null, command: showRenameDialog },
    { label: t("CategoryList.menuRemove"), icon: "pi pi-trash", visible: () => selectedContextMenuCategory.value !== null, command: confirmRemoval },
    { label: t("CategoryList.menuMoveUp"), icon: "pi pi-angle-up", command: categoryMoveUp,
        visible: () => selectedContextMenuCategory.value !== null && 
            categories.value.indexOf(selectedContextMenuCategory.value!) !== 0
    },
    { label: t("CategoryList.menuMoveDown"), icon: "pi pi-angle-down", command: categoryMoveDown,
        visible: () => selectedContextMenuCategory.value !== null && 
            categories.value.indexOf(selectedContextMenuCategory.value!) !== categories.value.length - 1
    },
    { separator: true, visible: () => selectedContextMenuCategory.value !== null },
    { label: t("CategoryList.addCategory"), icon: "pi pi-plus-circle", command: showAddDialog },
    { label: t("CategoryList.sortCategories"), icon: "pi pi-sort", command: sortCategories }

]);
const onContextMenu = (event: MouseEvent, category?: string) => {
    selectedContextMenuCategory.value = category || null;
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
    const draggedCategoryIndex = categories.value.indexOf(draggedCategory.value!);
    const newDraggedCategoryIndex = categoriesOnDrag.value.indexOf(draggedCategory.value!);
    // Check if the order has changed
    if (draggedCategory.value && draggedCategoryIndex !== newDraggedCategoryIndex) {
        updateCategories(categoriesOnDrag.value);
    } else if (event.dataTransfer?.types.includes("dnditem")) {
        let dndItem: DnDItem = JSON.parse(event.dataTransfer.getData("dnditem"));
        if (dndItem.type === "app" && selectedCategory.value !== categoryName) {
            if (!await invoke<boolean>("add_app_to_category", { app: dndItem.name, category: categoryName })) {
                messageDialog(t("CategoryList.titleAddApp"), t("CategoryList.msgAppExists", [dndItem.name, categoryName]), "warning");
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