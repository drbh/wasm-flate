
# Contributing

## Rough Roadmap

- [X] Compile compression to WASM  
- [X] Build useful functions for compression  
- [X] Compile useful functions to WASM  
- [X] Publish NPM package of WASM files and JS shim  
- [X] Add new functions for u8Array support  
- [X] Add basic API docs  
- [X] Write short medium article  
- [X] Compare with Native JS example  
- [X] Add node server side example  
- [X] Add Python server side example  
- [X] Make logo for lib based on WASM colorway  
- [ ] Deploy to WAPM  
- [ ] Add Golang example  
- [ ] Add benchmarking suite   
- [ ] Deploy multi file example  
- [ ] Release solid roadmap  
- [ ] Releae update schedule  
- [ ] Find partner for case study  



## Building with Rust 🦀🕸️ 

In order to build the wasm files with Rust, you'll need to clone the repo and run `wasm-pack` with `nodejs` as the target. This will create a set of files in `pkg` that can be used as a node module. 

```
git clone https://github.com/drbh/wasm-flate.git
cd wasm-flate
wasm-pack build --target nodejs
```

You should have the following new files  
```
pkg/    
├── LICENSE-APACHE    
├── LICENSE-MIT    
├── README.md    
├── wasm-flate.d.ts    
├── wasm-flate.js    
├── wasm-flate_bg.d.ts    
├── wasm-flate_bg.js    
├── wasm-flate_bg.wasm    
└── package.json    
```

