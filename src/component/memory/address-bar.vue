<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { to_hex } from '@/lib/hex';

const props = defineProps<{
    value: number
}>();
const emit = defineEmits(['update:value']);
watch(() => props.value, value => {
    text.value = to_hex(value, 8);
});

const text = ref(to_hex(props.value, 8));
const is_valid = (value: string) => {
    return value.length <= 8 && /^[0-9a-fA-F][_0-9a-fA-F]*$/.test(value);
};
const valid = computed(() => is_valid(text.value));
const on_change = (event: Event) => {
    const new_value = (event.target as HTMLInputElement).value;
    if (!is_valid(new_value))
        return;
    const num = Number.parseInt(new_value.replaceAll('_', ''), 16);
    emit('update:value', num);
    text.value = to_hex(props.value, 8);
};
</script>

<template>
    <span class="input">
        <code class="inner">0x</code>
        <input class="inner" type="text" :class="{ invalid: !valid }" v-model="text" @change="on_change">
    </span>
</template>

<style scoped>
input {
    outline: none;
    background: none;
    margin-left: -1.5em;
    padding-left: 1.4em;
}
code {
    margin-left: 0.4em;
}
.inner {
    border-radius: 4px;
    font-size: 1em;
    font-family: 'Noto Sans Mono';
}
</style>
