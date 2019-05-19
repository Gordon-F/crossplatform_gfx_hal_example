# Cross-platform [gfx-hal](https://github.com/gfx-rs/gfx) example project

Gfx-hal [quad example](https://github.com/gfx-rs/gfx/tree/master/examples/quad) running on Windows, Linux, Mac, iOS, Android.

## Desktop (Windows, Linux, Mac)

```bash
cd rust/game_desktop/
cargo run --features target
```

Where `target`: `mac` (Metal), `linux` (Vulkan), `pc_dx12` (DirectX 12) or `pc_dx11` (DirectX 11).

## iOS (Metal)

1. Install Rust iOS target: `rustup target add aarch64-apple-ios`
1. Install [XcodeGen](https://github.com/yonaskolb/XcodeGen)
1. `cd` to the `ios` directory
1. Run XcodeGen
1. Open project in Xcode
1. Run example on device (with metal support)

```bash
cd ios/
xcodegen
```

## Android (Vulkan)

1. Install Rust Android targets: `rustup target add armv7-linux-androideabi`
1. Install [cargo-apk](https://github.com/rust-windowing/android-rs-glue): `cargo install --git git@github.com:rust-windowing/android-rs-glue.git cargo-apk`
1. Install the OpenJDK (1.8)
1. Install the CMake
1. Install Android SDK
1. Install API level 24 support
1. Install [Android NDK 17c](https://developer.android.com/ndk/downloads/older_releases.html#ndk-17c-downloads) (why 17c? see [issue](https://github.com/rust-windowing/android-rs-glue/issues/208))
1. Install Gradle (4.6)
1. Set the environment variables `JAVA_HOME`, `NDK_HOME`, `ANDROID_HOME`, `GRADLE_HOME`.
1. `cd` to the `rust/game_android` directory
1. Run `cargo-apk build`
1. Install Vulkan [validation layers](https://developer.android.com/ndk/guides/graphics/validation-layer):
    1. `cd` to the `project_root/target/android-artifacts` directory
    1. `cp -fr $NDK_HOME/sources/third_party/vulkan/src/build-android/jniLibs/armeabi-v7a app/src/main/`
1. Run `gradle assembleDebug`
1. Run `gradle installDebug`
1. Run `adb logcat | grep RustAndroidGlueStdouterr`
1. Run example on connected device