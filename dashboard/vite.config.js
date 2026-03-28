import tailwindcss from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";
import path from "path";

export default defineConfig({
  plugins: [vue(), tailwindcss()],
  resolve: {
    alias: { '@': path.resolve(__dirname, './src') },
  },
  build: {
    outDir: "../web/dist", // output into Rust's web/ folder
    emptyOutDir: true,
  },
  server: {
    proxy: {
      "/api": "http://localhost:8000", // proxy API to Rust in dev
      "/ws": { target: "ws://localhost:8000", ws: true },
    },
  },
});
