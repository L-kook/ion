declare global {
    interface ImportMeta {
        extension: {
            test(message: string, callback: () => (any | Promise<any>)): void;
        };
    }
}

export type TestFunc = () => (any | Promise<any>)

export const test = (message: string, callback: TestFunc) => {
}

export const it = test

export const before = () => {}
export const after = () => {}
