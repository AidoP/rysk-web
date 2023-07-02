import { resolve } from 'path';

import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import wasm from 'vite-plugin-wasm';
import top_level_await from 'vite-plugin-top-level-await';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [top_level_await(), wasm(), vue()],
    build: {
        target: 'esnext',
        outDir: 'target/dist'
    },
    worker: {
        format: 'es',
        plugins: [top_level_await(), wasm()]
    },
    resolve: {
        alias: {
            '@': resolve('./src')
        }
    }
});
