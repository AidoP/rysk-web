export type AsyncRequest<Type, Request> = { id: number, type: Type, request: Request };
export type AsyncResponse<Response> = { id: number, response: Response };


type AsyncCallback<Response> = {
    resolve: (value: Response) => void,
    reject: (reason: unknown) => void
};

export type Messages<Type extends string | number> = Record<Type, { request: unknown, response: unknown }>;
export function message_processor<
    Type extends string | number,
    M extends Messages<Type>
>(
    processor: (type: Type, request: M[Type]['request']) => M[Type]['response']
): (event: MessageEvent<AsyncRequest<Type, M[Type]['request']>>) => void {
    return (event) => {
        postMessage({
            id: event.data.id,
            response: processor(event.data.type, event.data.request)
        });
    };
}

export class AsyncWorker<Type extends string | number, M extends Messages<Type>> extends Worker {
    next_id = 0;
    callbacks: Map<number, AsyncCallback<unknown>>;

    constructor(url: string, hook: (response: unknown) => void) {
        super(url, { type: 'module' });
        this.onmessage = this.response;
        this.callbacks = new Map();
        // Register a hook for the first response
        this.callbacks.set(this.next_id++, { resolve: hook, reject: () => {
            // nothing to do on failure
        } });
    }
    async request<T extends Type>(type: T, request: M[T]['request']): Promise<M[T]['response']> {
        const id = this.next_id++;
        this.postMessage({ id, type, request });
        let resolve: unknown;
        let reject: unknown;
        const promise = new Promise((new_resolve, new_reject) => {
            resolve = new_resolve;
            reject = new_reject;
        });
        this.callbacks.set(id, { resolve, reject } as AsyncCallback<unknown>);
        return promise as Promise<M[T]['response']>;
    }
    response(event: MessageEvent<AsyncResponse<unknown>>) {
        this.callbacks.get(event.data.id)?.resolve(event.data.response);
        this.callbacks.delete(event.data.id);
    }
}
