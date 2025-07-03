<template>
    <Dialog class="width-dialog dialog-no-select" v-model:visible="visible" modal :header="t('SettingsDialog.title')">
        <div class="flex flex-col gap-8">
            <div class="flex align-center">
                <span class="dialog-label no-select">{{ t('SettingsDialog.labelLanguage') }}</span>
                <Select class="flex-grow" size="small" v-model="dialogSettings.lang" :options="availableLocales" :option-label="lang => t(`LanguageCode.${lang}`)" />
            </div>
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-header-text">{{ t('SettingsDialog.labelHeaderText') }}</label>
                <InputText id="dialog-header-text" class="flex-grow" size="small" v-model="dialogSettings.headerText" :placeholder="t('DialogCommon.placeholderRequired')" autocomplete="off" />
            </div>
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-author">{{ t('SettingsDialog.labelAuthor') }}</label>
                <InputText id="dialog-author" class="flex-grow" size="small" v-model="dialogSettings.author" :placeholder="t('DialogCommon.placeholderOptional')" autocomplete="off" />
            </div>
            <div class="flex align-center">
                <label class="dialog-label no-select" for="dialog-toolbox-version">{{ t('SettingsDialog.labelToolboxVersion') }}</label>
                <InputText id="dialog-toolbox-version" class="flex-grow" size="small" v-model="dialogSettings.toolboxVersion" :placeholder="t('SettingsDialog.toolboxVersionPlaceholder')" autocomplete="off" />
            </div>
        </div>
        <Divider align="center" type="solid">
            <span class="no-select">{{ t('SettingsDialog.dividerTheme') }}</span>
        </Divider>
        <div class="flex flex-col gap-8">
            <div class="flex align-center">
                <span class="dialog-label no-select">{{ t('SettingsDialog.labelPreset') }}</span>
                <Select class="flex-grow" size="small" v-model="dialogSettings.theme" :options="dialogThemePresets" option-label="name" option-value="theme" />
            </div>
            <div class="flex align-center">
                <span class="dialog-label no-select">{{ t('SettingsDialog.labelBackgroundType') }}</span>
                <Select class="flex-grow" size="small" v-model="dialogSelectedThemeType" :options="dialogThemeTypes" option-label="name" option-value="type" />
            </div>
            <div v-show="dialogSelectedThemeType === 'Solid'" class="flex align-center">
                <span class="flex-grow dialog-label no-select">{{ t('SettingsDialog.labelColor') }}</span>
                <ThemeColorPicker v-model="dialogThemeColor1" />
            </div>
            <div v-show="dialogSelectedThemeType !== 'Solid'" class="flex align-center">
                <span class="flex-grow dialog-label no-select">{{ t('SettingsDialog.labelColor1') }}</span>
                <ThemeColorPicker v-model="dialogThemeColor1" />
            </div>
            <div v-show="dialogSelectedThemeType !== 'Solid'" class="flex align-center">
                <span class="flex-grow dialog-label no-select">{{ t('SettingsDialog.labelColor2') }}</span>
                <ThemeColorPicker v-model="dialogThemeColor2" />
            </div>
        </div>
        <template #footer>
            <Button :label="t('DialogCommon.btnCancel')" size="small" severity="secondary" @click="visible = false" />
            <Button :label="t('DialogCommon.btnSave')" size="small" :disabled="!dialogSettingsValid" @click="saveSettings" />
        </template>
    </Dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { ConfigBasicInfo, Theme, ThemeColor } from "../types";
import { cloneTheme } from "../util";
import themePresets, { ThemePreset } from "../themes";
import ThemeColorPicker from "./ThemeColorPicker.vue";
const { t, locale, availableLocales } = useI18n();

const visible = defineModel<boolean>("visible");
watch(visible, newValue => {
    if (newValue) {
        dialogSettings.value = {
            ...configBasicInfo,
            author: configBasicInfo.author || "",
            toolboxVersion: configBasicInfo.toolboxVersion ? configBasicInfo.toolboxVersion.join(".") : "",
            // deep copy the theme to avoid messing with configBasicInfo
            theme: cloneTheme(configBasicInfo.theme)
        };
    }
});
const { configBasicInfo } = defineProps<{ configBasicInfo: ConfigBasicInfo }>();
const emit = defineEmits<{ updateBasicInfo: [info: ConfigBasicInfo], updateSettingsTheme: [theme: Theme] }>()
const dialogSettings = ref({
    lang: "en",
    headerText: "",
    author: "",
    toolboxVersion: "",
    theme: {
        type: "LinearGradient",
        from: { type: "RGB", r: 0x28, g: 0x54, b: 0xb5 },
        to: { type: "RGB", r: 0x14, g: 0xc0, b: 0xd3 }
    } as Theme
});
watch(() => dialogSettings.value.theme, newValue => emit("updateSettingsTheme", newValue));
const dialogThemePresets = computed<ThemePreset[]>(() => {
    return [...themePresets, {
        name: t("SettingsDialog.presetCustom"),
        theme: dialogSettings.value.theme
    }];
});
const dialogThemeTypes = computed(() => [
    { name: t("SettingsDialog.themeTypeSolid"), type: "Solid" },
    { name: t("SettingsDialog.themeTypeLinearGradient"), type: "LinearGradient" },
    { name: t("SettingsDialog.themeTypeRadialGradient"), type: "RadialGradient" }
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
const saveSettings = async () => {
    const basicInfo = {
        lang: dialogSettings.value.lang,
        headerText: dialogSettings.value.headerText.trim(),
        author: dialogSettings.value.author.trim() === "" ? null : dialogSettings.value.author.trim(),
        toolboxVersion: dialogSettings.value.toolboxVersion === "" ? null : dialogSettings.value.toolboxVersion.split(".").map(Number) as [number, number],
        // deep copy the theme to avoid messing with configBasicInfo
        theme: cloneTheme(dialogSettings.value.theme)
    };
    if (await invoke("set_config_basic_info", { basicInfo })) {
        locale.value = basicInfo.lang;
        emit("updateBasicInfo", basicInfo);
        visible.value = false;
    }
};
</script>