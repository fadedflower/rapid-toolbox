<template>
    <div class="flex flex-col gap-2 height-full">
        <div class="app-list" @click="deselectApp" @contextmenu="onContextMenu($event)">
            <div class="app-toolbar toolbar flex justify-space-between align-center">
                <span class="toolbar-title no-select">{{ t("CategoryAppList.title") }}</span>
                <div class="flex gap-4">
                    <Button
                        class="toolbar-btn"
                        icon="pi pi-plus-circle"
                        variant="text"
                        size="small"
                        :disabled="category === null"
                        v-tooltip.bottom="{ value: t('CategoryAppList.addApps'), class: 'btn-tooltip', showDelay: 700 }"
                        @click="showAddDialog"
                    />
                    <Button
                        class="toolbar-btn"
                        icon="pi pi-sort"
                        variant="text"
                        size="small"
                        :disabled="category === null"
                        v-tooltip.bottom="{ value: t('CategoryAppList.sortApps'), class: 'btn-tooltip', showDelay: 700 }"
                        @click="sortApps"
                    />
                </div>
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
                <span class="empty-placeholder-text no-select">{{ emptyMessage }}</span>
            </div>
        </div>
        <div class="app-desc-bar no-select">{{ selectedAppDesc === "" ? t("CategoryAppList.noDescPlaceholder") : selectedAppDesc }}</div>
    </div>
    <Dialog class="width-dialog dialog-no-select" v-model:visible="addDialogVisible" modal :header="t('CategoryAppList.addApps')">
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
            :disabled="dialogApps.length === 0"
            :placeholder="dialogApps.length === 0 ? t('CategoryAppList.selectEmpty') : t('CategoryAppList.selectPlaceholder')"
            :empty-selection-message="t('CategoryAppList.selectEmptySelection')"
            :selected-items-label="t('CategoryAppList.selectMultipleSelection', ['{0}'])"
            :empty-filter-message="t('CategoryAppList.selectEmptyFilter')"
        >
            <template #option="slotProps">
                <div class="flex align-center">
                    <img class="add-app-icon" width="18" height="18" :src="slotProps.option.iconUrl" />
                    <span>{{ slotProps.option.name }}</span>
                </div>
            </template>
        </MultiSelect>
        <template #footer>
            <Button :label="t('DialogCommon.btnCancel')" size="small" severity="secondary" @click="addDialogVisible = false" />
            <Button :label="t('DialogCommon.btnAdd')" size="small" :disabled="dialogSelectedApps.length === 0" @click="addApps" />
        </template>
    </Dialog>
    <ContextMenu ref="app-menu" :model="appMenuItems" />
    <LibraryAppDialog v-model:visible="editAppDialogVisible" edit-mode :edit-app="selectedContextMenuAppMetadata" @update-app="editAppUpdate" />
</template>

<script setup lang="ts">
import { ref, computed, useTemplateRef, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useConfirm } from "primevue/useconfirm";
import { invoke } from "@tauri-apps/api/core";
import type { MenuItem } from "primevue/menuitem";
import { AppMetadata, DnDItem, DropEffect } from "../types";
import { useMessageDialog } from "../util";
import { useSingleMenu } from "../stores";
import GridAppItem from "./GridAppItem.vue";
import LibraryAppDialog from "./LibraryAppDialog.vue";
const { t } = useI18n();
const confirm = useConfirm();
const messageDialog = useMessageDialog();
const singleMenu = useSingleMenu();
const menuId = "app-menu";

const { category, searchKeyword } = defineProps<{ category: string | null, searchKeyword: string }>();

