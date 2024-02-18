{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
    pkgs.vulkan-loader
    pkgs.udev
    pkgs.alsaLib
    pkgs.wayland
    pkgs.libxkbcommon
  ]}"'';
  buildInputs = with pkgs; [
    mold
    pkg-config
    wayland
  ];
}
