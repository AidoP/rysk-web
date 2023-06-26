<script setup lang="ts">
    import { ref } from 'vue';
    import { useStore } from '@nanostores/vue';

    import { $vm } from '@store/vm';
    const vm = useStore($vm);

    const val = ref('');

    import disasm from '@panel/disasm.vue';
    import mem from '@panel/mem.vue';
</script>

<template>
    <header>
    </header>
    <body>
        <mem />
    </body>
    <footer>
    </footer>
</template>

<style scoped>
</style>
