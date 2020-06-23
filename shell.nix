let
  pkgs = import <nixpkgs> { };
  rustDirShell = import ./rust/shell.nix;
in pkgs.mkShell rec {
  ANDROID_SDK_ROOT = rustDirShell.ANDROID_SDK_ROOT;
  buildInputs = rustDirShell.buildInputs ++ (with pkgs; [
    jdk14
  ]);
  RUST_ANDROID_GRADLE_CARGO_COMMAND = rustDirShell.rustPath + "/bin/cargo";
  RUST_ANDROID_GRADLE_RUSTC_COMMAND = rustDirShell.rustPath + "/bin/rustc";
}
