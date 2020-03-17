# steps to reproduce

- used https://github.com/rollup/plugins/tree/master/packages/wasm

## this should probably work much like the webpack example, minus the webpack bug from
https://github.com/rustwasm/wasm-bindgen/blob/master/examples/hello_world/index.js

## build rust_wasm/pkg containing the js and wasm files

(this step can be skipped because I committed the files to git for convenience/ not everyone wants to install the rust things)
```bash
cd rust_wasm
wasm-pack build
```

```bash
cd ..
cd svelte_template
npm install
npm run dev
```

## I also added the bundle.js files to show the result of rollup bundling

# result:
error in console:
```
rust_wasm.js:9 Uncaught TypeError: undefined is not a function
    at add (rust_wasm.js:9)
    at Object.create [as c] (App.svelte:8)
    at init (index.mjs:1410)
    at new App (App.svelte:3)
    at main.js:3
    at main.js:8
add @ rust_wasm.js:9
create @ App.svelte:8
init @ index.mjs:1410
App @ App.svelte:3
(anonymous) @ main.js:3
(anonymous) @ main.js:8
```

because in in bundle.js:
```js
    function add(a, b) {
        var ret = undefined(a, b);
        return ret;
    }
```
rollup seems to optimize the call away

```js
function rust_wasm_bg(imports){return _loadWasmModule(0, 'AGFzbQEAAAABBwFgAn9/AX8DAgEABQMBABEHEAIGbWVtb3J5AgADYWRkAAAKDQELACAAIAFqQf8BcQsLUgEAQaSIwAALSQEAAAAAAAAAAQAAAAIAAAADAAAABAAAAAUAAAAAAAAAAQAAAAIAAAADAAAABAAAAAYAAAAEAAAABAAAAAcAAAAIAAAACQAAAAoAgwEJcHJvZHVjZXJzAghsYW5ndWFnZQEEUnVzdAAMcHJvY2Vzc2VkLWJ5AwVydXN0YyUxLjQzLjAtbmlnaHRseSAoNTY0NzU4YzRjIDIwMjAtMDMtMDgpBndhbHJ1cwYwLjE0LjAMd2FzbS1iaW5kZ2VuEjAuMi41OSAoZGI4ZDNlNDQxKQ==', imports)}

var wasm = /*#__PURE__*/Object.freeze({
        __proto__: null,
        'default': rust_wasm_bg
    });

```
As far as I can see the wasm module gets embedded into the js file but _loadWasmModule is never actually called, so this might actually be very close to being in a working state.


Thanks for you help!
