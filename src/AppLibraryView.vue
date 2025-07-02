<template>
    <main class="flex height-full">
        <DataTable
            id="app-library-table"
            class="width-full no-select"
            :value="apps"
            size="small"
            data-key="name"
            sort-field="name"
            :sort-order="1"
            scroll-height="calc(var(--view-height) - 46px)"
            selectionMode="single"
            :metaKeySelection="false"
            v-model:selection="selectedApp"
            v-model:context-menu-selection="selectedContextMenuApp"
            v-model:filters="filters"
            :global-filter-fields="['name']"
            table-style="max-width: 100%"
            @row-contextmenu="onContextMenu"
            @keydown.delete="removeOnKeyDown"
            @keydown.e="editOnKeyDown"
            @keydown.up="navigateOnKeyDown"
            @keydown.down="navigateOnKeyDown"
            scrollable
            resizable-columns
            show-gridlines
            striped-rows
            context-menu
        >
            <template #header>
                <div class="table-header flex justify-space-between align-center" @click="deselectApp">
                    <span class="table-header-title no-select">App library</span>
                    <div class="flex gap-4">
                        <Button
                            class="table-header-btn"
                            icon="pi pi-plus-circle"
                            size="small"
                            severity="primary"
                            text
                            v-tooltip.bottom="{ value: 'Add app', class: 'btn-tooltip', showDelay: 700 }"
                            @click="showAddAppDialog"
                        />
                        <Button
                            v-show="selectedApp !== null"
                            class="table-header-btn"
                            icon="pi pi-pencil"
                            size="small"
                            severity="primary"
                            text
                            v-tooltip.bottom="{ value: 'Edit app', class: 'btn-tooltip', showDelay: 700 }"
                            @click="showEditAppDialog"
                        />
                        <Button
                            v-show="selectedApp !== null"
                            class="table-header-btn"
                            icon="pi pi-trash"
                            size="small"
                            severity="primary"
                            text
                            v-tooltip.bottom="{ value: 'Remove app', class: 'btn-tooltip', showDelay: 700 }"
                            @click="confirmRemoval"
                        />
                    </div>
                </div>
            </template>
            <template #empty>
                <span>{{ emptyMessage }}</span>
            </template>
            <Column header="Icon" :pt="{ columnHeaderContent: 'justify-center' }">
                <template #body="slotProps">
                    <div class="flex justify-center align-center">
                        <img :src="slotProps.data.iconUrl" width="24" height="24" />
                    </div>
                </template>
            </Column>
            <Column header="Name" field="name"></Column>
            <Column header="Description" field="desc"></Column>
            <Column header="App path" field="appPath"></Column>
            <Column header="Launch args" field="launchArgs"></Column>
            <Column header="Working directory" field="workingDir"></Column>
        </DataTable>
    </main>
    <LibraryAppDialog v-model:visible="dialogVisible" :apps="apps" :edit-mode="dialogEditMode" :edit-app="selectedApp" @update-app="onUpdateApp" />
    <ContextMenu ref="app-menu" :model="appMenuItems" />
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed, useTemplateRef } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { DataTableFilterMeta, DataTableFilterMetaData, DataTableRowContextMenuEvent } from 'primevue/datatable';
import { useConfirm } from 'primevue/useconfirm';
import type { MenuItem } from "primevue/menuitem";
import { FilterMatchMode } from '@primevue/core/api';
import { AppMetadata } from './types';
import { useSingleMenu } from "./stores";
import LibraryAppDialog from './components/LibraryAppDialog.vue';
const confirm = useConfirm();
const singleMenu = useSingleMenu();
const menuId = "library-app-menu";

const { searchKeyword } = defineProps<{ searchKeyword: string }>();
const apps = ref<AppMetadata[]>([]);
const sortedApps = computed(() => apps.value.sort((a, b) => Intl.Collator(undefined, { numeric: true }).compare(a.name, b.name)));
const selectedApp = ref<AppMetadata | null>(null);
const filters = ref<DataTableFilterMeta>({
    global: { value: searchKeyword, matchMode: FilterMatchMode.CONTAINS }
});
watch(() => searchKeyword, newKeyword => {
    (filters.value["global"] as DataTableFilterMetaData).value = newKeyword;
});
const emptyMessage = computed(() => {
    return searchKeyword === "" ? "No apps available. Add apps by clicking on the plus button." : "No apps found.";
});

const deselectApp = (event: MouseEvent) => {
    if (event.target instanceof HTMLElement && 
        (event.target.classList.contains('table-header') || event.target.classList.contains('table-header-title'))) {
        selectedApp.value = null;
    }
};

