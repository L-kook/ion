declare module "ion:event_target" {
    export type EventListener<Ev> = (event: Ev) => any | Promise<any>;
    export type EventListenerOptions = { once?: boolean };

    export class EventTarget {
        addEventListener<Ev = Event>(
            type: string,
            listener: EventListener<Ev>,
            options?: EventListenerOptions
        ): void;
        removeEventListener<Ev = Event>(
            type: string,
            listener: EventListener<Ev>
        ): void;
        dispatchEvent(event: Event): void;
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
    }

    export class CustomEvent extends Event {
        detail: any;
        constructor(type: string, detail?: any);
    }
}
