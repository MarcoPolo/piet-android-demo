let
  pkgs = import <nixpkgs> { config = { android_sdk.accept_license = true; }; };
  androidComposition = pkgs.androidenv.composeAndroidPackages {
    includeNDK = true;
    ndkVersion = "21.0.6113669";
    buildToolsVersions = [ "29.0.2" ];
    platformVersions = [ "29" ];
    abiVersions = [ "arm64-v8a" "x86" "x86_64" ];
  };
in pkgs.mkShell rec {
  ANDROID_SDK_ROOT = androidComposition.androidsdk + "/libexec/android-sdk";
  buildInputs = [ androidComposition.androidsdk ];
  # TODO what is linux's path here?
  systemSpecificPath = if builtins.currentSystem == "x86_64-darwin" then "darwin-x86_64" else "linux-x86_64";
  ANDROID_CLANG = androidComposition.androidsdk
    + "/libexec/android-sdk/ndk-bundle/toolchains/llvm/prebuilt/" + systemSpecificPath + "/bin/x86_64-linux-android29-clang";
  CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER = ANDROID_CLANG;
  CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER = ANDROID_CLANG;
}
