import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";

export default defineConfig({
  clearScreen: false,
  plugins: [vue()],
  server: {
    port: 1430,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    target: "es2021",
  },
});
