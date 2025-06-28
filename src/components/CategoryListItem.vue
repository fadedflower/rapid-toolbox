<template>
    <li class="category-list-item no-select" :class="listItemClass" @click="emit('select')">
        {{ category }}
    </li>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const { category, selected = false, isDragging = false } = defineProps<{ category: string, selected?: boolean, isDragging?: boolean }>();
const emit = defineEmits<{ select: [] }>();
const listItemClass = computed<string>(() => selected ? 'list-item-selected' : (isDragging ? '' : 'list-item-focus'));

</script>

<style scoped>
.category-list-item {
    margin: var(--p-list-padding);
    margin-right: calc(0.25rem + 9px);
    padding: var(--p-form-field-padding-y) var(--p-form-field-padding-x);
    color: white;
    border-radius: var(--p-form-field-border-radius);
    transition: background var(--p-form-field-transition-duration), box-shadow var(--p-form-field-transition-duration), color var(--p-form-field-transition-duration);
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
}
.list-item-focus:hover {
    background-color: rgb(255 255 255 / .2);
    box-shadow: var(--p-form-field-shadow);
}
.list-item-focus:active {
    background-color: rgb(255 255 255 / .3);
    box-shadow: var(--p-form-field-shadow);
}
.list-item-selected {
    color: var(--p-primary-700);
    background-color: rgb(255 255 255 / .8);
    box-shadow: var(--p-form-field-shadow);
}
.list-item-selected:hover { background-color: rgb(255 255 255 / .7); }
.list-item-selected:active { background-color: rgb(255 255 255 / .7); }
</style>