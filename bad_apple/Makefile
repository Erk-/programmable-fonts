BASE_FONT=BadPfennig.ttf
TARGET_FONT=BadAppleWasm.ttf
WASM=bad_apple_bg.wasm


.PHONY: pkg/$(WASM)

$(TARGET_FONT): pkg/$(WASM) $(BASE_FONT)
	../bin/otfsurgeon -i $(BASE_FONT) add -o $(TARGET_FONT) Wasm < pkg/$(WASM)

pkg/$(WASM): src/*.rs
	wasm-pack build --target web
