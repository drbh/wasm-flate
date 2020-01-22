



# Server side

We want to build with WASI so we can run `wasm-flate` on a server. All we need to do is pass the target that we want and now we can execute the `wasm` binary with a server side VM like `wasmer`

### Build for wasi
```bash
cargo build --target wasm32-wasi --release --features "strings"
```

```bash
wasmer run target/wasm32-wasi/debug/basic.wasm 
```

### WAPM

```bash
wapm install drbh/flate
wapm run flate_basic "hello world"
```

### Topics 
- Using CLI app
- WAPM installation
- Integratation into code (Node, Go, Rust...)


# Client side

We want to build with `wasm bindgen` because it will automatically generate the needed JS files that communicate with the `wasm` binary.

This Javascript file can be generated for a varity of targets. We can build with no modules for static pages and also build for times we need a bundlers (webpack, parcel...).


### Build for ES6
```bash
wasm-pack build -- --features "browser"
```

### Build for NodeJS
```bash
wasm-pack build --target nodejs -- --features "browser"
```

### Build for browser
```bash
wasm-pack build --target no-modules -- --features "browser"
```

### Build for bundler
```bash
wasm-pack build --target bundler -- --features "browser"
```

### Topics 
- Comparing with pako
- Packaging with 


## Donate Here
If you found `wasm-flate` useful feel free to buy me a beer 🍺 or two 😀  
`BTC - 3QVK6D5QCZDSyLzFL3ZbELokyuSprRQQZF`
