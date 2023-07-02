import { atom, WritableAtom } from 'nanostores';

import { AsyncWorker } from '@/worker/async_worker';
import { VmMessage, VmMessageInterface } from '@/worker/vm_message';
import worker from '@/worker/vm?worker&url';
import { RegionRef } from '@/rysk_web/rysk_web';

export const $vm = atom(new AsyncWorker<VmMessage, VmMessageInterface>(worker));

export const $regions = atom([] as RegionRef[]);
export const $memory: WritableAtom<WebAssembly.Memory | undefined> = atom();