const dialogVisible = ref(false);
const dialogEditMode = ref(false);
const showAddAppDialog = () => {
    dialogEditMode.value = false;
    dialogVisible.value = true;
};
const showEditAppDialog = () => {
    dialogEditMode.value = true;
    dialogVisible.value = true;
};

const onUpdateApp = async (newApp: AppMetadata) => {
    await reloadAppList();
    selectedApp.value = newApp;
};
const confirmRemoval = () => {
    confirm.require({
        message: `Do you want to remove app "${selectedApp.value?.name}"?`,
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
        async accept() {
            if (await invoke('remove_app', { appName: selectedApp.value?.name })) {
                apps.value = apps.value.filter(app => app.name !== selectedApp.value?.name);
                selectedApp.value = null;
            }
        }
    });
};

const reloadAppList = async () => {
    apps.value = await invoke('get_all_app_list');
};

const selectedContextMenuApp = ref<AppMetadata | null>(null);
const appMenu = useTemplateRef("app-menu");
const appMenuItems = ref<MenuItem[]>([
    { label: "Edit", icon: "pi pi-pencil", command: showEditAppDialog },
    { label: "Remove", icon: "pi pi-trash", command: confirmRemoval }
]);
const onContextMenu = (event: DataTableRowContextMenuEvent) => {
    selectedApp.value = selectedContextMenuApp.value;
    singleMenu.open(menuId);
    appMenu.value?.show(event.originalEvent);
};

const removeOnKeyDown = () => {
    if (selectedApp.value !== null && !dialogVisible.value) {
        confirmRemoval();
    }
};
const editOnKeyDown = () => {
    if (selectedApp.value !== null && !dialogVisible.value) {
        showEditAppDialog();
    }
};
const navigateOnKeyDown = (event: KeyboardEvent) => {
    if (selectedApp.value !== null) {
        let index = sortedApps.value.indexOf(selectedApp.value);
        if (event.key === "ArrowUp" && index > 0) {
            selectedApp.value = sortedApps.value[index - 1];
        } else if (event.key === "ArrowDown" && index < sortedApps.value.length - 1) {
            selectedApp.value = sortedApps.value[index + 1];
        }
    }
};

watch(singleMenu.getMenuId, newId => {
    if (newId !== menuId) {
        appMenu.value?.hide();
    }
});

onMounted(async () => {
    await reloadAppList();
});
</script>

<style scoped>
#app-library-table {
    --p-datatable-header-color: white;
    --p-datatable-header-background: rgb(255 255 255 / .1);
    --p-datatable-header-cell-background: var(--p-zinc-100);
    --p-datatable-header-cell-selected-background: var(--p-zinc-100);
    --p-datatable-row-color: white;
    --p-datatable-row-background: rgb(255 255 255 / .1);
    --p-datatable-row-striped-background: rgb(255 255 255 / .08);
    --p-datatable-row-hover-color: white;
    --p-datatable-row-hover-background: rgb(255 255 255 / .25);
    --p-datatable-row-selected-color: white;
    --p-datatable-row-selected-background: rgb(255 255 255 / .38);
    --p-datatable-header-border-color: rgb(0 0 0 / 0);
    --p-datatable-header-cell-border-color: rgb(0 0 0 / 0);
    --p-datatable-body-cell-border-color: rgb(255 255 255 / .25);
    --p-datatable-row-focus-ring-style: none;
    overflow: hidden; /* prevent scrollbar from showing while resizing columns */
}
.table-header-btn {
    --p-button-text-primary-color: white;
    --p-button-text-primary-hover-background: rgb(255 255 255 / .3);
    --p-button-text-primary-active-background: rgb(255 255 255 / .5);
}
</style>

<style>
#app-library-table .p-datatable-tbody,
#app-library-table .p-datatable-column-title { font-size: 0.8rem; }
#app-library-table .p-datatable-tbody td { text-overflow: ellipsis; }
/* icon column */
#app-library-table th:nth-child(1),
#app-library-table td:nth-child(1) {
    max-width: 30px;
}
/* name column */
#app-library-table th:nth-child(2),
#app-library-table td:nth-child(2) {
    max-width: 250px;
}
/* desc column */
#app-library-table th:nth-child(3),
#app-library-table td:nth-child(3) {
    max-width: 250px;
}
/* app path column */
#app-library-table th:nth-child(4),
#app-library-table td:nth-child(4) {
    max-width: 175px;
}
/* launch args column */
#app-library-table th:nth-child(5),
#app-library-table td:nth-child(5) {
    max-width: 100px;
}
/* working dir column */
#app-library-table th:nth-child(6),
#app-library-table td:nth-child(6) {
    max-width: 170px;
}
</style>