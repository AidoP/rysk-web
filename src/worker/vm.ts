import { memory } from '@/rysk_web/rysk_web_bg.wasm';
import { message_processor } from './async_worker';
import { VmMessage, VmMessageInterface, VmMessageRequest } from './vm_message';
import { RyskVm } from '@/rysk_web';

const default_args = {
    hart: {
        memory_pages: 1024
    }
};
const vm = new RyskVm(default_args);
const mem = new Uint8Array(memory.buffer);

/* eslint-disable @typescript-eslint/no-explicit-any */
onmessage = message_processor<VmMessage, VmMessageInterface>((type, request) => {
    switch (type) {
    case VmMessage.GetMemory:
        return get_memory(request as any);
    case VmMessage.SetMemory:
        return set_memory(request as any);
    }
});
/* eslint-enable @typescript-eslint/no-explicit-any */

const get_memory = (request: VmMessageRequest<VmMessage.GetMemory>) => {
    const region = vm.get_memory(request.address);
    const offset = region.ptr + (request.address - region.address);
    return { memory: mem.slice(offset, offset + request.length) };
};
const set_memory = (request: VmMessageRequest<VmMessage.SetMemory>) => {
    const data = request as VmMessageRequest<VmMessage.SetMemory>;
    vm.set_memory(data.address, data.data);
    return {};
};
