export function setInterval(
    /** @type {Function} */ callback,
    /** @type {number} */ duration = 0,
    /** @type {Array<any>} */ ...args
) {
    return import.meta.extension.setInterval(() => callback(...args), duration);
}

export function clearInterval(/** @type {string} */ ref) {
    return import.meta.extension.clearInterval(ref);
}

globalThis.setInterval = setInterval;
globalThis.clearInterval = clearInterval;
