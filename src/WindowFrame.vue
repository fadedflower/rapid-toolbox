<template>
    <div id="app-container" class="width-app height-app flex flex-col" :style="themeStyle">
        <Toolbar id="title-bar" data-tauri-drag-region @dragover="preventDndAction" @drop="preventDndAction">
            <template #start>
                <span id="title" class="no-select" data-tauri-drag-region>{{ configBasicInfo.headerText }}</span>
            </template>
            <template #center>
                <IconField id="search-bar">
                    <InputIcon class="pi pi-search" />
                    <InputText v-model="searchKeyword" size="small" placeholder="Search for apps" autocomplete="off" />
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
    <Dialog class="width-dialog dialog-no-select" v-model:visible="settingsDialogVisible" modal header="Settings">
        <div class="flex flex-col gap-8">
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-header-text">Header text</label>
                <InputText id="dialog-header-text" class="flex-grow" size="small" v-model="dialogSettings.headerText" placeholder="Required" autocomplete="off" />
            </div>
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-author">Author</label>
                <InputText id="dialog-author" class="flex-grow" size="small" v-model="dialogSettings.author" placeholder="Optional" autocomplete="off" />
            </div>
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-toolbox-version">Toolbox version</label>
                <InputText id="dialog-toolbox-version" class="flex-grow" size="small" v-model="dialogSettings.toolboxVersion" placeholder="major.minor, Optional" autocomplete="off" />
            </div>
        </div>
        <Divider align="center" type="solid">
            <span class="no-select">Theme</span>
        </Divider>
        <div class="flex flex-col gap-8">
            <div class="flex align-center">
                <span class="dialog-label no-select" for="dialog-theme-preset">Preset</span>
                <Select id="dialog-theme-preset" class="flex-grow" size="small" v-model="dialogSettings.theme" :options="dialogThemePresets" option-label="name" option-value="theme" />
            </div>
            <div class="flex align-center">
                <span class="dialog-label no-select">Background type</span>
                <Select id="dialog-theme-type" class="flex-grow" size="small" v-model="dialogSelectedThemeType" :options="dialogThemeTypes" option-label="name" option-value="type" />
            </div>
            <div v-show="dialogSelectedThemeType === 'Solid'" class="flex align-center">
                <span class="flex-grow dialog-label no-select">Color</span>
                <ThemeColorPicker v-model="dialogThemeColor1" />
            </div>
            <div v-show="dialogSelectedThemeType !== 'Solid'" class="flex align-center">
                <span class="flex-grow dialog-label no-select">Color 1</span>
                <ThemeColorPicker v-model="dialogThemeColor1" />
            </div>
            <div v-show="dialogSelectedThemeType !== 'Solid'" class="flex align-center">
                <span class="flex-grow dialog-label no-select">Color 2</span>
                <ThemeColorPicker v-model="dialogThemeColor2" />
            </div>
        </div>
        <template #footer>
            <Button label="Cancel" size="small" severity="secondary" @click="settingsDialogVisible = false" />
            <Button label="Save" size="small" :disabled="!dialogSettingsValid" @click="saveSettings" />
        </template>
    </Dialog>
    <Dialog class="width-dialog dialog-no-select" v-model:visible="aboutDialogVisible" modal header="About">
        <div class="flex flex-col no-select">
            <span>{{ configBasicInfo.headerText }}</span>
            <span v-show="configBasicInfo.toolboxVersion">Version: {{ configBasicInfo.toolboxVersion?.[0] || 0 }}.{{ configBasicInfo.toolboxVersion?.[1] || 0 }}</span>
            <span v-show="configBasicInfo.author">Author: {{ configBasicInfo.author || "" }}</span>
            <span>Built using <a class="dialog-link" href="#" @click="openUrl('https://github.com/fadedflower/rapid-toolbox')">Rapid Toolbox</a></span>
            <span>Powered by <a class="dialog-link" href="#" @click="openUrl('https://tauri.app/')">Tauri</a> and <a class="dialog-link" href="#" @click="openUrl('https://vuejs.org/')">Vue.js</a></span>
        </div>
        <template #footer>
            <Button label="OK" size="small" @click="aboutDialogVisible = false" />
        </template>
    </Dialog>
    <ConfirmDialog />
    <Menu ref="frame-menu" :model="frameMenuItems" popup />
    <BlockUI :blocked="!configLoaded" full-screen />
</template>

<script setup lang="ts">
import { ref, onMounted, computed, useTemplateRef, watch } from "vue";
import { useConfirm } from "primevue/useconfirm";
import type { MenuItem } from "primevue/menuitem";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';
import { moveWindow, Position } from '@tauri-apps/plugin-positioner';
import { openUrl } from "@tauri-apps/plugin-opener";
import { ConfigBasicInfo, Theme, ThemeColor } from "./types";
import { messageDialog, getThemeStyle, cloneTheme, preventDndAction } from "./util";
import { useSingleMenu } from "./stores";
import themePresets, { ThemePreset } from "./themes";
import LauncherView from "./LauncherView.vue";
import AppLibraryView from "./AppLibraryView.vue";
import ThemeColorPicker from "./components/ThemeColorPicker.vue";
const appWindow = getCurrentWindow();
const confirm = useConfirm();
const singleMenu = useSingleMenu();
const menuId = "frame-menu";

const configLoaded = ref(false);
const configBasicInfo = ref<ConfigBasicInfo>({
    headerText: "Rapid Toolbox",
    author: null,
    toolboxVersion: null,
    theme: themePresets[0].theme
});
const themeStyle = computed(() => settingsDialogVisible.value ? getThemeStyle(dialogSettings.value.theme) : getThemeStyle(configBasicInfo.value.theme));

