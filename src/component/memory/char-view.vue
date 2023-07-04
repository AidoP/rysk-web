<script setup lang="ts">
import { Ref, computed, ref } from 'vue';
import CharCell from './char-cell.vue';
import { char_length, try_decode_bytes } from '@/rysk_web/rysk_web';
import { to_hex } from '@/lib/hex';

const props = defineProps<{
    memory?: Uint8Array,
    focused: boolean,
    selected?: [number, number],
    hovered?: [number, number],
    width: number,
    height: number,
    tabindex?: number
}>();
const emit = defineEmits<{
    (e: 'select', position?: [number, number]): void,
    (e: 'hover', position?: [number, number]): void,
    (e: 'write', position: [number, number], data: Uint8Array): void
}>();

const chars = computed(() => props.memory !== undefined ? try_decode_bytes(props.memory) : undefined);

const cells: Ref<typeof CharCell | null> = ref(null);
// Cell Events
const cell_select = (position?: [number, number]) => {
    emit('select', position);
};
const cell_hover = (position?: [number, number]) => {
    emit('hover', position);
};
const cell_set = ([x, y]: [number, number], data: Uint8Array) => {
    emit('write', [x, y], data);
    emit('select', [x + data.length, y]);
};
const cell_delete = ([x, y]: [number, number], left: boolean) => {
    const char = chars.value?.at(x+props.width*y);
    const length = char ? char_length(char) : 1;
    emit('write', [x, y], new Uint8Array(length));
    emit('select', [x + (left ? -1 : 1), y]);
};
</script>

<template>
    <div class="memory-view char">
        <div>
            <span v-for="(_, i) in props.width" :key="'offset-' + i" class="memory-cell char">{{ to_hex(i, 2) }}</span>
        </div>
        <div v-for="(_, y) in props.height" :key="'char-row-' + y">
            <char-cell
                v-for="(_, x) in props.width"
                ref="cells"
                :key="x+props.width*y"
                :position="[x, y]"
                :enabled="props.memory !== undefined && props.memory[x + props.width * y] !== undefined"
                :focused="props.focused"
                :selected="props.selected?.at(0) === x && props.selected?.at(1) === y"
                :hovered="props.hovered?.at(0) === x && props.hovered?.at(1) === y"
                :value="chars?.at(x+props.width*y)"
                class="memory-cell char"
                :tabindex="props.memory !== undefined && props.memory[x + props.width * y] !== undefined ? (props.tabindex ?? 0) + y : undefined"
                @select="cell_select"
                @hover="cell_hover"
                @set="cell_set"
                @delete="cell_delete"
            />
        </div>
    </div>
</template>

<style>
.memory-view.char {
    width: calc(16 * 1.4rem);
}
.memory-cell.char {
    width: 1.4rem;
}
</style>
