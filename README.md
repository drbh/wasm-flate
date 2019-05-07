# 🦀🕸️ wasm-flate  

The fastest compression and decompression in the browser.

Installation
```
npm i wasm-flate
```
or you can use the dev pre compiled files at (this is unstable and may not be maintained)
```
<script src="https://compiled-wasm-flate.s3.amazonaws.com/bootstrap.js"></script>
```

# Why use `wasm-flate`

## WASM npm library for fast (de)compression

- Uses WASM  
- As fast as C or Rust implementation  
- Works in all browsers  
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

### Compared to Pako

Test `pako` vs `wasm-flate` in your browsers [here](http://wasm-flate.s3-website-us-east-1.amazonaws.com/)

![image](images/compare.png)

In Chrome on my MacBook Pro, we see that pako takes about 143 ms where `wasm-flate` only takes about 21 ms. This is 6.8x faster.  

# Example Node use

```javascript
var flate = require('wasm-flate');

var data = "THIS IS EXAMPLE DATA TO COMPRESS"
var compressed_data = flate.zlib_encode(data)
var original_data = flate.zlib_decode(compressed_data)
```

### Compressing Data

```javascript
var data = "THIS IS EXAMPLE DATA TO COMPRESS"
var compressed_data = flate.zlib_encode(data)
// this data is zlib compressed and base64 encoded
```

```javascript
var data = "THIS IS EXAMPLE DATA TO COMPRESS"
var compressed_data = flate.gzip_encode(data)
// this data is gzip compressed and base64 encoded
```

```javascript
var data = "THIS IS EXAMPLE DATA TO COMPRESS"
var compressed_data = flate.deflate_encode(data)
// this data is deflate compressed and base64 encoded
```

### Decompressing Data

```javascript
var original_data = flate.zlib_decode("eNrtlEtOAzEMhvc9RTXrLpw4jmMu0EMgFk7GQQjRSjCgSlXvThgG1KpqeUgskOpN/Ppt61tkO5vPu14H7a7m2+a3aHjU3p5afD3G8yk/1vRh/bwalja0cuco+kARE/lucdRz92JjEzgiiZEAE0y23337Pgs2xbhCzOo5o5GnUvpamrSYSotbrooztQPxtAQ2Kk6sFvbcjvK+AHt1JMFqnzU5TqoYoU/76mF9b6vlx36DzOK5BK0eOVLJ5NAFBQPyTiUGqmBejyd8HgHftG6asFt8QVgkCgglCKcBMzI7hCRB4t/xdQYphkwxclOElBO7LD2IQ8SqVjBgqLH+N77nuPoLzx/zxOTPMqWAGC5cf/sP4Gm2Cbj9ANzYBrzwPeQ7vjezN2/3Cpfxnx4=")
```
output:  
```json
{
  "data": {
    "trades": [
      {
        "amountGet": "15624563852",
        "amountGive": "10155966503800000000",
        "get": "0xce7f06ba27b3e525ccdfc559cea9e523e5f91eae",
        "give": "0xa919efc72756222c072a1594efdba8178aa360d8",
        "tokenGet": "0xe0b7927c4af23765cb51314a0e0521a9645f0e2a",
        "tokenGive": "0x0000000000000000000000000000000000000000"
      }
  ], "..."
}
```

# Building with Rust

In order to build the wasm files with Rust, you'll need to clone the repo and run `wasm-pack` with `nodejs` as the target. This will create a set of files in `pkg` that can be used as a node module. 

```
git clone https://github.com/drbh/wasm-flate.git
cd wasm-flate
wasm-pack build --target nodejs
```

You should have the following new files  
```
pkg/
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── wasm-flate.d.ts
├── wasm-flate.js
├── wasm-flate_bg.d.ts
├── wasm-flate_bg.js
├── wasm-flate_bg.wasm
└── package.json
```


