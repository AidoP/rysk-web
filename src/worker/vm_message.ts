import { RegionRef } from '@/rysk_web/rysk_web';

export enum VmMessage {
    GetRegions,
    GetMemory,
    SetMemory
}
export type VmMessageInterface = {
    [VmMessage.GetRegions]: {
        request: undefined,
        response: RegionRef[]
    }
    [VmMessage.GetMemory]: {
        request: { address: number, length: number },
        response: (number | undefined)[]
    }
    [VmMessage.SetMemory]: {
        request: { address: number, data: Uint8Array },
        response: undefined
    }
};

export type VmMessageRequest<M extends VmMessage> = VmMessageInterface[M]['request'];
export type VmMessageResponse<M extends VmMessage> = VmMessageInterface[M]['response'];
