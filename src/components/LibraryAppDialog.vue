<template>
    <Dialog class="width-dialog dialog-no-select" v-model:visible="visible" modal :header="dialogHeader">
        <div class="flex flex-col gap-8">
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-app-name">Name</label>
                <InputText id="dialog-app-name" class="flex-grow" size="small" v-model="dialogAppMetadata.name" placeholder="Required" autocomplete="off" />
            </div>
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-app-desc">Description</label>
                <InputText id="dialog-app-desc" class="flex-grow" size="small" v-model="dialogAppMetadata.desc" placeholder="Optional" autocomplete="off" />
            </div>
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-app-path">App path</label>
                <div class="flex gap-4 flex-grow">
                    <InputText id="dialog-app-path" class="flex-grow" size="small" v-model="dialogAppMetadata.appPath" placeholder="Required" autocomplete="off" />
                    <Button icon="pi pi-folder-open" size="small" variant="outlined" @click="browseAppPath" />
                </div>
            </div>
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-app-launch-args">Launch args</label>
                <InputText id="dialog-app-launch-args" class="flex-grow" size="small" v-model="dialogAppMetadata.launchArgs" placeholder="Optional" autocomplete="off" />
            </div>
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-app-working-dir">Working directory</label>
                <div class="flex gap-4 flex-grow">
                    <InputText id="dialog-app-working-dir" class="flex-grow" size="small" v-model="dialogAppMetadata.workingDir" placeholder="Required" autocomplete="off" />
                    <Button icon="pi pi-folder-open" size="small" variant="outlined" @click="browseWorkingDir" />
                </div>
            </div>
            <div class="flex align-center">
                <span class="flex-grow dialog-label no-select">Icon</span>
                <div class="flex align-center gap-4">
                    <img v-if="dialogAppMetadata.iconUrl !== ''" class="icon-slot" :src="dialogAppMetadata.iconUrl" width="48" height="48" draggable="false" />
                    <span v-else class="icon-slot no-select">No icon</span>
                    <Button icon="pi pi-folder-open" size="small" variant="outlined" @click="browseIcon" />
                </div>
            </div>
            <div class="flex align-center gap-4">
                <Button
                    icon="pi pi-folder"
                    label="Use relative path"
                    size="small"
                    variant="outlined"
                    :disabled="dialogAppMetadata.appPath === '' && dialogAppMetadata.workingDir === ''"
                    @click="useRelativePath"
                />
                <Button
                    icon="pi pi-image"
                    label="Use built-in app icon"
                    size="small"
                    variant="outlined"
                    :disabled="dialogAppMetadata.appPath === ''"
                    @click="extractIconFromApp"
                />
            </div>
        </div>
        <template #footer>
            <Button label="Cancel" size="small" severity="secondary" @click="visible = false" />
            <Button :label="dialogSubmitBtnLabel" size="small" :disabled="!dialogAppMetadataValid" @click="submitDialog" />
        </template>
    </Dialog>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { path as pathApi } from '@tauri-apps/api';
import { useConfirm } from 'primevue/useconfirm';
import { messageDialog } from '../util';
import { AppMetadata } from '../types';
const confirm = useConfirm();

const visible = defineModel<boolean>("visible", { default: false });
const { apps, editMode, editApp = null } = defineProps<{ apps: AppMetadata[], editMode: boolean, editApp?: AppMetadata | null }>();
const emit = defineEmits<{ updateApp: [ newApp: AppMetadata ] }>();
const dialogHeader = computed(() => editMode ? `Edit "${editApp?.name}"` : "Add app");
const dialogSubmitBtnLabel = computed(() => editMode ? "Save" : "Add");
const dialogAppMetadata = ref<AppMetadata>({
    name: "",
    appPath: "",
    launchArgs: "",
    workingDir: "",
    desc: "",
    iconUrl: ""
});
const dialogAppMetadataValid = computed(() => {
    return dialogAppMetadata.value?.name.trim() !== "" &&
        (editApp?.name === dialogAppMetadata.value?.name.trim() || apps.findIndex(app => app.name === dialogAppMetadata.value?.name) === -1) &&
        dialogAppMetadata.value?.appPath.trim() !== "" &&
        dialogAppMetadata.value?.workingDir.trim() !== "" &&
        dialogAppMetadata.value?.iconUrl.trim() !== "";
});
watch(visible, newValue => {
    if (newValue) {
        if (editMode) {
            dialogAppMetadata.value = { ...editApp! };
        } else {
            dialogAppMetadata.value = {
                name: "",
                appPath: "",
                launchArgs: "",
                workingDir: "",
                desc: "",
                iconUrl: ""
            };
        }
    }
});

const browseAppPath = async () => {
    const path = await open({
        title: "Select app path",
        directory: false,
        filters: [{
            name: "Executable files",
            extensions: ["exe", "bat", "cmd"]
        }]
    });
    if (path) {
        dialogAppMetadata.value.appPath = path;
        dialogAppMetadata.value.workingDir = await pathApi.dirname(path);
    }
};

const browseWorkingDir = async () => {
    const dir = await open({
        title: "Select working directory",
        directory: true
    });
    if (dir) {
        dialogAppMetadata.value.workingDir = dir;
    }
};

const browseIcon = async () => {
    const iconPath = await open({
        title: "Select app icon",
        directory: false,
        filters: [{
            name: "Image files",
            extensions: ["bmp", "gif", "jpg", "jpeg", "png", "svg", "webp", "tif", "tiff"]
        }]
    });
    if (iconPath) {
        dialogAppMetadata.value.iconUrl = await invoke<string | null>("load_icon_from_file", { path: iconPath }) || "";
    }
};

const extractIconFromApp = async () => {
    dialogAppMetadata.value.iconUrl = await invoke<string | null>("load_icon_from_app", { path: dialogAppMetadata.value.appPath }) || "";
};

const useRelativePath = async () => {
    if (dialogAppMetadata.value.appPath !== "" && await pathApi.isAbsolute(dialogAppMetadata.value.appPath)) {
        const relativeAppPath = await invoke<string | null>("get_relative_path", { path: dialogAppMetadata.value.appPath });
        if (relativeAppPath) {
            dialogAppMetadata.value.appPath = relativeAppPath;
        } else {
            messageDialog(confirm, "Use relative path", "The working directory of the toolbox must be the prefix of app path.", "warning");
            return;
        }
    }
    if (dialogAppMetadata.value.workingDir !== "" && await pathApi.isAbsolute(dialogAppMetadata.value.workingDir)) {
        const relativeWorkingDir = await invoke<string | null>("get_relative_path", { path: dialogAppMetadata.value.workingDir });
        if (relativeWorkingDir) {
            dialogAppMetadata.value.workingDir = relativeWorkingDir;
        } else {
            messageDialog(confirm, "Use relative path", "The working directory of the toolbox must be the prefix of working directory.", "warning");
            return;
        }
    }
};

const submitDialog = async () => {
    if ((editMode && await invoke<boolean>("update_app", { appName: editApp?.name, appMetadataWithName: dialogAppMetadata.value })) ||
        (!editMode && await invoke<boolean>("add_app", { appMetadataWithName: dialogAppMetadata.value }))) {
        emit("updateApp", { ...dialogAppMetadata.value });
        visible.value = false;
    }
}
</script>

<style scoped>
.icon-slot {
    width: 48px;
    height: 48px;
    font-size: .75rem;
    align-content: center;
    text-align: center;
    border: 1px solid var(--p-surface-400);
}
</style>