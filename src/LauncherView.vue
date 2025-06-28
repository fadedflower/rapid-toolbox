<template>
    <main class="flex">
        <Splitter id="launcher-splitter" class="flex flex-basis-full">
            <SplitterPanel class="flex" :size="20" :min-size="20">
                <CategoryList v-model="selectedCategory" />
            </SplitterPanel>
            <SplitterPanel :size="80" :min-size="70">
                <CategoryAppList :category="selectedCategory" />
            </SplitterPanel>
        </Splitter>
    </main>
</template>

<script setup lang="ts">
import { ref } from "vue";
import CategoryList from "./components/CategoryList.vue";
import CategoryAppList from "./components/CategoryAppList.vue";

const selectedCategory = ref<string | null>(null);
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
    height: calc(var(--app-height) - 34px);
}
</style>