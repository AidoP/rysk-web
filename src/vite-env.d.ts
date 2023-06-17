/// <reference types="vite/client" />

declare module '*.vue' {
    import type { DefineComponent } from 'vue'
    // deno-lint-ignore ban-types
    const component: DefineComponent<unknown, unknown, unknown>
    export default component
}
