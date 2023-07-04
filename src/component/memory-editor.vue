<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import AddressBar from './memory/address-bar.vue';
import ByteView from './memory/byte-view.vue';
import CharView from './memory/char-view.vue';
import { to_hex } from '@/lib/hex';
import { useStore } from '@nanostores/vue';
import { $regions, $vm } from '@/store/vm';
import { VmMessage } from '@/worker/vm_message';

const props = defineProps<{
    width: number,
    height: number,
    tabindex?: number,
    min: number,
    max: number
}>();

const vm = useStore($vm);
const memory = ref();
const $address = ref(0xf000_0000);
const circular_address = (address: number) => {
    if (address < props.min) {
        address = 0xffff_ffff + address % 0x1_0000_0000;
    } else if (address > props.max) {
        address = address % 0x1_0000_0000;
    }
    return address;
};
const address = computed({
    get() {
        return $address.value;
    },
    set(new_value) {
        $address.value = Math.floor(circular_address(new_value) / props.width) * props.width;
    }
});
const get_memory = () => {
    const addr = address.value;
    // Get enough memory for the whole utf-8 code point if ending on utf-8 code point
    vm.value.request(VmMessage.GetMemory, { address: addr, length: props.width * props.height + 3 }).then(response => {
        memory.value = response;
    });
};
const set_memory = ([x, y]: [number, number], data: Uint8Array) => {
    vm.value.request(VmMessage.SetMemory, { address: address.value + x + props.width * y, data });
    get_memory();
};
$regions.subscribe(() => {
    get_memory();
});
watch(() => address.value, () => {
    get_memory();
});

const focused = ref();
const selected = ref();
const hovered = ref();
const height = computed(() => memory.value !== undefined ? Math.min(Math.ceil(memory.value.length / props.width), 16) : 0 );

// Cell Events
const cell_select = (position?: [number, number]): boolean => {
    if (position === undefined) {
        selected.value = undefined;
        return false;
    }
    let [x, y] = position;
    y += Math.floor(x / props.width);
    x = x % props.width;
    if (x < 0) {
        x += props.width;
    }
    if (y < 0) {
        address.value -= props.width;
        return false;
    } else if (y >= height.value) {
        address.value += props.width;
        return false;
    }
    selected.value = [x, y];
    return true;
};
const cell_hover = (position?: [number, number]) => {
    hovered.value = position;
};
const byte_select = (position?: [number, number]) => {
    if (cell_select(position)) {
        focused.value = 'byte';
    }
};
const char_select = (position?: [number, number]) => {
    if (cell_select(position)) {
        focused.value = 'char';
    }
};
const byte_write = (position: [number, number], value: number) => {
    set_memory(position, new Uint8Array([value]));
};
const char_write = (position: [number, number], data: Uint8Array) => {
    set_memory(position, data);
};
</script>

<template>
    <address-bar v-model:value="address" />
    <div @wheel="address += Math.sign($event.deltaY) * 2 * props.width">
        <div class="memory-view address">
            <span class="memory-cell address">Address</span>
            <span v-for="(_, y) in height" :key="'address-' + y" class="memory-cell address">0x{{ to_hex(circular_address(address + y * props.width), 8) }}</span>
        </div>
        <byte-view
            :memory="memory"
            :focused="focused === 'byte'"
            :selected="selected"
            :hovered="hovered"
            :width="props.width"
            :height="height"
            :tabindex="tabindex"
            @select="byte_select"
            @hover="cell_hover"
            @write="byte_write"
        />
        <char-view
            :memory="memory"
            :focused="focused === 'char'"
            :selected="selected"
            :hovered="hovered"
            :width="props.width"
            :height="height"
            :tabindex="tabindex"
            @select="char_select"
            @hover="cell_hover"
            @write="char_write"
        />
    </div>
</template>

<style>
.memory-view {
    display: inline-block;
}

.memory-view.address, .memory-cell.address {
    width: 7rem;
}

.memory-view.byte {
    margin-right: 1.4rem;
}

.memory-cell {
    display: inline-block;
    font-family: 'Noto Sans Mono';
    font-size: 1rem;
    height: 1.6rem;
    text-align: center;
    vertical-align: middle;
    cursor: default;
    outline: none;
}

.memory-cell.hovered {
    background-color: rgb(230, 230, 230);
}

.memory-cell.selected {
    background-color: rgb(210 210 210);
}

.memory-cell.disabled {
    background-color: rgb(40, 40, 40);
}
</style>
