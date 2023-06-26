import { defineConfig } from 'npm:vite@^4.3.9'
import vue from 'npm:@vitejs/plugin-vue@^4.2.3'
import 'npm:vue@^3.3.4'
import wasm from 'npm:vite-plugin-wasm@^3.2.2'

import 'npm:nanostores@^0.9.2'
import 'npm:@nanostores/vue@^0.9.0'

import { resolve } from 'https://deno.land/std@0.192.0/path/mod.ts';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [wasm(), vue()],
    build: {
        target: 'esnext',
        outDir: 'target/dist'
    },
    resolve: {
        alias: {
            '@src': resolve('./src'),
            '@store': resolve('./src/store'),
            '@c': resolve('./src/component'),
            '@panel': resolve('./src/component/panel')
        }
    }
})
