<script setup lang="ts">
    import { computed, ComputedRef, ref, Ref, watch } from 'vue';
    import { useStore } from '@nanostores/vue';
    import { region_type_to_string, RegionRef } from '@src/rysk_web/rysk_web';
    import { memory } from '@src/rysk_web/rysk_web_bg.wasm';

    import { to_fixed_hex } from "@src/util";

    import cell from '@panel/mem/cell.vue'
    
    import { $vm } from '@store/vm';
    const vm = useStore($vm);
    const buffer = new Uint8Array(memory.buffer);

    const regions: ComputedRef<RegionRef[]> = computed(() => vm.value.regions());
    const selected_region: Ref<number | undefined> = ref(undefined);
    const region: ComputedRef<RegionRef | undefined> = computed(() => selected_region.value !== undefined ? regions.value[selected_region.value] : undefined);

    const region_name = computed(() => {
        if (region.value !== undefined) {
            return region_type_to_string(region.value.ty);
        } else {
            return "None";
        }
    });

    import hex from '@c/hex.vue';
    let $offset = ref(0);
    const offset = computed({
        get() {
            return $offset.value;
        },
        set(new_value: number) {
            if (region.value !== undefined) {
                $offset.value = new_value;
            }
        }
    });

    watch(region, () => {
        offset.value = region?.value?.address ?? 0;
    });
</script>

<template>
    <div class="panel">
        <div class="address-bar">
            <hex v-model="offset" :disabled="region === undefined" :min="region?.address" :max="(region !== undefined ? (region?.address + region?.len) : undefined)" :align="4" />
            <select v-model="selected_region">
                <option disabled :value="undefined">Memory Region</option>
                <option v-for="(r, i) in regions" :value="i">{{ region_type_to_string(r.ty) }} @ 0x{{ to_fixed_hex(r.address) }}</option>
            </select>
            <div v-if="region !== undefined">
                <code>0x{{ to_fixed_hex(region.address) }} - 0x{{ to_fixed_hex(region.address + region.len) }}</code>
            </div>
        </div>
        <div @wheel="offset += Math.sign($event.deltaY) * 64">
            <div class="mem-header">
                <code class="mem-header-name">Address</code>
                <code class="mem-header-cell" v-for="index in 16">{{ to_fixed_hex(index-1, 2) }}</code>
            </div>
            <div class="mem-line" v-for="line in 16">
                <code class="mem-address">{{ "0x" + to_fixed_hex(offset + (line-1)*16) }}</code>
                <cell :buffer="buffer" :offset="(offset + (line-1)*16) + (index-1)" :region="region" v-for="index in 16" />
            </div>
        </div>
    </div>
</template>

<style scoped>
.address-bar {
    padding-bottom: 1rem;
}
.mem-header {
    border-bottom: 1px solid black;
}
.mem-header-name {
    display: inline-block;
    width: 7rem;
    font-size: 1rem;
}
.mem-header-cell {
    display: inline-block;
    padding: 0;
    width: 1.6em;
    font-size: 1rem;
    text-align: center;
}
.mem-address {
    display: inline-block;
    width: 7rem;
}
.mem-line {
    font-size: 1rem;
}
</style>
