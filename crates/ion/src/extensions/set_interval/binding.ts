declare global {
    interface ImportMeta {
        extension: {
            setInterval(
                callback: () => any | Promise<any>,
                duration: number
            ): number;
            clearInterval(timerRef: number): void;
        };
    }
}

export function setInterval(
    callback: (...args: Array<any>) => any | Promise<any>,
    duration: number = 0,
    ...args: Array<any>
): number {
    return import.meta.extension.setInterval(() => callback(...args), duration);
}

export function clearInterval(ref: number) {
    return import.meta.extension.clearInterval(ref);
}
