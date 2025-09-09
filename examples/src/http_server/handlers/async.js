/** @type {import("./binding").HandlerFunc} */
export function handler(req, res) {
    res.writeHead(200);

    setTimeout(() => {
      res.write("hello");
      res.write(" world");
      res.close()
    }, 1000)
};
