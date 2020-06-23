# Piet Android Demo

This is a Demo app that uses [Druid](https://github.com/xi-editor/druid/) and
[piet-android](https://github.com/marcopolo/piet).

## Screenshot
![android-screenshot](https://marcopolo.keybase.pub/android-druid.png)

## Running

### Quickstart 1-liner (Requires Nix)

```nix-shell --run './gradlew installDebug'```

### Setup

* Install Rust and cargo.
  * Add android specific targets: `rustup target add aarch64-linux-android
    armv7-linux-androideabi i686-linux-android x86_64-linux-android`

#### With Android Studio

* Install Android Studio
  * Install the Android SDK and NDK
* Hit the Run button.

#### With Nix

With Nix you can have a working setup from the command line without requiring
any more setup.
* Install Nix: https://nixos.org/download.html (Used to provide the android
  SDK/NDK)
* Install direnv: https://direnv.net/ (Used to setup environment variables)
  * Or just run inside of `nix-shell`. i.e. `nix-shell --run 'cargo check
    --target x86_64-linux-android'`
