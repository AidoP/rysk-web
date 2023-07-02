<script setup lang="ts">
import { Ref, ref } from 'vue';
import ByteCell from './byte-cell.vue';
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
    (e: 'write', position: [number, number], value: number): void,
}>();

const cells: Ref<typeof ByteCell | null> = ref(null);

// Cell Events
const cell_select = (position?: [number, number]) => {
    emit('select', position);
};
const cell_hover = (position?: [number, number]) => {
    emit('hover', position);
};
const cell_set = ([x, y]: [number, number], value: number) => {
    emit('write', [x, y], value);
    emit('select', [x + 1, y]);
};
const cell_delete = ([x, y]: [number, number], left: boolean) => {
    emit('write', [x, y], 0);
    emit('select', [x + (left ? -1 : 1), y]);
};
</script>

<template>
    <div class="memory-view byte">
        <div>
            <span v-for="(_, i) in props.width" :key="'offset-' + i" class="memory-cell byte">{{ to_hex(i, 2) }}</span>
        </div>
        <div v-for="(_, y) in props.height" :key="'byte-row-' + y">
            <byte-cell
                v-for="(_, x) in props.width"
                ref="cells"
                :key="x+props.width*y"
                :position="[x, y]"
                :focused="props.focused"
                :selected="props.selected?.at(0) === x && props.selected?.at(1) === y"
                :hovered="props.hovered?.at(0) === x && props.hovered?.at(1) === y"
                :value="props.memory?.at(x+props.width*y)"
                class="memory-cell byte"
                :tabindex="(props.tabindex ?? 0) + y"
                @select="cell_select"
                @hover="cell_hover"
                @set="cell_set"
                @delete="cell_delete"
            />
        </div>
    </div>
</template>

<style>
.memory-view.byte {
    width: calc(16 * 1.6rem);
}
.memory-cell.byte {
    width: 1.6rem;
}
</style>
