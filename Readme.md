using rollup with https://github.com/wasm-tool/rollup-plugin-rust

```
  [13:07:28] 200 ─ 1.15ms ─ /build/bundle.js
  [13:07:29] 200 ─ 0.65ms ─ /build/bundle.css.map
  [13:07:29] 404 ─ 0.19ms ─ /rust_wasm.wasm
  [13:07:29] 200 ─ 1.69ms ─ /build/bundle.js.map
```
the bundles wasm resource is not in the right place and the async api is difficult to use (with the dom) compared to https://github.com/webpack/webpack/tree/master/examples/wasm-simple
