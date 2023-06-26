import { atom } from 'nanostores'
import { RyskVm } from '@src/rysk_web';

const default_args = {
    hart: {
        memory_pages: 1024
    }
};

export const $vm = atom(new RyskVm(default_args));
