let
  pkgs = import <nixpkgs> { };
  rustDirShell = import ./rust/shell.nix;
in pkgs.mkShell rec {
  ANDROID_SDK_ROOT = rustDirShell.ANDROID_SDK_ROOT;
  buildInputs = rustDirShell.buildInputs ++ (with pkgs; [
    jdk14
  ]);
}
