<script setup lang="ts">
    import { ref } from 'vue'
    import * as rysk_web from './rysk_web'

    const instruction = ref('invalid opcode');
    function inspect(raw: string): string {
        const i = Number.parseInt(raw, 16);
        if (i) {
            return rysk_web.inspect(i);
        } else {
            return 'invalid opcode';
        }
    }
</script>

<template>
    <input class="" @input="event => instruction = inspect(event.target?.value)" type="text">
    <code>{{ instruction }}</code>
</template>

<style scoped>
</style>
