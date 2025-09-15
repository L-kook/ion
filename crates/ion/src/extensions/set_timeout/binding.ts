declare global {
    interface ImportMeta {
        extension: {
            setTimeout(
                callback: () => any | Promise<any>,
                duration: number
            ): number;
            clearTimeout(timerRef: number): void;
        };
    }
}

export function setTimeout(
    callback: (...args: Array<any>) => any | Promise<any>,
    duration: number = 0,
    ...args: Array<any>
): number {
    return import.meta.extension.setTimeout(() => callback(...args), duration);
}

export function clearTimeout(ref: number) {
    return import.meta.extension.clearTimeout(ref);
}
