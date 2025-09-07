interface ImportMeta {
    extension: {
        setTimeout(callback: () => any | Promise<any>, duration: number): string;
        clearTimeout(timerRef: string): void;
    }
}
