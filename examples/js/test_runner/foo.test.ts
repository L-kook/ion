import { test } from 'ion:test'

test("Should do something 1", () => {

})

test("Should do something 2", () => {
  
})

test("Should do something 3", () => {
  
})

test("Should do something 4", () => {
  
})

test("Should do something 5", () => {
  
})

test("Should do something 6", () => {
  
})

test("Should do something 7", () => {
  
})

test("Should do something 8", () => {
  
})

test("Should do something 9", () => {
  
})

// const et = new EventTarget()
const ev = new CustomEvent("foo", "hello world")

globalThis.addEventListener<CustomEvent>("foo", (event) => {
  console.log(event.detail)
})

globalThis.dispatchEvent(ev)
