# 🗜️⚡ wasm-flate <img src="https://img.shields.io/github/stars/drbh/wasm-flate.svg" /> [![wapm version](https://wapm.io/package/drbh/flate/badge.svg?style=flat)](https://wapm.io/package/drbh/flate)
 [![Tweet](https://img.shields.io/twitter/url/http/shields.io.svg?style=social)](https://twitter.com/intent/tweet?text=The%20fastest%20compression%20library%20in%20your%20browser.&url=https://github.com/drbh/wasm-flate&hashtags=wasm,js,webdev,rust,compression) 


<img src="https://raw.githubusercontent.com/drbh/wasm-flate/master/images/wasm-flate.png" width="320" />

[![npm version](https://badge.fury.io/js/wasm-flate.svg)](https://badge.fury.io/js/wasm-flate)
[![dependencies Status](https://david-dm.org/dwyl/esta/status.svg)](https://david-dm.org/dwyl/esta)

WebAssembly powered compression and decompression in the browser and server.

## Docs

[Documentation](https://drbh.github.io/wasm-flate/)


```js
var flate = require('wasm-flate');
var pako = require('pako');

var data = new Uint8Array( Buffer.from('Hello World') );
// Uint8Array [
//    72, 101, 108, 108,
//   111,  32,  87, 111,
//   114, 108, 100
// ]
```

compress with wasm-flate
```js
flate.deflate_encode_raw(data)
// Uint8Array [
//   243, 72, 205, 201, 201,
//    87,  8, 207,  47, 202,
//    73,  1,   0
// ]
```


Then compress with pako
```js
pako.deflateRaw(data)
// Uint8Array [
//   243, 72, 205, 201, 201,
//    87,  8, 207,  47, 202,
//    73,  1,   0
// ]
```
 
you can see the output is the same 🎉

you can even compress with one library and decompress with the other one

```js
var data = new Uint8Array( Buffer.from('Hello World') );
var compressed_with_wasm_flate = flate.deflate_encode_raw(data)
var decompressed_with_pako = pako.inflateRaw(compressed_with_wasm_flate)

// covert to a string - since JS doesnt let us directly compare UintArrays
var original = new TextDecoder("utf-8").decode(data)
var evaluated = new TextDecoder("utf-8").decode(decompressed_with_pako)
original === evaluated
// true
```


## Donate Here
If you found `wasm-flate` useful feel free to buy me a beer 🍺 or two 😀  
`BTC - 3QVK6D5QCZDSyLzFL3ZbELokyuSprRQQZF`
