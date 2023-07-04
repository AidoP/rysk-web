import { atom } from 'nanostores';

import { AsyncWorker } from '@/worker/async_worker';
import { VmMessage, VmMessageInterface } from '@/worker/vm_message';
import worker from '@/worker/vm?worker&url';
import { RegionRef } from '@/rysk_web/rysk_web';

export const $regions = atom([] as RegionRef[]);

export const $vm = atom(new AsyncWorker<VmMessage, VmMessageInterface>(worker, () => {
    $vm.get().request(VmMessage.GetRegions, undefined).then(regions => {
        $regions.set(regions);
    });
}));
