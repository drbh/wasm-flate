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

Just load the following script tag into your html page. You should have access to flate in the console now.
```html
<script src="https://unpkg.com/wasm-flate@0.1.11-alpha/dist/bootstrap.js"></script>
```

![console](https://raw.githubusercontent.com/drbh/wasm-flate/master/images/console.png)


## Functions

### base64 input and output
```javascript
flate.deflate_decode( base64EncodedString )
returns base64EncodedString
```
```javascript
flate.deflate_encode( base64EncodedString )
returns base64EncodedString
```
```javascript
flate.gzip_decode( base64EncodedString )
returns base64EncodedString
```
```javascript
flate.gzip_encode( base64EncodedString )
returns base64EncodedString
```
```javascript
flate.zlib_decode( base64EncodedString )
returns base64EncodedString
```
```javascript
flate.zlib_encode( base64EncodedString )
returns base64EncodedString
```

#### u8intArray input and output
```javascript
flate.deflate_decode_raw( u8intArray )
returns u8intArray
```
```javascript
flate.deflate_encode_raw( u8intArray )
returns u8intArray
```
```javascript
flate.gzip_decode_raw( u8intArray )
returns u8intArray
```
```javascript
flate.gzip_encode_raw( u8intArray )
returns u8intArray
```
```javascript
flate.zlib_decode_raw( u8intArray )
returns u8intArray
```
```javascript
flate.zlib_encode_raw( u8intArray )
returns u8intArray
```
