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