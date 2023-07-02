<script setup lang="ts">
import { to_hex } from '@/lib/hex';
import { computed, ref, Ref, watch } from 'vue';

const props = defineProps<{
    value: number | undefined,
    focused: boolean,
    selected: boolean,
    hovered: boolean,
    position: [number, number]
}>();
const emit = defineEmits<{
    (e: 'hover', position?: [number, number]): void,
    (e: 'select', position?: [number, number]): void,
    (e: 'set', position: [number, number], value: number): void,
    (e: 'delete', position: [number, number], left: boolean): void,
}>();
const set = (value: number) => {
    if (span.value !== null) {
        input_text.value = undefined;
    }
    emit('set', props.position, value);
};
watch(() => props.focused && props.selected, focused => {
    if (focused) {
        span.value?.focus();
    } else {
        span.value?.blur();
    }
});

const input_text: Ref<string | undefined> = ref();
const text = computed(() => {
    if (input_text.value !== undefined) {
        return input_text.value;
    } else if (props.value !== undefined) {
        return to_hex(props.value, 2);
    } else {
        return '';
    }
});
const span: Ref<HTMLSpanElement | null> = ref(null);

const on_blur = (_: Event) => {
    input_text.value = undefined;
    emit('select');
};

const hex_keys = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
const on_key_up = (event: KeyboardEvent) => {
    if (!hex_keys.includes(event.key) || props.value === undefined) {
        return;
    }
    if (input_text.value === undefined || input_text.value.length >= 2) {
        input_text.value = event.key;
    } else if (input_text.value.length == 1) {
        input_text.value += event.key;
        const value = Math.min(Math.max(Number.parseInt(input_text.value, 16), 0), 255);
        set(value);
    } else {
        input_text.value += event.key;
    }
};

function on_key_down(event: KeyboardEvent) {
    const [x, y] = props.position;
    switch (event.key) {
    case 'ArrowLeft':
        emit('select', [x - 1, y]);
        break;
    case 'ArrowRight':
        emit('select', [x + 1, y]);
        break;
    case 'ArrowUp':
        emit('select', [x, y - 1]);
        break;
    case 'ArrowDown':
        emit('select', [x, y + 1]);
        break;
    case 'Home':
        emit('select', [0, y]);
        break;
    case 'End':
        emit('select', [-1, y + 1]);
        break;
    case 'Backspace':
        emit('delete', props.position, true);
        break;
    case 'Delete':
        emit('delete', props.position, false);
        break;
    }
}

defineExpose({
    focus: () => span.value?.focus(),
    blur: () => span.value?.blur()
});
</script>
<template>
    <span
        ref="span"
        :class="{
            selected,
            hovered
        }"
        @click="emit('select', props.position)"
        @focus="emit('select', props.position)"
        @blur="on_blur"
        @keyup="on_key_up"
        @keydown="on_key_down"
        @mouseenter="emit('hover', props.position)"
        @mouseleave="emit('hover')"
    >
        {{ text }}
    </span>
</template>