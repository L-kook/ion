import * as stdEventTarget from 'ion:event_target'
import * as stdConsole from 'ion:console'
import * as stdTimersInterval from 'ion:timers/interval'
import * as stdTimersTimeout from 'ion:timers/timeout'

declare global {
  var self: typeof globalThis
  
  var addEventListener: InstanceType<typeof stdEventTarget.EventTarget>['addEventListener']
  var removeEventListener: InstanceType<typeof stdEventTarget.EventTarget>['removeEventListener']
  var dispatchEvent: InstanceType<typeof stdEventTarget.EventTarget>['dispatchEvent']
  type Event = stdEventTarget.Event;
  var Event: typeof stdEventTarget.Event
  type CustomEvent = stdEventTarget.CustomEvent;
  var CustomEvent: typeof stdEventTarget.CustomEvent

  var console: typeof stdConsole.default

  var setInterval: typeof stdTimersInterval.setInterval
  var clearInterval: typeof stdTimersInterval.clearInterval
  var setTimeout: typeof stdTimersTimeout.setTimeout
  var clearTimeout: typeof stdTimersTimeout.clearTimeout
}

export {}
