## Installation

Using `NPM`
```
npm i wasm-flate
```

Using `WAPM`
```
wapm install drbh/flate
```

Or use it in you HTML page with the precompiled CDN version
```html
<script src="https://unpkg.com/wasm-flate@0.1.11-alpha/dist/bootstrap.js"></script>
```

### For server side code 
check out the examples for code to decompress data in Python, Node and Go on the server side

## Resources
[Docs](https://github.com/drbh/wasm-flate/blob/master/DOCS.md) - get API reference and flate functions.  

[Examples](https://github.com/drbh/wasm-flate-examples) - Examples compressing in browser, sending to node server and decompressing... really fast!  

## Example Screenshot

![File Upload and Compress UI](https://raw.githubusercontent.com/drbh/wasm-flate-examples/master/images/fileinput.png)


### Compared to JS implementation (pako)

Test `pako` vs `wasm-flate` in your browsers [here](http://wasm-flate.s3-website-us-east-1.amazonaws.com/)

![image](https://raw.githubusercontent.com/drbh/wasm-flate/master/images/compare.png)

In Chrome on my MacBook Pro, we see that pako takes about 143 ms where `wasm-flate` only takes about 21 ms. This is 6.8x faster.  


## Donate Here
If you found `wasm-flate` useful feel free to buy me a beer 🍺 or two 😀  
`BTC - 3QVK6D5QCZDSyLzFL3ZbELokyuSprRQQZF`
