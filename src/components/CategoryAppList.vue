<template>
    <div class="flex flex-col gap-2 height-full">
        <div class="app-list" @click="deselectApp">
            <div class="toolbar flex justify-space-between align-center">
                <span class="toolbar-title no-select">Apps</span>
                <Button
                    class="toolbar-btn"
                    icon="pi pi-plus-circle"
                    variant="text"
                    size="small"
                    :disabled="category === null"
                    v-tooltip.bottom="{ value: 'Add apps', class: 'btn-tooltip', showDelay: 700 }"
                    @click="showAddDialog"
                />
            </div>
            <ScrollPanel v-if="appsShown.length > 0" class="app-list-scroll-panel scroll-panel">
                <div class="app-list-items flex flex-wrap">
                    <GridAppItem
                        v-for="app in appsShown"
                        :key="app.name"
                        draggable="true"
                        :app="app"
                        :selected="app.name === selectedApp"
                        @select="selectedApp = app.name"
                        @launch="launchApp(app.name)"
                        @contextmenu="onContextMenu($event, app.name)"
                        @dragstart="onDragStart($event, app.name)"
                        @dragend="draggedApp = null"
                        @dragover="onDragOver($event, app.name)"
                        @drop="onDrop"
                    />
                </div>
            </ScrollPanel>
            <div v-else class="flex justify-center align-center height-full">
                <span class="empty-placeholder-text no-select">No apps available</span>
            </div>
        </div>
        <div class="app-desc-bar no-select">{{ selectedAppDesc === "" ? "< No description >" : selectedAppDesc }}</div>
    </div>
    <Dialog class="width-dialog dialog-no-select" v-model:visible="addDialogVisible" modal header="Add apps">
        <MultiSelect
            class="width-full"
            size="small"
            scroll-height="200px"
            :max-selected-labels="3"
            v-model="dialogSelectedApps"
            :options="dialogApps"
            optionLabel="name"
            filter
            showClear
            placeholder="Select apps"
            empty-selection-message = "No selected app"
            empty-message="No available apps"
            selected-items-label="{0} apps selected"
        >
            <template #option="slotProps">
                <div class="flex align-center">
                    <img class="add-app-icon" width="18" height="18" :src="slotProps.option.iconUrl" />
                    <span>{{ slotProps.option.name }}</span>
                </div>
            </template>
        </MultiSelect>
        <template #footer>
            <Button label="Cancel" size="small" severity="secondary" @click="addDialogVisible = false" />
            <Button label="Add" size="small" @click="addApps" />
        </template>
    </Dialog>
    <ContextMenu ref="app-menu" :model="appMenuItems" />
</template>

<script setup lang="ts">
import { ref, computed, useTemplateRef, watch } from "vue";
import { useConfirm } from "primevue/useconfirm";
import { invoke } from "@tauri-apps/api/core";
import type { MenuItem } from "primevue/menuitem";
import { AppMetadata, DnDItem, DropEffect } from "../types";
import { messageDialog } from "../util";
import { useSingleMenu } from "../stores";
import GridAppItem from "./GridAppItem.vue";
const confirm = useConfirm();
const singleMenu = useSingleMenu();
const menuId = "app-menu";

const { category } = defineProps<{ category: string | null }>();

const apps = ref<AppMetadata[]>([]);
const selectedApp = ref<string | null>();
const selectedAppDesc = computed(() => {
    return selectedApp.value ? apps.value.find(a => a.name === selectedApp.value)!.desc : "Click on an app to show its description. Double-click to launch it.";
});
const reloadApps = async () => {
    apps.value = [];
    if (category !== null) {
        selectedApp.value = null;
        if (!category) return;
        const list = await invoke<AppMetadata[] | null>("get_app_list_by_category", { category: category });
        apps.value = list || [];
    }
};
watch(() => category, reloadApps);

const launchApp = async (appName: string) => {
    if (!await invoke<boolean>("launch_app", { appName })) {
        messageDialog(confirm, "Launch app", `Failed to launch app "${appName}". Please ensure the app config is correct.`, "error");
    }
};
const updateApps = async (newApps: string[]) => {
    if (await invoke<boolean>("update_apps_in_category", { apps: newApps, category })) {
        reloadApps();
    }
};

