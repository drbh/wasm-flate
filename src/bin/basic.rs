use wasm_flate::{zlib_encode, zlib_decode};
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let comp = zlib_encode(&args[1]);
	println!("{:?}", comp);
}