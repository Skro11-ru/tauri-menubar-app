import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],

  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: ["es2021", "chrome105", "safari16"],
    minify: process.env.TAURI_DEBUG ? false : undefined,
    sourcemap: process.env.TAURI_DEBUG === "true",
  },
});
