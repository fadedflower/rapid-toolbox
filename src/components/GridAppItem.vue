<template>
    <div class="grid-app-item flex flex-col align-center gap-2" :class="appItemClass" @click="emit('select')" @dblclick="emit('launch')">
        <img :src="app.iconUrl" width="32" height="32" class="app-icon no-select" draggable="false" />
        <span class="app-name no-select">{{ app.name }}</span>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { AppMetadata } from "../types";

const { app, selected = false, isDragging = false } = defineProps<{ app: AppMetadata, selected?: boolean, isDragging?: boolean }>();
const emit = defineEmits<{ select: [], launch: [] }>();
const appItemClass = computed<string>(() => selected ? 'app-item-selected' : (isDragging ? '' : 'app-item-focus'));

</script>

<style scoped>
.grid-app-item {
    width: 86px;
    padding: 5px 12px;
    transition: background var(--p-form-field-transition-duration);
}
.app-item-focus:hover { background-color: rgb(255 255 255 / .1); }
.app-item-focus:active { background-color: rgb(255 255 255 / .2); }
.app-item-selected {
    color: var(--p-primary-700);
    background-color: rgb(255 255 255 / .35);
}
.app-item-selected:hover { background-color: rgb(255 255 255 / .3); }
.app-item-selected:active { background-color: rgb(255 255 255 / .3); }

.app-name {
    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    max-height: 2lh;
    color: white;
    font-size: .7rem;
    line-height: 1.4;
    word-break: break-all;
    overflow: hidden;
    text-overflow: ellipsis;
    text-align: center;
}
</style>