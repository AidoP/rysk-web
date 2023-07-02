<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { to_hex } from '@/lib/hex';

const props = defineProps<{
    modelValue: number,
    max?: number,
    min?: number,
    align?: number,
    cyclic?: boolean,
    disabled?: boolean
}>();
const emit = defineEmits(['update:modelValue']);

const align = (value: number) => {
    if (props.align !== undefined) {
        return ((value >>> props.align) << props.align) >>> 0;
    } else {
        return value;
    }
};
watch(() => props.modelValue, (new_value) => {
    const min = props.min ?? 0;
    if (new_value < min) {
        // Cycle back to the maximum value
        if (props.cyclic && props.max !== undefined) {
            emit('update:modelValue', align(props.max - (min - new_value)));
        } else {
            emit('update:modelValue', align(min));
        }
    } else if (props.max !== undefined && new_value > props.max) {
        // Cycle back to the minimum value
        if (props.cyclic) {
            emit('update:modelValue', align(min + (new_value - props.max)));
        } else {
            emit('update:modelValue', align(props.max));
        }
    } else {
        const v = align(new_value);
        if (v !== new_value) {
            emit('update:modelValue', v);
        }
    }
});

const valid = ref(true);
const text_value = computed({
    get() {
        return to_hex(props.modelValue, 8);
    },
    set(new_value: string) {
        let is_valid = false;
        if (/^[0-9a-fA-F][_0-9a-fA-F]*$/.test(new_value)) {
            const num = Number.parseInt(new_value.replaceAll('_', ''), 16);
            is_valid = props.max !== undefined ? num <= props.max : true && num >= (props.min ?? 0);
            if (is_valid) {
                emit('update:modelValue', num);
            }
        }
        valid.value = is_valid;
    }
});
</script>

<template>
    <span :invalid="valid ? undefined : true" class="input">
        <code class="inner">0x</code>
        <input class="inner" type="text" :disabled="props?.disabled" :value="text_value" @change="text_value=($event.target as HTMLInputElement).value">
    </span>
</template>

<style scoped>
span {
    padding-right: 1.4em;
}
input {
    outline: none;
    background: none;
    margin: -1.4em;
    padding-left: 1.4em;
}
.inner {
    border: none;
    font-size: 1em;
    font-family: 'Noto Sans Mono';
}
</style>
