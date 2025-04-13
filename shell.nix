{ pkgs? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  # Tools needed to build the project
  nativeBuildInputs = [
    pkg-config
    # Add the wrapped mold linker
    mold-wrapped
    # Add clang, needed to drive mold via -fuse-ld=
    llvmPackages.clang # Or just `clang`
  ];

  # Libraries the project links against or needs at runtime
  buildInputs = [
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
