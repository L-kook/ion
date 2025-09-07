// @ts-check

export function setTimeout(
    /** @type {Function} */ callback,
    /** @type {number} */ duration = 0,
    /** @type {Array<any>} */ ...args
) {
    return import.meta.extension.setTimeout(() => callback(...args), duration)
}

export function clearTimeout(/** @type {string} */ ref) {
    return import.meta.extension.clearTimeout(ref)
}

// @ts-expect-error
globalThis.setTimeout = setTimeout
// @ts-expect-error
globalThis.clearTimeout = clearTimeout