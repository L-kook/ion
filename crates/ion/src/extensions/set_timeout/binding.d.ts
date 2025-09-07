export {};

declare global {
    var setTimeout: any;
    var clearTimeout: any;

    interface ImportMeta {
        extension: {
            setTimeout(
                callback: () => any | Promise<any>,
                duration: number
            ): string;
            clearTimeout(timerRef: string): void;
        };
    }
}
