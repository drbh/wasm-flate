# 🦀🕸️ Shipping Compressed Data

## WASM for fast (de)compression

This is a try package 


## Compressing Data

```javascript
var data = "THIS IS EXAMPLE DATA TO COMPRESS"
var compressed_data = zlib_encode(data)
// this data is zlib compressed and base64 encoded
```

```javascript
var data = "THIS IS EXAMPLE DATA TO COMPRESS"
var compressed_data = gzip_encode(data)
// this data is gzip compressed and base64 encoded
```

```javascript
var data = "THIS IS EXAMPLE DATA TO COMPRESS"
var compressed_data = deflate_encode(data)
// this data is deflate compressed and base64 encoded
```

## Decompressing Data


```javascript
var original_data = zlib_decode("eNrtlEtOAzEMhvc9RTXrLpw4jmMu0EMgFk7GQQjRSjCgSlXvThgG1KpqeUgskOpN/Ppt61tkO5vPu14H7a7m2+a3aHjU3p5afD3G8yk/1vRh/bwalja0cuco+kARE/lucdRz92JjEzgiiZEAE0y23337Pgs2xbhCzOo5o5GnUvpamrSYSotbrooztQPxtAQ2Kk6sFvbcjvK+AHt1JMFqnzU5TqoYoU/76mF9b6vlx36DzOK5BK0eOVLJ5NAFBQPyTiUGqmBejyd8HgHftG6asFt8QVgkCgglCKcBMzI7hCRB4t/xdQYphkwxclOElBO7LD2IQ8SqVjBgqLH+N77nuPoLzx/zxOTPMqWAGC5cf/sP4Gm2Cbj9ANzYBrzwPeQ7vjezN2/3Cpfxnx4=")
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