export {};

declare global {
    var setTimeout: any;
    var clearTimeout: any;

    interface ImportMeta {
        extension: {
            setInterval(
                callback: () => any | Promise<any>,
                duration: number
            ): string;
            clearInterval(timerRef: string): void;
        };
    }
}
