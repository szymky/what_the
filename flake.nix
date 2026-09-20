{
  description = "default rust dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        guiLibs = with pkgs; [

          alsa-lib
          udev

          wayland
          wayland-protocols
          libxkbcommon
          libGL
          vulkan-loader

          libX11
          libXcursor
          libXrandr
          libXi
          libXext
          libXrender
          libXau
          libXfixes
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            git
            ripgrep
            fd

            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
            pkg-config
            openssl
            gcc
          ] ++ guiLibs;

          env = {
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath guiLibs;
          };

          shellHook = ''
            echo "-> entered $(basename $PWD) dev shell"
            echo "-> rustc $(rustc --version)"
          '';
        };

        formatter = pkgs.nixpkgs-fmt;
      });
}
