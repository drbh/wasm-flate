# wasm-flate

<img src="https://raw.githubusercontent.com/drbh/wasm-flate/master/images/wasm-flate.png" width="320" />

[![npm version](https://badge.fury.io/js/wasm-flate.svg)](https://badge.fury.io/js/wasm-flate)
[![dependencies Status](https://david-dm.org/dwyl/esta/status.svg)](https://david-dm.org/dwyl/esta)


# Why use `wasm-flate`

## WASM npm library for fast (de)compression

- Uses WASM  
- As fast as C or Rust implementation  
- Works in all browsers  
- Runs on server side
- GZIP supported  
- ZLIB supported  
- DELFATE supported  

### Uses
- Shipping compressed data from server and decompress in browser
- Shipping compressed data to server by compressing in browser
- Better UX for mobile (fast decompress - slow data fetch)
- Better UX for people with sparse networks   
- Less resource use on server side  
- Decreased storage need  
- Leveraging growing WASM ecosystem  

This package allows you to quickly compress and decompress data in the browser. The process is simple and optmized to execute as fast as your browser can run.

#### Compress
Pass a string or Uint8Array to the compression function you choose. The contents will be compressed and encoded to base64. The returned value will be a base64 encoded string.

#### Decompress
Pass a base64 string of the compressed data and it will return a base64 decompressed value.  


### Topics
- [contributing](./developer.md)
- [reasoning](./reasoning.md)


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
- [WAPM installation](./node.md)


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
- [Comparing with pako](./comparision.md)
- [Using in browser](./browser.md)
- [Using in node](./node.md)

## Donate Here
If you found `wasm-flate` useful feel free to buy me a beer 🍺 or two 😀  
`BTC - 3QVK6D5QCZDSyLzFL3ZbELokyuSprRQQZF`