const currentView = ref<"launcher" | "appLibrary">("launcher");
const currentViewComponent = computed(() => currentView.value === "launcher" ? LauncherView : AppLibraryView);
const searchKeyword = ref("");

const frameMenu = useTemplateRef("frame-menu");
const toggleViewMenuItem = computed<MenuItem>(() => {
    return currentView.value === "launcher" ? {
        label: "App library",
        icon: "pi pi-list",
        command: () => currentView.value = "appLibrary"
    } : {
        label: "Launcher",
        icon: "pi pi-objects-column",
        command: () => currentView.value = "launcher"
    };
});
const frameMenuItems = computed<MenuItem[]>(() => [
    toggleViewMenuItem.value,
    { label: "Settings", icon: "pi pi-cog", command: showSettingsDialog },
    { label: "About", icon: "pi pi-info-circle", command: () => aboutDialogVisible.value = true }
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
const dialogSettings = ref({
    headerText: "",
    author: "",
    toolboxVersion: "",
    theme: {
        type: "LinearGradient",
        from: { type: "RGB", r: 0x28, g: 0x54, b: 0xb5 },
        to: { type: "RGB", r: 0x14, g: 0xc0, b: 0xd3 }
    } as Theme
});
const dialogThemePresets = computed<ThemePreset[]>(() => {
    return [...themePresets, {
        name: "Custom",
        theme: dialogSettings.value.theme
    }];
});
const dialogThemeTypes = ref([
    { name: "Solid", type: "Solid" },
    { name: "Linear gradient", type: "LinearGradient" },
    { name: "Radial gradient", type: "RadialGradient" }
]);
const dialogSelectedThemeType = computed({
    get: () => dialogSettings.value.theme.type,
    set: value => {
        if (value === "Solid") {
            dialogSettings.value.theme = {
                type: "Solid",
                color: dialogSettings.value.theme.type === "Solid" ? dialogSettings.value.theme.color : dialogSettings.value.theme.from
            };
        } else if (value === "LinearGradient") {
            dialogSettings.value.theme = {
                type: "LinearGradient",
                from: dialogSettings.value.theme.type === "Solid" ? dialogSettings.value.theme.color : dialogSettings.value.theme.from,
                to: dialogSettings.value.theme.type === "Solid" ? dialogSettings.value.theme.color : dialogSettings.value.theme.to
            };
        } else {
            dialogSettings.value.theme = {
                type: "RadialGradient",
                from: dialogSettings.value.theme.type === "Solid" ? dialogSettings.value.theme.color : dialogSettings.value.theme.from,
                to: dialogSettings.value.theme.type === "Solid" ? dialogSettings.value.theme.color : dialogSettings.value.theme.to
            };
        }
    }
});
const dialogSettingsValid = computed(() => {
    return dialogSettings.value.headerText.trim() !== "" &&
        (dialogSettings.value.toolboxVersion === "" || /^\d{1,3}\.\d{1,3}$/.test(dialogSettings.value.toolboxVersion));
});
const dialogThemeColor1 = computed<ThemeColor>({
    get: () => dialogSettings.value.theme.type === "Solid" ? dialogSettings.value.theme.color : dialogSettings.value.theme.from,
    set: value => {
        if (dialogSettings.value.theme.type === "Solid") {
            dialogSettings.value.theme.color = value;
        } else {
            dialogSettings.value.theme.from = value;
        }
    }
});
const dialogThemeColor2 = computed<ThemeColor>({
    get: () => dialogSettings.value.theme.type === "Solid" ? { type: "RGB", r: 0, g: 0, b: 0 } as ThemeColor : dialogSettings.value.theme.to,
    set: value => {
        if (dialogSettings.value.theme.type !== "Solid") {
            dialogSettings.value.theme.to = value;
        }
    }
});
const showSettingsDialog = () => {
    dialogSettings.value = {
        ...configBasicInfo.value,
        author: configBasicInfo.value.author || "",
        toolboxVersion: configBasicInfo.value.toolboxVersion ? configBasicInfo.value.toolboxVersion.join(".") : "",
        // deep copy the theme to avoid messing with configBasicInfo
        theme: cloneTheme(configBasicInfo.value.theme)
    };
    settingsDialogVisible.value = true;
};

const saveSettings = async () => {
    configBasicInfo.value = {
        headerText: dialogSettings.value.headerText.trim(),
        author: dialogSettings.value.author.trim() === "" ? null : dialogSettings.value.author.trim(),
        toolboxVersion: dialogSettings.value.toolboxVersion === "" ? null : dialogSettings.value.toolboxVersion.split(".").map(Number) as [number, number],
        // deep copy the theme to avoid messing with configBasicInfo
        theme: cloneTheme(dialogSettings.value.theme)
    };
    if (await invoke("set_config_basic_info", { basicInfo: configBasicInfo.value })) {
        settingsDialogVisible.value = false;
    }
};

onMounted(async () => {
    moveWindow(Position.Center);
    // show the window after a short delay to prevent white screen on startup
    setTimeout(async () => { await invoke("show_window") }, 50);
    if (!await invoke<boolean>('load_config')) {
        messageDialog(confirm, "Config", "Failed to load config. Please ensure the config file is valid.", "error", () => appWindow.close());
    } else {
        configLoaded.value = true;
        configBasicInfo.value = await invoke<ConfigBasicInfo>('get_config_basic_info');
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

.dialog-link { color: var(--p-dialog-color); }
</style>