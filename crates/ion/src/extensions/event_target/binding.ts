type EventListener = (event: Event) => any | Promise<any>;
type EventListenerOptions = { once?: boolean };

const listenersGlobal: Map<string, Array<[EventListener, EventListenerOptions]>> = new Map()

export class EventTarget {
    addEventListener(
        type: string,
        listener: EventListener,
        options: EventListenerOptions = {}
    ) {
        let listeners = listenersGlobal.get(type);
        if (!listeners) {
            listeners = [];
            listenersGlobal.set(type, listeners);
        }
        listeners.push([listener, options]);
    }

    removeEventListener(type: string, listener: EventListener) {
        let listeners = listenersGlobal.get(type);
        if (!listeners) {
            return;
        }
        let index = listeners.findIndex((v) => v[0] === listener);
        listeners.splice(index, 1);
    }

    dispatchEvent(event: Event) {
        const listeners = listenersGlobal.get(event.type)?.values() || [];
        for (const [listener, options] of listeners) {
            event.target = this;
            event.currentTarget = this;
            listener(event);
            event.target = null;
            event.currentTarget = null;
            if (options.once) {
                this.removeEventListener(event.type, listener)
            }
        }
    }
}

export class Event {
    isTrusted: boolean;
    bubbles: boolean;
    cancelBubble: boolean;
    cancelable: boolean;
    composed: boolean;
    currentTarget: null | EventTarget;
    defaultPrevented: boolean;
    eventPhase: number;
    returnValue: boolean;
    srcElement: null;
    target: null | EventTarget;
    timeStamp: number;
    type: string;

    constructor(type: string) {
        this.isTrusted = false;
        this.bubbles = false;
        this.cancelBubble = false;
        this.cancelable = false;
        this.composed = false;
        this.currentTarget = null;
        this.defaultPrevented = false;
        this.eventPhase = 0;
        this.returnValue = false;
        this.srcElement = null;
        this.target = null;
        this.timeStamp = Date.now();
        this.type = type;
    }
}

export class CustomEvent extends Event {
    detail: any;

    constructor(type: string, detail?: any) {
        super(type)
        this.detail = detail
    }
}
