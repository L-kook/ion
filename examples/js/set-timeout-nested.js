import console from 'ion:console'
import { setTimeout } from 'ion:timers/timeout'

console.log("Sync start")

setTimeout(() => {
    console.log("Async starting")

    setTimeout(() => console.log("Async done 1000"), 1000)
    setTimeout(() => console.log("Async done 2000"), 2000)
    setTimeout(() => console.log("Async done 3000"), 3000)
    setTimeout(() => console.log("Async done 4000"), 4000)
}, 1000)

console.log("Sync end")
