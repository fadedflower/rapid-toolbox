<template>
    <Dialog class="width-dialog dialog-no-select" v-model:visible="visible" modal :header="t('AboutDialog.title')">
        <div class="flex flex-col no-select">
            <span>{{ configBasicInfo.headerText }}</span>
            <span v-show="configBasicInfo.toolboxVersion">{{ t("AboutDialog.version", [configBasicInfo.toolboxVersion?.[0] || 0, configBasicInfo.toolboxVersion?.[1] || 0]) }}</span>
            <span v-show="configBasicInfo.author">{{ t("AboutDialog.author", [configBasicInfo.author || ""]) }}</span>
            <i18n-t keypath="AboutDialog.builtUsing" tag="span">
                <a class="dialog-link" href="#" @click="openUrl('https://github.com/fadedflower/rapid-toolbox')">Rapid Toolbox</a>
            </i18n-t>
            <i18n-t keypath="AboutDialog.poweredBy" tag="span">
                <a class="dialog-link" href="#" @click="openUrl('https://tauri.app/')">Tauri</a>
                <a class="dialog-link" href="#" @click="openUrl('https://vuejs.org/')">Vue.js</a>
            </i18n-t>
        </div>
        <template #footer>
            <Button :label="t('DialogCommon.btnOK')" size="small" @click="visible = false" />
        </template>
    </Dialog>
</template>

<script setup lang="ts">
import { useI18n, I18nT } from "vue-i18n";
import { openUrl } from "@tauri-apps/plugin-opener";
import { ConfigBasicInfo } from "../types";
const { t } = useI18n();

const visible = defineModel<boolean>("visible");
const { configBasicInfo } = defineProps<{ configBasicInfo: ConfigBasicInfo }>();
</script>

<style scoped>
.dialog-link { color: var(--p-dialog-color); }
</style>