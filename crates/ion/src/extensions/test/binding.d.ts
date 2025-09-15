declare module "ion:test" {
    export type TestFunc = () => any | Promise<any>;

    export function test(message: string, callback: TestFunc): void;
    export function it(message: string, callback: TestFunc): void;

    export function before(callback: TestFunc): void;
    export function after(callback: TestFunc): void;
}
