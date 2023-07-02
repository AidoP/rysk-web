export enum VmMessage {
    GetMemory,
    SetMemory
}
export type VmMessageInterface = {
    [VmMessage.GetMemory]: {
        request: { address: number, length: number },
        response: { memory: Uint8Array }
    }
    [VmMessage.SetMemory]: {
        request: { address: number, data: Uint8Array },
        response: object
    }
};

export type VmMessageRequest<M extends VmMessage> = VmMessageInterface[M]['request'];
export type VmMessageResponse<M extends VmMessage> = VmMessageInterface[M]['response'];
