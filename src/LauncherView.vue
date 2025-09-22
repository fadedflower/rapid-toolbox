<template>
    <main
        class="flex"
        @keydown="appListRef?.keyboardHandler($event.key)"
    >
        <Splitter id="launcher-splitter" class="flex flex-basis-full">
            <SplitterPanel class="flex launcher-splitter-panel" :size="20" :min-size="20">
                <CategoryList v-model="selectedCategory" />
            </SplitterPanel>
            <SplitterPanel class="launcher-splitter-panel" :size="80" :min-size="70">
                <CategoryAppList ref="app-list" :category="selectedCategory" :search-keyword="searchKeyword" />
            </SplitterPanel>
        </Splitter>
    </main>
</template>

<script setup lang="ts">
import { ref, useTemplateRef } from "vue";
import CategoryList from "./components/CategoryList.vue";
import CategoryAppList from "./components/CategoryAppList.vue";
const { searchKeyword } = defineProps<{ searchKeyword: string }>();
const selectedCategory = ref<string | null>(null);
const appListRef = useTemplateRef("app-list");
</script>

<style scoped>
#category-list {
    --p-listbox-option-selected-background: rgb(255 255 255 / .8);
    border: none;
}
#launcher-splitter {
    --p-splitter-background: rgb(0 0 0 / 0);
    --p-splitter-gutter-background: rgb(0 0 0 / 0);
    border: none;
    height: var(--view-height);
}
.launcher-splitter-panel { outline: none; }
</style>