# Basic Documentation 📖

## Installation
```bash
npm i wasm-flate
```

## Using in Node App
```javascript
var flate = require('wasm-flate');
```

## Using in Browser
```
..coming soon..
```

## Functions

### base64 input and output
```
flate.deflate_decode( base64EncodedString )
returns base64EncodedString
```
```
flate.deflate_encode( base64EncodedString )
returns base64EncodedString
```
```
flate.gzip_decode( base64EncodedString )
returns base64EncodedString
```
```
flate.gzip_encode( base64EncodedString )
returns base64EncodedString
```
```
flate.zlib_decode( base64EncodedString )
returns base64EncodedString
```
```
flate.zlib_encode( base64EncodedString )
returns base64EncodedString
```

#### u8intArray input and output
```
flate.deflate_decode_raw( u8intArray )
returns u8intArray
```
```
flate.deflate_encode_raw( u8intArray )
returns u8intArray
```
```
flate.gzip_decode_raw( u8intArray )
returns u8intArray
```
```
flate.gzip_encode_raw( u8intArray )
returns u8intArray
```
```
flate.zlib_decode_raw( u8intArray )
returns u8intArray
```
```
flate.zlib_encode_raw( u8intArray )
returns u8intArray
```