const appsShown = computed<AppMetadata[]>(() => {
    const appList = draggedApp.value ? appsOnDrag.value : apps.value;
    return searchKeyword.trim() === "" ? appList : appList.filter(app => app.name.toLowerCase().includes(searchKeyword.trim().toLowerCase()));
});
const emptyMessage = computed(() => {
    return searchKeyword === "" ? t("CategoryAppList.emptyPlaceholder") : t("CategoryAppList.notFoundPlaceholder");
});
const apps = ref<AppMetadata[]>([]);
const selectedApp = ref<string | null>();
const selectedAppDesc = computed(() => {
    return selectedApp.value ? apps.value.find(a => a.name === selectedApp.value)!.desc : t("CategoryAppList.selectAppInstruction");
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
        messageDialog(t("CategoryAppList.titleLaunchApp"), t("CategoryAppList.msgFailedToLaunchApp", [appName]), "error");
    }
};
const openFileLocation = async (appName: string) => {
    if (!await invoke<boolean>("open_app_file_location", { appName })) {
        messageDialog(t("CategoryAppList.titleOpenFileLocation"), t("CategoryAppList.msgFailedToOpenFileLocation", [appName]), "error");
    }
};
const updateApps = async (newApps: string[]) => {
    if (await invoke<boolean>("update_apps_in_category", { apps: newApps, category })) {
        reloadApps();
    }
};

const addDialogVisible = ref(false);
const editAppDialogVisible = ref(false);
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
const editAppUpdate = async (newApp: AppMetadata) => {
    if (selectedApp.value === selectedContextMenuApp.value) {
        await reloadApps();
        selectedApp.value = newApp.name;
    }
};

const deselectApp = (event: MouseEvent) => {
    if (event.target instanceof HTMLElement &&
        (event.target.classList.contains("p-scrollpanel-content") || event.target.classList.contains("app-list-items") || event.target.classList.contains("app-toolbar"))) {
        selectedApp.value = null;
    }
};

const sortApps = async () => {
    await updateApps(apps.value.sort((a, b) => Intl.Collator(undefined, { numeric: true }).compare(a.name, b.name)).map(app => app.name));
}

const confirmRemoval = () => {
    confirm.require({
        message: t("CategoryAppList.msgConfirmRemoval", [selectedContextMenuApp.value, category]),
        header: t("CategoryAppList.titleRemoveApp"),
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
            if (selectedApp.value === selectedContextMenuApp.value) {
                selectedApp.value = null;
            }
            updateApps(apps.value.filter(app => app.name !== selectedContextMenuApp.value).map(app => app.name));
        }
    });
};

const selectedContextMenuApp = ref<string | null>(null);
const selectedContextMenuAppMetadata = computed(() => {
    return selectedContextMenuApp.value ? apps.value.find(app => app.name === selectedContextMenuApp.value) : null;
});
const appMenu = useTemplateRef("app-menu");
const appMenuItems = computed<MenuItem[]>(() => [
    { label: t("CategoryAppList.menuLaunch"), icon: "pi pi-play", visible: () => selectedContextMenuApp.value !== null, command: () => launchApp(selectedContextMenuApp.value!) },
    { label: t("CategoryAppList.menuOpenFileLocation"), icon: "pi pi-folder-open", visible: () => selectedContextMenuApp.value !== null, command: () => openFileLocation(selectedContextMenuApp.value!) },
    { label: t("CategoryAppList.menuEdit"), icon: "pi pi-pencil", visible: () => selectedContextMenuApp.value !== null, command: () => editAppDialogVisible.value = true },
    { label: t("CategoryAppList.menuRemove"), icon: "pi pi-trash", visible: () => selectedContextMenuApp.value !== null, command: confirmRemoval },
    { separator: true, visible: () => selectedContextMenuApp.value !== null },
    { label: t("CategoryAppList.addApps"), icon: "pi pi-plus-circle", command: showAddDialog },
    { label: t("CategoryAppList.sortApps"), icon: "pi pi-sort", command: sortApps }

]);
const onContextMenu = (event: MouseEvent, appName?: string) => {
    selectedContextMenuApp.value = appName || null;
    singleMenu.open(menuId);
    appMenu.value?.show(event);
};
watch(singleMenu.getMenuId, newId => {
    if (newId !== menuId) {
        appMenu.value?.hide();
    }
});

const appsOnDrag = ref<AppMetadata[]>([]);
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