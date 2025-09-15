declare global {
    interface ImportMeta {
        extension: {
            test(message: string, callback: () => (any | Promise<any>)): void;
        };
    }
}

export type TestFunc = () => (any | Promise<any>)

let tests: Array<[string, TestFunc]> = []

export const test = (message: string, callback: TestFunc) => {
  tests.push([message, callback])
}

export const it = test

export const before = () => {}

export const after = () => {}

export const getTests = (): Array<[string, TestFunc]> => {
  return [
    ...tests
  ]
}