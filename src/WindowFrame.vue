<template>
    <div id="app-container" class="width-app height-app flex flex-col">
        <Toolbar id="title-bar" data-tauri-drag-region>
            <template #start>
                <span id="title" class="no-select" data-tauri-drag-region>Rapid Toolbox</span>
            </template>
            <template #center>
                <IconField id="search-bar">
                    <InputIcon class="pi pi-search" />
                    <InputText size="small" placeholder="Search for apps" autocomplete="off" />
                </IconField>
            </template>
            <template #end>
                <ButtonGroup>
                    <Button class="window-control" icon="pi pi-bars" variant="text" size="small" @click="" />
                    <Button class="window-control" icon="pi pi-minus" variant="text" size="small" @click="appWindow.minimize();" />
                    <Button class="window-control" icon="pi pi-times" severity="danger" variant="text" size="small" @click="appWindow.close();" />
                </ButtonGroup>
            </template>
        </Toolbar>
        <component v-if="configLoaded" :is="LauncherView" />
    </div>
    <ConfirmDialog />
    <BlockUI :blocked="!configLoaded" full-screen />
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useConfirm } from "primevue/useconfirm";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';
import { moveWindow, Position } from '@tauri-apps/plugin-positioner';
import { messageDialog } from "./util";
import LauncherView from "./LauncherView.vue";

const appWindow = getCurrentWindow();
const confirm = useConfirm();
const configLoaded = ref(false);

onMounted(async () => {
    moveWindow(Position.Center);
    // show the window after a short delay to prevent white screen on startup
    setTimeout(async () => { await invoke("show_window") }, 50);
    if (!await invoke<boolean>('load_config')) {
        messageDialog(confirm, "Config", "Failed to load config. Please ensure the config file is valid.", "error", () => appWindow.close());
    } else {
        configLoaded.value = true;
    }
});
</script>

<style scoped>

#app-container {
    background: linear-gradient(to top right, #2854b5, #14c0d3);
}

#title-bar {
    --p-toolbar-padding: 0;
    --p-toolbar-border-radius: 0;
    --p-toolbar-background: rgb(255 255 255 / .2);
    border: none;
}

#title {
    margin-left: 10px;
    color: white;
    font-size: medium;
}

#search-bar {
    --p-inputtext-color: var(--p-gray-100);
    --p-inputtext-background: rgb(from var(--p-gray-300) r g b / .2);
    --p-inputtext-sm-font-size: 12px;
    --p-inputtext-sm-padding-y: 3px;
    --p-inputtext-placeholder-color: var(--p-gray-300);
    --p-iconfield-icon-color: var(--p-gray-200);
    --p-inputtext-border-color: rgb(0 0 0 / 0);
    --p-inputtext-hover-border-color: var(--p-gray-200);
}

#search-bar .p-inputtext { width: 400px; }

.window-control {
    --p-button-border-radius: 0;
    --p-button-text-primary-color: white;
    --p-button-text-primary-hover-background: rgb(255 255 255 / .3);
    --p-button-text-primary-active-background: rgb(255 255 255 / .5);
    --p-button-text-danger-color: white;
    --p-button-text-danger-hover-background: var(--p-rose-500);
    --p-button-text-danger-active-background: var(--p-rose-600);
}
</style>