import * as stdEventTarget from 'ion:event_target'
import * as stdConsole from 'ion:console'
import * as stdTimersInterval from 'ion:timers/interval'
import * as stdTimersTimeout from 'ion:timers/timeout'

declare global {
  var self: typeof globalThis
  
  var addEventListener: InstanceType<typeof stdEventTarget.EventTarget>['addEventListener']
  var removeEventListener: InstanceType<typeof stdEventTarget.EventTarget>['removeEventListener']
  var dispatchEvent: InstanceType<typeof stdEventTarget.EventTarget>['dispatchEvent']
  var Event: typeof stdEventTarget.Event
  var CustomEvent: typeof stdEventTarget.CustomEvent

  var console: typeof stdConsole.console

  var setInterval: typeof stdTimersInterval.setInterval
  var clearInterval: typeof stdTimersInterval.clearInterval
  var setTimeout: typeof stdTimersTimeout.setTimeout
  var clearTimeout: typeof stdTimersTimeout.clearTimeout
}

export {}
