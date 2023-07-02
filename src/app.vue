<script setup lang="ts">
import MemoryEditor from '@/component/memory-editor.vue';

</script>

<template>
    <header>
    </header>
    <body>
        <memory-editor :width="16" :min="0xf000_0000" :max="0xffff_ffff" />
    </body>
    <footer>
    </footer>
</template>

<style scoped>
</style>
