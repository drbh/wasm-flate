from wasmer import Instance

wasm_bytes = open(
    "/Users/drbh2/wapm_packages/drbh/flate@0.1.0/wasm_flate_bg.wasm",
    "rb"
).read()

instance = Instance(wasm_bytes)


# instance has no exports!