<template>
    <div id="app-container" class="width-app height-app flex flex-col" :style="themeStyle">
        <Toolbar id="title-bar" data-tauri-drag-region @dragover="preventDndAction" @drop="preventDndAction">
            <template #start>
                <span id="title" class="no-select" data-tauri-drag-region>{{ configBasicInfo.headerText }}</span>
            </template>
            <template #center>
                <IconField id="search-bar">
                    <InputIcon class="pi pi-search" />
                    <InputText v-model="searchKeyword" size="small" :placeholder="t('WindowFrame.searchPlaceholder')" autocomplete="off" />
                </IconField>
            </template>
            <template #end>
                <ButtonGroup>
                    <Button class="window-control" icon="pi pi-bars" variant="text" size="small" @click="openMenu" />
                    <Button class="window-control" icon="pi pi-minus" variant="text" size="small" @click="appWindow.minimize();" />
                    <Button class="window-control" icon="pi pi-times" severity="danger" variant="text" size="small" @click="appWindow.close();" />
                </ButtonGroup>
            </template>
        </Toolbar>
        <component v-if="configLoaded" :is="currentViewComponent" :search-keyword="searchKeyword" />
    </div>
    <SettingsDialog
        v-model:visible="settingsDialogVisible"
        :config-basic-info="configBasicInfo"
        @update-basic-info="info => configBasicInfo = info"
        @update-settings-theme="theme => dialogSettingsTheme = theme"
    />
    <AboutDialog v-model:visible="aboutDialogVisible" :config-basic-info="configBasicInfo" />
    <ConfirmDialog class="no-select" />
    <Menu ref="frame-menu" :model="frameMenuItems" popup />
    <BlockUI :blocked="!configLoaded" full-screen />
</template>

<script setup lang="ts">
import { ref, onMounted, computed, useTemplateRef, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { MenuItem } from "primevue/menuitem";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';
import { moveWindow, Position } from '@tauri-apps/plugin-positioner';
import { ConfigBasicInfo, Theme } from "./types";
import { useMessageDialog, getThemeStyle, preventDndAction } from "./util";
import { useSingleMenu, useAppList } from "./stores";
import themePresets from "./themes";
import LauncherView from "./LauncherView.vue";
import AppLibraryView from "./AppLibraryView.vue";
import SettingsDialog from "./components/SettingsDialog.vue";
import AboutDialog from "./components/AboutDialog.vue";
const { t, locale } = useI18n();
const appWindow = getCurrentWindow();
const messageDialog = useMessageDialog();
const singleMenu = useSingleMenu();
const menuId = "frame-menu";
const appListStore = useAppList();

const configLoaded = ref(false);
const configBasicInfo = ref<ConfigBasicInfo>({
    lang: "en",
    headerText: "Rapid Toolbox",
    author: null,
    toolboxVersion: null,
    theme: themePresets[0].theme
});
const themeStyle = computed(() => settingsDialogVisible.value ? getThemeStyle(dialogSettingsTheme.value) : getThemeStyle(configBasicInfo.value.theme));

const currentView = ref<"launcher" | "appLibrary">("launcher");
const currentViewComponent = computed(() => currentView.value === "launcher" ? LauncherView : AppLibraryView);
const searchKeyword = ref("");

const frameMenu = useTemplateRef("frame-menu");
const toggleViewMenuItem = computed<MenuItem>(() => {
    return currentView.value === "launcher" ? {
        label: t("WindowFrame.menuAppLibrary"),
        icon: "pi pi-list",
        command: () => {
            searchKeyword.value = "";
            currentView.value = "appLibrary";
        }
    } : {
        label: t("WindowFrame.menuLauncher"),
        icon: "pi pi-objects-column",
        command: () => {
            searchKeyword.value = "";
            currentView.value = "launcher";
        }
    };
});
const frameMenuItems = computed<MenuItem[]>(() => [
    toggleViewMenuItem.value,
    { label: t("WindowFrame.menuSettings"), icon: "pi pi-cog", command: () => settingsDialogVisible.value = true },
    { label: t("WindowFrame.menuAbout"), icon: "pi pi-info-circle", command: () => aboutDialogVisible.value = true }
]);
const openMenu = (event: MouseEvent) => {
    singleMenu.open(menuId);
    frameMenu.value?.show(event);
};
watch(singleMenu.getMenuId, newId => {
    if (newId !== menuId) {
        frameMenu.value?.hide();
    }
});

const aboutDialogVisible = ref(false);
const settingsDialogVisible = ref(false);
const dialogSettingsTheme = ref<Theme>(themePresets[0].theme);

onMounted(async () => {
    moveWindow(Position.Center);
    // show the window after a short delay to prevent white screen on startup
    setTimeout(async () => { await invoke("show_window") }, 50);
    if (!await invoke<boolean>("load_config")) {
        messageDialog(t("WindowFrame.titleConfig"), t("WindowFrame.msgFailedToLoadConfig"), "error", () => appWindow.close());
    } else {
        configBasicInfo.value = await invoke<ConfigBasicInfo>("get_config_basic_info");
        locale.value = configBasicInfo.value.lang;
        appListStore.reloadApps();
        configLoaded.value = true;
    }
});
</script>

<style scoped>
#title-bar {
    --p-toolbar-padding: 0;
    --p-toolbar-border-radius: 0;
    --p-toolbar-background: rgb(255 255 255 / .2);
    border: none;
}

#title {
    max-width: 11rem;
    margin-left: 10px;
    color: white;
    font-size: medium;
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
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
    --p-inputtext-focus-border-color: var(--p-gray-200);
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