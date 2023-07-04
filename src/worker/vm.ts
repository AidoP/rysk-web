import { memory } from '@/rysk_web/rysk_web_bg.wasm';
import { message_processor } from './async_worker';
import { VmMessage, VmMessageInterface, VmMessageRequest } from './vm_message';
import { RegionRef, RegionType, RyskVm } from '@/rysk_web';

const default_args = {
    hart: {
        memory_pages: 1024
    }
};
const vm = new RyskVm(default_args);
let regions: RegionRef[] = [];
const vm_memory = new Uint8Array(memory.buffer);

/* eslint-disable @typescript-eslint/no-explicit-any */
onmessage = message_processor<VmMessage, VmMessageInterface>((type, request) => {
    switch (type) {
    case VmMessage.GetRegions:
        regions = vm.regions();
        return regions;
    case VmMessage.GetMemory:
        return get_memory(request as any);
    case VmMessage.SetMemory:
        return set_memory(request as any);
    }
});
/* eslint-enable @typescript-eslint/no-explicit-any */

// Signal that messages will be processed now
postMessage({ id: 0, response: null });

const process_region = (request: { address: number, length: number }, region: RegionRef, memory: (number | undefined)[]): (number | undefined)[] => {
    if (region.ty == RegionType.Dram) {
        const mem = vm.get_memory(Math.max(region.address, request.address));
        const len = Math.min(mem.len, request.length - memory.length);
        return [...memory, ...vm_memory.subarray(mem.ptr, mem.ptr + len)];
    } else {
        const len = Math.min(region.len - (request.address - region.address), request.length - memory.length);
        return [...memory, ...new Array(len).fill(undefined)];
    }
};

const get_memory = (request: VmMessageRequest<VmMessage.GetMemory>): (number | undefined)[] => {
    let memory: (number | undefined)[] = [];
    for (const region of regions) {
        if (request.address > region.address + region.len) {
            continue;
        }
        if (region.address > request.address + request.length) {
            break;
        }
        memory = process_region(request, region, memory);
    }
    if (memory.length < request.length) {
        const wrapped_region = regions[0];
        wrapped_region.address += 0xffff_ffff;
        memory = process_region(request, wrapped_region, memory);
    }
    return memory;
};

const set_memory = (request: VmMessageRequest<VmMessage.SetMemory>) => {
    const data = request as VmMessageRequest<VmMessage.SetMemory>;
    vm.set_memory(data.address, data.data);
    return undefined;
};
