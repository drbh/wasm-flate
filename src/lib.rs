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

// #[wasm_bindgen]
// pub fn greet(base_compressed: &str) -> String {
//     let compressed_bytes = base64::decode(&base_compressed).unwrap();
//     let mut d = ZlibDecoder::new(&compressed_bytes[..]);
//     let mut s = String::new();
//     d.read_to_string(&mut s).unwrap();
//     return s;
// }

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
