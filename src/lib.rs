// mod utils;
extern crate base64;
extern crate flate2;

use flate2::Compression;
use wasm_bindgen::prelude::*;

use flate2::read::ZlibDecoder;
use std::io::Read;

use flate2::write::ZlibEncoder;
use std::io::Write;

use flate2::read::GzDecoder;

use flate2::write::GzEncoder;

use flate2::read::DeflateDecoder;

use flate2::write::DeflateEncoder;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

/*
base64 input and output
*/

#[wasm_bindgen]
pub fn zlib_decode(base_compressed: &str) -> String {
    let compressed_bytes = base64::decode(&base_compressed).unwrap();
    let mut d = ZlibDecoder::new(&compressed_bytes[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    return s;
}
#[wasm_bindgen]
pub fn zlib_encode(base_raw: &str) -> String {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw.as_bytes())
        .expect("could not compress");
    let compressed_bytes = e.finish();
    let s = base64::encode(&compressed_bytes.unwrap());
    return s;
}

#[wasm_bindgen]
pub fn gzip_decode(base_compressed: &str) -> String {
    let compressed_bytes = base64::decode(&base_compressed).unwrap();
    let mut d = GzDecoder::new(&compressed_bytes[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    return s;
}

#[wasm_bindgen]
pub fn gzip_encode(base_raw: &str) -> String {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw.as_bytes())
        .expect("could not compress");
    let compressed_bytes = e.finish();
    let s = base64::encode(&compressed_bytes.unwrap());
    return s;
}

#[wasm_bindgen]
pub fn deflate_decode(base_compressed: &str) -> String {
    let compressed_bytes = base64::decode(&base_compressed).unwrap();
    let mut deflater = DeflateDecoder::new(&compressed_bytes[..]);
    let mut s = String::new();
    deflater.read_to_string(&mut s).unwrap();
    return s;
}

#[wasm_bindgen]
pub fn deflate_encode(base_raw: &str) -> String {
    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw.as_bytes()).unwrap();
    let bytes = e.finish().unwrap();
    let s = base64::encode(&bytes);
    return s;
}

/*
u8array input and output
*/

#[wasm_bindgen]
pub fn zlib_decode_raw(base_compressed: &[u8]) -> Vec<u8> {
    let mut d = ZlibDecoder::new(&base_compressed[..]);
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer).unwrap();
    return buffer;
}

#[wasm_bindgen]
pub fn zlib_encode_raw(base_raw: &[u8]) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw).expect("could not compress");
    let compressed_bytes = e.finish();
    return compressed_bytes.unwrap();
}

#[wasm_bindgen]
pub fn gzip_decode_raw(base_compressed: &[u8]) -> Vec<u8> {
    let mut d = GzDecoder::new(&base_compressed[..]);
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer).unwrap();
    return buffer;
}

#[wasm_bindgen]
pub fn gzip_encode_raw(base_raw: &[u8]) -> Vec<u8> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw).expect("could not compress");
    let compressed_bytes = e.finish();
    return compressed_bytes.unwrap();
}

#[wasm_bindgen]
pub fn deflate_decode_raw(base_compressed: &[u8]) -> Vec<u8> {
    let mut d = DeflateDecoder::new(&base_compressed[..]);
    let mut buffer = Vec::new();
    d.read_to_end(&mut buffer).unwrap();
    return buffer;
}

#[wasm_bindgen]
pub fn deflate_encode_raw(base_raw: &[u8]) -> Vec<u8> {
    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
    e.write_all(base_raw).unwrap();
    let compressed_bytes = e.finish();
    return compressed_bytes.unwrap();
}
