// @ts-check

export function setTimeout(
    /** @type {Function} */ callback,
    /** @type {number} */ duration,
    /** @type {Array<any>} */ ...args
) {
    // @ts-expect-error
    return import.meta.extension.setTimeout(() => callback(...args), duration)
}

export function clearTimeout(/** @type {number} */ ref) {
    // @ts-expect-error
    return import.meta.extension.clearTimeout(ref)
}
