
### Compared to JS implementation (pako)

Test `pako` vs `wasm-flate` in your browsers [here](http://wasm-flate.s3-website-us-east-1.amazonaws.com/)

![image](https://raw.githubusercontent.com/drbh/wasm-flate/master/images/compare.png)

In Chrome on my MacBook Pro, we see that pako takes about 143 ms where `wasm-flate` only takes about 21 ms. This is 6.8x faster.  