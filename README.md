# Cross-platform [gfx-hal](https://github.com/gfx-rs/gfx) example project

Gfx-hal [quad example](https://github.com/gfx-rs/gfx/tree/master/examples/quad) running on Windows, Linux, Mac, iOS, ~~Android~~ (see #1)

## Desktop (Windows, Linux, Mac)

```bash
cd rust/game_desktop/
cargo run --features target
```

Where `target`: `mac` (Metal), `linux` (Vulkan), `pc_dx12` (DirectX 12) or `pc_dx11` (DirectX 11).

## iOS

1. Install [XcodeGen](https://github.com/yonaskolb/XcodeGen)
2. `cd` to the `ios` directory
3. Run XcodeGen
4. Open project in Xcode
5. Run example on device (with metal support)

```bash
cd ios/
xcodegen
```