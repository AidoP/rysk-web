<script setup lang="ts">
import { to_hex } from '@/lib/hex';
import { computed, ref, Ref, watch } from 'vue';

const props = defineProps<{
    value: number | null | undefined,
    position: [number, number],
    enabled: boolean,
    focused: boolean,
    selected: boolean,
    hovered: boolean
}>();
const emit = defineEmits<{
    (e: 'hover', position?: [number, number]): void,
    (e: 'select', position?: [number, number]): void,
    (e: 'set', position: [number, number], value: Uint8Array): void,
    (e: 'delete', position: [number, number], left: boolean): void
}>();

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
        return input_text.value ?? '';
    }
    if (props.value !== undefined && props.value !== null) {
        return String.fromCodePoint(props.value);
    }
    return '';
});
const span: Ref<HTMLSpanElement | null> = ref(null);

const on_blur = (_: Event) => {
    input_text.value = undefined;
    emit('select');
};

const on_input = (e: Event) => {
    const event = e as InputEvent;
    switch (event.inputType) {
    case 'insertCompositionText':
        if (event.isComposing) {
            input_text.value = event.data !== null ? event.data : undefined;
        } else {
            input_text.value = undefined;
            if (event.data !== null) {
                emit('set', props.position, new TextEncoder().encode(event.data));
            }
        }
        break;
    case 'insertText':
        if (event.data !== null) {
            emit('set', props.position, new TextEncoder().encode(event.data));
        }
        break;
    case 'insertLineBreak':
    case 'insertParagraph':
        if (event.data !== null) {
            emit('set', props.position, new TextEncoder().encode(event.data));
        }
        break;
    case 'insertFromDrop':
    case 'insertFromPaste':
        if (!event.dataTransfer)
            break;
        emit('set', props.position, new TextEncoder().encode(event.dataTransfer.getData('text/plain')));
        // TODO: Support other types & files. File data should be read in the worker thread.
        break;
    default:
        if (span.value !== null) {
            span.value.innerText = props.value ? String.fromCodePoint(props.value) : '';
        }
        break;
    }
    if (span.value !== null)
        span.value.innerText = text.value;
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
            'utf8-continuation': props.value === undefined,
            'utf8-invalid': props.value === null,
            disabled: !props.enabled,
            selected: props.selected,
            hovered: props.hovered
        }"
        :contenteditable="props.enabled ? true : undefined"
        :tooltip="props.value !== null && props.value !== undefined ? 'U+' + to_hex(props.value, 4).toUpperCase() : undefined"
        @click="emit('select', props.position)"
        @focus="emit('select', props.position)"
        @blur="on_blur"
        @input="on_input"
        @keydown="on_key_down"
        @mouseenter="emit('hover', props.position)"
        @mouseleave="emit('hover')"
    >
        {{ text }}
    </span>
</template>

<style scoped>
span {
    caret-color: transparent;
}
.utf8-continuation {
    background-color: rgb(201, 255, 223);
}
.utf8-invalid {
    background-color: rgb(255, 97, 97);
}
</style>