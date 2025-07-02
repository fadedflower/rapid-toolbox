<template>
    <div class="color-picker-btn" :style="colorStyle" @click="colorPickerPopup?.toggle($event)" />
    <Popover class="color-picker-popup" ref="color-picker">
        <ChromePicker v-model="rawColor" :formats="['rgb', 'hsl']" />
    </Popover>
</template>

<script setup lang="ts">
import { computed, useTemplateRef } from "vue";
import { ChromePicker } from "vue-color";
import { ThemeColor } from "../types";
import { getThemeColorCssValue } from "../util";
type RawColor = { r: number, g: number, b: number } | { h: number, s: number, l: number };

const color = defineModel<ThemeColor>({ default: { type: "RGB", r: 0, g: 0, b: 0 } });
const rawColor = computed<RawColor>({
    get: () => themeColorToRawColor(color.value || { type: "RGB", r: 0, g: 0, b: 0 }),
    set: value => {
        color.value = rawColorToThemeColor(value);
    }
});
const colorStyle = computed(() => {
    return { backgroundColor: getThemeColorCssValue(color.value || { type: "RGB", r: 0, g: 0, b: 0 }) };
});
const colorPickerPopup = useTemplateRef("color-picker");

const rawColorToThemeColor = (color: RawColor): ThemeColor =>  {
    if ("r" in color) {
        return { type: "RGB", r: color.r, g: color.g, b: color.b };
    } else {
        return { type: "HSL", h: color.h, s: color.s, l: color.l };
    }
};
const themeColorToRawColor = (color: ThemeColor): RawColor => {
    if (color.type === "RGB") {
        return { r: color.r, g: color.g, b: color.b };
    } else {
        return { h: color.h, s: color.s, l: color.l };
    }
};
</script>

<style scoped>
.color-picker-btn {
    width: 50px;
    height: 33px;
    cursor: pointer;
}
</style>

<style>
.color-picker-popup { --p-popover-content-padding: 0; }
</style>