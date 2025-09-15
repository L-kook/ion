import { EventTarget, Event, CustomEvent } from 'ion:event_target'
import { console } from 'ion:console'
import { setInterval, clearInterval } from 'ion:timers/interval'
import { setTimeout, clearTimeout } from 'ion:timers/timeout'

globalThis.self = globalThis

Object.setPrototypeOf(globalThis, new EventTarget);
globalThis.Event = Event
globalThis.CustomEvent = CustomEvent

globalThis.setInterval = setInterval
globalThis.clearInterval = clearInterval
globalThis.setTimeout = setTimeout
globalThis.clearTimeout = clearTimeout

globalThis.console = console