const addDialogVisible = ref(false);
const dialogApps = ref<AppMetadata[]>([]);
const dialogSelectedApps = ref<AppMetadata[]>([]);
const showAddDialog = async () => {
    dialogSelectedApps.value = [];
    let availableApps = await invoke<AppMetadata[] | null>("get_available_app_list_by_category", { category });
    if (availableApps) {
        dialogApps.value = availableApps;
        addDialogVisible.value = true;
    }
};
const addApps = async () => {
    if (await invoke<boolean>("add_app_list_to_category", { apps: dialogSelectedApps.value.map(app => app.name), category })) {
        await reloadApps();
        addDialogVisible.value = false;
    }
};

const deselectApp = (event: MouseEvent) => {
    if (event.target instanceof HTMLElement &&
        (event.target.classList.contains("p-scrollpanel-content") || event.target.classList.contains("app-list-items") || event.target.classList.contains("app-toolbar"))) {
        selectedApp.value = null;
    }
};

const confirmRemoval = () => {
    confirm.require({
        message: `Do you want to remove app "${selectedContextMenuApp.value}"?`,
        header: "Remove app",
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
            if (selectedApp.value === selectedContextMenuApp.value) {
                selectedApp.value = null;
            }
            updateApps(apps.value.filter(app => app.name !== selectedContextMenuApp.value).map(app => app.name));
            selectedContextMenuApp.value = null;
        }
    });
};

const selectedContextMenuApp = ref<string | null>(null);
const appMenu = useTemplateRef("app-menu");
const appMenuItems = ref<MenuItem[]>([
    { label: "Launch", icon: "pi pi-play", command: () => launchApp(selectedContextMenuApp.value!) },
    { label: "Remove", icon: "pi pi-trash", command: confirmRemoval }
]);
const onContextMenu = (event: PointerEvent, appName: string) => {
    selectedContextMenuApp.value = appName;
    singleMenu.open(menuId);
    appMenu.value?.show(event);
};
watch(singleMenu.getMenuId, newId => {
    if (newId !== menuId) {
        appMenu.value?.hide();
    }
});

const appsOnDrag = ref<AppMetadata[]>([]);
const appsShown = computed<AppMetadata[]>(() => draggedApp.value ? appsOnDrag.value : apps.value);
const draggedApp = ref<string | null>(null);
const onDragStart = (event: DragEvent, appName: string) => {
    let dndItem: DnDItem = { type: "app", name: appName };
    event.dataTransfer?.setData("dnditem", JSON.stringify(dndItem));
    appsOnDrag.value = apps.value.slice();
    draggedApp.value = appName;
};
const onDragOver = (event: DragEvent, appName: string) => {
    event.preventDefault();
    let dropEffect: DropEffect = "none";
    if (draggedApp.value) {
        dropEffect = "move";
        if (draggedApp.value !== appName) {
            const draggedIndex = appsOnDrag.value.findIndex(app => app.name === draggedApp.value);
            const draggedMetadata = appsOnDrag.value[draggedIndex];
            const overIndex = appsOnDrag.value.findIndex(app => app.name === appName);
            // move apps
            appsOnDrag.value.splice(draggedIndex, 1);
            appsOnDrag.value.splice(overIndex, 0, draggedMetadata);
        }
    }
    if (event.dataTransfer) {
        event.dataTransfer.dropEffect = dropEffect;
    }
};
const onDrop = (event: DragEvent) => {
    event.preventDefault();
    const draggedAppIndex = apps.value.findIndex(app => app.name === draggedApp.value);
    const newDraggedAppIndex = appsOnDrag.value.findIndex(app => app.name === draggedApp.value);
    // Check if the order has changed
    if (draggedApp.value && draggedAppIndex !== newDraggedAppIndex) {
        updateApps(appsOnDrag.value.map(app => app.name));
    }
};
</script>

<style scoped>
.app-list {
    height: calc(100% - 28px - 4px - 2px);
    margin-right: 4px;
    background-color: rgb(255 255 255 / .13);
}
.app-list-items {
    padding: var(--p-list-padding);
    padding-left: .6rem;
}
.app-list-scroll-panel { height: calc(100% - 28px - 4px - 2px); }
.app-desc-bar {
    height: 28px;
    align-content: center;
    margin-right: 4px;
    margin-bottom: 4px;
    padding: 0 .6rem;
    color: white;
    font-size: .8rem;
    background-color: rgb(255 255 255 / .13);
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
}
.add-app-icon { margin-right: 5px }
</style>