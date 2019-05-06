import * as wasm from "wasm-data-minify";

// window.decompress = function(value) {
//     var og = wasm.greet(value);
//     console.log(og)
//     return og
// }

window.zlib_decode = function(value) {
    var dec = wasm.zlib_decode(value);
    return dec
}

window.zlib_encode = function(value) {
    var enc = wasm.zlib_encode(value);
    return enc
}

window.gzip_decode = function(value) {
    var dec = wasm.gzip_decode(value);
    return dec
}

window.gzip_encode = function(value) {
    var enc = wasm.gzip_encode(value);
    return enc
}

window.deflate_decode = function(value) {
    var dec = wasm.deflate_decode(value);
    return dec
}

window.deflate_encode = function(value) {
    var enc = wasm.deflate_encode(value);
    return enc
}


window.timePression = function(original_data, argument) {
    var a = performance.now();
    var compression_algo = console.log
    switch (argument) {
        case "z":
            compression_algo = zlib_encode
            break;
        case "g":
            compression_algo = gzip_encode
            break;
        default:
            compression_algo = deflate_encode
    }

    var compressed_data = compression_algo(original_data)
    var b = performance.now();
    var compress_ratio = (original_data.length - compressed_data.length) / original_data.length

    var pay = {
        "original-length": original_data.length,
        "compressed-length": compressed_data.length,
        "compression-ratio": compress_ratio,
        "compression-time": b - a
    }
    return pay
}