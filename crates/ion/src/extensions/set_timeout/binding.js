// @ts-check
// @ts-expect-error
const { onTimeoutCallback, createSetTimeout } = import.meta.extension;

/** @type {Map<number, Function>} */
const refs = new Map();
let counter = 0;

onTimeoutCallback((/** @type {number} */ ref) => {
    const callback = refs.get(ref)
    if (!callback) {
        return
    }
    callback()
});

export function setTimeout(
    /** @type {Function} */ callback,
    /** @type {number} */ duration,
    /** @type {Array<any>} */ ...args
) {
    const ref = counter;
    counter += 1;

    refs.set(ref, () => {
        callback(...args);
        refs.delete(ref);
    });

    createSetTimeout(ref, duration);

    return ref;
}

export function clearTimeout(/** @type {number} */ ref) {
    refs.delete(ref);
}
