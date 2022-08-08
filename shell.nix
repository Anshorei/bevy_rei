{ pkgs ? import <nixpkgs> {
    overlays = [
      (import "${fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz"}/overlay.nix")
    ];
  }
}:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    (fenix.complete.withComponents [
      "cargo" "rustc" "rustfmt" "rust-analyzer"
    ])
    gcc
  ];
  
  buildInputs = with pkgs; [
    rustfmt
    wayland
    xlibsWrapper
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11
    xorg.libX11.out
    xorg.libX11.dev.out
    vulkan-tools
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
    libxkbcommon.dev
    udev.dev
    alsa-lib.dev
    pkg-config
  ];
  
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-loader
  ]);
  
  RUST_BACKTRACE = 1;
}
