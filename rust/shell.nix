let
  # Get the mozilla nixpkg overlay for a convient rust installation
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/e912ed483e980dfb4666ae0ed17845c4220e5e7c.tar.gz");
  pkgs = import <nixpkgs> {
    config = { android_sdk.accept_license = true; };
    overlays = [ moz_overlay ];
  };
  androidComposition = pkgs.androidenv.composeAndroidPackages {
    includeNDK = true;
    ndkVersion = "21.0.6113669";
    buildToolsVersions = [ "29.0.2" ];
    platformVersions = [ "29" ];
    abiVersions = [ "arm64-v8a" "x86" "x86_64" ];
  };
  rust = (pkgs.rustChannels.stable.rust.override {
    targets =
      [ "i686-linux-android" "x86_64-linux-android" "aarch64-linux-android" ];
  });
in pkgs.mkShell rec {
  rustPath = rust;
  ANDROID_SDK_ROOT = androidComposition.androidsdk + "/libexec/android-sdk";
  buildInputs = [ androidComposition.androidsdk rust ];
  # TODO what is linux's path here?
  systemSpecificPath = if builtins.currentSystem == "x86_64-darwin" then
    "darwin-x86_64"
  else
    "linux-x86_64";
  ANDROID_CLANG = androidComposition.androidsdk
    + "/libexec/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/"
    + systemSpecificPath + "/bin/x86_64-linux-android29-clang";
  CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER = ANDROID_CLANG;
  CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER = ANDROID_CLANG;
}
