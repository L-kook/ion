declare module "ion:timers/timeout" {
    export function setTimeout(
        callback: (...args: Array<any>) => any | Promise<any>,
        duration?: number,
        ...args: Array<any>
    ): number;

    export function clearTimeout(ref: number): void;
}
