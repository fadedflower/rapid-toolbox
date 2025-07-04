import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import Components from "unplugin-vue-components/vite";
import { PrimeVueResolver } from "@primevue/auto-import-resolver";
import { visualizer } from "rollup-plugin-visualizer";
import replace from "@rollup/plugin-replace";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
    Components({
      resolvers: [PrimeVueResolver()]
    }),
    visualizer({
      open: false,
      filename: "stats.html",
      gzipSize: true,
      brotliSize: true
    }),
    replace({
      preventAssignment: true,
      values: {
        __VUE_I18N_LEGACY_API__: "false"
      }
    })
  ],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  // optimize bundle
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          vue: ["@vue/runtime-core", "@vue/runtime-dom", "@vue/reactivity", "@vue/shared"],
          primevue: [
            "primevue/multiselect",
            "primevue/select",
            "primevue/dialog",
            "primevue/confirmdialog",
            "primevue/contextmenu",
            "primevue/menu",
            "primevue/tooltip",
            "primevue/splitter"
          ],
          "primevue-datatable": ["primevue/datatable"],
          tauri: ["@tauri-apps/api"],
          "vue-color": ["vue-color"]
        }
      }
    }
  }
}));
