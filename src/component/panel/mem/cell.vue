<script setup lang="ts">
import { ComputedRef, computed, ref, watch } from 'vue';
import { to_fixed_hex } from "@src/util";
import { RegionRef, region_type_to_string } from '@src/rysk_web/rysk_web';
import { buffer } from 'stream/consumers';

const props = defineProps<{
    offset: number,
    buffer: Uint8Array,
    region?: RegionRef
}>();

const ptr: ComputedRef<number | undefined> = computed(() => {
    if (props.region === undefined)
        return undefined;
    // Cell points before the region
    if (props.offset < props.region.address)
        return undefined;
    // Get offset relative to start of region
    const offset = props.offset - props.region.address;
    // Cell points after the region
    if (offset >= props.region.len)
        return undefined;
    // Offset base pointer
    return props.region.ptr + offset;
});

const valid = ref(true);
const text_value = computed({
    get() {
        if (ptr.value !== undefined)
            return to_fixed_hex(props.buffer[ptr.value], 2);
        else
            return undefined;
    },
    set(new_value?: string) {
        if (new_value === undefined)
            return;
        let is_valid = false;
        if (/^[0-9a-fA-F][_0-9a-fA-F]*$/.test(new_value)) {
            const num = Number.parseInt(new_value, 16);
            is_valid = num <= 255 && num >= 0;
            if (is_valid && ptr.value !== undefined)
                props.buffer[ptr.value] = num;
        }
        valid.value = is_valid;
    }
});

</script>

<template>
    <input class="mem-cell" type="text" :disabled="props.region === undefined || text_value === undefined" :invalid="valid ? undefined : true" :value="text_value" @change="text_value=($event.target as HTMLInputElement).value">
</template>

<style scoped>
.mem-cell {
    border: none;
    outline: none;
    background: none;
    font-family: 'Noto Sans Mono';
    padding: 0;
    font-size: 1rem;
    width: 1.6em;
    text-align: center;
}
.mem-cell[disabled] {
    background-color: lightgray;
}
</style>
