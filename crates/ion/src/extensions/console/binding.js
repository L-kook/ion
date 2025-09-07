export default class Console {
    static log(/** @type {Array<any>} */ ...args) {
        import.meta.extension.log(...args);
    }

    static error(/** @type {Array<any>} */ ...args) {
        import.meta.extension.error(...args);
    }

    static warn(/** @type {Array<any>} */ ...args) {
        import.meta.extension.warn(...args);
    }
}

export const console = Console;

globalThis.console = console;
