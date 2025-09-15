declare module "ion:console" {
    export default class Console {
        static log(/** @type {Array<any>} */ ...args: any[]): void
        static error(/** @type {Array<any>} */ ...args: any[]): void
        static warn(/** @type {Array<any>} */ ...args: any[]): void
    }

    export const console: Console;
}
