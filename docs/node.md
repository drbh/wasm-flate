# Example Node use

```javascript
var flate = require('wasm-flate');

var data = "THIS IS EXAMPLE DATA TO COMPRESS"
var compressed_data = flate.zlib_encode(data)
var original_data = flate.zlib_decode(compressed_data)
```

### Full Use Example

This will read in the string as a u8array, compress, decompress and compare the output with the original value. This show cases all the main uses of `wasm-flate`  

```javascript
var flate = require('wasm-flate');
var data = new Uint8Array( Buffer.from('wasm-flate is awesome!') );
var comp = flate.deflate_encode_raw(data)
var decomp = flate.deflate_decode_raw(comp)
JSON.stringify(data) === JSON.stringify(decomp) 
```

### Compressing Data

Here we can see how to compress data as a string, and it will return a base64 encoded compressed value.  

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

The folowing output is some compressed JSON of ECR20 tokens traded on a DEX (the data used is arbitrary!)  
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