export default class Console {
    static log(...args) {
        import.meta.extension.log(...args)
    }

    static error(...args) {
        import.meta.extension.error(...args)
    }

    static warn(...args) {
        import.meta.extension.warn(...args)
    }
}

export const console = Console
