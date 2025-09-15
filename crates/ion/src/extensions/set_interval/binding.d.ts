declare module "ion:timers/interval" {
    export function setInterval(
        callback: (...args: Array<any>) => any | Promise<any>,
        duration?: number,
        ...args: Array<any>
    ): number;

    export function clearInterval(ref: string): void;
}
